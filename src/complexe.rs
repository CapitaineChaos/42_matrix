use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
use num_traits::MulAdd;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complexe {
    pub re: f32,
    pub im: f32,
}

impl Complexe {
    pub fn new(re: f32, im: f32) -> Self {
        Self { re, im }
    }
}

impl fmt::Display for Complexe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.im < 0.0 {
            write!(f, "{:>9.3} - {:>9.3}i", self.re, -self.im)
        } else {
            write!(f, "{:>9.3} + {:>9.3}i", self.re, self.im)
        }
    }
}

impl Default for Complexe {
    fn default() -> Self {
        Self { re: 0.0, im: 0.0 }
    }
}

// (a+bi)+(c+di)=(a+c)+(b+d)i
impl Add for Complexe {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

// (a+bi)-(c+di)=(a−c)+(b−d)i
impl Sub for Complexe {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

// (a+bi)(c+di)=(ac−bd)+(ad+bc)i
impl Mul for Complexe {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

// (a+bi)/(c+di)= ((a+bi)(c-di))/(c^2+d^2)
impl Div for Complexe {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let denom = rhs.re * rhs.re + rhs.im * rhs.im;

        Self {
            re: (self.re * rhs.re + self.im * rhs.im) / denom,
            im: (self.im * rhs.re - self.re * rhs.im) / denom,
        }
    }
}

// (a+bi).mul_add(c+di, e+fi) = (a+bi)*(c+di) + (e+fi)
impl MulAdd for Complexe {
    type Output = Self;

    fn mul_add(self, a: Self, b: Self) -> Self::Output {
        self * a + b
    }
}