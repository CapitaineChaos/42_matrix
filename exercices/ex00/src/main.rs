// Un module local défini dans le fichier vector.rs
// On le déclare ici pour que Rust le compile avec ce crate.
mod vector;

// On importe la fonction depuis la crate utils (dossier ../utils)
use utils::print_result;

// On importe notre type local
use vector::Vector;

fn main() {
    let v = Vector::new(1.0, 2.0, 3.0);
    let w = Vector::new(4.0, 5.0, 6.0);

    let sum = v.add(&w);
    print_result("v + w", &format!("{:?}", sum));

    let scaled = v.scale(2.0);
    print_result("v * 2", &format!("{:?}", scaled));
}
