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
        fmt::Display::fmt(self, f)
    }
}

fn color_code(color: u8) -> &'static str {
    match color {
        0 => "\x1b[1;31m", // Red
        1 => "\x1b[1;32m", // Green
        2 => "\x1b[1;33m", // Yellow
        3 => "\x1b[1;34m", // Blue
        4 => "\x1b[1;35m", // Magenta
        5 => "\x1b[1;36m", // Cyan
        6 => "\x1b[1;37m", // White
        _ => "\x1b[0m",    // Reset
    }
}

pub fn print_header(text: &str) {
    let c = '=';
    let title_size = text.len() + 2;
    let sz = 80;
    let left_size = (sz - title_size) / 2;
    let right_size = sz - title_size - left_size;
    let color_code: &str = color_code(3);
    println!("\n{}{}", color_code, c.to_string().repeat(sz));
    println!("{}{}{}", c.to_string(), " ".repeat(sz - 2), c.to_string());
    println!("{}{}{}{}{}", c.to_string(), " ".repeat(left_size), text, " ".repeat(right_size), c.to_string());
    println!("{}{}{}", c.to_string(), " ".repeat(sz - 2), c.to_string());
    println!("{}{}\n", c.to_string().repeat(sz), "\x1b[0m");
}

pub fn print_title(text: &str) {
    let c = '¤';
    let title_size = text.len() + 2;
    let sz = 80;
    let left_size = (sz - title_size) / 2;
    let right_size = sz - title_size - left_size;
    let color_code: &str = color_code(3);
    println!("\n{}{}", color_code, c.to_string().repeat(sz));
    println!("{}{}{}{}{}{}", color_code, c.to_string().repeat(left_size), " ", text, " ", c.to_string().repeat(right_size));
    println!("{}{}\n", c.to_string().repeat(sz), "\x1b[0m");
}

pub fn print_sep() {
    let c = '~';
    let count = 80;
    let color_code: &str = color_code(2);
    println!("\n{}{}{}\n", color_code, c.to_string().repeat(count), "\x1b[0m");
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complexe {
    pub re: Reel,
    pub im: Reel,
}

impl Complexe {
    pub fn new(re: Reel, im: Reel) -> Self {
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
