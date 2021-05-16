use std::ops::{Add, Range};

// O(n)
fn brute_force(data: &[u64]) -> Option<usize> {
    for (i, d) in data.iter().enumerate() {
        if i as u64 == *d {
            return Some(i);
        }
    }
    None
}

// O(log n)
fn binary_search(data: &[u64]) -> Option<usize> {
    fn inner(list: &[u64], range: Range<usize>) -> Option<usize> {
        let mid = (range.start + range.end) / 2;
        match list[mid].cmp(&(mid as u64)) {
            std::cmp::Ordering::Equal => Some(mid),
            std::cmp::Ordering::Less => inner(list, (mid + 1)..range.end),
            std::cmp::Ordering::Greater => inner(list, 0..mid),
        }
    }
    inner(data, 0..data.len())
}

fn binary_search_not_distinct(data: &[u64]) -> Option<usize> {
    fn inner(data: &[u64], range: Range<usize>) -> Option<usize> {
        let mid = (range.start + range.end) / 2;
        let mid_val = data[mid];
        if mid as u64 == mid_val {
            return Some(mid);
        }
        // search the left side
        let rightmost_possible = mid.saturating_sub(1).min(mid_val as usize);
        let left = inner(data, range.start..rightmost_possible);
        if left.is_some() {
            return left;
        }
        // seach the right side
        let leftmost_possible = mid.add(1).max(mid_val as usize);
        inner(data, leftmost_possible..range.end)
    }
    inner(data, 0..data.len())
}

fn main() {
    println!("Hello, world!");
}
