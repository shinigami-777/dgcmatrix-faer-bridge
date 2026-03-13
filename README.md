## dgcmatrix-faer-bridge

This demonstrates the pattern for conversion of dgcmatrix (R Matrix package) to faer parse matrix representation. \
The dgcmatrix in R stores sparse matrices in CSC format with three slots:
- `x` — non-zero values
- `i` — row indices (0-based internally)
- `p` — column pointers

We extract the slots on the R side and send them to Rust like this:
```
A <- Matrix(
  c(2,-1,-1,0,
   -1,2,0,-1,
   -1,0,2,-1,
   0,-1,-1,2),
  4,4,
  sparse=TRUE
)
rust_fn(A@i, A@p, A@x, nrow(A), ncol(A))
```

This [struct](https://github.com/shinigami-777/dgcmatrix-faer-bridge/blob/main/src/dgcmatrix.rs#L6) shows the extracted slices from R side.

This implementation shows the process that happens after that extraction and transfer of data is done. The pattern is reusable.

Build the crate using `cargo build`.

### Basic usage test
The slots are manually passed in Rust and sparse matrix is constructed. We can then verify the matrix.
```
let p = vec![0, 3, 6, 9, 12];
    let i = vec![0,1,2,0,1,3,0,2,3,1,2,3];
    let x = vec![2.0,-1.0,-1.0,-1.0,2.0,-1.0,-1.0,2.0,-1.0,-1.0,-1.0,2.0];

    let view = DgCMatrixView::new(4,4,&p,&i,&x);
    let mat = dgcmatrix_to_faer(&view).unwrap();
```

To run the test use `cargo run --example basic-test`.
