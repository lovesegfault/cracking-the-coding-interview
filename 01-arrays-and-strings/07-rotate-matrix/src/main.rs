fn rotate_matrix(mat: &mut [Vec<u32>]) {
    // validate that it's a square matrix
    let row_len = mat.len();
    let col_len = mat.get(0).map(|c| c.len()).unwrap_or(0);
    assert_ne!(row_len, 0);
    assert_eq!(row_len, col_len);

    let n = mat.len();
    dbg!(n);
    for layer in 0..(n / 2) {
        let first = layer;
        let last = n - 1 - layer;
        for i in first..last {
            let offset = i - first;
            let top = mat[first][i];
            // left -> top
            mat[first][i] = mat[last - offset][first];
            // bottom -> left
            mat[last - offset][first] = mat[last][last - offset];
            // right -> bottom
            mat[last][last - offset] = mat[i][last];
            // top -> right
            mat[i][last] = top;
        }
    }
}

fn print_mat(mat: &[Vec<u32>]) {
    for row in mat.iter() {
        print!("[");
        for col in row.iter() {
            print!(" {}", col);
        }
        println!(" ]");
    }
    println!("");
}

fn main() {
    let mut mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    print_mat(&mat);
    rotate_matrix(&mut mat);
    print_mat(&mat);
}
