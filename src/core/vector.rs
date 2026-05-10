use std::fmt;
use std::iter;
use std::ops::{Index, IndexMut};

// ============================================================
// Struct
// ============================================================

#[derive(Debug, Clone)]
pub struct Vector<K> {
    pub(crate) size: usize,
    pub(crate) data: Vec<K>,
}

// ============================================================
// Core accessors
// ============================================================

impl<K> Vector<K> {
    pub fn size(&self) -> usize {
        self.size
    }
}

impl<K> AsRef<Vector<K>> for Vector<K> {
    fn as_ref(&self) -> &Vector<K> {
        self
    }
}

// ============================================================
// Constructors
// ============================================================

impl<K: Default> Vector<K> {
    pub fn new(size: usize) -> Self {
        Vector {
            size,
            data: iter::repeat_with(K::default)
                .take(size)
                .collect(),
        }
    }
}

impl<K, const N: usize> From<[K; N]> for Vector<K> {
    fn from(array: [K; N]) -> Self {
        Self {
            size: N,
            data: array.into(),
        }
    }
}

// ============================================================
// Indexing
// ============================================================

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

// ============================================================
// Display
// ============================================================

impl<K: fmt::Display> fmt::Display for Vector<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for i in 0..self.size {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", self.data[i])?;
        }
        write!(f, "]")
    }
}
