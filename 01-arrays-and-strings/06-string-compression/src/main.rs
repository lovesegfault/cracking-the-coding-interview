fn string_compression(s: &str) -> String {
    let mut compressed = String::with_capacity(s.len());

    let mut repeat_count = 0;
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        repeat_count += 1;

        match chars.peek() {
            Some(x) if *x == c => continue,
            _ => {
                compressed.push_str(&format!("{}{}", c, repeat_count));
                repeat_count = 0;
            }
        }
    }

    if compressed.len() > s.len() {
        s.to_owned()
    } else {
        compressed
    }
}

fn main() {
    print!("checking string_compression: ");
    let a = "aabcccccaaa";
    assert_eq!(string_compression(a), "a2b1c5a3");
    println!("OK");
}
