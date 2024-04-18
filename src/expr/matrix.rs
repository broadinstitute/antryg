use std::ops::Index;
use crate::slate;

pub(crate) struct Matrix {
    pub(crate) n_cols: usize,
    elems: Vec<slate::Key>,
}

impl Index<usize> for Matrix {
    type Output = [slate::Key];

    fn index(&self, i: usize) -> &Self::Output {
        let start = i * self.n_cols;
        &self.elems[start..start + self.n_cols]
    }
}