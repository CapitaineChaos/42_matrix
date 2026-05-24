#[path = "common/mod.rs"]
mod common;

use common::display::{print_header, print_sep, print_title};
use matrix42::{Vector, angle_cos};

fn main() {
    print_header("Ex05 : Cosine");

    print_title("Cosine tests (f32)");
    let u1 = Vector::from([-1.0_f32, 0.0]);
    let v1 = Vector::from([0.0_f32, 1.0]);
    println!("{}", angle_cos(&u1, &v1));

    print_sep();

    let u2 = Vector::from([1.0_f32, 0.0]);
    let v2 = Vector::from([1.0_f32, 0.0]);
    println!("{}", angle_cos(&u2, &v2)); 

    print_sep();

    let u3 = Vector::from([1.0_f32, 1.0]);
    let v3 = Vector::from([-1.0_f32, 1.0]);
    println!("{}", angle_cos(&u3, &v3));

    print_sep();

    let u4 = Vector::from([0.3_f32, 1.0, 0.5]);
    let v4 = Vector::from([1.0_f32, -1.0, 2.5]);
    println!("{}", angle_cos(&u4, &v4));

    
    print_sep();
    
    let u5 = Vector::from([1.0_f32, 0.0]);
    let v5 = Vector::from([-1.0_f32, 0.0]);
    println!("{}", angle_cos(&u5, &v5)); // -1

    print_sep();

    let u6 = Vector::from([1.0_f32, 0.0]);
    let v6 = Vector::from([-1.0_f32, 1.0]);
    println!("{}", angle_cos(&u6, &v6)); // environ -0.70710677

}
