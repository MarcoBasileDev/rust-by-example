#![allow(dead_code)]

// 1a: Create a `Colorful` trait with a single `color` method which takes an immutable reference to
// self and returns a String describing that item's color.
trait Colorful {
    fn color(&self) -> String;
}

trait EvenOdd {
    fn is_even(&self) -> bool;
}

// 1b. Implement the `Colorful` trait for the `Hat` struct:
// The `colorful` method of the `Colorful` trait implementation for the `Hat` struct should return:
// - "red" if the size is 0 through 5
// - "green" - if the size is 6 or 7
// - "blue" - for any other size

struct Hat {
    size: i32,
}

impl Colorful for Hat {
    fn color(&self) -> String {
        match self.size {
            0..=5 => "red".to_string(),
            6 | 7  => "green".to_string(),
            _ => "blue".to_string(),
        }
    }
}

impl Colorful for i32 {
    fn color(&self) -> String {
        if self.is_even() {
            "orange".to_string()
        } else {
            "purple".to_string()
        }
    }
}

impl EvenOdd for i32 {
    fn is_even(&self) -> bool {
        *self % 2 == 0
    }
}

fn main() {
    let small_hat = Hat { size: 2 };
    let medium_hat = Hat { size: 7 };
    let large_hat = Hat { size: 100 };

    describe_three_hats(&small_hat, &medium_hat, &large_hat);

    // 2. Implement the Colorful trait for the type i32. The `colorful` method for an i32 should
    // return these String values:
    //   - "orange" - If the number is even (see hint)
    //   - "purple" - If the number is odd

    println!("4 is {}", 4.color());
    println!("5 is {}", 5.color());

    // 3. Let's replace the is_even function with a trait implementation!
    // - Comment out the is_even function at the bottom of this file so you can't use it anymore.
    // - Create a trait named `EvenOdd` with a method `is_even`. It should take an immutable
    //   reference to self and return a bool.
    // - Implement EvenOdd for i32. You can copy-and-paste the logic from the is_even function that
    //   you commented out earlier.
    // - Refactor the `colorful` method for `i32` to use the is_even method.

    // Challenge: Write a generic function named `fortune` that takes anything that implements the
    // Colorful trait and prints out the color in some message (for example: "The color I see in
    // your future is ...").

    fortune(small_hat);
    fortune(2);
}

fn fortune<T: Colorful>(item: T) {
    println!("The color I see in your future is {}!", item.color());
}

// A function used by some provided code.
fn describe_three_hats(hat1: &Hat, hat2: &Hat, hat3: &Hat) {
    for hat in [hat1, hat2, hat3] {
        let largeness = if hat.size < 3 {
            "small"
        } else if hat.size < 9 {
            "medium"
        } else {
            "large"
        };
        println!("The {} hat is {}", largeness, hat.color());
    }
}

// You can use this function to check if a number is even (true) or odd (false).
// fn is_even(number: i32) -> bool {
//     number % 2 == 0
// }