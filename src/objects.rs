use crate::{
    Color, Intersection, Intersections, Material, Matrix4, Point, PointLight, Ray, Vector3,
};
use std::any::Any;
use std::fmt::Debug;
use std::vec;

pub trait Shape: Any + Debug {
    fn as_any(&self) -> &Any;
    fn normal_at(&self, world_point: Point) -> Vector3;
    fn intersect(&self, ray: Ray) -> Intersections;
    fn lighting(
        &self,
        light: PointLight,
        position: Point,
        eye_vector: Vector3,
        normal_vector: Vector3,
        in_shadow: bool,
    ) -> Color;
}

#[derive(Copy, Clone, Debug, PartialEq)]
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

    fn normal_at(&self, world_point: Point) -> Vector3 {
        let object_point = self.transform.inverse() * world_point;
        let object_normal = object_point - Point::default();
        let world_normal = self.transform.inverse().transpose() * object_normal;
        world_normal.normalize()
    }

    fn intersect(&self, ray: Ray) -> Intersections {
        let ray = ray.transform(self.transform.inverse());
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

    fn lighting(
        &self,
        light: PointLight,
        position: Point,
        eye_vector: Vector3,
        normal_vector: Vector3,
        in_shadow: bool,
    ) -> Color {
        self.material
            .lighting(light, position, eye_vector, normal_vector, in_shadow)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transforms;
    use std::f64::consts::PI;
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
    fn test_computing_normal_on_translated_sphere() {
        let mut s = Sphere::default();
        s.transform = transforms::translation(0.0, 1.0, 0.0);
        assert_eq!(
            Vector3::new(0.0, 0.70711, -0.70711),
            s.normal_at(Point::new(0.0, 1.70711, -0.70711))
        );
    }

    #[test]
    fn test_computing_normal_on_transformed_sphere() {
        let mut s = Sphere::default();
        s.transform = transforms::scaling(1.0, 0.5, 1.0) * transforms::rotation_z(PI / 5.0);
        assert_eq!(
            Vector3::new(0.0, 0.97014, -0.24254),
            s.normal_at(Point::new(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0))
        );
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

    #[test]
    fn test_intersecting_scaled_sphere_with_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let mut s = Sphere::default();
        s.transform = transforms::scaling(2.0, 2.0, 2.0);
        let mut xs = s.intersect(r).into_iter();
        assert_eq!(3.0, xs.next().unwrap().time);
        assert_eq!(7.0, xs.next().unwrap().time);
        assert!(xs.next().is_none());
    }

    #[test]
    fn test_intersecting_translated_sphere_with_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let mut s = Sphere::default();
        s.transform = transforms::translation(5.0, 0.0, 0.0);
        let mut xs = s.intersect(r).into_iter();
        assert!(xs.next().is_none());
    }
}
