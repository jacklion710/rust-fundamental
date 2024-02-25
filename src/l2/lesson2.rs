// lesson2.rs
// Function to demonstrate borrowing
fn calculate_length(s: &String) -> usize {
    s.len() // Return length of the string
}

// Function to demonstrate lifetimes
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Function to greet a user
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn run() {
    // Ownership example
    let s1 = String::from("Hello, Rust!");
    let s2 = s1; // s1's ownership is moved to s2
    println!("{}", s2); // s1 can no longer be used

    // Using the borrowing function
    let len = calculate_length(&s2);
    println!("The length of '{}' is {}.", s2, len);

    // Using the lifetimes function
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // Scalar and compound types
    let int_example: i32 = 100;
    let float_example: f64 = 3.14;
    let bool_example: bool = true;
    let char_example: char = 'R';
    let tuple_example: (i32, f64, u8) = (500, 6.4, 1);
    let array_example: [i32; 3] = [1, 2, 3];

    // Display scalar and compound types
    println!("Int: {}, Float: {}, Bool: {}, Char: {}", int_example, float_example, bool_example, char_example);
    println!("Tuple: ({}, {}, {})", tuple_example.0, tuple_example.1, tuple_example.2);
    println!("Array: {:?}", array_example);

    // Greeting function usage
    let greeting = greet("Rustacean");
    println!("{}", greeting);

    // Control flow examples
    // if-else
    let number = 7;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // loop
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            break;
        }
    }
    println!("Loop exited at count = {}", count);

    // while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    // for
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
