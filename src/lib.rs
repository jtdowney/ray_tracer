mod canvas;
pub mod color;
mod intersection;
mod light;
mod material;
mod matrix;
pub mod point;
mod ray;
mod sphere;
pub mod transformations;
mod vector;

pub use canvas::*;
pub use color::*;
pub use intersection::*;
pub use light::*;
pub use material::*;
pub use matrix::*;
pub use point::*;
pub use ray::*;
pub use sphere::*;
pub use vector::*;

pub const EPSILON: f64 = 0.0001;
