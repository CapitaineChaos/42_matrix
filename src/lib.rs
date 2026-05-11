pub mod types;
pub mod impls;
pub mod algorithms;
pub mod traits;
pub mod prelude;

pub use types::{Complex, Matrix, Vector};
pub use algorithms::linear_combination;
pub use algorithms::linear_interpolation::lerp;
