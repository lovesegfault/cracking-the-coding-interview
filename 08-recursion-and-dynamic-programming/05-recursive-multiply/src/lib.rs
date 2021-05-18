use std::collections::HashMap;

/*
 * a * b = a + a + ... + a b times.
 *
 */
pub fn recursive_mul(a: u64, b: u64) -> u64 {
    match b {
        0 => 0,
        1 => a,
        n => a + recursive_mul(a, n - 1),
    }
}

// the same as above, but we try to minimize the recursion
pub fn recursive_mul2(mut a: u64, mut b: u64) -> u64 {
    fn inner(larger: u64, smaller: u64) -> u64 {
        match smaller {
            0 => 0,
            1 => larger,
            n => larger + inner(larger, n - 1),
        }
    }
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }
    inner(a, b)
}

// uses a trick to only multiply half of numbers that are divisible by two
pub fn recursive_mul3(a: u64, b: u64) -> u64 {
    #[inline(always)]
    fn inner_smaller(mut a: u64, mut b: u64) -> u64 {
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        inner(a, b)
    }

    fn inner(larger: u64, smaller: u64) -> u64 {
        match smaller {
            0 => return 0,
            1 => return larger,
            _ => (),
        }

        let s = smaller / 2;
        let half_a = inner(larger, s);
        let mut half_b = half_a;
        if smaller % 2 == 1 {
            half_b = inner(larger, smaller - s);
        }

        half_a + half_b
    }

    inner_smaller(a, b)
}

// layers memoization to the previous trick
pub fn recursive_mul4(a: u64, b: u64) -> u64 {
    type MulMap = HashMap<(u64, u64), u64>;

    #[inline(always)]
    fn inner_smaller(mut a: u64, mut b: u64, memo: &mut MulMap) -> u64 {
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }

        inner_memo(a, b, memo)
    }

    #[inline(always)]
    fn inner_memo(a: u64, b: u64, memo: &mut MulMap) -> u64 {
        match memo.get(&(a, b)) {
            Some(v) => *v,
            None => {
                let val = inner(a, b, memo);
                memo.insert((a, b), val);
                val
            }
        }
    }

    fn inner(larger: u64, smaller: u64, memo: &mut MulMap) -> u64 {
        match smaller {
            0 => return 0,
            1 => return larger,
            _ => (),
        }

        let s = smaller / 2;
        let half_a = inner_memo(larger, s, memo);
        let mut half_b = half_a;
        if smaller % 2 == 1 {
            half_b = inner_memo(larger, smaller - s, memo);
        }

        half_a + half_b
    }

    let mut memo: MulMap = HashMap::with_capacity(a as usize);

    inner_smaller(a, b, &mut memo)
}

// resolves odd multiplication to even multiplication and an increment
pub fn recursive_mul5(mut a: u64, mut b: u64) -> u64 {
    fn inner(larger: u64, smaller: u64) -> u64 {
        match smaller {
            0 => return 0,
            1 => return larger,
            _ => (),
        }

        let s = smaller / 2;
        let half_mul = inner(larger, s);
        if smaller % 2 == 0 {
            half_mul + half_mul
        } else {
            half_mul + half_mul + larger
        }
    }

    if b > a {
        std::mem::swap(&mut a, &mut b);
    }

    inner(a, b)
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use paste::paste;

    use crate::*;

    macro_rules! gen_mul_test {
        {$fn: ident} => {
            paste!{
                #[test]
                fn [<test_ $fn>]() {
                    (0..20)
                        .into_iter()
                        .permutations(2)
                        .map(|p| (p[0], p[1]))
                        .for_each(|(a, b)| assert_eq!(a * b, $fn(a, b)));
                }
            }
        };
    }

    gen_mul_test! {recursive_mul}
    gen_mul_test! {recursive_mul2}
    gen_mul_test! {recursive_mul3}
    gen_mul_test! {recursive_mul4}
    gen_mul_test! {recursive_mul5}
}
