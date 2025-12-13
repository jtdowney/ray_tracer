mod camera;
mod canvas;
pub mod color;
mod intersection;
mod light;
mod material;
mod matrix;
mod obj_parser;
pub mod pattern;
mod point;
mod ray;
pub mod shape;
pub mod transform;
mod vector;
mod world;

pub use camera::{Camera, camera};
pub use canvas::{Canvas, canvas, canvas_with_pixels};
pub use color::{Color, color};
pub use intersection::{Intersection, hit, intersection, intersection_with_uv};
pub use light::{PointLight, point_light};
pub use material::{Material, material};
pub use matrix::{Matrix, Matrix2, Matrix3, Matrix4, identity_matrix, matrix};
pub use obj_parser::ObjParser;
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
