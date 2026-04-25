use matrix42::vector::Vector;
use matrix42::display::{print_header, print_title, print_sep};
use matrix42::complexe::Complexe;


use matrix42::linear_combination;

fn main() {

    print_header("Ex01 : Linear combination");

    print_title("Vector tests (Complexe)");

    let e1 = Vector::from([Complexe::new(1., 0.), Complexe::new(0., 0.), Complexe::new(0., 0.)]);
    let e2 = Vector::from([Complexe::new(0., 0.), Complexe::new(1., 0.), Complexe::new(0., 0.)]);
    let e3 = Vector::from([Complexe::new(0., 0.), Complexe::new(0., 0.), Complexe::new(1., 0.)]);
    let v1 = Vector::from([Complexe::new(1., 0.), Complexe::new(2., 0.), Complexe::new(3., 0.)]);
    let v2 = Vector::from([Complexe::new(0., 0.), Complexe::new(10., 0.), Complexe::new(-100., 0.)]);
    println!("{}", linear_combination(&[&e1, &e2, &e3], &[Complexe::new(10., 0.), Complexe::new(-2., 0.), Complexe::new(0.5, 0.)]));
    // [10.]
    // [-2.]
    // [0.5]
    println!("{}", linear_combination(&[&v1, &v2], &[Complexe::new(10., 0.), Complexe::new(-2., 0.)]));
    // [10.]
    // [0.]
    // [230.]

    print_sep();

    print_title("Matrix tests (Complexe)");

    let m1 = matrix42::matrix::Matrix::from([[Complexe::new(1., 0.), Complexe::new(4., 0.)], [Complexe::new(6., 0.), Complexe::new(0.5, 0.)]]);
    let m2 = matrix42::matrix::Matrix::from([[Complexe::new(0., 0.), Complexe::new(10., 0.)], [Complexe::new(-100., 0.), Complexe::new(0., 0.)]]);
    println!("{}", linear_combination(&[&m1, &m2], &[Complexe::new(10., 0.), Complexe::new(-2., 0.)]));
    // [[10., 20.], [260., 5.]]
}