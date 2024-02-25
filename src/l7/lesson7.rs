// lesson7.rs
use std::fmt::Display;

// Understanding traits and defining shared behavior
pub trait Summary {
    fn summarize(&self) -> String;
}

#[allow(dead_code)] // ELABORATE ON THIS
struct Article {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// Exploring lifetimes further for advanced memory safety
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() { x } else { y }
}

// Trait objects for dynamic dispatch
fn print_summaries(items: &[&dyn Summary]) {
    for item in items {
        println!("Summary: {}", item.summarize());
    }
}

pub fn run() {
    let article = Article {
        headline: String::from("Rust Language Announces Async/Await"),
        location: String::from("Internet"),
        author: String::from("Mozilla"),
        content: String::from("Today, the Rust programming language announced ..."),
    };

    println!("Article summary: {}", article.summarize());

    let text1 = String::from("long text");
    let text2 = String::from("short text");
    let announcement = "Now comparing text lengths";
    println!(
        "The longest text is '{}'",
        longest_with_an_announcement(text1.as_str(), text2.as_str(), announcement)
    );

    // Dynamic dispatch with trait objects
    let articles = [&article as &dyn Summary];
    print_summaries(&articles);
}
