use crate::traits::LinearElement;
use crate::types::Vector;

pub trait CrossProduct<K>: Sized {
    fn cross(u: &Self, v: &Self) -> Self;
}

pub fn cross_product<K, T: CrossProduct<K>>(u: &T, v: &T) -> T {
    T::cross(u, v)
}

impl<K: LinearElement> CrossProduct<K> for Vector<K> {
    fn cross(u: &Self, v: &Self) -> Self {
        if u.size() != 3 || v.size() != 3 {
            panic!("Cross product is only defined for 3D vectors");
        }
        let x = u.data[1] * v.data[2] - u.data[2] * v.data[1];
        let y = u.data[2] * v.data[0] - u.data[0] * v.data[2];
        let z = u.data[0] * v.data[1] - u.data[1] * v.data[0];
        Vector::from([x, y, z])
    }
}
