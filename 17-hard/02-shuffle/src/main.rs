use rand::prelude::*;

enum Card {
    // placeholder
}

fn shuffle(deck: &mut [Card; 52]) {
    let mut rng = rand::thread_rng();
    for i in 0..deck.len() {
        let j = rng.gen_range(0..deck.len());
        deck.swap(i, j);
    }
}

fn main() {
    println!("Hello, world!");
}
