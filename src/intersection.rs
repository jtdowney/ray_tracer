use crate::Sphere;
use num::Num;
use ord_subset::{OrdSubset, OrdSubsetIterExt};
use std::{iter::FromIterator, rc::Rc, slice, vec};

pub fn intersection<T>(time: T, object: Rc<Sphere<T>>) -> Intersection<T>
where
    T: Copy + PartialEq,
{
    Intersection { time, object }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Intersection<T>
where
    T: Copy + PartialEq,
{
    pub time: T,
    pub object: Rc<Sphere<T>>,
}

pub struct Intersections<T>(Vec<Intersection<T>>)
where
    T: Copy + PartialEq;

impl<T> Intersections<T>
where
    T: Copy + PartialEq,
{
    pub fn empty() -> Self {
        Intersections(vec![])
    }

    pub fn iter(&self) -> slice::Iter<Intersection<T>> {
        self.0.iter()
    }
}

impl<T> Intersections<T>
where
    T: Copy + PartialEq + OrdSubset + Num,
{
    pub fn hit(&self) -> Option<Intersection<T>> {
        self.iter()
            .cloned()
            .filter(|i| i.time >= T::zero())
            .ord_subset_min_by_key(|i| i.time)
    }
}

impl<T> From<Vec<Intersection<T>>> for Intersections<T>
where
    T: Copy + PartialEq,
{
    fn from(intersections: Vec<Intersection<T>>) -> Self {
        Intersections(intersections)
    }
}

impl<T> FromIterator<Intersection<T>> for Intersections<T>
where
    T: Copy + PartialEq,
{
    fn from_iter<I: IntoIterator<Item = Intersection<T>>>(iter: I) -> Self {
        let intersections = iter.into_iter().collect();
        Intersections(intersections)
    }
}

impl<T> IntoIterator for Intersections<T>
where
    T: Copy + PartialEq,
{
    type Item = Intersection<T>;
    type IntoIter = vec::IntoIter<Intersection<T>>;

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
        let i = intersection(3.5, s.clone());

        assert_eq!(i.time, 3.5);
        assert_eq!(i.object, s);
    }

    #[test]
    fn hit_with_all_positive_times() {
        let s = sphere();
        let i1 = Intersection {
            time: 1,
            object: s.clone(),
        };
        let i2 = Intersection {
            time: 2,
            object: s.clone(),
        };
        let xs = Intersections(vec![i1.clone(), i2]);
        let i = xs.hit().unwrap();
        assert_eq!(i, i1);
    }

    #[test]
    fn hit_with_some_negative_times() {
        let s = sphere();
        let i1 = Intersection {
            time: -1,
            object: s.clone(),
        };
        let i2 = Intersection {
            time: 1,
            object: s.clone(),
        };
        let xs = Intersections(vec![i2.clone(), i1]);
        let i = xs.hit().unwrap();
        assert_eq!(i, i2);
    }

    #[test]
    fn hit_with_all_negative_times() {
        let s = sphere();
        let i1 = Intersection {
            time: -2,
            object: s.clone(),
        };
        let i2 = Intersection {
            time: -1,
            object: s.clone(),
        };
        let xs = Intersections(vec![i2, i1]);
        assert!(xs.hit().is_none());
    }

    #[test]
    fn hit_lowest_positive_intersection() {
        let s = sphere();
        let i1 = Intersection {
            time: 5,
            object: s.clone(),
        };
        let i2 = Intersection {
            time: 7,
            object: s.clone(),
        };
        let i3 = Intersection {
            time: -3,
            object: s.clone(),
        };
        let i4 = Intersection {
            time: 2,
            object: s.clone(),
        };
        let xs = Intersections(vec![i1, i2, i3, i4.clone()]);
        let i = xs.hit().unwrap();
        assert_eq!(i, i4);
    }
}
