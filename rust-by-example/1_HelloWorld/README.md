# 1. Hello, world!

## Compile a binary
`$ rustc hello.rs`

## Block comments

It's possible to add '/' before the block comment to uncomment the whole block.
For example:

    /* <- add another '/' before the 1st one to uncomment the whole block
    
    println!("Now");
    println!("everything");
    println!("executes!");
    // line comments inside are not affected by either state
    
    // */

## Formatted print

Printing is handled by a series of macros defined in std::fmt some of which are:
- format!: write formatted text to String
- print!: same as format! but the text is printed to the console.
- println!: same as print! but a newline is appended.
- eprint!: same as print! but the text is printed to the standard error
- eprintln!: same as eprint! but a newline is appended.

## Display

fmt::Debug hardly looks compact and clean, so it is often advantageous to customize the output appearance. 
This is done by manually implementing fmt::Display, which uses the {} print marker.
There is no ideal style for all types and the std library doesn't presume to dictate one.
You have to implement your own style.

Implementing fmt::Display for a structure where the elements must each be handled sequentially is tricky. 
The problem is that each write! generates a fmt::Result. 
Proper handling of this requires dealing with all the results. 
Rust provides the ? operator for exactly this purpose.

## Formatting

We've seen that formatting is specified via a format string:

- format!("{}", foo) -> "3735928559"
- format!("0x{:X}", foo) -> "0xDEADBEEF"
- format!("0o{:o}", foo) -> "0o33653337357"
- The same variable (foo) can be formatted differently depending on which argument type is used: X vs o vs unspecified.

This formatting functionality is implemented via traits, and there is one trait for each argument type. 
The most common formatting trait is Display, which handles cases where the argument type is left unspecified: {} for instance.