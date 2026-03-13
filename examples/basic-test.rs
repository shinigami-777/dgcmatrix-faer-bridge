use dgcmatrix_faer_bridge::{DgCMatrixView, dgcmatrix_to_faer};

fn main() {

    let p = vec![0, 3, 6, 9, 12];
    let i = vec![0,1,2,0,1,3,0,2,3,1,2,3];
    let x = vec![2.0,-1.0,-1.0,-1.0,2.0,-1.0,-1.0,2.0,-1.0,-1.0,-1.0,2.0];

    let view = DgCMatrixView::new(4,4,&p,&i,&x);
    let mat = dgcmatrix_to_faer(&view).unwrap();

    println!("Matrix {}x{}", mat.nrows(), mat.ncols());
    for col in 0..mat.ncols() {
        let rows = mat.row_indices_of_col(col);
        let vals = mat.values_of_col(col);

        for (row, val) in rows.zip(vals) {
            println!("A[{}, {}] = {}", row, col, val);
        }
    }
}