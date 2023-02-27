use ord_subset::OrdSubsetIterExt;

use crate::Sphere;

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

#[cfg(test)]
mod tests {
    use crate::sphere;

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
}
