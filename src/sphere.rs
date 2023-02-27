use crate::{identity_matrix, intersection::Intersection, Matrix4, Ray, ORIGIN};

pub fn sphere() -> Sphere {
    Sphere::default()
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    transform: Matrix4,
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
                t: t1,
                object: self,
            });
            intersections.push(Intersection {
                t: t2,
                object: self,
            });
        }

        intersections
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            transform: identity_matrix(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        point, ray,
        transform::{scaling, translation},
        vector,
    };

    use super::*;

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(2, xs.len());
        assert_eq!(4.0, xs[0].t);
        assert_eq!(6.0, xs[1].t);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = ray(point(0, 1, -5), vector(0, 0, 1));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(2, xs.len());
        assert_eq!(5.0, xs[0].t);
        assert_eq!(5.0, xs[1].t);
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
        assert_eq!(-1.0, xs[0].t);
        assert_eq!(1.0, xs[1].t);
    }

    #[test]
    fn sphere_behind_ray() {
        let r = ray(point(0, 0, 5), vector(0, 0, 1));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(2, xs.len());
        assert_eq!(-6.0, xs[0].t);
        assert_eq!(-4.0, xs[1].t);
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
        assert_eq!(3.0, xs[0].t);
        assert_eq!(7.0, xs[1].t);
    }

    #[test]
    fn intersecting_translated_sphere() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let mut s = sphere();
        s.transform = translation(5, 0, 0);
        let xs = s.intersect(r);
        assert!(xs.is_empty());
    }
}
