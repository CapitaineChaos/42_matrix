use std::ops::{Index, IndexMut};

use crate::types::Vector;

impl<K> Index<usize> for Vector<K> {
    type Output = K;

    fn index(&self, index: usize) -> &K {
        &self.data[index]
    }
}

impl<K> IndexMut<usize> for Vector<K> {
    fn index_mut(&mut self, index: usize) -> &mut K {
        &mut self.data[index]
    }
}
