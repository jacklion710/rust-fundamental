use std::env;

mod l1;
mod l2;
mod l3;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "lesson1" => l1::lesson1::run(),
            "lesson2" => l2::lesson2::run(),
            "lesson3" => l3::lesson3::run(),
            // Add more cases as more lessons are added
            _ => println!("Lesson not found"),
        }
    } else {
        println!("Please specify a lesson to run (e.g., 'cargo run lesson1')");
    }
}
