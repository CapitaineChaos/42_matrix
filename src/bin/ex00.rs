// use matrix42::matrix::Matrix;
// use matrix42::common::Complexe;
use matrix42::{common::print_sep, vector::Vector};

fn main() {
    let mut u = Vector::from([2.0, 3.0]);
    let v = Vector::from([5.0, 7.0]);

    let mut w = Vector::new(2);
    w[0] = u[0] + v[0];
    println!("{}", w);
    w = &u + &v;
    println!("{}", w);

    // w.add(2);
    print_sep(2,'=', 45);
    u.add(&v);
    println!("{}", u);

    println!("{}", &w * 4.0);
    println!("{}", &w * 4.0);
    println!("{}", 4.0 * w);
}
