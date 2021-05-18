use rand::prelude::*;

fn random_set(seed: &[i64], size: usize) -> Vec<i64> {
    let mut set = vec![0; size];
    let mut rng = rand::thread_rng();
    for e in set.iter_mut() {
        *e = seed[rng.gen_range(0..seed.len())];
    }
    set
}

fn main() {
    println!("Hello, world!");
}
