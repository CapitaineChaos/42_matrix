use crate::types::Complex;

/// Module (valeur absolue) d'un scalaire, retourné en f32.
/// f32  → |x|
/// Complex → √(re² + im²)
pub trait Magnitude {
    fn magnitude(self) -> f32;
}

impl Magnitude for f32 {
    fn magnitude(self) -> f32 {
        self.max(-self)
    }
}

impl Magnitude for Complex {
    fn magnitude(self) -> f32 {
        (self.re * self.re + self.im * self.im).powf(0.5)
    }
}
