use std::collections::HashMap;

pub fn run() {
    // Working with Vec
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    for i in &v {
        println!("Vec item: {}", i);
    }

    // Working with String
    let mut s = String::new();
    s.push_str("hello");
    s.push(' ');
    s.push_str("world");
    println!("String: {}", s);

    // Working with HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    for (key, value) in &scores {
        println!("{} team's score: {}", key, value);
    }

    // Demonstrate generic function
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    // Example struct that implements the `Summary` trait
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };
    notify(article);

    // Example using lifetimes
    let string1 = String::from("long string is long");
    let result = longest(string1.as_str(), "short");
    println!("The longest string is {}", result);
}

// Generic function to find the largest in a list
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Trait definition
pub trait Summary {
    fn summarize(&self) -> String;
}

// Implementing trait for a struct
#[allow(dead_code)]
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// Function accepting anything that implements the Summary trait
fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

// Function demonstrating lifetimes
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
