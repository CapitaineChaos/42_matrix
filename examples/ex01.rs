#[path = "common/mod.rs"]
mod common;

use common::display::{print_header, print_sep, print_title};
use matrix42::{Matrix, Vector, linear_combination};

fn main() {
    print_header("Ex01 : Linear Combination");

    print_title("Vector tests (f32)");

    let e1 = Vector::from([1.0_f32, 0.0, 0.0]);
    let e2 = Vector::from([0.0_f32, 1.0, 0.0]);
    let e3 = Vector::from([0.0_f32, 0.0, 1.0]);
    println!("{}", linear_combination(&[&e1, &e2, &e3], &[10.0_f32, -2.0, 0.5]));
    // [10.], [-2.], [0.5]

    print_sep();

    let v1 = Vector::from([1.0_f32, 2.0, 3.0]);
    let v2 = Vector::from([0.0_f32, 10.0, -100.0]);
    println!("{}", linear_combination(&[&v1, &v2], &[10.0_f32, -2.0]));
    // [10.], [0.], [230.]

    print_title("Matrix tests (f32)");

    let m1 = Matrix::from([[1.0_f32, 4.0], [6.0, 0.5]]);
    let m2 = Matrix::from([[0.0_f32, 10.0], [-100.0, 0.0]]);
    println!("{}", linear_combination(&[&m1, &m2], &[10.0_f32, -2.0]));
}
