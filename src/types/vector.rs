use std::iter;

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
