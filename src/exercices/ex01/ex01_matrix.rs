use std::ops::{Add, Mul};
use num_traits::MulAdd;

use crate::matrix::Matrix;
use crate::LinearCombination;

impl<K> LinearCombination<K> for Matrix<K>
where K: Copy + Add<Output = K> + Mul<Output = K> + MulAdd<Output = K> + Default,
{
    fn lc(m: &[&Self], coeffs: &[K]) -> Self {
        assert!(m.len() == coeffs.len(), "The number of matrices and coefficients must be the same");
        let shape = m[0].shape();
        let size = m[0].size();
        let mut result = Matrix::new(m[0].shape());

        for i in 0..m.len() {
            assert!(m[i].shape() == shape, "All matrices must be of the same shape for linear combination");
            for j in 0..size {
                result[j] = coeffs[i].mul_add(m[i][j], result[j]);
            }
        }

        result
    }
}


