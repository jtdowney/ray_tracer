use std::{any::Any, fmt::Debug, ptr};

use crate::{
    identity_matrix, intersection::Intersection, material, Material, Matrix4, Point, Ray, Vector,
};

pub mod cube;
pub mod plane;
pub mod sphere;

pub trait Geometry: 'static + Debug + Sync {
    fn local_intersection<'a>(&'a self, shape: &'a Shape, ray: Ray) -> Vec<Intersection>;
    fn local_normal_at(&self, point: Point) -> Vector;
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
pub struct Shape {
    pub transform: Matrix4,
    pub material: Material,
    geometry: Box<dyn Geometry>,
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        self.transform == other.transform && ptr::eq(&self.geometry, &other.geometry)
    }
}

impl Shape {
    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let local_ray = ray.transform(self.transform.inverse());
        self.geometry.local_intersection(self, local_ray)
    }

    pub fn normal_at(&self, world_point: Point) -> Vector {
        let inv = self.transform.inverse();
        let local_point = inv * world_point;
        let local_normal = self.geometry.local_normal_at(local_point);
        let world_normal = inv.transpose() * local_normal;
        world_normal.normalize()
    }
}

impl<G: Geometry> From<G> for Shape {
    fn from(geometry: G) -> Self {
        Shape {
            transform: identity_matrix(),
            material: material(),
            geometry: Box::new(geometry),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        f64::consts::PI,
        sync::{Arc, Mutex},
    };

    use approx::assert_abs_diff_eq;

    use crate::{
        point, ray,
        transform::{rotation_z, scaling, translation},
        vector,
    };

    use super::*;

    #[derive(Clone, Debug, Default)]
    struct TestShape {
        saved_ray: Arc<Mutex<Option<Ray>>>,
    }

    impl Geometry for TestShape {
        fn local_intersection<'a>(&'a self, _shape: &'a Shape, ray: Ray) -> Vec<Intersection> {
            let mut saved_ray = self.saved_ray.lock().unwrap();
            *saved_ray = Some(ray);
            vec![]
        }

        fn local_normal_at(&self, Point { x, y, z }: Point) -> Vector {
            vector(x, y, z)
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    fn test_shape() -> Shape {
        TestShape::default().into()
    }

    #[test]
    fn intersecting_scaled_shape() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let mut s = test_shape();
        s.transform = scaling(2, 2, 2);
        s.intersect(r);

        let shape = s.geometry.as_any().downcast_ref::<TestShape>().unwrap();
        let saved_ray = shape.saved_ray.lock().unwrap().unwrap();
        assert_eq!(point(0.0, 0.0, -2.5), saved_ray.origin);
        assert_eq!(vector(0.0, 0.0, 0.5), saved_ray.direction);
    }

    #[test]
    fn intersecting_translated_shape() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let mut s = test_shape();
        s.transform = translation(5, 0, 0);
        s.intersect(r);

        let shape = s.geometry.as_any().downcast_ref::<TestShape>().unwrap();
        let saved_ray = shape.saved_ray.lock().unwrap().unwrap();
        assert_eq!(point(-5, 0, -5), saved_ray.origin);
        assert_eq!(vector(0, 0, 1), saved_ray.direction);
    }

    #[test]
    fn computing_normal_on_translated_shape() {
        let mut s = test_shape();
        s.transform = translation(0, 1, 0);
        assert_abs_diff_eq!(
            vector(0.0, 0.70711, -0.70711),
            s.normal_at(point(0.0, 1.70711, -0.70711))
        );
    }

    #[test]
    fn computing_normal_on_transformed_shape() {
        let mut s = test_shape();
        s.transform = scaling(1.0, 0.5, 1.0) * rotation_z(PI / 5.0);
        assert_abs_diff_eq!(
            vector(0.0, 0.97014, -0.24254),
            s.normal_at(point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0))
        );
    }
}
