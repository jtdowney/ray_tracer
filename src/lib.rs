mod bounds;
mod camera;
mod canvas;
pub mod color;
mod intersection;
mod light;
mod material;
mod matrix;
mod patterns;
mod point;
mod ray;
mod render;
mod shapes;
pub mod transforms;
mod vector;
mod world;

pub use crate::bounds::Bounds;
pub use crate::camera::Camera;
pub use crate::canvas::Canvas;
pub use crate::color::Color;
pub use crate::intersection::{Intersection, Intersections};
pub use crate::light::PointLight;
pub use crate::material::{Material, MaterialBuilder};
pub use crate::matrix::{Matrix, Matrix2, Matrix3, Matrix4};
pub use crate::patterns::{
    CheckersPattern, CheckersPatternBuilder, GradientPattern, GradientPatternBuilder, Pattern,
    RingPattern, RingPatternBuilder, SolidPattern, SolidPatternBuilder, StripePattern,
    StripePatternBuilder,
};
pub use crate::point::Point;
pub use crate::ray::Ray;
pub use crate::render::{render, render_parallel};
pub use crate::shapes::{
    Cone, ConeBuilder, Cube, CubeBuilder, Cylinder, CylinderBuilder, Group, GroupBuilder, Plane,
    PlaneBuilder, Shape, Sphere, SphereBuilder,
};
pub use crate::vector::{Vector, Vector3, Vector4};
pub use crate::world::{World, WorldBuilder};

const EPSILON: f64 = 1e-4;
