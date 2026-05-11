use std::ops::{Add, Mul, Sub};

use crate::types::Matrix;

// ============================================================
// Operator overloads  (+, -, *)
// ============================================================

impl<K: Copy + Default + Add<Output = K>> Add<&Matrix<K>> for &Matrix<K> {
    type Output = Matrix<K>;

    fn add(self, rhs: &Matrix<K>) -> Matrix<K> {
        let mut out = self.clone();
        out.add_in_place(rhs);
        out
    }
}

impl<K: Copy + Add<Output = K>> Add for Matrix<K> {
    type Output = Matrix<K>;

    fn add(mut self, rhs: Self) -> Matrix<K> {
        self.add_in_place(&rhs);
        self
    }
}

impl<K: Copy + Default + Sub<Output = K>> Sub<&Matrix<K>> for &Matrix<K> {
    type Output = Matrix<K>;

    fn sub(self, rhs: &Matrix<K>) -> Matrix<K> {
        let mut out = self.clone();
        out.sub_in_place(rhs);
        out
    }
}

impl<K: Copy + Sub<Output = K>> Sub for Matrix<K> {
    type Output = Matrix<K>;

    fn sub(mut self, rhs: Self) -> Matrix<K> {
        self.sub_in_place(&rhs);
        self
    }
}

impl<K: Copy + Default + Mul<Output = K>> Mul<K> for &Matrix<K> {
    type Output = Matrix<K>;

    fn mul(self, scalar: K) -> Matrix<K> {
        let mut out = self.clone();
        out.scl_in_place(scalar);
        out
    }
}

impl<K: Copy + Mul<Output = K>> Mul<K> for Matrix<K> {
    type Output = Matrix<K>;

    fn mul(mut self, scalar: K) -> Matrix<K> {
        self.scl_in_place(scalar);
        self
    }
}
