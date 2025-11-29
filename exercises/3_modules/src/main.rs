use modules::{FIRST, SECOND, THIRD};
use modules::sound;

fn main() {
    print!("Listening to animal {}: ", FIRST);
    sound::dog();

    print!("Listening to animal {}: ", SECOND);
    sound::cat();

    print!("Listening to animal {}: ", THIRD);
    sound::fox();
}

// Challenge 2
//
// Create an `animal::prelude` module which re-exports all of the constants and functions of the
// library. (A real library would only re-export the most commonly-used items in its prelude.)
//
// Change your `use` statement(s) in main.rs to just `use animal::prelude::*`
//
// Hint: You will need `pub use` to re-export an item, for more details see:
// https://doc.rust-lang.org/reference/items/use-declarations.html#use-visibility