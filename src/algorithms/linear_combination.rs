use crate::traits::LinearElement;
use crate::types::{Matrix, Vector};

// ============================================================
// Trait
// ============================================================

pub trait LinearCombination<K>: Sized {
    fn lc(items: &[&Self], coeffs: &[K]) -> Self;
}

pub fn linear_combination<K, T: LinearCombination<K>>(items: &[&T], coeffs: &[K]) -> T {
    T::lc(items, coeffs)
}

// ============================================================
// Vector
// ============================================================

impl<K: LinearElement> LinearCombination<K> for Vector<K> {
    fn lc(vectors: &[&Self], coeffs: &[K]) -> Self {
        let size = vectors[0].size();
        let mut result = Vector::new(size);
        for (v, &c) in vectors.iter().zip(coeffs.iter()) {
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

impl<K: LinearElement> LinearCombination<K> for Matrix<K> {
    fn lc(matrices: &[&Self], coeffs: &[K]) -> Self {
        let shape = matrices[0].shape();
        let size = matrices[0].size();
        let mut result = Matrix::new(shape);
        for (m, &c) in matrices.iter().zip(coeffs.iter()) {
            for j in 0..size {
                result[j] = c.mul_add(m[j], result[j]);
            }
        }
        result
    }
}
