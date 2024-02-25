fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut space_index = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            space_index = i;
            break;
        }
    }

    &s[(space_index + 1)..]
}

pub fn run() {
    let s = String::from("hello world");

    let hello = first_word(&s);
    let world = second_word(&s);

    println!("First word: {}, Second word: {}", hello, world);
}