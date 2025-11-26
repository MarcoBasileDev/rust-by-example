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