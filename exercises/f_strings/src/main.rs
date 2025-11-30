#![allow(unused_mut, unused_assignments)]

// Q. What's the difference between a string literal and a borrowed string slice?
//
// A. A string literal is what is written in your source code. e.g. "this is a string literal",
// while a borrowed strings slice (&str) is the *type* of the string literal. So:
//
//   let my_name: &str = "Nathan";
//
// The variable my_name is a borrowed string slice, initialized by the string literal "Nathan".

fn main() {
    println!("\u{2728}");

    let mut favorite = String::new();
    favorite = "🍓".to_string();

    if favorite != "" {
        println!("Everyone's favorite fruit is: {favorite}");
    }

    let saying = "Now\nthe time\nfor all\ngreat man";
    println!("{saying}");

    println!("Challenge");
    let saying = r#"Now
the time
for all
great man"#;
    println!("{saying}");
}