use crate::{Intersections, Material, Matrix4, Point, Ray, Vector3};
use std::any::Any;
use std::fmt::Debug;

mod cone;
mod cube;
mod cylinder;
mod plane;
mod sphere;

pub use self::cone::{Cone, ConeBuilder};
pub use self::cube::{Cube, CubeBuilder};
pub use self::cylinder::{Cylinder, CylinderBuilder};
pub use self::plane::{Plane, PlaneBuilder};
pub use self::sphere::{Sphere, SphereBuilder};

pub trait Shape: Any + Debug {
    fn as_any(&self) -> &Any;
    fn as_any_mut(&mut self) -> &mut Any;
    fn box_clone(&self) -> Box<Shape + Sync + Send>;
    fn local_normal_at(&self, point: Point) -> Vector3;
    fn local_intersect(&self, ray: Ray) -> Intersections;
    fn material(&self) -> &Material;
    fn transform(&self) -> &Matrix4;

    fn normal_at(&self, point: Point) -> Vector3 {
        let transform_inverse = self.transform().inverse();
        let local_point = transform_inverse * point;
        let local_normal = self.local_normal_at(local_point);
        let world_normal = transform_inverse.transpose() * local_normal;

        world_normal.normalize()
    }

    fn intersect(&self, ray: Ray) -> Intersections {
        let local_ray = ray.transform(self.transform().inverse());
        self.local_intersect(local_ray)
    }
}

impl Clone for Box<Shape + Sync + Send> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

impl PartialEq for &Shape {
    fn eq(&self, other: &&Shape) -> bool {
        std::ptr::eq(*self, *other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transforms;
    use std::cell::RefCell;
    use std::f64::consts::PI;
    use std::sync::Mutex;

    #[derive(Debug)]
    struct TestShape {
        transform: Matrix4,
        material: Material,
        saved_ray: Mutex<RefCell<Option<Ray>>>,
    }

    impl Default for TestShape {
        fn default() -> Self {
            TestShape {
                transform: Matrix4::identity(),
                material: Material::default(),
                saved_ray: Mutex::new(RefCell::new(None)),
            }
        }
    }

    impl Shape for TestShape {
        fn as_any(&self) -> &Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut Any {
            self
        }

        fn box_clone(&self) -> Box<Shape + Sync + Send> {
            unimplemented!()
        }

        fn local_normal_at(&self, Point { x, y, z }: Point) -> Vector3 {
            Vector3::new(x, y, z)
        }

        fn local_intersect(&self, ray: Ray) -> Intersections {
            *self.saved_ray.lock().unwrap().borrow_mut() = Some(ray);
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
    fn intersecting_scaled_shape_with_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let mut s = TestShape::default();
        s.transform = transforms::scaling(2.0, 2.0, 2.0);
        s.intersect(r);
        let saved_ray = s.saved_ray.lock().unwrap();
        assert_eq!(
            Point::new(0.0, 0.0, -2.5),
            saved_ray.borrow().unwrap().origin
        );
        assert_eq!(
            Vector3::new(0.0, 0.0, 0.5),
            saved_ray.borrow().unwrap().direction
        );
    }

    #[test]
    fn intersecting_translated_shape_with_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let mut s = TestShape::default();
        s.transform = transforms::translation(5.0, 0.0, 0.0);
        s.intersect(r);
        let saved_ray = s.saved_ray.lock().unwrap();
        assert_eq!(
            Point::new(-5.0, 0.0, -5.0),
            saved_ray.borrow().unwrap().origin
        );
        assert_eq!(
            Vector3::new(0.0, 0.0, 1.0),
            saved_ray.borrow().unwrap().direction
        );
    }

    #[test]
    fn normal_on_translated_shape() {
        let mut s = TestShape::default();
        s.transform = transforms::translation(0.0, 1.0, 0.0);
        assert_eq!(
            Vector3::new(0.0, 0.70711, -0.70711),
            s.normal_at(Point::new(0.0, 1.70711, -0.70711))
        );
    }

    #[test]
    fn normal_on_transformed_shape() {
        let mut s = TestShape::default();
        s.transform = transforms::scaling(1.0, 0.5, 1.0) * transforms::rotation_z(PI / 5.0);
        assert_eq!(
            Vector3::new(0.0, 0.97014, -0.24254),
            s.normal_at(Point::new(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0))
        );
    }
}
