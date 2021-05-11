use std::collections::HashSet;

fn is_unique(s: &str) -> bool {
    let mut counts: HashSet<char> = HashSet::with_capacity(s.len());
    for c in s.chars() {
        match counts.insert(c) {
            true => continue,
            false => return false,
        }
    }
    true
}

// only works for ASCII
fn is_unique_no_data_structures(s: &str) -> bool {
    // a bitvec
    let mut seen_chars: u32 = 0;
    for c in s.chars() {
        let mask = (c as u32) - ('a' as u32);
        if seen_chars & (1 << mask) > 0 {
            return false;
        }
        seen_chars |= 1 << mask;
    }
    return true;
}

fn main() {
    let not_unique = "hmdsflkuwehflkjanflkiuewhflaehuhflwkfejhwleuinf";
    let unique = "abcdefghijklmnopqrstuvwxyz";

    print!("checking is_unique: ");
    assert_eq!(is_unique(not_unique), false);
    assert_eq!(is_unique(unique), true);
    println!("OK");

    print!("checking is_unique_no_data_structures: ");
    assert_eq!(is_unique_no_data_structures(not_unique), false);
    assert_eq!(is_unique_no_data_structures(unique), true);
    println!("OK");
}
