pub mod core;
pub mod scalar;
pub mod ops;
pub mod prelude;

pub use core::{Matrix, Vector};
pub use scalar::Complex;
pub use ops::{linear_combination, LinearCombination};
