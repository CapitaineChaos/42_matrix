#[path = "common/mod.rs"]
mod common;

use common::display::{print_header, print_sep, print_title};
use matrix42::{Vector, dot_product};

fn main() {
    print_header("Ex03 : Dot Product");

    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([4., 5.]);
    println!("{}", u.dot(&v));

}

// let mut u = Vector::from([0., 0.]);
// let v = Vector::from([1., 1.]);
// println!("{}", u.dot(v));
// // 0.0
// let mut u = Vector::from([1., 1.]);
// let v = Vector::from([1., 1.]);
// println!("{}", u.dot(v));
// // 2.0
// let mut u = Vector::from([-1., 6.]);
// let v = Vector::from([3., 2.]);
// println!("{}", u.dot(v));
// // 9.0