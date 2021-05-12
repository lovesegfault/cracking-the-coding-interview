// you may only call this fn once
#[inline(always)]
pub fn is_substring(a: &str, b: &str) -> bool {
    b.contains(a)
}

pub fn is_rotation(a: &str, b: &str) -> bool {
    let a_l = a.len();
    let b_l = b.len();
    if a_l != b_l || a_l == 0 {
        return false;
    }

    let double = format!("{}{}", b, b);
    is_substring(a, &double)
}

fn main() {
    print!("checking is_rotation: ");
    let a = "waterbottle";
    let b = "erbottlewat";
    assert_eq!(is_rotation(a, b), true);
    println!("OK");
}
