use matrix42::vector::Vector;
use matrix42::display::{print_header, print_title, print_sep};

use matrix42::linear_combination;

fn main() {

    print_header("Ex01 : Linear combination");

    print_title("Vector tests (Real)");

    let e1 = Vector::from([1., 0., 0.]);
    let e2 = Vector::from([0., 1., 0.]);
    let e3 = Vector::from([0., 0., 1.]);
    let v1 = Vector::from([1., 2., 3.]);
    let v2 = Vector::from([0., 10., -100.]);
    println!("{}", linear_combination(&[&e1, &e2, &e3], &[10.0, -2.0, 0.5]));
    // [10.]
    // [-2.]
    // [0.5]
    println!("{}", linear_combination(&[&v1, &v2], &[10.0, -2.0]));
    // [10.]
    // [0.]
    // [230.]

    print_sep();

    print_title("Matrix tests (Real)");

    let m1 = matrix42::matrix::Matrix::from([[1., 4.], [6., 0.5]]);
    let m2 = matrix42::matrix::Matrix::from([[0., 10.], [-100., 0.]]);
    println!("{}", linear_combination(&[&m1, &m2], &[10.0, -2.0]));
    // [[10., 20.], [260., 5.]]
}