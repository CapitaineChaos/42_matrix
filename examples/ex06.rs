#[path = "common/mod.rs"]
mod common;

use common::display::{print_header, print_sep, print_title};
use matrix42::{Vector, cross_product};

fn main() {
    print_header("Ex06 : Cross Product");

    print_title("Cross product tests (f32)");
    let u1 = Vector::from([1.0_f32, 0.0, 0.0]);
    let v1 = Vector::from([0.0_f32, 1.0, 0.0]);
    println!("{}", cross_product(&u1, &v1)); // [0, 0, 1]

    print_sep();

    let u2 = Vector::from([1.0_f32, 2.0, 3.0]);
    let v2 = Vector::from([4.0_f32, 5.0, 6.0]);
    println!("{}", cross_product(&u2, &v2)); // [-3, 6, -3]

    print_sep();
}