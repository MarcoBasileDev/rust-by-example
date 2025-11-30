#![allow(unused_mut, unused_variables)]

fn main() {
    let mut count = 0;
    let mut bunnies = 2;

    loop {
        count += 1;
        bunnies *= 2;

        if bunnies > 500 { break }
    }

    println!(
        "Bunnies doubled {} times before there were more than 500",
        count
    );

    let mut sum = 0;

    for num in 7..=23 {
        sum += num;
    }

    println!("The sum is {}", sum);

    let mut fives: Vec<i32> = vec![];
    let mut number = 5;

    while fives.len() < 12 {
        fives.push(number);
        number += 5;
    }

    println!("Here are the first 12 multiples of 5: {:?}", fives);

    // 4. Use `if`, `else if` and `else` inside the `for` loop below to do the following:
    //
    // - If the number is 0, then add 7 to `total`
    // - If the number is 1 or 2 then add 30 to `total`
    // - If the number is anything else, subtract 5 from `total`
    //
    // Hint: The total should be 52

    let mut total = 0;
    let numbers = vec![0, 1, 2, 3, 4, 5];
    for number in numbers {
        // (write your `if/else` expression here)
    }

    println!("The total is {}", total);

    // Challenge: Change the implementation of your answers to #1-#3 as follows:
    //
    // - Change #1 to use `while`
    // - Change #2 to use `loop`
    // - Change #3 to use `for` and a range (multiply the range value by 5 inside your loop before
}