use std::ops::{Add, Mul, Sub};

use crate::types::Vector;

// ============================================================
// Operator overloads  (+, -, *)
// ============================================================

impl<K: Copy + Default + Add<Output = K>> Add<&Vector<K>> for &Vector<K> {
    type Output = Vector<K>;

    fn add(self, rhs: &Vector<K>) -> Vector<K> {
        let mut out = self.clone();
        out.add_in_place(rhs);
        out
    }
}

impl<K: Copy + Add<Output = K>> Add for Vector<K> {
    type Output = Vector<K>;

    fn add(mut self, rhs: Self) -> Vector<K> {
        self.add_in_place(&rhs);
        self
    }
}

impl<K: Copy + Default + Sub<Output = K>> Sub<&Vector<K>> for &Vector<K> {
    type Output = Vector<K>;

    fn sub(self, rhs: &Vector<K>) -> Vector<K> {
        let mut out = self.clone();
        out.sub_in_place(rhs);
        out
    }
}

impl<K: Copy + Sub<Output = K>> Sub for Vector<K> {
    type Output = Vector<K>;

    fn sub(mut self, rhs: Self) -> Vector<K> {
        self.sub_in_place(&rhs);
        self
    }
}

impl<K: Copy + Default + Mul<Output = K>> Mul<K> for &Vector<K> {
    type Output = Vector<K>;

    fn mul(self, scalar: K) -> Vector<K> {
        let mut out = self.clone();
        out.scl_in_place(scalar);
        out
    }
}

impl<K: Copy + Mul<Output = K>> Mul<K> for Vector<K> {
    type Output = Vector<K>;

    fn mul(mut self, scalar: K) -> Vector<K> {
        self.scl_in_place(scalar);
        self
    }
}
