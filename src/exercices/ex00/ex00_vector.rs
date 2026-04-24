use std::ops::{Add, Sub, Mul};

use crate::{common::{Complexe, Reel}, vector::Vector};


// Implement addition
impl<K> Vector<K>
where K: Copy + Add<Output = K>,
{
    fn add_in_place(&mut self, v: &Vector<K>) {
        assert!(self.size() == v.size(), "Vectors must be of the same size for addition");

        for i in 0..self.size() {
            self[i] = self[i] + v[i];    
        }
    }

    // u.add(v); u.add(&v);
    pub fn add<V>(&mut self, v: V)
    where V: AsRef<Vector<K>>,
    {
        self.add_in_place(v.as_ref());
    }

}

// Implement subtraction
impl<K> Vector<K>
where K: Copy + Sub<Output = K>,
{
    fn sub_in_place(&mut self, v: &Vector<K>) {
        assert!(self.size() == v.size(), "Vectors must be of the same size for subtraction");

        for i in 0..self.size() {
            self[i] = self[i] - v[i];    
        }
    }
    
    // u.sub(v); u.sub(v&);
    pub fn sub<V>(&mut self, v: V)
    where V: AsRef<Vector<K>>,
    {
        self.sub_in_place(v.as_ref());
    }
}

// Implement scalar multiplication
impl<K> Vector<K>
where K: Copy + Mul<Output = K>,
{
    fn scl_in_place(&mut self, scalar: K) {
        for i in 0..self.size() {
            self[i] = self[i] * scalar;    
        }
    }

    // u.scl(scalar); u.scl(scalar);
    pub fn scl<S>(&mut self, scalar: S)
    where S: Into<K>,
    {
        self.scl_in_place(scalar.into());
    }
    
}



// oxxxxxxx[====================================> Traits implementations



// w = &u + &v;
impl<K> Add<&Vector<K>> for &Vector<K>
where K: Copy + Default + Add<Output = K>,
{
    type Output = Vector<K>;

    fn add(self, rhs: &Vector<K>) -> Self::Output {
        let mut out = self.clone();
        out.add_in_place(rhs);
        out
    }
}


// w = u + v;
impl<K> std::ops::Add for Vector<K>
where K: Copy + std::ops::Add<Output = K>,
{
    type Output = Vector<K>;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.add_in_place(&rhs);
        self
    }
}


// w = &u - &v;
impl<K> Sub<&Vector<K>> for &Vector<K>
where K: Copy + Default + Sub<Output = K>,
{
    type Output = Vector<K>;

    fn sub(self, rhs: &Vector<K>) -> Self::Output {
        let mut out = self.clone();
        out.sub_in_place(rhs);
        out
    }
}


// w = u - v;
impl<K> std::ops::Sub for Vector<K>
where K: Copy + std::ops::Sub<Output = K>,
{
    type Output = Vector<K>;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.sub_in_place(&rhs);
        self
    }
}


// w = &u * scalar;
impl<K> Mul<K> for &Vector<K>
where K: Copy + Default + Mul<Output = K>,
{
    type Output = Vector<K>;

    fn mul(self, rhs: K) -> Self::Output {
        let mut out = self.clone();
        out.scl_in_place(rhs);
        out
    }
}


// w = u * scalar;
impl<K> std::ops::Mul<K> for Vector<K>
where K: Copy + std::ops::Mul<Output = K>,
{
    type Output = Vector<K>;

    fn mul(mut self, rhs: K) -> Self::Output {
        self.scl_in_place(rhs);
        self
    }
}


// w = R_scalar * &u;
impl Mul<&Vector<Reel>> for Reel {
    type Output = Vector<Reel>;

    fn mul(self, rhs: &Vector<Reel>) -> Self::Output {
        let mut out = rhs.clone();
        out.scl_in_place(self);
        out
    }
}

// w = R_scalar * u;
impl Mul<Vector<Reel>> for Reel {
    type Output = Vector<Reel>;

    fn mul(self, mut rhs: Vector<Reel>) -> Self::Output {
        rhs.scl_in_place(self);
        rhs
    }
}

// w = C_scalar * &u;
impl Mul<&Vector<Complexe>> for Complexe {
    type Output = Vector<Complexe>;

    fn mul(self, rhs: &Vector<Complexe>) -> Self::Output {
        let mut out = rhs.clone();
        out.scl_in_place(self);
        out
    }
}


// w = C_scalar * u;
impl Mul<Vector<Complexe>> for Complexe {
    type Output = Vector<Complexe>;

    fn mul(self, mut rhs: Vector<Complexe>) -> Self::Output {
        rhs.scl_in_place(self);
        rhs
    }
}
