mod camera;
mod canvas;
mod color;
mod intersection;
mod light;
mod material;
pub mod matrix;
mod objects;
mod point;
mod ray;
mod render;
mod scalar;
pub mod transforms;
mod vector;
pub mod world;

pub use crate::camera::Camera;
pub use crate::canvas::Canvas;
pub use crate::color::Color;
pub use crate::intersection::{Intersection, Intersections};
pub use crate::light::PointLight;
pub use crate::material::Material;
pub use crate::matrix::{Matrix, Matrix2, Matrix3, Matrix4};
pub use crate::objects::Sphere;
pub use crate::point::Point;
pub use crate::ray::Ray;
pub use crate::render::render;
pub use crate::scalar::Scalar;
pub use crate::vector::{Vector, Vector3, Vector4};
pub use crate::world::World;
