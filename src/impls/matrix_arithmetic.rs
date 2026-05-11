use std::ops::{Add, Mul, Sub};

use crate::types::Matrix;

// ============================================================
// Methods - add / sub / scl
// ============================================================

impl<K: Copy + Add<Output = K>> Matrix<K> {
    pub(crate) fn add_in_place(&mut self, rhs: &Matrix<K>) {
        for (lhs, rhs) in self.data.iter_mut().zip(rhs.data.iter()) {
            *lhs = *lhs + *rhs;
        }
    }

    pub fn add<M: AsRef<Matrix<K>>>(&mut self, rhs: M) {
        self.add_in_place(rhs.as_ref());
    }
}

impl<K: Copy + Sub<Output = K>> Matrix<K> {
    pub(crate) fn sub_in_place(&mut self, rhs: &Matrix<K>) {
        for (lhs, rhs) in self.data.iter_mut().zip(rhs.data.iter()) {
            *lhs = *lhs - *rhs;
        }
    }

    pub fn sub<M: AsRef<Matrix<K>>>(&mut self, rhs: M) {
        self.sub_in_place(rhs.as_ref());
    }
}

impl<K: Copy + Mul<Output = K>> Matrix<K> {
    pub(crate) fn scl_in_place(&mut self, scalar: K) {
        for i in 0..self.size() {
            self[i] = self[i] * scalar;
        }
    }

    pub fn scl(&mut self, scalar: K) {
        self.scl_in_place(scalar);
    }
}

