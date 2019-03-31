use crate::{Intersection, Intersections, Material, Matrix4, Point, Ray, Shape, Vector3};
use std::any::Any;
use std::vec;

#[derive(Debug, PartialEq)]
pub struct Sphere {
    pub transform: Matrix4,
    pub material: Material,
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            transform: Matrix4::identity(),
            material: Material::default(),
        }
    }
}

impl PartialEq<Shape> for Sphere {
    fn eq(&self, other: &Shape) -> bool {
        other
            .as_any()
            .downcast_ref::<Self>()
            .map_or(false, |x| x == self)
    }
}

impl Shape for Sphere {
    fn as_any(&self) -> &Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut Any {
        self
    }

    fn local_normal_at(&self, point: Point) -> Vector3 {
        point - Point::default()
    }

    fn local_intersect(&self, ray: Ray) -> Intersections {
        let object_to_ray = ray.origin - Point::default();
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(object_to_ray);
        let c = object_to_ray.dot(object_to_ray) - 1.0;
        let discriminant = b.powi(2) - 4.0 * a * c;

        let mut intersections = vec![];
        if discriminant >= 0.0 {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            intersections.push(Intersection {
                time: t1,
                object: self,
            });
            intersections.push(Intersection {
                time: t2,
                object: self,
            });
        }

        Intersections(intersections)
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn transform(&self) -> &Matrix4 {
        &self.transform
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn test_spheres_default_transformation() {
        let s = Sphere::default();
        assert_eq!(Matrix4::identity(), s.transform);
    }

    #[test]
    fn test_normal_on_sphere_at_x_axis() {
        let s = Sphere::default();
        assert_eq!(
            Vector3::new(1.0, 0.0, 0.0),
            s.normal_at(Point::new(1.0, 0.0, 0.0))
        );
    }

    #[test]
    fn test_normal_on_sphere_at_y_axis() {
        let s = Sphere::default();
        assert_eq!(
            Vector3::new(0.0, 1.0, 0.0),
            s.normal_at(Point::new(0.0, 1.0, 0.0))
        );
    }

    #[test]
    fn test_normal_on_sphere_at_z_axis() {
        let s = Sphere::default();
        assert_eq!(
            Vector3::new(0.0, 0.0, 1.0),
            s.normal_at(Point::new(0.0, 0.0, 1.0))
        );
    }

    #[test]
    fn test_normal_on_sphere_at_nonaxial_point() {
        let s = Sphere::default();
        assert_eq!(
            Vector3::new(
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0
            ),
            s.normal_at(Point::new(
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0
            ))
        );
    }

    #[test]
    fn test_normal_is_a_normalized_vector() {
        let s = Sphere::default();
        let n = s.normal_at(Point::new(
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
        ));
        assert_eq!(n, n.normalize());
    }

    #[test]
    fn test_ray_intersects_sphere_at_two_points() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let mut xs = s.intersect(r).into_iter();
        assert_eq!(4.0, xs.next().unwrap().time);
        assert_eq!(6.0, xs.next().unwrap().time);
        assert!(xs.next().is_none());
    }

    #[test]
    fn test_ray_intersects_sphere_at_tangent() {
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let mut xs = s.intersect(r).into_iter();
        assert_eq!(5.0, xs.next().unwrap().time);
        assert_eq!(5.0, xs.next().unwrap().time);
        assert!(xs.next().is_none());
    }

    #[test]
    fn test_ray_misses_sphere() {
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = s.intersect(r).into_iter();
        assert_eq!(0, xs.count());
    }

    #[test]
    fn test_ray_originates_inside_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let mut xs = s.intersect(r).into_iter();
        assert_eq!(-1.0, xs.next().unwrap().time);
        assert_eq!(1.0, xs.next().unwrap().time);
        assert!(xs.next().is_none());
    }

    #[test]
    fn test_sphere_behind_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector3::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let mut xs = s.intersect(r).into_iter();
        assert_eq!(-6.0, xs.next().unwrap().time);
        assert_eq!(-4.0, xs.next().unwrap().time);
        assert!(xs.next().is_none());
    }

    #[test]
    fn test_intersection_has_time_and_object() {
        let s = Sphere::default();
        let i = Intersection {
            time: 3.5,
            object: &s,
        };

        assert_eq!(3.5, i.time);
        assert!(ptr::eq(&s as &Shape, i.object));
    }

    #[test]
    fn test_intersect_sets_objects() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let mut xs = s.intersect(r).into_iter();
        assert_eq!(&s, xs.next().unwrap().object);
        assert_eq!(&s, xs.next().unwrap().object);
        assert!(xs.next().is_none());
    }
}
