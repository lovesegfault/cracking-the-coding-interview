type Set<T> = Vec<T>;

/*
 * { }       -> { }
 * {a}       -> { }, {a}
 * {a, b}    -> { }, {a}, {b}, {a, b}
 * {a, b, c} -> { }, {a}, {b}, {c}, {a, b}, {a, c}, {b, c}, {a, b, c}
*/
pub fn powersets<T: Clone>(set: &Set<T>) -> Set<Set<T>> {
    let powerset_size = 2_usize.pow(set.len() as u32);
    let mut powerset = Set::with_capacity(powerset_size);

    for i in 0..powerset_size {
        let mut subset = Vec::with_capacity(set.len()); // not sure if this is the best guess?
        for j in 0..set.len() {
            if i & (1 << j) > 0 {
                subset.push(set[j].clone());
            }
        }
        powerset.push(subset);
    }

    powerset
}

fn main() {
    let a = vec!["a"];
    let ab = vec!["a", "b"];
    let abc = vec!["a", "b", "c"];
    println!("{:?}", powersets(&a));
    println!("{:?}", powersets(&ab));
    println!("{:?}", powersets(&abc));
}
