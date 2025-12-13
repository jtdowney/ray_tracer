use std::any::Any;

use bon::builder;

use crate::{
    EPSILON, Intersection, Material, Vector, identity_matrix, intersection_with_uv, material,
    matrix::Matrix4,
    point::Point,
    ray::Ray,
    shape::{Geometry, Shape},
};

#[builder(finish_fn = build)]
#[must_use]
pub fn smooth_triangle(
    #[builder(start_fn)] p1: Point,
    #[builder(start_fn)] p2: Point,
    #[builder(start_fn)] p3: Point,
    #[builder(start_fn)] n1: Vector,
    #[builder(start_fn)] n2: Vector,
    #[builder(start_fn)] n3: Vector,
    #[builder(default = identity_matrix())] transform: Matrix4,
    #[builder(default = material(), into)] material: Material,
) -> Shape {
    let e1 = p2 - p1;
    let e2 = p3 - p1;
    let shape = Shape::new(SmoothTriangle {
        p1,
        p2,
        p3,
        n1,
        n2,
        n3,
        e1,
        e2,
    });
    shape.set_transform(transform);
    shape.set_material(material);
    shape
}

pub struct SmoothTriangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
    pub n1: Vector,
    pub n2: Vector,
    pub n3: Vector,
    pub e1: Vector,
    pub e2: Vector,
}

impl Geometry for SmoothTriangle {
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
        vec![intersection_with_uv(t, shape.clone(), u, v)]
    }

    fn local_normal_at(&self, _point: Point, hit: Option<&Intersection>) -> Vector {
        let hit = hit.expect("SmoothTriangle requires hit intersection with u/v");
        let u = hit.u.expect("SmoothTriangle requires u coordinate");
        let v = hit.v.expect("SmoothTriangle requires v coordinate");

        self.n2 * u + self.n3 * v + self.n1 * (1.0 - u - v)
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
    use std::slice;

    use approx::assert_relative_eq;

    use super::*;
    use crate::{EPSILON, intersection_with_uv, point, ray, vector};

    fn test_smooth_triangle() -> Shape {
        smooth_triangle(
            point(0, 1, 0),
            point(-1, 0, 0),
            point(1, 0, 0),
            vector(0, 1, 0),
            vector(-1, 0, 0),
            vector(1, 0, 0),
        )
        .build()
    }

    #[test]
    fn constructing_smooth_triangle() {
        let p1 = point(0, 1, 0);
        let p2 = point(-1, 0, 0);
        let p3 = point(1, 0, 0);
        let n1 = vector(0, 1, 0);
        let n2 = vector(-1, 0, 0);
        let n3 = vector(1, 0, 0);

        let tri = smooth_triangle(p1, p2, p3, n1, n2, n3).build();

        let inner = tri.inner();
        let st = inner
            .geometry
            .as_any()
            .downcast_ref::<SmoothTriangle>()
            .expect("Should be a SmoothTriangle");

        assert_eq!(st.p1, p1);
        assert_eq!(st.p2, p2);
        assert_eq!(st.p3, p3);
        assert_eq!(st.n1, n1);
        assert_eq!(st.n2, n2);
        assert_eq!(st.n3, n3);
    }

    #[test]
    fn intersection_with_smooth_triangle_stores_uv() {
        let tri = test_smooth_triangle();
        let r = ray(point(-0.2, 0.3, -2.0), vector(0, 0, 1));
        let xs = tri.intersect(r);
        assert_eq!(xs.len(), 1);
        assert_relative_eq!(xs[0].u.unwrap(), 0.45, epsilon = EPSILON);
        assert_relative_eq!(xs[0].v.unwrap(), 0.25, epsilon = EPSILON);
    }

    #[test]
    fn smooth_triangle_uses_uv_to_interpolate_normal() {
        let tri = test_smooth_triangle();
        let i = intersection_with_uv(1.0, tri.clone(), 0.45, 0.25);
        let n = tri.normal_at_with_hit(point(0, 0, 0), Some(&i));
        assert_relative_eq!(n.x(), -0.5547, epsilon = EPSILON);
        assert_relative_eq!(n.y(), 0.83205, epsilon = EPSILON);
        assert_relative_eq!(n.z(), 0.0, epsilon = EPSILON);
    }

    #[test]
    fn preparing_normal_on_smooth_triangle() {
        let tri = test_smooth_triangle();
        let i = intersection_with_uv(1.0, tri.clone(), 0.45, 0.25);
        let r = ray(point(-0.2, 0.3, -2.0), vector(0, 0, 1));
        let comps = i.prepare_computations(r, slice::from_ref(&i));
        assert_relative_eq!(comps.normalv.x(), -0.5547, epsilon = EPSILON);
        assert_relative_eq!(comps.normalv.y(), 0.83205, epsilon = EPSILON);
        assert_relative_eq!(comps.normalv.z(), 0.0, epsilon = EPSILON);
    }
}
