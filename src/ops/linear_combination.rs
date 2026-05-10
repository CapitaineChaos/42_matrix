use std::ops::{Add, Mul};
use num_traits::MulAdd;

use crate::core::{Matrix, Vector};

// ============================================================
// Trait
// ============================================================

pub trait LinearCombination<K>: Sized {
    fn lc(items: &[&Self], coeffs: &[K]) -> Self;
}

/// Free function - sugar on top of the trait.
///
/// ```rust
/// let result = linear_combination(&[&u, &v], &[a, b]);
/// ```
pub fn linear_combination<K, T: LinearCombination<K>>(items: &[&T], coeffs: &[K]) -> T {
    T::lc(items, coeffs)
}

// ============================================================
// Vector
// ============================================================

impl<K> LinearCombination<K> for Vector<K>
where
    K: Copy + Default + Add<Output = K> + Mul<Output = K> + MulAdd<Output = K>,
{
    fn lc(vectors: &[&Self], coeffs: &[K]) -> Self {
        assert!(!vectors.is_empty(), "Need at least one vector");
        assert!(vectors.len() == coeffs.len(), "Number of vectors and coefficients must match");
        let size = vectors[0].size();
        let mut result = Vector::new(size);
        for (v, &c) in vectors.iter().zip(coeffs.iter()) {
            assert!(v.size() == size, "All vectors must have the same size");
            for j in 0..size {
                result[j] = c.mul_add(v[j], result[j]);
            }
        }
        result
    }
}

// ============================================================
// Matrix
// ============================================================

impl<K> LinearCombination<K> for Matrix<K>
where
    K: Copy + Default + Add<Output = K> + Mul<Output = K> + MulAdd<Output = K>,
{
    fn lc(matrices: &[&Self], coeffs: &[K]) -> Self {
        assert!(!matrices.is_empty(), "Need at least one matrix");
        assert!(matrices.len() == coeffs.len(), "Number of matrices and coefficients must match");
        let shape = matrices[0].shape();
        let size = matrices[0].size();
        let mut result = Matrix::new(shape);
        for (m, &c) in matrices.iter().zip(coeffs.iter()) {
            assert!(m.shape() == shape, "All matrices must have the same shape");
            for j in 0..size {
                result[j] = c.mul_add(m[j], result[j]);
            }
        }
        result
    }
}
