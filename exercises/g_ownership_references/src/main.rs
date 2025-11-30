#![allow(unused_mut, unused_variables)]

fn inspect(arg: &String) {
    if arg.ends_with("s") {
        println!("{} is plural", arg);
    } else {
        println!("{} is singular", arg);
    }
}

fn change(some_string: &mut String) {
    if !some_string.ends_with("s") {
        some_string.push_str("s");
    }
}

fn eat(arg: String) -> bool {
    arg.starts_with("b") && arg.contains("a")
}

fn bedazzle(s: &mut String) {
    *s = "sparkly".to_string();
}

fn main() {
    // This fancy stuff either gets the first argument as a String, or prints
    // usage and exits if an argument was not supplied to the program.
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });

    inspect(&arg);

    change(&mut arg);
    println!("I have many {}", arg);


    if eat(arg) {
       println!("Might be bananas");
    } else {
       println!("Not bananas");
    }

    let mut material = "mud".to_string();
    println!("This material is just `{}`.", material);
    bedazzle(&mut material);
    println!("Wow! Now the material is `{}`!", material);
}