use ord_subset::OrdSubsetIterExt;

use crate::shape::Shape;

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

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;
    use crate::{EPSILON, shape::sphere};

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
}
