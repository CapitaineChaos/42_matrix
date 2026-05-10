use std::ops::{Add, Sub, Mul};

use crate::core::{Matrix, Vector};

// ============================================================
// Vector - add / sub / scl
// ============================================================

impl<K: Copy + Add<Output = K>> Vector<K> {
    fn add_in_place(&mut self, rhs: &Vector<K>) {
        for (lhs, rhs) in self.data.iter_mut().zip(rhs.data.iter()) {
            *lhs = *lhs + *rhs;
        }
    }

    pub fn add<V: AsRef<Vector<K>>>(&mut self, rhs: V) {
        self.add_in_place(rhs.as_ref());
    }
}

impl<K: Copy + Sub<Output = K>> Vector<K> {
    fn sub_in_place(&mut self, rhs: &Vector<K>) {
        for (lhs, rhs) in self.data.iter_mut().zip(rhs.data.iter()) {
            *lhs = *lhs - *rhs;
        }
    }

    pub fn sub<V: AsRef<Vector<K>>>(&mut self, rhs: V) {
        self.sub_in_place(rhs.as_ref());
    }
}

impl<K: Copy + Mul<Output = K>> Vector<K> {
    fn scl_in_place(&mut self, scalar: K) {
        for lhs in self.data.iter_mut() {
            *lhs = *lhs * scalar;
        }
    }

    pub fn scl<S: Into<K>>(&mut self, scalar: S) {
        self.scl_in_place(scalar.into());
    }
}

// ============================================================
// Vector - operator overloads  (+, -, *)
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

// ============================================================
// Matrix - add / sub / scl
// ============================================================

impl<K: Copy + Add<Output = K>> Matrix<K> {
    fn add_in_place(&mut self, rhs: &Matrix<K>) {
        for (lhs, rhs) in self.data.iter_mut().zip(rhs.data.iter()) {
            *lhs = *lhs + *rhs;
        }
    }

    pub fn add<M: AsRef<Matrix<K>>>(&mut self, rhs: M) {
        self.add_in_place(rhs.as_ref());
    }
}

impl<K: Copy + Sub<Output = K>> Matrix<K> {
    fn sub_in_place(&mut self, rhs: &Matrix<K>) {
        for (lhs, rhs) in self.data.iter_mut().zip(rhs.data.iter()) {
            *lhs = *lhs - *rhs;
        }
    }

    pub fn sub<M: AsRef<Matrix<K>>>(&mut self, rhs: M) {
        self.sub_in_place(rhs.as_ref());
    }
}

impl<K: Copy + Mul<Output = K>> Matrix<K> {
    fn scl_in_place(&mut self, scalar: K) {
        for i in 0..self.size() {
            self[i] = self[i] * scalar;
        }
    }

    pub fn scl(&mut self, scalar: K) {
        self.scl_in_place(scalar);
    }
}

// ============================================================
// Matrix - operator overloads  (+, -, *)
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
