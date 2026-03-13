use crate::dgcmatrix::DgCMatrixView;

#[derive(Debug)]
pub enum DgCMatrixError {
    ColumnPointerLengthMismatch,
    IndexLengthMismatch,
    NegativeIndex,
}

pub fn validate(view: &DgCMatrixView) -> Result<(), DgCMatrixError> {

    if view.p.len() != view.ncol + 1 {
        return Err(DgCMatrixError::ColumnPointerLengthMismatch);
    }

    let nnz = view.p[view.ncol] as usize;

    if view.i.len() != nnz || view.x.len() != nnz {
        return Err(DgCMatrixError::IndexLengthMismatch);
    }

    if view.i.iter().any(|&v| v < 0) || view.p.iter().any(|&v| v < 0) {
        return Err(DgCMatrixError::NegativeIndex);
    }

    Ok(())
}
