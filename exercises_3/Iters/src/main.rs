fn print_elements(elements: &Vec<String>) {
    for element in elements {
        println!("{}", element);
    }
}

fn print_elements_iterator(elements: &[String]) {
    elements.iter().for_each(|element| { println!("{}", element); });
}

fn print_elements_two_times(elements: &[String]) {
    elements
        .iter()
        .map(|element| format!("{} {}", element, element))
        .for_each(|element| { println!("{}", element); });
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
    print_elements_iterator(&colors[1..3]);
    print_elements_two_times(&colors);
}
