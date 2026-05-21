#[path = "common/mod.rs"]
mod common;

use common::display::{print_header, print_sep, print_title};
use matrix42::Vector;

fn main() {
    print_header("Ex04 : Norms");

    print_title("Norm 1 tests (f32)");
    let v1 = Vector::from([1.0_f32, -2.0, 3.0]);
    println!("{}", v1.norm_1());
    // 6.0

    print_sep();

    print_title("Norm tests (f32)");
    let v2 = Vector::from([3.0_f32, 4.0]);
    println!("{}", v2.norm());
    // 5.0

    print_sep();

    print_title("Norm Inf tests (f32)");
    let v3 = Vector::from([-1.0_f32, -5.0, 3.0]);
    println!("{}", v3.norm_inf());
    // 5.0
}
