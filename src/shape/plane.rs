use bon::builder;

use crate::{
    EPSILON, Intersection, Material, Vector, identity_matrix, intersection, material,
    matrix::Matrix4,
    point::Point,
    ray::Ray,
    shape::{Geometry, Shape},
    vector,
};

#[builder(finish_fn = build, derive(Into))]
#[must_use]
pub fn plane(
    #[builder(default = identity_matrix())] transform: Matrix4,
    #[builder(default = material(), into)] material: Material,
) -> Shape {
    let mut shape: Shape = Plane.into();
    shape.transform = transform;
    shape.material = material;
    shape
}

pub struct Plane;

impl Geometry for Plane {
    fn local_intersection<'shape>(
        &self,
        shape: &'shape Shape,
        ray: Ray,
    ) -> Vec<Intersection<'shape>> {
        let mut xs = vec![];
        if ray.direction.y.abs() < EPSILON {
            return xs;
        }

        let time = -ray.origin.y / ray.direction.y;
        xs.push(intersection(time, shape));
        xs
    }

    fn local_normal_at(&self, _point: Point) -> Vector {
        vector(0, 1, 0)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use crate::{EPSILON, point, ray, shape::plane, vector};

    #[test]
    fn normal_of_plane_is_constant_everywhere() {
        let p = plane().build();
        let n1 = p.normal_at(point(0, 0, 0));
        let n2 = p.normal_at(point(10, 0, -10));
        let n3 = p.normal_at(point(-5, 0, 150));
        assert_eq!(n1, vector(0, 1, 0));
        assert_eq!(n2, vector(0, 1, 0));
        assert_eq!(n3, vector(0, 1, 0));
    }

    #[test]
    fn intersect_with_ray_parallel_to_plane() {
        let p = plane().build();
        let r = ray(point(0, 10, 0), vector(0, 0, 1));
        let xs = p.intersect(r);
        assert!(xs.is_empty());
    }

    #[test]
    fn intersect_with_coplanar_ray() {
        let p = plane().build();
        let r = ray(point(0, 0, 0), vector(0, 0, 1));
        let xs = p.intersect(r);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_intersecting_plane_from_above() {
        let p = plane().build();
        let r = ray(point(0, 1, 0), vector(0, -1, 0));
        let xs = p.intersect(r);
        assert_eq!(xs.len(), 1);
        assert_relative_eq!(xs[0].time, 1.0, epsilon = EPSILON);
        assert!(std::ptr::eq(xs[0].object, &raw const p));
    }

    #[test]
    fn ray_intersecting_plane_from_below() {
        let p = plane().build();
        let r = ray(point(0, -1, 0), vector(0, 1, 0));
        let xs = p.intersect(r);
        assert_eq!(xs.len(), 1);
        assert_relative_eq!(xs[0].time, 1.0, epsilon = EPSILON);
        assert!(std::ptr::eq(xs[0].object, &raw const p));
    }
}
