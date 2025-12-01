fn main() {
    // Here are some String variables to use. There are many ways to create a String!
    let item = String::from("socks");
    let animal = "fox".to_string();
    let container = "box".to_owned();
    let material = "rocks".into(); // .into() works as long as you use the value as a String later

    // 1. Create a Vec<String> named `things` and move all the strings above into it.

    let mut things: Vec<String> = vec![item, animal, container, material];
    println!("{:?}", things); // `:?` means "the debug representation"

    // 2. Print out the length of the `things` vector using the `len` method.

    println!("things has a length of {}", things.len());

    // 3. We want to use the `animal` variable in the (commented-out) code below, but we cannot
    // because the value has been moved into `things`. Uncomment the code below and change it to use
    // array indexing (with square brackets []) to index into `things` to access the `fox` String.

    println!("What does the {} say?", things[1]);

    // 4. Sort `things` by calling the `sort` method. The variable needs to be mutable for this to
    // compile without errors.

    things.sort();
    println!("Sorted values: {things:?}");

    // 5. Use a `for` loop to print out each item in `things`. It is okay to consume `things`, since
    // we won't be using it anymore after this.

    for item in things {
        println!("{}", item);
    }

    // Challenge: Create a vector named `buffer` containing 1024 zeroes using the `vec!` macro.

    let mut buffer: Vec<i16> = vec![0; 1000];

    // Challenge 2: Use a `for` loop and array indexing to change each entry in `buffer` to be its
    // index value multiplied by 2.

    for (i, x) in buffer.iter_mut().enumerate() {
        *x = (i as i16) * 2;
    }

    println!("Here's a buffer full of even values: {buffer:?}");
}