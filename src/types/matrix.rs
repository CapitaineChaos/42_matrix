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

        let data = std::iter::repeat_with(K::default)
            .take(len)
            .collect();

        Matrix { rows, cols, data }
    }
}

impl<K, const R: usize, const C: usize> From<[[K; C]; R]> for Matrix<K> {
    fn from(array: [[K; C]; R]) -> Self {
        let rows = R;
        let cols = C;
        let len = rows * cols;

        let mut data: Vec<Option<K>> = std::iter::repeat_with(|| None)
            .take(len)
            .collect();

        // Column-major storage
        for (row, line) in array.into_iter().enumerate() {
            for (col, value) in line.into_iter().enumerate() {
                data[col * rows + row] = Some(value);
            }
        }

        let data = data.into_iter()
            .map(Option::unwrap)
            .collect();

        Matrix { rows, cols, data }
    }
}
