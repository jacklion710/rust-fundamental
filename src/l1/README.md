# Rust Programming for Beginners: Lesson 1

Welcome to the first lesson in our Rust programming series! Rust is a modern systems programming language focused on safety, speed, and concurrency. It's a great tool for building efficient, high-performance software. In this lesson, we'll cover the basics to get you started: setting up Rust, writing your first Rust program, understanding variables and mutability, and learning about functions.

## Setting Up Rust

Before we dive into the language, you'll need to set up your Rust development environment. Rust's installation is straightforward, thanks to `rustup`, the Rust toolchain installer.

1. **Install Rust**: Visit [the official Rust website](https://www.rust-lang.org/tools/install) and follow the instructions to install Rust on your machine.
2. **Verify Installation**: Open a terminal and run `rustc --version` and `cargo --version` to ensure Rust and Cargo (Rust's package manager and build system) are correctly installed.

## Writing Your First Rust Program

With Rust installed, it's time to write your first Rust program: the classic "Hello, World!".

1. **Create a New Project**: Run `cargo new hello_world` in your terminal. This command creates a new Rust project named `hello_world`.
2. **Navigate to Your Project**: Change into your project directory with `cd hello_world`.
3. **Understand the Project Structure**: Cargo generates a basic project structure:
   - `Cargo.toml`: The manifest file for Rust projects.
   - `src/main.rs`: The main source file where your Rust code lives.
4. **Edit `main.rs`**: Open `src/main.rs` in your text editor, and you'll see the "Hello, World!" program already written:
```rust
fn main() {
    println!("Hello, World!");
}
```

5. **Run Your Program:** Back in the terminal, run `cargo run` from your project directory. Cargo will compile and execute your program, printing "Hello, World!" to the console.

## Understanding Variables and Mutability

Rust is a statically typed language, which means it checks the types of all variables at compile time. However, Rust has a powerful feature known as "type inference," which allows you to often omit explicit type annotations.

* **Immutable Variables:** By default, variables in Rust are immutable. Once a value is bound to a name, you cannot change that value.

```rust
let x = 5;
```

* **Mutable Variables:** To make a variable mutable, use the `mut` keyword.

```rust
let mut y = 5;
y = 10; // This is allowed because `y` is mutable.
```

## Functions in Rust

Functions are declared using the `fn` keyword, followed by the function's name, parameters, and return type. Here's a simple function that adds two to a number:

```rust
fn add_two(x: i32) -> i32 {
    x + 2
}
```

* **Calling Functions:** You can call this function and use its return value in your program.

```rust
let number = 5;
let result = add_two(number);
println!("{} plus two is {}", number, result);
```

## Recap

Congratulations! You've set up your Rust environment, written your first program, and learned about variables, mutability, and functions. Rust has much more to offer, and we'll dive deeper in upcoming lessons. Stay tuned!