use crate::{
    identity_matrix, intersection::Intersection, material, Material, Matrix4, Point, Ray, Vector,
    ORIGIN,
};

pub fn sphere() -> Sphere {
    Sphere::default()
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    pub transform: Matrix4,
    pub material: Material,
}

impl Sphere {
    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let ray = ray.transform(self.transform.inverse());
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
                object: self,
            });
            intersections.push(Intersection {
                time: t2,
                object: self,
            });
        }

        intersections
    }

    pub fn normal_at(&self, world_point: Point) -> Vector {
        let inv = self.transform.inverse();
        let object_point = inv * world_point;
        let object_normal = object_point - ORIGIN;
        let world_normal = inv.transpose() * object_normal;
        world_normal.normalize()
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            transform: identity_matrix(),
            material: material(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use approx::assert_abs_diff_eq;

    use crate::{
        point, ray,
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
        let p = point(0.0, 1.70711, -0.70711);
        assert_abs_diff_eq!(vector(0.0, 0.70711, -0.70711), s.normal_at(p));
    }

    #[test]
    fn normal_on_transformed_sphere() {
        let mut s = sphere();
        s.transform = scaling(1.0, 0.5, 1.0) * rotation_z(PI / 5.0);
        let p = point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        assert_abs_diff_eq!(vector(0.0, 0.97014, -0.24254), s.normal_at(p));
    }
}
