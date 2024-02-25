# Exploring Structs, Enums, and Pattern Matching in Rust

Welcome to Lesson 3 of our Rust series, where we delve into some of Rust's core features that make it both powerful and unique. This lesson covers structs, enums, and pattern matching, providing you with a solid understanding of how to use these features in your Rust programs.

## Structs in Rust

Structs, or structures, are custom data types that let you name and package together multiple related values that make up a meaningful group.

### Defining and Using a Struct

```rust
struct Book {
    title: String,
    author: String,
    pages: u32,
}
```

With a `Book` struct, we can create instances of this type to represent specific books:

```rust
let book = Book {
    title: String::from("The Rust Programming Language"),
    author: String::from("Steve Klabnik and Carol Nichols"),
    pages: 552,
};
println!("{} by {} has {} pages.", book.title, book.author, book.pages);
```

## Enums and Pattern Matching

Enums allow you to define a type by enumerating its possible variants. Pattern matching is a powerful feature in Rust that works hand in hand with enums, enabling you to execute code based on which variant an enum value is.

### Defining an Enum

```rust
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}
```

### Processing Enums with Pattern Matching

```rust
fn inspect(event: &WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click { x, y } => println!("clicked at x={}, y={}.", x, y),
    }
}
```

Using `match`, we can inspect a **`WebEvent`** and take action depending on its variant.

## Advanced Pattern Matching

Pattern matching isn't limited to enums. You can use it with primitives and structs, and even control flow!

### The match Statement

```rust
fn match_number(n: i32) {
    match n {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }
}
```

## Simplified Control Flow with `if let`

`if let` offers a convenient way to write a match that only cares about one of the cases:

```rust
fn if_let_example() {
    let some_u8_value = Some(3u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
```

## Understanding `if let`

The `if let` syntax lets you combine `if` and `let` into a less verbose way to handle values that match one pattern while ignoring the rest. It's particularly useful with enums that have multiple variants but you're only interested in one.

Traditional Match Statement
Normally, when you're using a `match` statement, you might have to cover all possible cases (or use `_` to ignore the rest), even if you're only interested in one:

```rust
match some_u8_value {
    Some(3) => println!("three"),
    _ => (), // Ignore all other values
}
```

### Simplified with `if let`

`if let` simplifies this pattern:

```rust
if let Some(3) = some_u8_value {
    println!("three");
}
```

Here's what happens in this `if let` statement:

**Pattern Matching: `Some(3)`** is the pattern we're interested in. `if let` checks if some_u8_value matches this pattern.

**Binding:** If `some_u8_value` is `Some(3)`, then the code inside the block executes. In this case, it prints `"three"`.

**Ignoring Other Cases:** Unlike a `match` where you need to explicitly handle other cases (or explicitly ignore them with `_`), `if let` automatically ignores all other cases.

## Conclusion

In this lesson, we've taken a closer look at Rust's structs, enums, and pattern matching. These features are integral to Rust's approach to type safety and pattern matching, enabling you to write concise, expressive, and safe code. Stay tuned for more lessons as we continue to explore Rust's powerful features.