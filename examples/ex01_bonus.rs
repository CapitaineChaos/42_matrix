#[path = "common/mod.rs"]
mod common;

use common::display::{print_header, print_sep, print_title};
use matrix42::{Matrix, Vector, Complex, linear_combination};

fn main() {
    print_header("Ex01 : Linear Combination (Complex)");

    print_title("Vector tests (Complex)");

    let z = Complex::new(0.0, 0.0);
    let e1 = Vector::from([Complex::new(1.0, 0.0), z, z]);
    let e2 = Vector::from([z, Complex::new(1.0, 0.0), z]);
    let e3 = Vector::from([z, z, Complex::new(1.0, 0.0)]);
    println!("{}", linear_combination(
        &[&e1, &e2, &e3],
        &[Complex::new(10.0, 0.0), Complex::new(-2.0, 0.0), Complex::new(0.5, 0.0)],
    ));

    print_sep();

    let v1 = Vector::from([Complex::new(1.0, 0.0), Complex::new(2.0, 0.0), Complex::new(3.0, 0.0)]);
    let v2 = Vector::from([z, Complex::new(10.0, 0.0), Complex::new(-100.0, 0.0)]);
    println!("{}", linear_combination(
        &[&v1, &v2],
        &[Complex::new(10.0, 0.0), Complex::new(-2.0, 0.0)],
    ));

    print_title("Matrix tests (Complex)");

    let m1 = Matrix::from([[Complex::new(1.0, 0.0), Complex::new(4.0, 0.0)],
                            [Complex::new(6.0, 0.0), Complex::new(0.5, 0.0)]]);
    let m2 = Matrix::from([[z, Complex::new(10.0, 0.0)],
                            [Complex::new(-100.0, 0.0), z]]);
    println!("{}", linear_combination(
        &[&m1, &m2],
        &[Complex::new(10.0, 0.0), Complex::new(-2.0, 0.0)],
    ));
}
