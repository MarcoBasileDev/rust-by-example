fn next_language<'a>(languages: &'a [String], current: &str) -> &'a str {
    let mut found = false;

    for lang in languages {
        if found {
            return lang;
        }

        if lang == current {
            found = true;
        }
    }

    languages.last().unwrap()
}

fn last_language(languages: &[String]) -> &str {
    languages.last().unwrap()
}

fn longest_language<'a>(lang_a: &'a str, lang_b: &'a str) -> &'a str {
    if lang_a.len() >= lang_b.len() {
        lang_a
    } else {
        lang_b
    }
}

fn longest_language_in_list(languages: &[String]) -> &str {
    let mut longest = &languages[0];

    for lang in languages.iter().skip(1) {
        if lang.len() > longest.len() {
            longest = lang;
        }
    }

    longest

    //other approach
    // languages
    //     .iter()
    //     .map(|s| s.as_str())
    //     .max_by_key(|s| s.len())
    //     .unwrap()
}


fn main() {
    let languages = vec![
        String::from("rust"),
        String::from("go"),
        String::from("typescript"),
    ];

    let result = next_language(&languages, "go");
    println!("{}", result);

    let last = last_language(&languages);
    println!("{}", last);

    let longest = longest_language("go", "Rust");
    println!("{}", longest);

    let longest = longest_language_in_list(&languages);
    println!("{}", longest);

}
