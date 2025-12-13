mod camera;
mod canvas;
pub mod color;
mod intersection;
mod light;
mod material;
mod matrix;
pub mod pattern;
mod point;
mod ray;
pub mod shape;
pub mod transform;
mod vector;
mod world;

pub use camera::{Camera, camera};
pub use canvas::{Canvas, canvas};
pub use color::{Color, color};
pub use intersection::{Intersection, hit, intersection};
pub use light::{PointLight, point_light};
pub use material::{Material, material};
pub use matrix::{Matrix, Matrix2, Matrix3, Matrix4, identity_matrix, matrix};
pub use point::{ORIGIN, Point, point};
pub use ray::{Ray, ray};
pub use shape::*;
pub use vector::{Vector, vector};
pub use world::{World, default_world};

pub const EPSILON: f32 = 0.0001;
pub const REFLECTION_DEPTH: usize = 5;

#[must_use]
pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    value.max(min).min(max)
}
