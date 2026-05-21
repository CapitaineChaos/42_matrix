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
        let mut result = Vector::new(vectors[0].size());
        for (v, &c) in vectors.iter().zip(coeffs.iter()) {
            for (res, &val) in result.data.iter_mut().zip(v.data.iter()) {
                *res = c.mul_add(val, *res);
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
        let mut result = Matrix::new(matrices[0].shape());
        for (m, &c) in matrices.iter().zip(coeffs.iter()) {
            for (res, &val) in result.data.iter_mut().zip(m.data.iter()) {
                *res = c.mul_add(val, *res);
            }
        }
        result
    }
}
