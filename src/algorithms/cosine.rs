use crate::traits::Magnitude;
use crate::types::{Complex, Vector};

pub trait AngleCosine<K>: Sized {
    fn cosine(u: &Self, v: &Self) -> K;
}

pub fn angle_cos<K, T: AngleCosine<K>>(u: &T, v: &T) -> K {
    T::cosine(u, v)
}

impl AngleCosine<f32> for Vector<f32> {
    fn cosine(u: &Self, v: &Self) -> f32 {
        let dot = u.dot(v);
        let norm_u = u.norm();
        let norm_v = v.norm();
        if norm_u == 0.0 || norm_v == 0.0 {
            0.0
        } else {
            dot / (norm_u * norm_v)
        }
    }
}

impl AngleCosine<f32> for Vector<Complex> {
    fn cosine(u: &Self, v: &Self) -> f32 {
        let dot = u.dot(v);
        let norm_u = u.norm();
        let norm_v = v.norm();
        if norm_u == 0.0 || norm_v == 0.0 {
            0.0
        } else {
            dot.magnitude() / (norm_u * norm_v)
        }
    }
}
