use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
use std::fmt;
use std::ops::{Index, IndexMut};

use crate::scalar::ScalarFormat;


#[derive(Debug)]
pub struct Vector<K> {
    size: usize,
    data: *mut K,
}


impl<K> Vector<K> {
    pub fn size(&self) -> usize {
        self.size
    }
}


// Implement AsRef to allow borrowing a Vector as a reference to itself
impl<K> AsRef<Vector<K>> for Vector<K> {
    fn as_ref(&self) -> &Vector<K> {
        self
    }
}


// Implement Clone for Vector to allow cloning of vectors
impl<K: Clone + Default> Clone for Vector<K> {
    fn clone(&self) -> Self {
        let mut out = Vector::new(self.size);

        for i in 0..self.size {
            out[i] = self[i].clone();
        }

        out
    }
}


// Implement From trait to allow conversion from scratch with a size to a Vector
impl<K: Default> Vector<K> {
    pub fn new(size: usize) -> Self {
        if size == 0 {
            return Vector {
                size: 0,
                data: std::ptr::NonNull::<K>::dangling().as_ptr(),
            };
        }

        let layout = Layout::array::<K>(size).expect("Failed to create layout");
        let data = unsafe { alloc(layout) as *mut K };

        if data.is_null() {
            handle_alloc_error(layout);
        }

        for i in 0..size {
            unsafe {
                std::ptr::write(data.add(i), K::default());
            }
        }

        Vector { size, data }
    }
}


// Implement From trait to allow conversion from an array to a Vector
impl<K, const N: usize> From<[K; N]> for Vector<K> {
    fn from(array: [K; N]) -> Self {
        let size = N;
        
        if size == 0 {
            return Vector {
                size: 0,
                // Use a dangling pointer for zero-sized vectors, as it will never be dereferenced
                data: std::ptr::NonNull::<K>::dangling().as_ptr(),
            };
        }

        // Create a layout for the vector for the given size and type K
        let layout = Layout::array::<K>(size).expect("Failed to create layout");

        // Allocate memory for the vector
        let data = unsafe { alloc(layout) as *mut K };

        // Check if allocation was successful
        if data.is_null() {
            handle_alloc_error(layout);
        }

        // Copy elements from the array to the allocated memory
        for (i, value) in array.into_iter().enumerate() {
            unsafe {
                std::ptr::write(data.add(i), value);
            }
        }

        Vector { size, data }
    }
}


// Implement Drop trait to properly deallocate memory when a Vector goes out of scope
impl<K> Drop for Vector<K> {
    fn drop(&mut self) {
        if self.size == 0 {
            return;
        }

        // Drop each element in the vector to properly release any resources they may hold
        for i in 0..self.size {
            unsafe {
                std::ptr::drop_in_place(self.data.add(i));
            }
        }

        // Deallocate the memory used by the vector
        let layout = Layout::array::<K>(self.size).expect("Failed to create layout");
        unsafe {
            dealloc(self.data as *mut u8, layout);
        }
    }
}


// Implement Index for Vector to allow access to elements using indexing
impl<K> Index<usize> for Vector<K> {
    type Output = K;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.size, "Index out of bounds");
        unsafe { &*self.data.add(index) }
    }
}


// Implement IndexMut for Vector to allow mutable access to elements
impl<K>  IndexMut<usize> for Vector<K> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.size, "Index out of bounds");
        unsafe { &mut *self.data.add(index) }
    }
}


// Implement Display for Vector to allow printing the vector in a readable format
impl<K: ScalarFormat> fmt::Display for Vector<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for i in 0..self.size {
            if i > 0 {
                write!(f, ", ")?;
            }
            unsafe {
                (*self.data.add(i)).fmt_scalar(f)?;
            }
        }
        write!(f, "]")
    }
}
