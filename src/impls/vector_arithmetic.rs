use std::ops::{Add, Mul, Sub};

use crate::types::Vector;

// ============================================================
// Methods - add / sub / scl
// ============================================================

impl<K: Copy + Add<Output = K>> Vector<K> {
    pub(crate) fn add_in_place(&mut self, rhs: &Vector<K>) {
        for (lhs, rhs) in self.data.iter_mut().zip(rhs.data.iter()) {
            *lhs = *lhs + *rhs;
        }
    }

    pub fn add<V: AsRef<Vector<K>>>(&mut self, rhs: V) {
        self.add_in_place(rhs.as_ref());
    }
}

impl<K: Copy + Sub<Output = K>> Vector<K> {
    pub(crate) fn sub_in_place(&mut self, rhs: &Vector<K>) {
        for (lhs, rhs) in self.data.iter_mut().zip(rhs.data.iter()) {
            *lhs = *lhs - *rhs;
        }
    }

    pub fn sub<V: AsRef<Vector<K>>>(&mut self, rhs: V) {
        self.sub_in_place(rhs.as_ref());
    }
}

impl<K: Copy + Mul<Output = K>> Vector<K> {
    pub(crate) fn scl_in_place(&mut self, scalar: K) {
        for lhs in self.data.iter_mut() {
            *lhs = *lhs * scalar;
        }
    }

    pub fn scl<S: Into<K>>(&mut self, scalar: S) {
        self.scl_in_place(scalar.into());
    }
}
