use crate::Sphere;
use ord_subset::OrdSubsetIterExt;
use std::{iter::FromIterator, slice, vec};

pub fn intersection(time: f64, object: &Sphere) -> Intersection {
    Intersection { time, object }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Intersection<'o> {
    pub time: f64,
    pub object: &'o Sphere,
}

pub struct Intersections<'o>(Vec<Intersection<'o>>);

impl<'o> Intersections<'o> {
    pub fn empty() -> Self {
        Intersections(vec![])
    }

    pub fn iter(&self) -> slice::Iter<Intersection<'o>> {
        self.0.iter()
    }
}

impl<'o> Intersections<'o> {
    pub fn hit(&self) -> Option<&Intersection<'o>> {
        self.iter()
            .filter(|i| i.time >= 0.0)
            .ord_subset_min_by_key(|i| i.time)
    }
}

impl<'o> From<Vec<Intersection<'o>>> for Intersections<'o> {
    fn from(intersections: Vec<Intersection<'o>>) -> Self {
        Intersections(intersections)
    }
}

impl<'o> FromIterator<Intersection<'o>> for Intersections<'o> {
    fn from_iter<I: IntoIterator<Item = Intersection<'o>>>(iter: I) -> Self {
        let intersections = iter.into_iter().collect();
        Intersections(intersections)
    }
}

impl<'o> IntoIterator for Intersections<'o> {
    type Item = Intersection<'o>;
    type IntoIter = vec::IntoIter<Intersection<'o>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sphere;

    #[test]
    fn intersection_has_time_and_object() {
        let s = sphere();
        let i = intersection(3.5, &s);

        assert_eq!(i.time, 3.5);
        assert_eq!(i.object, &s);
    }

    #[test]
    fn hit_with_all_positive_times() {
        let s = sphere();
        let i1 = Intersection {
            time: 1.0,
            object: &s,
        };
        let i2 = Intersection {
            time: 2.0,
            object: &s,
        };
        let xs = Intersections(vec![i1.clone(), i2]);
        let i = xs.hit().unwrap();
        assert_eq!(i, &i1);
    }

    #[test]
    fn hit_with_some_negative_times() {
        let s = sphere();
        let i1 = Intersection {
            time: -1.0,
            object: &s,
        };
        let i2 = Intersection {
            time: 1.0,
            object: &s,
        };
        let xs = Intersections(vec![i2.clone(), i1]);
        let i = xs.hit().unwrap();
        assert_eq!(i, &i2);
    }

    #[test]
    fn hit_with_all_negative_times() {
        let s = sphere();
        let i1 = Intersection {
            time: -2.0,
            object: &s,
        };
        let i2 = Intersection {
            time: -1.0,
            object: &s,
        };
        let xs = Intersections(vec![i2, i1]);
        assert!(xs.hit().is_none());
    }

    #[test]
    fn hit_lowest_positive_intersection() {
        let s = sphere();
        let i1 = Intersection {
            time: 5.0,
            object: &s,
        };
        let i2 = Intersection {
            time: 7.0,
            object: &s,
        };
        let i3 = Intersection {
            time: -3.0,
            object: &s,
        };
        let i4 = Intersection {
            time: 2.0,
            object: &s,
        };
        let xs = Intersections(vec![i1, i2, i3, i4.clone()]);
        let i = xs.hit().unwrap();
        assert_eq!(i, &i4);
    }
}
