pub mod types;
pub mod impls;
pub mod algorithms;
pub mod traits;
pub mod prelude;

pub use types::{Complex, Matrix, Vector};
pub use algorithms::linear_combination;
pub use algorithms::linear_interpolation::lerp;
pub use algorithms::dot_product::dot_product;
pub use algorithms::norm::{norm, norm_1, norm_inf};
pub use algorithms::cosine::angle_cos;
pub use algorithms::cross_product::cross_product;
