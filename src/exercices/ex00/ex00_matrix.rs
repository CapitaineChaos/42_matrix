use std::ops::{Add, Sub, Mul};

use crate::{common::{Complexe, Reel}, matrix::Matrix};


// Implement addition
impl<K> Matrix<K>
where K: Copy + Add<Output = K>,
{
    fn add_in_place(&mut self, m: &Matrix<K>) {
        assert!(self.shape() == m.shape(), "Matrixes must have the same dimensions for addition");

        for i in 0..self.size() {
            self[i] = self[i] + m[i];
        }
    }

    // m.add(m1); m.add(&m1);
    pub fn add<M>(&mut self, m: M)
    where M: AsRef<Matrix<K>>,
    {
        self.add_in_place(m.as_ref());
    }
}


// Implement subtraction
impl<K> Matrix<K>
where K: Copy + Sub<Output = K>,
{
    fn sub_in_place(&mut self, m: &Matrix<K>) {
        assert!(self.shape() == m.shape(), "Matrixes must have the same dimensions for subtraction");

        for i in 0..self.size() {
            self[i] = self[i] - m[i];
        }
    }

    // m.sub(m1); m.sub(&m1);
    pub fn sub<M>(&mut self, m: M)
    where M: AsRef<Matrix<K>>,
    {
        self.sub_in_place(m.as_ref());
    }
}


// Implement scalar multiplication
impl<K> Matrix<K>
where K: Copy + Mul<Output = K>,
{
    fn scl_in_place(&mut self, scalar: K) {
        for i in 0..self.size() {
            self[i] = self[i] * scalar;
        }
    } 

    // m.scl(scalar);
    pub fn scl(&mut self, scalar: K) {
        self.scl_in_place(scalar);
    }
}


// oxxxxxxx[====================================> Traits implementations



// m = &m1 + &m2;
impl<K> Add<&Matrix<K>> for &Matrix<K>
where K: Copy + Default + Add<Output = K>,
{
    type Output = Matrix<K>;

    fn add(self, rhs: &Matrix<K>) -> Self::Output {
        let mut out = self.clone();
        out.add_in_place(rhs);
        out
    }
}


// m = m1 + m2;
impl<K> std::ops::Add for Matrix<K>
where K: Copy + std::ops::Add<Output = K>,
{
    type Output = Matrix<K>;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.add_in_place(&rhs);
        self
    }
}


// m = &m1 - &m2;
impl<K> Sub<&Matrix<K>> for &Matrix<K>
where K: Copy + Default + Sub<Output = K>,
{
    type Output = Matrix<K>;

    fn sub(self, rhs: &Matrix<K>) -> Self::Output {
        let mut out = self.clone();
        out.sub_in_place(rhs);
        out
    }
}


// m = m1 - m2;
impl<K> std::ops::Sub for Matrix<K>
where K: Copy + std::ops::Sub<Output = K>,
{
    type Output = Matrix<K>;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.sub_in_place(&rhs);
        self
    }
}


// m = &m1 * scalar;
impl<K> Mul<K> for &Matrix<K>
where K: Copy + Default + Mul<Output = K>,
{
    type Output = Matrix<K>;

    fn mul(self, rhs: K) -> Self::Output {
        let mut out = self.clone();
        out.scl_in_place(rhs);
        out
    }
}


// m = m1 * scalar;
impl<K> std::ops::Mul<K> for Matrix<K>
where K: Copy + std::ops::Mul<Output = K>,
{
    type Output = Matrix<K>;

    fn mul(mut self, rhs: K) -> Self::Output {
        self.scl_in_place(rhs);
        self
    }
}


// m = R_scalar * &m1;
impl Mul<&Matrix<Reel>> for Reel {
    type Output = Matrix<Reel>;

    fn mul(self, rhs: &Matrix<Reel>) -> Self::Output {
        let mut out = rhs.clone();
        out.scl_in_place(self);
        out
        }
}


// m = R_scalar * m1;
impl Mul<Matrix<Reel>> for Reel {
    type Output = Matrix<Reel>;

    fn mul(self, mut rhs: Matrix<Reel>) -> Self::Output {
        rhs.scl_in_place(self);
        rhs
    }
}


// m = C_scalar * &m1;
impl Mul<&Matrix<Complexe>> for Complexe {
    type Output = Matrix<Complexe>;

    fn mul(self, rhs: &Matrix<Complexe>) -> Self::Output {
        let mut out = rhs.clone();
        out.scl_in_place(self);
        out
    }
}

// m = C_scalar * m1;
impl Mul<Matrix<Complexe>> for Complexe {
    type Output = Matrix<Complexe>;

    fn mul(self, mut rhs: Matrix<Complexe>) -> Self::Output {
        rhs.scl_in_place(self);
        rhs
    }
}
