use crate::{Intersection, Material, Matrix4, Point, Ray, Vector, identity_matrix, material};

mod sphere;

pub use sphere::sphere;

pub trait Geometry {
    fn local_intersection<'shape>(
        &self,
        shape: &'shape Shape,
        ray: Ray,
    ) -> Vec<Intersection<'shape>>;
    fn local_normal_at(&self, point: Point) -> Vector;
}

pub struct Shape {
    pub transform: Matrix4,
    pub material: Material,
    geometry: Box<dyn Geometry>,
}

impl Shape {
    /// # Panics
    /// Panics if the shape's transform matrix is not invertible.
    #[must_use]
    pub fn intersect(&self, ray: Ray) -> Vec<Intersection<'_>> {
        let local_ray = ray.transform(self.transform.inverse().expect("invertable"));
        self.geometry.local_intersection(self, local_ray)
    }

    /// # Panics
    /// Panics if the shape's transform matrix is not invertible.
    #[must_use]
    pub fn normal_at(&self, world_point: Point) -> Vector {
        let inverse = self.transform.inverse().expect("invertible");
        let object_point = inverse * world_point;
        let object_normal = self.geometry.local_normal_at(object_point);
        let world_normal = inverse.transpose() * object_normal;
        world_normal.normalize()
    }
}

impl<G: Geometry + 'static> From<G> for Shape {
    fn from(geometry: G) -> Self {
        Shape {
            transform: identity_matrix(),
            material: material(),
            geometry: Box::new(geometry),
        }
    }
}
