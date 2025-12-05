use crate::{Intersection, Matrix4, Ray, identity_matrix};

mod sphere;

pub use sphere::{Sphere, sphere};

pub trait Geometry {
    fn local_intersection<'shape>(
        &self,
        shape: &'shape Shape,
        ray: Ray,
    ) -> Vec<Intersection<'shape>>;
}

pub struct Shape {
    pub transform: Matrix4,
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
}

impl<G: Geometry + 'static> From<G> for Shape {
    fn from(geometry: G) -> Self {
        Shape {
            transform: identity_matrix(),
            geometry: Box::new(geometry),
        }
    }
}
