use matrix42::matrix::Matrix;
use matrix42::vector::Vector;
use matrix42::common::Complexe;
use matrix42::common::{print_header, print_sep, print_title};

fn main() {
    
    
    print_header("Ex00 : Add, Substract and Scale");


    print_title("Vector tests (Complexe)");

    let c1 = Complexe::new(2.0, 3.0);
    let c2 = Complexe::new(5.0, 7.0);
    let c3 = Complexe::new(1.0, 1.0);
    let c4 = Complexe::new(0.0, 1.0);
    
    println!("Testing addition:");
    let mut u1 = Vector::from([c1, c2]);
    let v1 = Vector::from([c3, c4]);
    println!("u = {}\nv = {}", &u1, &v1);
    u1.add(v1);
    println!("u + v = {}", &u1);

    print_sep();

    println!("Testing subtraction:");
    let mut u2 = Vector::from([c1, c2]);
    let v2 = Vector::from([c3, c4]);
    println!("u = {}\nv = {}", &u2, &v2);
    u2.sub(v2);
    println!("u - v = {}", &u2);

    print_sep();

    println!("Testing scalar multiplication:");
    let mut u3 = Vector::from([c1, c2]);
    let scalar = Complexe::new(2.0, 3.0);
    println!("u = {}\ns = {}", &u3, scalar);
    u3.scl(scalar);
    println!("u * s = {}", &u3);


    let c5 = Complexe::new(1.0, 2.0);
    let c6 = Complexe::new(3.0, 4.0);
    let c7 = Complexe::new(0.5, -2.0);
    let c8 = Complexe::new(7.0, -1.0);
    print_title("Matrix tests (Complexe)");

    println!("Testing addition:");
    let mut m1 = Matrix::from([[c1, c2], [c3, c4]]);
    let m2 = Matrix::from([[c5, c6], [c7, c8]]);
    println!("m1 = \n{}\nm2 = \n{}", &m1, &m2);
    m1.add(m2);
    println!("m1 + m2 = \n{}", &m1);

    print_sep();

    println!("Testing subtraction:");
    let mut m3 = Matrix::from([[c1, c2], [c3, c4]]);
    let m4 = Matrix::from([[c5, c6], [c7, c8]]);
    println!("m1 = \n{}\nm2 = \n{}", &m3, &m4);
    m3.sub(m4);
    println!("m1 - m2 = \n{}", &m3);

    print_sep();

    println!("Testing scalar multiplication:");
    let mut m5 = Matrix::from([[c1, c2], [c3, c4]]);
    let scalar = Complexe::new(2.0, 3.0);
    println!("m = \n{}\ns = {}", &m5, scalar);
    m5.scl(scalar);
    println!("m * s = \n{}", &m5);
    
}
