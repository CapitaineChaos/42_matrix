use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
use std::fmt;
use std::ops::{Index, IndexMut};

use crate::scalar::ScalarFormat;


#[derive(Debug)]
pub struct Matrix<K> {
    rows: usize,
    cols: usize,
    data: *mut K,
}


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

// Implement AsRef to allow borrowing a Matrix as a reference to itself
impl<K> AsRef<Matrix<K>> for Matrix<K> {
    fn as_ref(&self) -> &Matrix<K> {
        self
    }
}


// Implement Clone for Matrix to allow cloning of matrixes
impl <K: Clone + Default> Clone for Matrix<K> {
    fn clone(&self) -> Self {
        let mut out = Matrix::new(self.shape());

        for i in 0..self.size() {
            out[i] = self[i].clone();
        }

        out
    }
}


// Implement From trait to allow conversion from scratch with dimensions to a Matrix
impl<K: Default> Matrix<K> {
    pub fn new(shape: (usize, usize)) -> Self {
        let (rows, cols) = shape;
        let len = rows * cols;

        if len == 0 {
            return Matrix {
                rows,
                cols,
                data: std::ptr::NonNull::<K>::dangling().as_ptr(),
            };
        }

        let layout = Layout::array::<K>(len).expect("Failed to create layout");
        let data = unsafe { alloc(layout) as *mut K };

        if data.is_null() {
            handle_alloc_error(layout);
        }

        for i in 0..len {
            unsafe {
                std::ptr::write(data.add(i), K::default());
            }
        }

        Matrix { rows, cols, data }
    }
}


// Implement From trait to allow conversion from a 2D array to a Matrix
impl<K, const R: usize, const C: usize> From<[[K; C]; R]> for Matrix<K> {
    fn from(array: [[K; C]; R]) -> Self {
        let rows = R;
        let cols = C;
        let len = rows * cols;

        if len == 0 {
            return Matrix {
                rows,
                cols,
                // Use a dangling pointer for zero-sized matrices, as it will never be dereferenced
                data: std::ptr::NonNull::<K>::dangling().as_ptr(),
            };
        }

        // Create a layout for the matrix for the given size and type K
        let layout: Layout = Layout::array::<K>(len).expect("Failed to create layout");

        // Allocate memory for the matrix
        let data: *mut K = unsafe { alloc(layout) as *mut K };

        // Check if allocation was successful
        if data.is_null() {
            handle_alloc_error(layout);
        }

        // Copy elements from the array to the allocated memory
        for (row, line) in array.into_iter().enumerate() {
            for (col, value) in line.into_iter().enumerate() {
                let offset = col * rows + row;
                unsafe {
                    std::ptr::write(data.add(offset), value);
                }
            }
        }

        Matrix { rows, cols, data }
    }
}


// Implement Drop trait to properly deallocate memory when a Matrix goes out of scope
impl<K> Drop for Matrix<K> {
    fn drop(&mut self) {
        if self.size() == 0 {
            return;
        }
    
        // Drop each element in the matrix to properly release any resources they may hold
        for i in 0..self.size() {
            unsafe {
                std::ptr::drop_in_place(self.data.add(i));
            }
        }

        // Deallocate the memory used by the matrix
        let layout = Layout::array::<K>(self.size()).expect("Failed to create layout");
        unsafe {
            dealloc(self.data as *mut u8, layout);
        }
    }
}

// Implement Index for Matrix to allow access to elements using linear indexing
impl<K> Index<usize> for Matrix<K> {
    type Output = K;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.size(), "Index out of bounds");
        unsafe { &*self.data.add(index) }
    }
}

// Implement IndexMut for Matrix to allow mutable access to elements using linear indexing
impl<K> IndexMut<usize> for Matrix<K> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.size(), "Index out of bounds");
        unsafe { &mut *self.data.add(index) }
    }
}

// Implement Index for Matrix to allow access to elements using (row, column) indexing
impl<K> Index<(usize, usize)> for Matrix<K> {
    type Output = K;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        assert!(row < self.rows, "Row index out of bounds");
        assert!(col < self.cols, "Column index out of bounds");

        // Calculate the offset in column-major order
        let offset = col * self.rows + row;
        unsafe { &*self.data.add(offset) }
    }
}


// Implement IndexMut for Matrix to allow mutable access to elements
impl<K> IndexMut<(usize, usize)> for Matrix<K> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, col) = index;
        assert!(row < self.rows, "Row index out of bounds");
        assert!(col < self.cols, "Column index out of bounds");

        // Calculate the offset in column-major order
        let offset = col * self.rows + row;
        unsafe { &mut *self.data.add(offset) }
    }
}


// Implement Display for Matrix to allow printing the matrix in a readable format
impl<K: ScalarFormat> fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[\n")?;
        for row in 0..self.rows {
            write!(f, "  [")?;
            for col in 0..self.cols {
                if col > 0 {
                    write!(f, ", ")?;
                }
                unsafe {
                    (*self.data.add(col * self.rows + row)).fmt_scalar(f)?;
                }
            }
            write!(f, "]\n")?;
        }
        write!(f, "]")
    }
}
