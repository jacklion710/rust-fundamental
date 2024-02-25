// lesson5.rs
use std::fs::File;
use std::io::{self, Read};

// Function to demonstrate using `Option` for handling potential division by zero
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

// Function to demonstrate using `Result` for handling possible file reading errors
fn read_file_contents(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn run() {
    // Example of handling division with `Option`
    let division_result = divide(10.0, 0.0);
    match division_result {
        Some(quotient) => println!("Quotient is {}", quotient),
        None => println!("Cannot divide by 0"),
    }

    // Example of handling file reading with `Result`
    let file_path = "hello.txt"; // Adjust the file path as needed
    match read_file_contents(file_path) {
        Ok(contents) => println!("File contents of '{}': {}", file_path, contents),
        Err(e) => println!("Error reading file '{}': {:?}", file_path, e),
    }

    // Additional demonstration of using `Option` and `Result` together
    let division_result = divide(10.0, 2.0);
    if let Some(quotient) = division_result {
        println!("Result: {}", quotient);
    } else {
        println!("Failed to divide.");
    }

    // Handling file read for a possibly nonexistent file
    let non_existent_file_path = "nonexistent_file.txt"; // Adjust as needed
    match read_file_contents(non_existent_file_path) {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("Failed to read '{}': {}", non_existent_file_path, e),
    }
}
