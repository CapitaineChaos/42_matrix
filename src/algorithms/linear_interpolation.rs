use crate::traits::LinearElement;
use crate::types::{Matrix, Vector};

// ============================================================
// Trait
// ============================================================

pub trait Lerp<Rhs, K> {
    type Output;

    fn lerp(self, rhs: Rhs, t: K) -> Self::Output;
}

// ============================================================
// Public function
// ============================================================

pub fn lerp<U, V, K>(u: U, v: V, t: K) -> <U as Lerp<V, K>>::Output
where
    U: Lerp<V, K>,
{
    u.lerp(v, t)
}


impl<K: LinearElement> Lerp<K, K> for K {
    type Output = K;

    fn lerp(self, rhs: K, t: K) -> K {
        self + t * (rhs - self)
    }
}

impl Lerp<Complex, f32> for Complex {
    type Output = Complex;

    fn lerp(self, rhs: Complex, t: f32) -> Complex {
        self + (rhs - self) * t
    }
}

// ============================================================
// Vector
// ============================================================

impl<'a, K: LinearElement> Lerp<&'a Vector<K>, K> for &'a Vector<K> {
    type Output = Vector<K>;

    fn lerp(self, rhs: &'a Vector<K>, t: K) -> Vector<K> {
        assert_eq!(self.size(), rhs.size());

        let mut result = Vector::new(self.size());

        for (j, cell) in result.data.iter_mut().enumerate() {
            *cell = lerp(self[j], rhs[j], t);
        }

        result
    }
}

// ============================================================
// Matrix
// ============================================================

impl<'a, K: LinearElement> Lerp<&'a Matrix<K>, K> for &'a Matrix<K> {
    type Output = Matrix<K>;

    fn lerp(self, rhs: &'a Matrix<K>, t: K) -> Matrix<K> {
        assert_eq!(self.shape(), rhs.shape());

        let mut result = Matrix::new(self.shape());

        for (j, cell) in result.data.iter_mut().enumerate() {
            *cell = lerp(self[j], rhs[j], t);
        }

        result
    }
}
