use std::fmt;
use std::ops::{Add, Div, Mul, Sub};


pub type Reel = f32;


pub trait ScalarFormat {
    fn fmt_scalar(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

impl ScalarFormat for Reel {
    fn fmt_scalar(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:>9.3}", self)
    }
}

impl ScalarFormat for Complexe {
    fn fmt_scalar(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.im < 0.0 {
            write!(f, "{:>9.3} - {:>9.3}i", self.re, -self.im)
        } else {
            write!(f, "{:>9.3} + {:>9.3}i", self.re, self.im)
        }
    }
}

pub fn print_sep(color: u8,c: char, count: usize) {
    let color_code: &str;
    match color {
        0 => color_code = "\x1b[1;31m", // Red
        1 => color_code = "\x1b[1;32m", // Green
        2 => color_code = "\x1b[1;33m", // Yellow
        3 => color_code = "\x1b[1;34m", // Blue
        4 => color_code = "\x1b[1;35m", // Magenta
        5 => color_code = "\x1b[1;36m", // Cyan
        6 => color_code = "\x1b[1;37m", // White
        _ => color_code = "\x1b[0m",    // Reset
    }
    println!("{}{}{}", color_code, c.to_string().repeat(count), "\x1b[0m");
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complexe {
    re: Reel,
    im: Reel,
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
