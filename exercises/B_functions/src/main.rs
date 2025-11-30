// Silence some warnings
#![allow(unused_variables)]

fn main() {
    let number: f64 = 3.989;

    let number: i32 = convert_to_integer(number);
    inspect_integer(number);

    let answer = 42;
    println!("The answer is {}", answer);

    let sum = add(number, answer);
    println!("{} + {} = {}", number, answer, sum);

    let countdown: i32;
    if answer < 100 {
        countdown = 10;
    } else {
        println!("The answer is clearly wrong.");
        countdown = 0;
    }
    println!("The countdown begins at {}", countdown);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
fn inspect_integer(x: i32) {
    println!("The integer is {}", x);
}

fn convert_to_integer(num: f64) -> i32 {
    // For more information on using `as` to cast between numeric types, see:
    // https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast
    num.round() as i32
}