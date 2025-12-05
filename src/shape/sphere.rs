use crate::{
    Intersection, ORIGIN, Ray,
    shape::{Geometry, Shape},
};

#[must_use]
pub fn sphere() -> Shape {
    Sphere.into()
}

pub struct Sphere;

impl Geometry for Sphere {
    fn local_intersection<'shape>(
        &self,
        shape: &'shape Shape,
        ray: Ray,
    ) -> Vec<Intersection<'shape>> {
        let sphere_to_ray = ray.origin - ORIGIN;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
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
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use crate::{identity_matrix, point, ray, shape::sphere, transform, vector, EPSILON};

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_relative_eq!(xs[0].time, 4.0, epsilon = EPSILON);
        assert_relative_eq!(xs[1].time, 6.0, epsilon = EPSILON);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = ray(point(0, 1, -5), vector(0, 0, 1));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_relative_eq!(xs[0].time, 5.0, epsilon = EPSILON);
        assert_relative_eq!(xs[1].time, 5.0, epsilon = EPSILON);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = ray(point(0, 2, -5), vector(0, 0, 1));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = ray(point(0, 0, 0), vector(0, 0, 1));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_relative_eq!(xs[0].time, -1.0, epsilon = EPSILON);
        assert_relative_eq!(xs[1].time, 1.0, epsilon = EPSILON);
    }

    #[test]
    fn sphere_is_behind_ray() {
        let r = ray(point(0, 0, 5), vector(0, 0, 1));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_relative_eq!(xs[0].time, -6.0, epsilon = EPSILON);
        assert_relative_eq!(xs[1].time, -4.0, epsilon = EPSILON);
    }

    #[test]
    fn intersect_sets_object_on_intersection() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert!(std::ptr::eq(xs[0].object, &raw const s));
        assert!(std::ptr::eq(xs[1].object, &raw const s));
    }

    #[test]
    fn sphere_default_transformation() {
        let s = sphere();
        assert_eq!(s.transform, identity_matrix());
    }

    #[test]
    fn changing_sphere_transformation() {
        let mut s = sphere();
        let t = transform::translation(2, 3, 4);
        s.transform = t;
        assert_eq!(s.transform, t);
    }

    #[test]
    fn intersecting_scaled_sphere_with_ray() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let mut s = sphere();
        s.transform = transform::scaling(2, 2, 2);
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_relative_eq!(xs[0].time, 3.0, epsilon = EPSILON);
        assert_relative_eq!(xs[1].time, 7.0, epsilon = EPSILON);
    }

    #[test]
    fn intersecting_translated_sphere_with_ray() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let mut s = sphere();
        s.transform = transform::translation(5, 0, 0);
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 0);
    }
}
