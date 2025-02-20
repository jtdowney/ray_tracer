mod camera;
mod canvas;
mod color;
mod intersection;
mod light;
mod material;
mod matrix;
mod pattern;
mod point;
mod ray;
mod shapes;
pub mod transform;
mod vector;
mod world;

pub use camera::{Camera, camera};
pub use canvas::Canvas;
pub use color::{Color, color};
pub use intersection::{hit, intersection};
pub use light::{PointLight, point_light};
pub use material::{Material, material};
pub use matrix::{Matrix, Matrix2, Matrix3, Matrix4, identity_matrix, matrix};
pub use pattern::{Pattern, checkers_pattern, gradiant_pattern, ring_pattern, stripe_pattern};
pub use point::{Point, point};
pub use ray::{Ray, ray};
pub use shapes::Shape;
pub use shapes::cone::{Cone, cone};
pub use shapes::cube::{Cube, cube};
pub use shapes::cylinder::{Cylinder, cylinder};
pub use shapes::plane::{Plane, plane};
pub use shapes::sphere::{Sphere, sphere};
pub use vector::{Vector, vector};
pub use world::{World, default_world, world};

pub const EPSILON: f64 = 0.0001;
pub const REFLECTION_DEPTH: u8 = 5;

pub const ORIGIN: Point = Point {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};
pub const BLACK: Color = Color {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
};
pub const WHITE: Color = Color {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
};

pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    value.max(min).min(max)
}
