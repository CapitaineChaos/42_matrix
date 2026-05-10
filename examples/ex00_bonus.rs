#[path = "common/mod.rs"]
mod common;

use common::display::{print_header, print_sep, print_title};
use matrix42::{Matrix, Vector, Complex};

fn main() {
    print_header("Ex00 : Add, Subtract and Scale (Complex)");

    print_title("Vector tests (Complex)");

    let c1 = Complex::new(2.0, 3.0);
    let c2 = Complex::new(5.0, 7.0);
    let c3 = Complex::new(1.0, 1.0);
    let c4 = Complex::new(0.0, 1.0);

    let mut u = Vector::from([c1, c2]);
    let v = Vector::from([c3, c4]);
    println!("u = {}\nv = {}", &u, &v);
    u.add(&v);
    println!("u + v = {}", &u);

    print_sep();

    let mut u = Vector::from([c1, c2]);
    let v = Vector::from([c3, c4]);
    println!("u = {}\nv = {}", &u, &v);
    u.sub(&v);
    println!("u - v = {}", &u);

    print_sep();

    let mut u = Vector::from([c1, c2]);
    let s = Complex::new(2.0, 3.0);
    println!("u = {}\ns = {}", &u, s);
    u.scl(s);
    println!("u * s = {}", &u);

    print_title("Matrix tests (Complex)");

    let c5 = Complex::new(1.0, 2.0);
    let c6 = Complex::new(3.0, 4.0);
    let c7 = Complex::new(0.5, -2.0);
    let c8 = Complex::new(7.0, -1.0);

    let mut m1 = Matrix::from([[c1, c2], [c3, c4]]);
    let m2 = Matrix::from([[c5, c6], [c7, c8]]);
    println!("m1 =\n{}\nm2 =\n{}", &m1, &m2);
    m1.add(&m2);
    println!("m1 + m2 =\n{}", &m1);

    print_sep();

    let mut m3 = Matrix::from([[c1, c2], [c3, c4]]);
    let m4 = Matrix::from([[c5, c6], [c7, c8]]);
    println!("m1 =\n{}\nm2 =\n{}", &m3, &m4);
    m3.sub(&m4);
    println!("m1 - m2 =\n{}", &m3);

    print_sep();

    let mut m5 = Matrix::from([[c1, c2], [c3, c4]]);
    let s = Complex::new(2.0, 3.0);
    println!("m =\n{}\ns = {}", &m5, s);
    m5.scl(s);
    println!("m * s =\n{}", &m5);
}
