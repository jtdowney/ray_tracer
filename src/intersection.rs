use ord_subset::OrdSubsetIterExt;

use crate::{Point, Ray, Vector, shape::Shape};

pub fn intersection<T>(t: T, object: &Shape) -> Intersection<'_>
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
    T: IntoIterator<Item = &'a Intersection<'a>>,
{
    iter.into_iter()
        .filter(|i| i.time >= 0.0)
        .ord_subset_min_by_key(|i| i.time)
        .copied()
}

#[derive(Copy, Clone)]
pub struct Intersection<'a> {
    pub time: f64,
    pub object: &'a Shape,
}

impl Intersection<'_> {
    #[must_use]
    pub fn prepare_computations(&self, ray: Ray) -> Computations<'_> {
        let point = ray.position(self.time);
        let eyev = -ray.direction;
        let normalv = self.object.normal_at(point);
        let inside = normalv.dot(&eyev) < 0.0;
        let normalv = if inside { -normalv } else { normalv };

        Computations {
            time: self.time,
            object: self.object,
            point,
            eyev,
            normalv,
            inside,
        }
    }
}

pub struct Computations<'a> {
    pub time: f64,
    pub object: &'a Shape,
    pub point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    pub inside: bool,
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;
    use crate::{EPSILON, point, ray, shape::sphere, vector};

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = sphere().build();
        let i = intersection(3.5, &s);
        assert_relative_eq!(i.time, 3.5, epsilon = EPSILON);
        assert!(std::ptr::eq(i.object, &raw const s));
    }

    #[test]
    fn aggregating_intersections() {
        let s = sphere().build();
        let i1 = intersection(1, &s);
        let i2 = intersection(2, &s);
        let xs = [i1, i2];
        assert_eq!(xs.len(), 2);
        assert_relative_eq!(xs[0].time, 1.0, epsilon = EPSILON);
        assert_relative_eq!(xs[1].time, 2.0, epsilon = EPSILON);
    }

    #[test]
    fn hit_when_all_intersections_have_positive_t() {
        let s = sphere().build();
        let i1 = intersection(1, &s);
        let i2 = intersection(2, &s);
        let xs = vec![i2, i1];
        let i = hit(&xs).unwrap();
        assert_relative_eq!(i.time, i1.time, epsilon = EPSILON);
    }

    #[test]
    fn hit_when_some_intersections_have_negative_t() {
        let s = sphere().build();
        let i1 = intersection(-1, &s);
        let i2 = intersection(1, &s);
        let xs = vec![i2, i1];
        let i = hit(&xs).unwrap();
        assert_relative_eq!(i.time, i2.time, epsilon = EPSILON);
    }

    #[test]
    fn hit_when_all_intersections_have_negative_t() {
        let s = sphere().build();
        let i1 = intersection(-2, &s);
        let i2 = intersection(-1, &s);
        let xs = vec![i2, i1];
        let i = hit(&xs);
        assert!(i.is_none());
    }

    #[test]
    fn hit_is_always_lowest_nonnegative_intersection() {
        let s = sphere().build();
        let i1 = intersection(5, &s);
        let i2 = intersection(7, &s);
        let i3 = intersection(-3, &s);
        let i4 = intersection(2, &s);
        let xs = vec![i1, i2, i3, i4];
        let i = hit(&xs).unwrap();
        assert_relative_eq!(i.time, i4.time, epsilon = EPSILON);
    }

    #[test]
    fn precomputing_state_of_intersection() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let shape = sphere().build();
        let i = intersection(4, &shape);
        let comps = i.prepare_computations(r);
        assert_relative_eq!(comps.time, i.time, epsilon = EPSILON);
        assert!(std::ptr::eq(comps.object, i.object));
        assert_eq!(comps.point, point(0, 0, -1));
        assert_eq!(comps.eyev, vector(0, 0, -1));
        assert_eq!(comps.normalv, vector(0, 0, -1));
    }

    #[test]
    fn hit_when_intersection_occurs_on_outside() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let shape = sphere().build();
        let i = intersection(4, &shape);
        let comps = i.prepare_computations(r);
        assert!(!comps.inside);
    }

    #[test]
    fn hit_when_intersection_occurs_on_inside() {
        let r = ray(point(0, 0, 0), vector(0, 0, 1));
        let shape = sphere().build();
        let i = intersection(1, &shape);
        let comps = i.prepare_computations(r);
        assert_eq!(comps.point, point(0, 0, 1));
        assert_eq!(comps.eyev, vector(0, 0, -1));
        assert!(comps.inside);
        assert_eq!(comps.normalv, vector(0, 0, -1));
    }
}
