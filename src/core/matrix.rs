use std::fmt;
use std::ops::{Index, IndexMut};

// ============================================================
// Struct
// ============================================================

#[derive(Debug, Clone)]
pub struct Matrix<K> {
    pub(crate) rows: usize,
    pub(crate) cols: usize,
    pub(crate) data: Vec<K>,
}

// ============================================================
// Core accessors
// ============================================================

impl<K> Matrix<K> {
    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn size(&self) -> usize {
        self.rows * self.cols
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }
}

impl<K> AsRef<Matrix<K>> for Matrix<K> {
    fn as_ref(&self) -> &Matrix<K> {
        self
    }
}

// ============================================================
// Constructors
// ============================================================

impl<K: Default> Matrix<K> {
    pub fn new(shape: (usize, usize)) -> Self {
        let (rows, cols) = shape;
        let len = rows * cols;
        if len == 0 {
            return Matrix {
                rows,
                cols,
                data: Vec::new(),
            };
        }
        let mut data = Vec::with_capacity(len);
        for _ in 0..len {
            data.push(K::default());
        }
        Matrix { rows, cols, data }
    }
}

impl<K: Default, const R: usize, const C: usize> From<[[K; C]; R]> for Matrix<K> {
    fn from(array: [[K; C]; R]) -> Self {
        let rows = R;
        let cols = C;
        let len = rows * cols;
        if len == 0 {
            return Matrix {
                rows,
                cols,
                data: Vec::new(),
            };
        }
        let mut data = Vec::with_capacity(len);
        for _ in 0..len {
            data.push(K::default());
        }
        // Column-major storage
        for (row, line) in array.into_iter().enumerate() {
            for (col, value) in line.into_iter().enumerate() {
                data[col * rows + row] = value;
            }
        }
        Matrix { rows, cols, data }
    }
}

// ============================================================
// Indexing
// ============================================================

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

// ============================================================
// Display
// ============================================================

impl<K: fmt::Display> fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[")?;
        for row in 0..self.rows {
            write!(f, "  [")?;
            for col in 0..self.cols {
                if col > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", self.data[col * self.rows + row])?;
            }
            writeln!(f, "]")?;
        }
        write!(f, "]")
    }
}
