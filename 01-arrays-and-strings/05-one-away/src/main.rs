fn one_away(a: &str, b: &str) -> bool {
    // work around rust not letting us index into a string, assume ASCII
    let a = a.chars().collect::<Vec<_>>();
    let b = b.chars().collect::<Vec<_>>();
    // we can cheaply verify that no one operation will make strings with length differences >1 the
    // same. This is O(1).
    let len_delta = (a.len() as i64 - b.len() as i64).abs() as usize;
    if len_delta > 1 {
        return false;
    }

    let (shorter, longer) = if a.len() < b.len() { (a, b) } else { (b, a) };
    let (mut idx_s, mut idx_l) = (0, 0);
    let mut difference = false;

    while idx_s < shorter.len() && idx_l < longer.len() {
        // if we find a mismatch
        if shorter[idx_s] != longer[idx_l] {
            // and it's the second one
            if difference {
                return false;
            }
            // mark that we've hit our difference quota of 1
            difference = true;

            // if this could be fixed with a replace, we move the short pointer
            if shorter.len() == longer.len() {
                idx_s += 1;
            }
        } else {
            // move the short pointer when chars match
            idx_s += 1;
        }
        // the longer pointer always progresses
        idx_l += 1;
    }
    true
}

fn main() {
    print!("checking one_away: ");
    let a = "pale";
    let b = "pales";
    let c = "ple";
    let d = "bale";
    let e = "bake";
    assert_eq!(one_away(a, b), true);
    assert_eq!(one_away(a, c), true);
    assert_eq!(one_away(a, d), true);
    assert_eq!(one_away(a, e), false);
    println!("OK");
}
