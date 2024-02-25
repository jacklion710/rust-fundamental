// lesson3.rs
// Define a struct named `Book`
struct Book {
    title: String,
    author: String,
    pages: u32,
}

// Define an enum named `WebEvent`
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

// Function to process a web event
fn inspect(event: &WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click { x, y } => println!("clicked at x={}, y={}.", x, y),
    }
}

// Function to match a number with different conditions
fn match_number(n: i32) {
    match n {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }
}

// Function to demonstrate `if let` with Option<u8>
fn if_let_example() {
    let some_u8_value = Some(3u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}

pub fn run() {
    // Struct usage example
    let book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        pages: 552,
    };
    println!("{} by {} has {} pages.", book.title, book.author, book.pages);

    // Enum usage and pattern matching example
    let events = [
        WebEvent::KeyPress('x'),
        WebEvent::Paste(String::from("my text")),
        WebEvent::Click { x: 20, y: 80 },
        WebEvent::PageLoad,
        WebEvent::PageUnload,
    ];

    for event in events.iter() {
        inspect(event);
    }

    // `match` example
    match_number(2);
    match_number(13);
    match_number(42);

    // `if let` example
    if_let_example();
}
