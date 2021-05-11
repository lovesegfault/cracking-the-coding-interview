use std::collections::HashMap;

fn check_permutation(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut counts: HashMap<char, (usize, usize)> = HashMap::with_capacity(a.len());

    for (a, b) in a.chars().zip(b.chars()) {
        counts
            .entry(a)
            .and_modify(|(count_a, _)| *count_a += 1)
            .or_insert((1, 0));
        counts
            .entry(b)
            .and_modify(|(_, count_b)| *count_b += 1)
            .or_insert((0, 1));
    }

    counts.values().all(|(count_a, count_b)| count_a == count_b)
}

fn main() {
    let a = "abcdefghijklmnopqrstuvwxyz";
    let b = "okpqduwnjfhvxcregsatzylmib";
    let c = "aaaaaaaaaaaaaaaaaaaaaaaaaa";
    let d = "abcde";

    print!("checking check_permutation: ");
    assert_eq!(check_permutation(a, b), true);
    assert_eq!(check_permutation(a, c), false);
    assert_eq!(check_permutation(a, d), false);
    println!("OK");
}
