use faer::sparse::{SparseColMat, SymbolicSparseColMat};

use crate::dgcmatrix::DgCMatrixView;
use crate::validate::validate;

pub fn dgcmatrix_to_faer(
    view: &DgCMatrixView,
) -> Result<SparseColMat<usize, f64>, &'static str> {

    validate(view).map_err(|_| "invalid dgCMatrix structure")?;

    let nrow = view.nrow;
    let ncol = view.ncol;

    let col_ptrs: Vec<usize> =
        view.p.iter().map(|&v| v as usize).collect();
    let row_indices: Vec<usize> =
        view.i.iter().map(|&v| v as usize).collect();

    // Build symbolic sparse matrix
    let sym = SymbolicSparseColMat::new_checked(
        nrow,
        ncol,
        col_ptrs,
        None,
        row_indices,
    );

    let values: Vec<f64> = view.x.to_vec();
    let mat = SparseColMat::new(sym, values);
    Ok(mat)
}