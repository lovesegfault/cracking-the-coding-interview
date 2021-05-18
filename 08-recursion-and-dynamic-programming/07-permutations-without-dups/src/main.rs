// 1! -> 1
// "a"   -> "a"
// 2! = 2
// "ab"  -> "ab", "ba"
// 3! = 6
// "abc" -> "abc", "acb" "cab", "bac", "cba", "bca"
//
// ab -> cab, acb, abc
// ba -> cba, bca, bac
fn permutations(unique: &mut [char]) -> Vec<Vec<char>> {
    // used to give us a good hint on the amount of permutations, and avoid reallocations of the
    // result vec
    fn sterlings_approximation(n: u64) -> u64 {
        use std::f64::consts::{E, TAU};

        let x = n as f64;
        ((TAU * x).sqrt() * (x / E).powi(n as i32)) as u64
    }

    // inner recursion
    fn inner(perms: &mut Vec<Vec<char>>, s: &mut [char], substr: std::ops::Range<usize>) {
        if substr.is_empty() {
            perms.push(s.to_vec());
            return;
        }
        let start = substr.start;
        let end = substr.end;
        for i in substr {
            // this swapping lets us avoid reallocating the vec
            s.swap(start, i);
            inner(perms, s, (start + 1)..end);
            s.swap(start, i);
        }
    }

    let permutation_size = sterlings_approximation(unique.len() as u64) as usize;
    let mut perms = Vec::with_capacity(permutation_size);

    let l = unique.len();
    inner(&mut perms, unique, 0..l);

    perms
}

fn permutations_str(unique: &str) -> Vec<String> {
    let mut unique = unique.chars().collect::<Vec<_>>();
    permutations(&mut unique)
        .into_iter()
        .map(|vc| vc.into_iter().collect())
        .collect()
}

fn main() {
    dbg!(permutations_str("abc"));
}
