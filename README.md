# rcsh

**R**aw **C** **Sh**ell is a minimalist shell with no built in commands. You write entirely in C code and use `return;` to execute your code.

Unlike that silly `csh` where certain features and functions are added to make it more appropriate as a shell, this literally just pipes what you type into it into gcc.

Have fun!

## principles of rcsh
- Everything you type is placed into a buffer. Upon typing `return;` into the shell, the buffer is placed into an `int main()` function, executed, and then cleared, allowing you to write a new block of code.
- The buffer is updated every time you hit enter. You cannot modify previously entered lines. When your code is executed and cleared, you cannot edit it, you must type it out again.
- `exit(_);` closes the shell. Your program must compile successfully for the function to have any effect.
- In the resulting code given to gcc, every C89 header is imported before that main function.
- If you type your own `#include` directives into the shell, they are handled specially and placed above the main function. When your code is cleared on `return;`, the `#include` directives remain.
- `Ctrl-L` clears the current screen and also whatever program you were typing.