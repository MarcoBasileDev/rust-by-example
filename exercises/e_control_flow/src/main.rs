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

    count = 0;
    bunnies = 2;

    // while variants
    while bunnies < 500 {
        count += 1;
        bunnies *= 2;
    }

    println!(
        "Bunnies doubled {} times before there were more than 500 - while variant",
        count
    );

    let mut sum = 0;

    for num in 7..=23 {
        sum += num;
    }

    println!("The sum is {}", sum);

    // loop variant
    sum = 0;
    let mut num = 7;

    loop {
        sum += num;

        if num == 23 {
            break;
        }

        num += 1;
    }

    println!("The sum is {} - loop variant", sum);

    let mut fives: Vec<i32> = vec![];
    let mut number = 5;

    while fives.len() < 12 {
        fives.push(number);
        number += 5;
    }

    println!("Here are the first 12 multiples of 5: {:?}", fives);

    let mut fives: Vec<i32> = vec![];
    number = 5;

    for i in  1..=12{
        fives.push(number * i);
    }

    println!("Here are the first 12 multiples of 5: {:?} - for variant", fives);

    let mut total = 0;
    let numbers = vec![0, 1, 2, 3, 4, 5];

    for number in numbers {
        if number == 0 {
            total += 7
        } else if number == 1 || number == 2 {
            total += 30
        } else {
            total -= 5
        }
    }

    println!("The total is {}", total);
}