use std::any::Any;

use bon::builder;

use crate::{
    EPSILON, Intersection, Material, Vector, identity_matrix, intersection, material,
    matrix::Matrix4,
    point::Point,
    ray::Ray,
    shape::{Geometry, Shape},
};

#[builder(finish_fn = build)]
#[must_use]
pub fn triangle(
    #[builder(start_fn)] p1: Point,
    #[builder(start_fn)] p2: Point,
    #[builder(start_fn)] p3: Point,
    #[builder(default = identity_matrix())] transform: Matrix4,
    #[builder(default = material(), into)] material: Material,
) -> Shape {
    let e1 = p2 - p1;
    let e2 = p3 - p1;
    let normal = e2.cross(&e1).normalize();
    let shape = Shape::new(Triangle {
        p1,
        p2,
        p3,
        e1,
        e2,
        normal,
    });
    shape.set_transform(transform);
    shape.set_material(material);
    shape
}

pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
    pub e1: Vector,
    pub e2: Vector,
    pub normal: Vector,
}

impl Geometry for Triangle {
    fn local_intersection(&self, shape: &Shape, ray: Ray) -> Vec<Intersection> {
        let dir_cross_e2 = ray.direction.cross(&self.e2);
        let det = self.e1.dot(&dir_cross_e2);

        if det.abs() < EPSILON {
            return vec![];
        }

        let f = 1.0 / det;
        let p1_to_origin = ray.origin - self.p1;
        let u = f * p1_to_origin.dot(&dir_cross_e2);

        if !(0.0..=1.0).contains(&u) {
            return vec![];
        }

        let origin_cross_e1 = p1_to_origin.cross(&self.e1);
        let v = f * ray.direction.dot(&origin_cross_e1);

        if v < 0.0 || (u + v) > 1.0 {
            return vec![];
        }

        let t = f * self.e2.dot(&origin_cross_e1);
        vec![intersection(t, shape.clone())]
    }

    fn local_normal_at(&self, _point: Point, _hit: Option<&Intersection>) -> Vector {
        self.normal
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;
    use crate::{EPSILON, point, ray, vector};

    fn test_triangle() -> Shape {
        triangle(point(0, 1, 0), point(-1, 0, 0), point(1, 0, 0)).build()
    }

    #[test]
    fn constructing_a_triangle() {
        let p1 = point(0, 1, 0);
        let p2 = point(-1, 0, 0);
        let p3 = point(1, 0, 0);
        let t = triangle(p1, p2, p3).build();

        let inner = t.inner();
        let tri = inner
            .geometry
            .as_any()
            .downcast_ref::<Triangle>()
            .expect("Should be a Triangle");

        assert_eq!(tri.p1, p1);
        assert_eq!(tri.p2, p2);
        assert_eq!(tri.p3, p3);
        assert_eq!(tri.e1, vector(-1, -1, 0));
        assert_eq!(tri.e2, vector(1, -1, 0));
        assert_eq!(tri.normal, vector(0, 0, -1));
    }

    #[test]
    fn finding_normal_on_triangle() {
        let t = test_triangle();
        let n1 = t.normal_at(point(0.0, 0.5, 0.0));
        let n2 = t.normal_at(point(-0.5, 0.75, 0.0));
        let n3 = t.normal_at(point(0.5, 0.25, 0.0));

        let inner = t.inner();
        let tri = inner
            .geometry
            .as_any()
            .downcast_ref::<Triangle>()
            .expect("Should be a Triangle");

        assert_eq!(n1, tri.normal);
        assert_eq!(n2, tri.normal);
        assert_eq!(n3, tri.normal);
    }

    #[test]
    fn intersecting_ray_parallel_to_triangle() {
        let t = test_triangle();
        let r = ray(point(0, -1, -2), vector(0, 1, 0));
        let xs = t.intersect(r);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_misses_p1_p3_edge() {
        let t = test_triangle();
        let r = ray(point(1, 1, -2), vector(0, 0, 1));
        let xs = t.intersect(r);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_misses_p1_p2_edge() {
        let t = test_triangle();
        let r = ray(point(-1, 1, -2), vector(0, 0, 1));
        let xs = t.intersect(r);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_misses_p2_p3_edge() {
        let t = test_triangle();
        let r = ray(point(0, -1, -2), vector(0, 0, 1));
        let xs = t.intersect(r);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_strikes_triangle() {
        let t = test_triangle();
        let r = ray(point(0.0, 0.5, -2.0), vector(0, 0, 1));
        let xs = t.intersect(r);
        assert_eq!(xs.len(), 1);
        assert_relative_eq!(xs[0].time, 2.0, epsilon = EPSILON);
    }
}
