use std::ops::Index;
use crate::slate;

pub(crate) struct Matrix {
    pub(crate) n_rows: usize,
    pub(crate) n_cols: usize,
    elems: Vec<slate::Key>,
}

impl Matrix {
    pub(crate) fn fill(n_rows: usize, n_cols: usize,
                       f: impl Fn(usize, usize) -> slate::Key) -> Self {
        let mut elems = Vec::with_capacity(n_rows * n_cols);
        for i in 0..n_rows {
            for j in 0..n_cols {
                elems.push(f(i, j));
            }
        }
        Matrix { n_rows, n_cols, elems }
    }
}

impl Index<usize> for Matrix {
    type Output = [slate::Key];

    fn index(&self, i: usize) -> &Self::Output {
        let start = i * self.n_cols;
        &self.elems[start..start + self.n_cols]
    }
}