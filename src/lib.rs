mod camera;
mod canvas;
pub mod color;
mod intersection;
mod light;
mod material;
mod matrix;
pub mod point;
mod ray;
mod shape;
pub mod transformations;
mod vector;
mod world;

pub use camera::*;
pub use canvas::*;
pub use color::*;
pub use intersection::*;
pub use light::*;
pub use material::*;
pub use matrix::*;
pub use point::*;
pub use ray::*;
pub use shape::*;
pub use vector::*;
pub use world::*;

pub const EPSILON: f64 = 0.0001;
