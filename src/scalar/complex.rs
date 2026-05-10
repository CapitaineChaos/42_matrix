use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
use num_traits::MulAdd;

// ============================================================
// Struct
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub re: f32,
    pub im: f32,
}

impl Complex {
    pub fn new(re: f32, im: f32) -> Self {
        Self { re, im }
    }
}

// ============================================================
// Standard traits
// ============================================================

impl Default for Complex {
    fn default() -> Self {
        Self { re: 0.0, im: 0.0 }
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.im < 0.0 {
            write!(f, "{} - {}i", self.re, -self.im)
        } else {
            write!(f, "{} + {}i", self.re, self.im)
        }
    }
}

// ============================================================
// Arithmetic — (a+bi) ± (c+di), (a+bi)(c+di), (a+bi)/(c+di)
// ============================================================

// (a+bi)+(c+di) = (a+c)+(b+d)i
impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self { re: self.re + rhs.re, im: self.im + rhs.im }
    }
}

// (a+bi)-(c+di) = (a-c)+(b-d)i
impl Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self { re: self.re - rhs.re, im: self.im - rhs.im }
    }
}

// (a+bi)(c+di) = (ac-bd)+(ad+bc)i
impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

// (a+bi)/(c+di) = ((a+bi)(c-di)) / (c²+d²)
impl Div for Complex {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let denom = rhs.re * rhs.re + rhs.im * rhs.im;
        Self {
            re: (self.re * rhs.re + self.im * rhs.im) / denom,
            im: (self.im * rhs.re - self.re * rhs.im) / denom,
        }
    }
}

// (a+bi).mul_add(c+di, e+fi) = (a+bi)*(c+di) + (e+fi)
impl MulAdd for Complex {
    type Output = Self;

    fn mul_add(self, a: Self, b: Self) -> Self {
        self * a + b
    }
}
