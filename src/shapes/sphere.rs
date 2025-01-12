use std::any::Any;

use crate::{intersection::Intersection, Point, Ray, Vector, ORIGIN};

use super::{Geometry, Shape};

pub fn sphere() -> Shape {
    Sphere.into()
}

#[cfg(test)]
pub fn glass_sphere() -> Shape {
    let mut sphere = sphere();
    sphere.material.transparency = 1.0;
    sphere.material.refractive_index = 1.5;
    sphere
}

#[derive(Clone, Copy, Debug)]
pub struct Sphere;

impl Geometry for Sphere {
    fn local_intersection<'a>(&'a self, shape: &'a Shape, ray: Ray) -> Vec<Intersection<'a>> {
        let sphere_to_ray = ray.origin - ORIGIN;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b.powi(2) - (4.0 * a * c);

        let mut intersections = vec![];
        if discriminant >= 0.0 {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            intersections.push(Intersection {
                time: t1,
                object: shape,
            });
            intersections.push(Intersection {
                time: t2,
                object: shape,
            });
        }

        intersections
    }

    fn local_normal_at(&self, point: Point) -> Vector {
        point - ORIGIN
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_1_SQRT_2, PI};

    use approx::assert_abs_diff_eq;

    use crate::{
        identity_matrix, point, ray,
        transform::{rotation_z, scaling, translation},
        vector,
    };

    use super::*;

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(2, xs.len());
        assert_eq!(4.0, xs[0].time);
        assert_eq!(6.0, xs[1].time);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = ray(point(0, 1, -5), vector(0, 0, 1));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(2, xs.len());
        assert_eq!(5.0, xs[0].time);
        assert_eq!(5.0, xs[1].time);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = ray(point(0, 2, -5), vector(0, 0, 1));
        let s = sphere();
        let xs = s.intersect(r);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = ray(point(0, 0, 0), vector(0, 0, 1));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(2, xs.len());
        assert_eq!(-1.0, xs[0].time);
        assert_eq!(1.0, xs[1].time);
    }

    #[test]
    fn sphere_behind_ray() {
        let r = ray(point(0, 0, 5), vector(0, 0, 1));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(2, xs.len());
        assert_eq!(-6.0, xs[0].time);
        assert_eq!(-4.0, xs[1].time);
    }

    #[test]
    fn intersect_sets_object() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(2, xs.len());
        assert_eq!(&s, xs[0].object);
        assert_eq!(&s, xs[1].object);
    }

    #[test]
    fn default_transformation() {
        let s = sphere();
        assert_eq!(identity_matrix(), s.transform);
    }

    #[test]
    fn intersecting_scaled_sphere() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let mut s = sphere();
        s.transform = scaling(2, 2, 2);
        let xs = s.intersect(r);
        assert_eq!(2, xs.len());
        assert_eq!(3.0, xs[0].time);
        assert_eq!(7.0, xs[1].time);
    }

    #[test]
    fn intersecting_translated_sphere() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let mut s = sphere();
        s.transform = translation(5, 0, 0);
        let xs = s.intersect(r);
        assert!(xs.is_empty());
    }

    #[test]
    fn normal_at_x_axis_point() {
        let s = sphere();
        assert_eq!(vector(1, 0, 0), s.normal_at(point(1, 0, 0)));
    }

    #[test]
    fn normal_at_y_axis_point() {
        let s = sphere();
        assert_eq!(vector(0, 1, 0), s.normal_at(point(0, 1, 0)));
    }

    #[test]
    fn normal_at_z_axis_point() {
        let s = sphere();
        assert_eq!(vector(0, 0, 1), s.normal_at(point(0, 0, 1)));
    }

    #[test]
    fn normal_at_nonaxial_point() {
        let s = sphere();
        let p = point(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        );
        assert_eq!(
            vector(
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0
            ),
            s.normal_at(p)
        )
    }

    #[test]
    fn normal_is_normalized() {
        let s = sphere();
        let p = point(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        );
        let n = s.normal_at(p);
        assert_eq!(n, n.normalize());
    }

    #[test]
    fn normal_on_translated_sphere() {
        let mut s = sphere();
        s.transform = translation(0, 1, 0);
        let p = point(0.0, 1.0 + FRAC_1_SQRT_2, -FRAC_1_SQRT_2);
        assert_abs_diff_eq!(vector(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2), s.normal_at(p));
    }

    #[test]
    fn normal_on_transformed_sphere() {
        let mut s = sphere();
        s.transform = scaling(1.0, 0.5, 1.0) * rotation_z(PI / 5.0);
        let p = point(0.0, 2.0_f64.sqrt() / 2.0, -(2.0_f64.sqrt()) / 2.0);
        assert_abs_diff_eq!(vector(0.0, 0.97014, -0.24254), s.normal_at(p));
    }
}
