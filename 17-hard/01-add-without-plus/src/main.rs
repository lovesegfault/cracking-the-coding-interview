fn add_recursive(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    let sum = a ^ b;
    let carry = (a & b) << 1;
    add_recursive(sum, carry)
}

fn add_iteratively(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let sum = a ^ b;
        let carry = (a & b) << 1;
        a = sum;
        b = carry;
    }
    a
}

fn main() {
    use itertools::Itertools;

    (-100..100)
        .permutations(2)
        .map(|p| (p[0], p[1]))
        .for_each(|(a, b)| {
            assert_eq!(a + b, add_iteratively(a, b));
            assert_eq!(a + b, add_recursive(a, b));
        });
}
