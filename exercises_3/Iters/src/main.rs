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

fn shorten_strings(elements: &mut [String]) {
    elements.iter_mut().for_each(|element| { element.truncate(1)});
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements
        .iter()
        .map(|element| element.to_uppercase())
        .collect()
}

fn in_place_to_uppercase(elements: &mut Vec<String>) {
    elements.iter_mut().for_each(|element| { element.make_ascii_uppercase(); });
}

fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a.into_iter().for_each(|element| { vec_b.push(element); });
}

fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements
        .iter()
        .map(|element| { element.chars().map(|c| c.to_string()).collect() })
        .collect()
}

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
        String::from("yellow")
    ];

    print_elements(&colors);
    print_elements_iterator(&colors);
    print_elements_iterator(&colors[1..3]);
    print_elements_two_times(&colors);

    shorten_strings(&mut colors);
    print_elements_iterator(&mut colors);

    let uppercase_vec = to_uppercase(&colors);
    print_elements(&uppercase_vec);

    in_place_to_uppercase(&mut colors);
    print_elements_iterator(&mut colors);

    let mut destination = vec![];
    move_elements(uppercase_vec, &mut destination);
    println!("Destination {:#?}", &destination);

    let exploded = explode(&destination);
    println!("Exploded {:?}", &exploded);
}
