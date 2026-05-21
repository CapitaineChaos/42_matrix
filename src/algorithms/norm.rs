use crate::traits::Magnitude;
use crate::types::Vector;

// ============================================================
// Norm 1  ->  ||v||₁ = Σ|xᵢ|
// ============================================================

pub fn norm_1<K: Magnitude + Copy>(v: &Vector<K>) -> f32 {
    v.data
        .iter()
        .map(|&x| x.magnitude())
        .fold(0.0_f32, |acc, x| acc + x)
}

impl<K: Magnitude + Copy> Vector<K> {
    pub fn norm_1(&self) -> f32 {
        norm_1(self)
    }
}

// ============================================================
// Norm  ->  ||v||₂ = √(Σ|xᵢ|²)
// ============================================================

pub fn norm<K: Magnitude + Copy>(v: &Vector<K>) -> f32 {
    v.data
        .iter()
        .map(|&x| x.magnitude().powi(2))
        .fold(0.0_f32, |acc, x| acc + x)
        .powf(0.5)
}

impl<K: Magnitude + Copy> Vector<K> {
    pub fn norm(&self) -> f32 {
        norm(self)
    }
}

// ============================================================
// Norm Inf  ->  ||v||∞ = max|xᵢ|
// ============================================================

pub fn norm_inf<K: Magnitude + Copy>(v: &Vector<K>) -> f32 {
    v.data
        .iter()
        .map(|&x| x.magnitude())
        .fold(0.0_f32, f32::max)
}

impl<K: Magnitude + Copy> Vector<K> {
    pub fn norm_inf(&self) -> f32 {
        norm_inf(self)
    }
}
