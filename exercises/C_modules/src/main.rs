use C_modules::prelude::*;

fn main() {
    print!("Listening to animal {}: ", FIRST);
    dog();

    print!("Listening to animal {}: ", SECOND);
    cat();

    print!("Listening to animal {}: ", THIRD);
    fox();
}