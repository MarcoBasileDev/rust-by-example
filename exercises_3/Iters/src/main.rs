fn print_elements(elements: &Vec<String>) {
    for element in elements {
        println!("{}", element);
    }
}

fn print_elements_iterator(elements: &Vec<String>) {
    elements.iter().for_each(|element| { println!("{}", element); });
}

fn main() {
    let colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
        String::from("yellow")
    ];

    print_elements(&colors);
    print_elements_iterator(&colors);
}
