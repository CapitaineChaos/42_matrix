use std::ops::{Add, Sub, Mul};

use crate::{common::{Complexe, Reel}, vector::Vector};

pub fn linear_combination<K>(u: &[Vector<K>], coeffs: &[K]) -> Vector<K>
where K: Copy + Add<Output = K> + Sub<Output = K> + Mul<Output = K> + Default,
{
    assert!(u.len() == coeffs.len(), "The number of vectors and coefficients must be the same");

    let size = u[0].size();
    let mut result = Vector::new(size);

    for (v, &c) in u.iter().zip(coeffs.iter()) {
        let mut temp = v.clone();
        temp.scl(c);
        result.add(temp);
    }

    result
}
