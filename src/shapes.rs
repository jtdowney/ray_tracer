use crate::{Intersections, Material, Matrix4, Point, Ray, Vector3};
use std::any::Any;
use std::fmt::Debug;

mod plane;
mod sphere;

pub use self::plane::Plane;
pub use self::sphere::Sphere;

pub trait Shape: Any + Debug {
    fn as_any(&self) -> &Any;
    fn as_any_mut(&mut self) -> &mut Any;
    fn local_normal_at(&self, point: Point) -> Vector3;
    fn local_intersect(&self, ray: Ray) -> Intersections;
    fn material(&self) -> &Material;
    fn transform(&self) -> &Matrix4;

    fn normal_at(&self, point: Point) -> Vector3 {
        let local_point = self.transform().inverse() * point;
        let local_normal = self.local_normal_at(local_point);
        let world_normal = self.transform().inverse().transpose() * local_normal;

        world_normal.normalize()
    }

    fn intersect(&self, ray: Ray) -> Intersections {
        let local_ray = ray.transform(self.transform().inverse());
        self.local_intersect(local_ray)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transforms;
    use std::cell::RefCell;
    use std::f64::consts::PI;

    #[derive(Debug, Default)]
    struct TestShape {
        transform: Matrix4,
        material: Material,
        saved_ray: RefCell<Option<Ray>>,
    }

    impl Shape for TestShape {
        fn as_any(&self) -> &Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut Any {
            self
        }

        fn local_normal_at(&self, Point { x, y, z }: Point) -> Vector3 {
            Vector3::new(x, y, z)
        }

        fn local_intersect(&self, ray: Ray) -> Intersections {
            *self.saved_ray.borrow_mut() = Some(ray);
            Intersections(vec![])
        }

        fn material(&self) -> &Material {
            &self.material
        }

        fn transform(&self) -> &Matrix4 {
            &self.transform
        }
    }

    #[test]
    fn test_intersecting_scaled_shape_with_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let mut s = TestShape::default();
        s.transform = transforms::scaling(2.0, 2.0, 2.0);
        s.intersect(r);
        assert_eq!(
            Point::new(0.0, 0.0, -2.5),
            s.saved_ray.borrow().unwrap().origin
        );
        assert_eq!(
            Vector3::new(0.0, 0.0, 0.5),
            s.saved_ray.borrow().unwrap().direction
        );
    }

    #[test]
    fn test_intersecting_translated_shape_with_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let mut s = TestShape::default();
        s.transform = transforms::translation(5.0, 0.0, 0.0);
        s.intersect(r);
        assert_eq!(
            Point::new(-5.0, 0.0, -5.0),
            s.saved_ray.borrow().unwrap().origin
        );
        assert_eq!(
            Vector3::new(0.0, 0.0, 1.0),
            s.saved_ray.borrow().unwrap().direction
        );
    }

    #[test]
    fn test_normal_on_translated_shape() {
        let mut s = TestShape::default();
        s.transform = transforms::translation(0.0, 1.0, 0.0);
        assert_eq!(
            Vector3::new(0.0, 0.70711, -0.70711),
            s.normal_at(Point::new(0.0, 1.70711, -0.70711))
        );
    }

    #[test]
    fn test_normal_on_transformed_shape() {
        let mut s = TestShape::default();
        s.transform = transforms::scaling(1.0, 0.5, 1.0) * transforms::rotation_z(PI / 5.0);
        assert_eq!(
            Vector3::new(0.0, 0.97014, -0.24254),
            s.normal_at(Point::new(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0))
        );
    }
}
