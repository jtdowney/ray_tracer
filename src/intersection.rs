use crate::{matrix, Point, Ray, Scalar, Sphere, Vector3};
use num_traits::{Float, Zero};
use std::iter::Sum;
use std::ops::{Add, Mul, Sub};
use std::vec;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Computations<'a, T>
where
    T: Scalar + Sub<Output = T>,
    f64: From<T>,
{
    pub time: T,
    pub object: &'a Sphere<T>,
    pub point: Point<T>,
    pub eye_vector: Vector3<T>,
    pub normal_vector: Vector3<T>,
    pub inside: bool,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Intersection<'a, T>
where
    T: Scalar + Sub<Output = T>,
    f64: From<T>,
{
    pub time: T,
    pub object: &'a Sphere<T>,
}

impl<'a, T> Intersection<'a, T>
where
    T: Scalar + Add<Output = T> + Float + Mul<Output = T> + Sub<Output = T> + Sum<T> + Zero,
    f64: From<T>,
{
    pub fn prepare_computations(
        &self,
        ray: Ray<T>,
    ) -> Result<Computations<T>, matrix::NotInvertableError> {
        let point = ray.position(self.time);
        let eye_vector = -ray.direction;
        let mut normal_vector = self.object.normal_at(point)?;
        let inside: bool;

        if normal_vector.dot(eye_vector) < T::zero() {
            inside = true;
            normal_vector = -normal_vector;
        } else {
            inside = false;
        }

        Ok(Computations {
            time: self.time,
            object: self.object,
            point,
            eye_vector,
            normal_vector,
            inside,
        })
    }
}

#[derive(Debug)]
pub struct Intersections<'a, T>
where
    T: Scalar + Sub<Output = T>,
    f64: From<T>,
{
    pub intersections: Vec<Intersection<'a, T>>,
}

impl<'a, T> IntoIterator for Intersections<'a, T>
where
    T: Scalar + Sub<Output = T>,
    f64: From<T>,
{
    type Item = Intersection<'a, T>;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.intersections.into_iter()
    }
}

impl<'a, T> Intersections<'a, T>
where
    T: Scalar + Sub<Output = T> + PartialOrd + Zero,
    f64: From<T>,
{
    pub fn hit(self) -> Option<Intersection<'a, T>> {
        self.into_iter()
            .filter(|i| i.time >= T::zero())
            .min_by(|a, b| a.time.partial_cmp(&b.time).unwrap())
    }

    pub fn is_hit(self) -> bool {
        self.hit().is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hit_with_all_positive_times() {
        let s = Sphere::new();
        let i1 = Intersection {
            time: 1.0,
            object: &s,
        };
        let i2 = Intersection {
            time: 2.0,
            object: &s,
        };
        let xs = Intersections {
            intersections: vec![i1, i2],
        };
        let i = xs.hit().unwrap();
        assert_eq!(i1, i);
    }

    #[test]
    fn test_hit_with_some_negative_times() {
        let s = Sphere::new();
        let i1 = Intersection {
            time: -1.0,
            object: &s,
        };
        let i2 = Intersection {
            time: 1.0,
            object: &s,
        };
        let xs = Intersections {
            intersections: vec![i2, i1],
        };
        let i = xs.hit().unwrap();
        assert_eq!(i2, i);
    }

    #[test]
    fn test_hit_with_all_negative_times() {
        let s = Sphere::new();
        let i1 = Intersection {
            time: -2.0,
            object: &s,
        };
        let i2 = Intersection {
            time: -1.0,
            object: &s,
        };
        let xs = Intersections {
            intersections: vec![i2, i1],
        };
        assert!(xs.hit().is_none());
    }

    #[test]
    fn test_hit_lowest_positive_intersection() {
        let s = Sphere::new();
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
        let xs = Intersections {
            intersections: vec![i1, i2, i3, i4],
        };
        let i = xs.hit().unwrap();
        assert_eq!(i4, i);
    }

    #[test]
    fn test_precomputing_state_of_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = Intersection {
            time: 4.0,
            object: &shape,
        };
        let comps = i.prepare_computations(r).unwrap();
        assert_eq!(4.0, comps.time);
        assert_eq!(&shape, comps.object);
        assert_eq!(Point::new(0.0, 0.0, -1.0), comps.point);
        assert_eq!(Vector3::new(0.0, 0.0, -1.0), comps.eye_vector);
        assert_eq!(Vector3::new(0.0, 0.0, -1.0), comps.normal_vector);
        assert_eq!(false, comps.inside);
    }

    #[test]
    fn test_precomputing_state_of_intersection_with_hit_inside() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = Intersection {
            time: 1.0,
            object: &shape,
        };
        let comps = i.prepare_computations(r).unwrap();
        assert_eq!(Point::new(0.0, 0.0, 1.0), comps.point);
        assert_eq!(Vector3::new(0.0, 0.0, -1.0), comps.eye_vector);
        assert_eq!(Vector3::new(0.0, 0.0, -1.0), comps.normal_vector);
        assert_eq!(true, comps.inside);
    }
}
