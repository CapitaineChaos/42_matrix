use std::ops::{Index, IndexMut};

use crate::types::Matrix;

impl<K> Index<usize> for Matrix<K> {
    type Output = K;

    fn index(&self, index: usize) -> &K {
        &self.data[index]
    }
}

impl<K> IndexMut<usize> for Matrix<K> {
    fn index_mut(&mut self, index: usize) -> &mut K {
        &mut self.data[index]
    }
}

/// Row-column indexing: `m[(row, col)]`
impl<K> Index<(usize, usize)> for Matrix<K> {
    type Output = K;

    fn index(&self, (row, col): (usize, usize)) -> &K {
        &self.data[col * self.rows + row]
    }
}

impl<K> IndexMut<(usize, usize)> for Matrix<K> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut K {
        &mut self.data[col * self.rows + row]
    }
}