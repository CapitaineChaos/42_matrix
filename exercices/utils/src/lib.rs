/// Affiche une valeur en la précédant de son label.
/// Utilisé dans tous les exercices pour uniformiser les sorties.
pub fn print_result(label: &str, value: &str) {
    println!("{}: {}", label, value);
}


pub fn print_vector<K: std::fmt::Debug>(vector: &vector<K>) {
    println!("Vector (size: {})", vector.size);
    for i in 0..vector.size {
        println!("  [{}] = {:?}", i, vector[i]);
    }
}
