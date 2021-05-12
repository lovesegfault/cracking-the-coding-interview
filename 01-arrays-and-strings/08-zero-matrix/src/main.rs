use std::collections::HashSet;

fn zero_matrix(mat: &mut [Vec<u64>]) {
    // first we find all the colums and rows with zeroes in the matrix
    let mut rows_with_zeros: HashSet<usize> = HashSet::new();
    let mut cols_with_zeros: HashSet<usize> = HashSet::new();
    for (i, row) in mat.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == 0 {
                rows_with_zeros.insert(i);
                cols_with_zeros.insert(j);
            }
        }
    }

    // then we replace them with all zeroes
    for &row in rows_with_zeros.iter() {
        mat[row].fill(0);
    }

    for &col in cols_with_zeros.iter() {
        mat.iter_mut()
            .map(|row| &mut row[col])
            .for_each(|pos| *pos = 0);
    }
}

fn main() {
    print!("checking zero_matrix: ");
    let mut a = vec![vec![1, 2, 0], vec![4, 5, 6], vec![7, 8, 9]];
    let zeroed = vec![vec![0, 0, 0], vec![4, 5, 0], vec![7, 8, 0]];
    zero_matrix(&mut a);
    assert_eq!(a, zeroed);
    println!("OK");
}
