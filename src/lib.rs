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

pub use camera::{camera, Camera};
pub use canvas::Canvas;
pub use color::{color, Color};
pub use intersection::{hit, intersection};
pub use light::{point_light, PointLight};
pub use material::{material, Material};
pub use matrix::{identity_matrix, matrix, Matrix, Matrix2, Matrix3, Matrix4};
pub use pattern::{checkers_pattern, gradiant_pattern, ring_pattern, stripe_pattern, Pattern};
pub use point::{point, Point};
pub use ray::{ray, Ray};
pub use shapes::cone::{cone, Cone};
pub use shapes::cube::{cube, Cube};
pub use shapes::cylinder::{cylinder, Cylinder};
pub use shapes::plane::{plane, Plane};
pub use shapes::sphere::{sphere, Sphere};
pub use shapes::Shape;
pub use vector::{vector, Vector};
pub use world::{default_world, world, World};

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
