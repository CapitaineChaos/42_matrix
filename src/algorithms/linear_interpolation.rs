use crate::types::{Complex, Matrix, Vector};

// ============================================================
// Trait
// ============================================================

pub trait Lerp {
    type Output;

    fn lerp(self, rhs: Self, t: f32) -> Self::Output;
}

// ============================================================
// Public function
// ============================================================

pub fn lerp<V: Lerp>(u: V, v: V, t: f32) -> V::Output {
    u.lerp(v, t)
}

// ============================================================
// f32
// ============================================================

impl Lerp for f32 {
    type Output = f32;

    fn lerp(self, rhs: f32, t: f32) -> f32 {
        self + t * (rhs - self)
    }
}

// ============================================================
// Complex
// ============================================================

impl Lerp for Complex {
    type Output = Complex;

    fn lerp(self, rhs: Complex, t: f32) -> Complex {
        self + (rhs - self) * Complex { re: t, im: 0.0 }
    }
}

// ============================================================
// Vector
// ============================================================

impl Lerp for Vector<f32> {
    type Output = Vector<f32>;

    fn lerp(self, rhs: Vector<f32>, t: f32) -> Vector<f32> {
        assert_eq!(self.size(), rhs.size());
        Vector {
            size: self.size,
            data: self.data.iter().zip(&rhs.data).map(|(&a, &b)| lerp(a, b, t)).collect(),
        }
    }
}

// ============================================================
// Matrix
// ============================================================

impl Lerp for Matrix<f32> {
    type Output = Matrix<f32>;

    fn lerp(self, rhs: Matrix<f32>, t: f32) -> Matrix<f32> {
        assert_eq!(self.shape(), rhs.shape());
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: self.data.iter().zip(&rhs.data).map(|(&a, &b)| lerp(a, b, t)).collect(),
        }
    }
}
