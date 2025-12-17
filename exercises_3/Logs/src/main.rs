use std::fs;
use std::io::{Error};

fn extract_errors(text: &str) -> Vec<&str> {
    let split_text = text.split("\n");
    let mut result = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            result.push(line);
        }
    }
    result
}

fn main() {
    match fs::read_to_string("logs.txt") {
        Ok(content) => {
            let error_logs = extract_errors(&content);
            fs::write("errors.txt", error_logs.join("\n")).expect("failed to read logs.txt");
        },
        Err(e) => {
            println!("Failed to read file: {}", e);
        }
    }

    match divide(5.0, 0.0) {
        Ok(result) => {
            println!("Result: {}", result);
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    match verify_email("example@email".to_string()) {
        Ok(_result) => {},
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("can't divide by zero"))
    } else {
        Ok(a / b)
    }
}

// If in your Ok case you don't have anything to return, just return an empty tuple
fn verify_email(email: String) -> Result<(), Error> {
    if email.contains("@") {
        Ok(())
    } else {
        Err(Error::other("email must contain an @"))
    }
}