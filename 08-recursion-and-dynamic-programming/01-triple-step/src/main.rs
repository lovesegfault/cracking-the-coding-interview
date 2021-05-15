fn ways(steps: usize) -> usize {
    let mut memo: Vec<usize> = vec![0; steps];

    fn inner(steps: usize, memo: &mut [usize]) -> usize {
        match steps {
            0 | 1 => 1,
            s if memo[s] > 0 => memo[s],
            s => {
                memo[s] = inner(steps - 1, memo) + inner(steps - 2, memo);
                memo[s]
            }
        }
    }

    inner(steps, &mut memo)
}

fn main() {
    println!("Hello, world!");
}
