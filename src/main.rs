use std::error::Error;
use std::process::Command;
use std::env;
use std::str;
use regex::Regex;
use lazy_static::lazy_static;
use pancurses::{initscr};
use pancurses::Input::{Character, KeyBackspace};
lazy_static!{
    pub static ref DIR_WRITE: Regex = Regex::new("dir = \"(.*?)\";(\n|)").unwrap();
    pub static ref EXIT_CALL: Regex = Regex::new("exit\\(([0-9]{1,3})\\);(\n|)").unwrap();
    pub static ref INCLUDE_PATH: Regex = Regex::new("#include (.*?);(\n|)").unwrap();
}

const CTRL_C: char = 3 as char;

const CTRL_L: char = 12 as char;


fn main() -> Result<(), Box<dyn Error>> {
    // curses instances
    let window = initscr();
    window.refresh();
    window.keypad(true);
    window.scrollok(true);
    pancurses::noecho();
    pancurses::cbreak();

    // set up the directory variable.
    let mut dir = env::current_dir()?.as_os_str().to_string_lossy().to_string();

    // set up the buffer for the C code to compile
    let mut greaterbuffer = String::new();
    let mut includebuffer = String::new();
    
    // some booleans for controlling the main loop
    let mut last_call = false; // whether or not this is the last function to execute; relevant to exit();
    let mut looping = true; // whether or not to keep going.

    let mut prefix = format!("{} > ",dir);
    
    // print the directory prompt
    //println!("\x1b[1m{}>\x1b[0m",dir);

    // on the main thread we want to listen to inputs
    let mut buffer = String::new();
    window.addstr(prefix);
    while looping {
        window.refresh();
        // get pressed character.
        if let Some(a) = window.getch() {
            prefix = format!("{} > ",dir);
            match a {
                KeyBackspace | Character('\u{7f}') => {
                    if buffer.len() >= 1 {
                        buffer.truncate(buffer.len()-1);
                        window.mv(window.get_cur_y(), window.get_cur_x()-1);
                        window.delch();
                    }
                },
                Character(CTRL_C) => {

                },
                Character(CTRL_L) => {
                    window.clear();
                    window.addstr(prefix);
                    buffer.truncate(0);
                }
                Character('\n') => {
                    // intercept writes to the dir variable and update our local variable
                    if DIR_WRITE.is_match(buffer.as_str()) {
                        let path = DIR_WRITE.replace_all(&buffer,"$1").to_string();
                        dir = path;
                    }
                    // intercept writes to the exit call and set our local last_call variable if it's gonna be called.
                    if EXIT_CALL.is_match(buffer.as_str()) {
                        last_call = true;
                    }
                    if INCLUDE_PATH.is_match(buffer.as_str()) {
                        includebuffer.push_str(&buffer);
                    } else {
                        greaterbuffer.push_str(&buffer);
                    }
                    
                    window.mv(window.get_cur_y()+1, prefix.len() as i32);
                    // if the buffer currently contains "return", execute the function
                    if buffer.starts_with("return") {
                        let code = format!(r"
                        #include <assert.h>;
                        #include <ctype.h>;
                        #include <errno.h>;
                        #include <float.h>;
                        #include <limits.h>;
                        #include <locale.h>;
                        #include <math.h>;
                        #include <setjmp.h>;
                        #include <signal.h>;
                        #include <stdarg.h>;
                        #include <stdbool.h>;
                        #include <stddef.h>;
                        #include <stdio.h>;
                        #include <stdlib.h>;
                        #include <string.h>;
                        #include <time.h>;
                        {inc}
                        char * dir = {dir};
                        int main() {{
                            {buf}
                        }};",inc=includebuffer,dir=format!("\"{}\"",dir),buf=greaterbuffer);
                        // compile the code.
                        let output = Command::new("sh")
                                .arg("-c")
                                .arg(format!("echo '{}' | gcc -x c -O -w -o tmp - && chmod +x tmp && ./tmp && rm tmp",code))
                                .output()
                                .expect("failed to execute gcc");
                        window.mv(window.get_cur_y(), 0);
                        let out = str::from_utf8(&output.stdout)?;
                        let err = str::from_utf8(&output.stderr)?;
                        window.addstr(err);
                        window.addstr(out);
                        window.mv(window.get_cur_y(), 0);
                        window.addstr(prefix);
                        greaterbuffer.truncate(0);
                        if last_call && err == "" {
                            looping = false;
                        }
                    } 
                    buffer.truncate(0);
                }
                Character(a) => {
                    window.addch(a);
                    buffer.push(a);
                },
                _ => {}
            }
        }
    };
    window.delwin();
    Ok(())
}
