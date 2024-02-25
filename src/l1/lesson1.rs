// lesson1.rs
pub fn run() {
    // Introduction to printing to the console.
    println!("Hello world from Lesson 1");

    // Demonstrating immutability vs. mutability.
    let x = 5; // immutable
    println!("The value of x is: {}", x);
    let mut y = 5; // mutable
    println!("The value of y is: {}", y);
    y = 6; // This is allowed because y is mutable.
    println!("The value of y after modification is: {}", y);

    // Variable type annotations.
    let z: i32 = 5;
    println!("The value of z is: {}", z);
    let is_true: bool = true;
    println!("The value of is_true is: {}", is_true);

    // Function usage.
    let result = add_two(z);
    println!("The result of adding two to z is: {}", result);

    // Control flow with if.
    if z > 5 {
        println!("z is greater than 5");
    } else {
        println!("z is not greater than 5");
    }
}

// Defining a function that adds two to a number.
fn add_two(x: i32) -> i32 {
    x + 2
}