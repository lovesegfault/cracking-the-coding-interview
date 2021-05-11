use std::collections::{hash_map::Entry, HashMap};

// Sentences are permutations of one another if they have the same character count.
// Sentences are palindromes when they have for i=0..len; j=len..0  w[i]==w[j]
// A sentences is a permutation of a palindrome when every character within it appears an even
// number of times, and _at most one_ character appears exactly once.
fn palindrome_permutation(s: &str) -> bool {
    let mut char_counts: HashMap<char, usize> = HashMap::with_capacity(s.len());
    let mut odd_chars: usize = 0;
    for c in s
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_ascii_lowercase())
    {
        match char_counts.entry(c) {
            Entry::Occupied(mut entry) => {
                let new_count = entry.get() + 1;
                if new_count % 2 == 0 {
                    odd_chars -= 1;
                } else {
                    odd_chars += 1;
                }
                entry.insert(new_count);
            }
            Entry::Vacant(entry) => {
                entry.insert(1);
                odd_chars += 1;
            }
        }
    }
    odd_chars == 0 || odd_chars == 1
}

fn palindrome_permutation_2(s: &str) -> bool {
    let mut char_counts: HashMap<char, usize> = HashMap::with_capacity(s.len());
    for c in s
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_ascii_lowercase())
    {
        match char_counts.entry(c) {
            Entry::Occupied(mut entry) => {
                let new_count = entry.get() + 1;
                entry.insert(new_count);
            }
            Entry::Vacant(entry) => {
                entry.insert(1);
            }
        }
    }
    char_counts
        .values()
        .filter(|&&count| count % 2 == 1)
        .count()
        <= 1
}

fn palindrome_permutation_3(s: &str) -> bool {
    let s_iter = s
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_ascii_lowercase());

    let mut odd_bitvec: usize = 0;

    for c in s_iter {
        let c_idx = c as usize - 'a' as usize;
        let bit_mask = 1 << c_idx;
        if odd_bitvec & bit_mask == 0 {
            odd_bitvec |= bit_mask;
        } else {
            odd_bitvec &= !bit_mask;
        }
    }
    odd_bitvec.count_ones() == 0 || odd_bitvec.count_ones() == 1
}

fn main() {
    let a = "Tact Coa";
    let b = "abc";
    let c = "abcba";
    print!("checking palindrome_permutation: ");
    assert_eq!(palindrome_permutation(a), true);
    assert_eq!(palindrome_permutation(b), false);
    assert_eq!(palindrome_permutation(c), true);
    println!("OK");

    print!("checking palindrome_permutation_2: ");
    assert_eq!(palindrome_permutation_2(a), true);
    assert_eq!(palindrome_permutation_2(b), false);
    assert_eq!(palindrome_permutation_2(c), true);
    println!("OK");

    print!("checking palindrome_permutation_3: ");
    assert_eq!(palindrome_permutation_3(a), true);
    assert_eq!(palindrome_permutation_3(b), false);
    assert_eq!(palindrome_permutation_3(c), true);
    println!("OK");
}
