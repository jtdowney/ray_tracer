use std::any::Any;

use crate::{
    intersection::{intersection, Intersection},
    vector, Shape, Vector, EPSILON,
};

use super::Geometry;

pub fn plane() -> Shape {
    Plane.into()
}

#[derive(Clone, Copy, Debug)]
pub struct Plane;

impl Geometry for Plane {
    fn local_intersection<'a>(&'a self, shape: &'a Shape, ray: crate::Ray) -> Vec<Intersection> {
        let mut xs = vec![];
        if ray.direction.y.abs() < EPSILON {
            return xs;
        }

        let time = -ray.origin.y / ray.direction.y;
        xs.push(intersection(time, shape));
        xs
    }

    fn local_normal_at(&self, _point: crate::Point) -> Vector {
        vector(0, 1, 0)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{point, ray};

    use super::*;

    #[test]
    fn normal_is_constant_everywhere() {
        let p = plane();
        assert_eq!(vector(0, 1, 0), p.normal_at(point(0, 0, 0)));
        assert_eq!(vector(0, 1, 0), p.normal_at(point(10, 0, -10)));
        assert_eq!(vector(0, 1, 0), p.normal_at(point(-5, 0, 150)));
    }

    #[test]
    fn intersecting_ray_parallel() {
        let p = plane();
        let r = ray(point(0, 10, 0), vector(0, 0, 1));
        let xs = p.intersect(r);
        assert!(xs.is_empty());
    }

    #[test]
    fn intersecting_ray_coplanar() {
        let p = plane();
        let r = ray(point(0, 0, 0), vector(0, 0, 1));
        let xs = p.intersect(r);
        assert!(xs.is_empty());
    }

    #[test]
    fn intersecting_ray_above() {
        let p = plane();
        let r = ray(point(0, 1, 0), vector(0, -1, 0));
        let xs = p.intersect(r);
        assert_eq!(1, xs.len());
        assert_eq!(1.0, xs[0].time);
        assert_eq!(&p, xs[0].object);
    }
}
