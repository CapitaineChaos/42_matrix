use std::ops::{Add, Mul};
use num_traits::MulAdd;

use crate::vector::Vector;
use crate::LinearCombination;

impl<K> LinearCombination<K> for Vector<K>
where K: Copy + Add<Output = K> + Mul<Output = K> + MulAdd<Output = K> + Default,
{
    fn lc(u: &[&Self], coeffs: &[K]) -> Self {
        assert!(u.len() == coeffs.len(), "The number of vectors and coefficients must be the same");

        let size = u[0].size();
        let mut result = Vector::new(size);

        for i in 0..u.len() {
            assert!(u[i].size() == size, "All vectors must be of the same size for linear combination");
            for j in 0..size {
                result[j] = coeffs[i].mul_add(u[i][j], result[j]);
            }
        }

        result
    }
}


