#[path = "common/mod.rs"]
mod common;

use common::display::{print_header, print_sep, print_title};
use matrix42::{Matrix, Vector};

fn main() {
    print_header("Ex00 : Add, Subtract and Scale");

    print_title("Vector tests (f32)");

    let mut u = Vector::from([2.0_f32, 3.0]);
    let v = Vector::from([5.0_f32, 7.0]);
    println!("u = {}\nv = {}", &u, &v);
    u.add(&v);
    println!("u + v = {}", &u);

    print_sep();

    let mut u = Vector::from([2.0_f32, 3.0]);
    let v = Vector::from([5.0_f32, 7.0]);
    println!("u = {}\nv = {}", &u, &v);
    u.sub(&v);
    println!("u - v = {}", &u);

    print_sep();

    let mut u = Vector::from([2.0_f32, 3.0]);
    println!("u = {}", &u);
    u.scl(2.0_f32);
    println!("u * 2 = {}", &u);

    print_title("Matrix tests (f32)");

    let mut m1 = Matrix::from([[1.0_f32, 2.0], [3.0, 4.0]]);
    let m2 = Matrix::from([[7.0_f32, 4.0], [-2.0, 2.0]]);
    println!("m1 =\n{}\nm2 =\n{}", &m1, &m2);
    m1.add(&m2);
    println!("m1 + m2 =\n{}", &m1);

    print_sep();

    let mut m3 = Matrix::from([[1.0_f32, 2.0], [3.0, 4.0]]);
    let m4 = Matrix::from([[7.0_f32, 4.0], [-2.0, 2.0]]);
    println!("m1 =\n{}\nm2 =\n{}", &m3, &m4);
    m3.sub(&m4);
    println!("m1 - m2 =\n{}", &m3);

    print_sep();

    let mut m5 = Matrix::from([[1.0_f32, 2.0], [3.0, 4.0]]);
    println!("m =\n{}", &m5);
    m5.scl(2.0_f32);
    println!("m * 2 =\n{}", &m5);
}
