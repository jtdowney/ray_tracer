use ord_subset::OrdSubsetIterExt;

use crate::{Point, Ray, Sphere, Vector};

pub fn intersection<T>(t: T, object: &Sphere) -> Intersection
where
    T: Into<f64>,
{
    Intersection {
        time: t.into(),
        object,
    }
}

pub fn hit<'a, T>(iter: T) -> Option<Intersection<'a>>
where
    T: IntoIterator<Item = Intersection<'a>>,
{
    iter.into_iter()
        .filter(|i| i.time >= 0.0)
        .ord_subset_min_by_key(|i| i.time)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Intersection<'a> {
    pub time: f64,
    pub object: &'a Sphere,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Computations<'a> {
    pub time: f64,
    pub object: &'a Sphere,
    pub point: Point,
    pub eye_vector: Vector,
    pub normal_vector: Vector,
    pub inside: bool,
}

impl<'a> Intersection<'a> {
    pub fn prepare_computations(self, ray: Ray) -> Computations<'a> {
        let time = self.time;
        let object = self.object;
        let point = ray.position(time);
        let eye_vector = -ray.direction;
        let mut normal_vector = object.normal_at(point);
        let inside = normal_vector.dot(eye_vector) < 0.0;

        if inside {
            normal_vector = -normal_vector;
        }

        Computations {
            time,
            object,
            point,
            eye_vector,
            normal_vector,
            inside,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{point, ray, sphere, vector};

    use super::*;

    #[test]
    fn hit_when_all_positive_intersections() {
        let s = sphere();
        let i1 = intersection(1, &s);
        let i2 = intersection(2, &s);
        let xs = vec![i2, i1];
        assert_eq!(Some(i1), hit(xs));
    }

    #[test]
    fn hit_when_some_negative_intersections() {
        let s = sphere();
        let i1 = intersection(-1, &s);
        let i2 = intersection(1, &s);
        let xs = vec![i2, i1];
        assert_eq!(Some(i2), hit(xs));
    }

    #[test]
    fn hit_when_all_negative_intersections() {
        let s = sphere();
        let i1 = intersection(-2, &s);
        let i2 = intersection(-1, &s);
        let xs = vec![i2, i1];
        assert_eq!(None, hit(xs));
    }

    #[test]
    fn hit_is_lowest_nonnegative_intersection() {
        let s = sphere();
        let i1 = intersection(5, &s);
        let i2 = intersection(7, &s);
        let i3 = intersection(-3, &s);
        let i4 = intersection(2, &s);
        let xs = vec![i1, i2, i3, i4];
        assert_eq!(Some(i4), hit(xs));
    }

    #[test]
    fn precomputing_state_of_intersection() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let shape = sphere();
        let i = intersection(4, &shape);
        let comps = i.prepare_computations(r);
        assert_eq!(i.time, comps.time);
        assert_eq!(i.object, comps.object);
        assert_eq!(point(0, 0, -1), comps.point);
        assert_eq!(vector(0, 0, -1), comps.eye_vector);
        assert_eq!(vector(0, 0, -1), comps.normal_vector);
    }

    #[test]
    fn hit_when_intersection_occurs_on_outside() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let shape = sphere();
        let i = intersection(4, &shape);
        let comps = i.prepare_computations(r);
        assert!(!comps.inside);
    }

    #[test]
    fn hit_when_intersection_occurs_on_inside() {
        let r = ray(point(0, 0, 0), vector(0, 0, 1));
        let shape = sphere();
        let i = intersection(1, &shape);
        let comps = i.prepare_computations(r);
        assert!(comps.inside);
        assert_eq!(point(0, 0, 1), comps.point);
        assert_eq!(vector(0, 0, -1), comps.eye_vector);
        assert_eq!(vector(0, 0, -1), comps.normal_vector);
    }
}
