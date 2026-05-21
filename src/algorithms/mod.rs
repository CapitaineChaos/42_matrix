pub mod linear_combination;
pub mod linear_interpolation;
pub mod dot_product;
pub mod norm;

pub use linear_combination::linear_combination;
pub use linear_interpolation::lerp;
pub use dot_product::dot_product;
pub use norm::{norm, norm_1, norm_inf};