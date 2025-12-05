mod canvas;
pub mod color;
mod intersection;
mod matrix;
mod point;
mod ray;
pub mod shape;
pub mod transform;
mod vector;

pub use canvas::{Canvas, canvas};
pub use color::{Color, color};
pub use intersection::{Intersection, hit, intersection};
pub use matrix::{Matrix, Matrix2, Matrix3, Matrix4, identity_matrix, matrix};
pub use point::{ORIGIN, Point, point};
pub use ray::{Ray, ray};
pub use shape::*;
pub use vector::{Vector, vector};

pub const EPSILON: f64 = 0.0001;

#[must_use]
pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    value.max(min).min(max)
}
