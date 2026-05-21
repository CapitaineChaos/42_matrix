#[path = "common/mod.rs"]
mod common;

use common::display::{print_header, print_sep, print_title};
use matrix42::{Matrix, Vector, lerp};

fn main() {
    print_header("Ex02 : Linear Interpolation");

    print_title("Floats tests (f32)");
    println!("{}", lerp(0., 1., 0.));
    println!("{}", lerp(0., 1., 1.));
    println!("{}", lerp(0., 1., 0.5));
    println!("{}", lerp(21., 42., 0.3));

    print_title("Vector tests (f32)");

    let e1 = Vector::from([1.0_f32, 0.0, 0.0]);
    let e2 = Vector::from([0.0_f32, 1.0, 0.0]);
    println!("{}", lerp(e1, e2, 0.5_f32));
    // [0.5], [0.5], [0.]

    print_sep();

    let v1 = Vector::from([1.0_f32, 2.0, 3.0]);
    let v2 = Vector::from([0.0_f32, 10.0, -100.0]);
    println!("{}", lerp(v1, v2, 0.25_f32));
    // [0.75], [4.5], [-24.25]

    print_title("Matrix tests (f32)");

    let m1 = Matrix::from([[1.0_f32, 4.0], [6.0, 0.5]]);
    let m2 = Matrix::from([[0.0_f32, 10.0], [-100.0, 0.0]]);
    println!("{}", lerp(m1, m2, 0.25_f32));
}
