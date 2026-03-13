// View of an R dgCMatrix sparse matrix
// R dgCMatrix stores matrices in CSC format
// with three slots: x, i, p
// This struct borrows the slices coming from R.
#[derive(Debug)]
pub struct DgCMatrixView<'a> {
    pub nrow: usize,
    pub ncol: usize,

    pub p: &'a [i32],
    pub i: &'a [i32],
    pub x: &'a [f64],
}

impl<'a> DgCMatrixView<'a> {
    pub fn nnz(&self) -> usize {
        self.x.len()
    }

    pub fn new(
        nrow: usize,
        ncol: usize,
        p: &'a [i32],
        i: &'a [i32],
        x: &'a [f64],
    ) -> Self {
        Self { nrow, ncol, p, i, x }
    }
}
