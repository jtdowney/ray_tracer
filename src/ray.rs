use crate::matrix;
use crate::{Matrix4, Point, Scalar, Sphere, Vector3};
use num_traits::{Float, Zero};
use std::iter::Sum;
use std::ops::{Add, Mul, Sub};
use std::vec;

#[derive(Copy, Clone, Debug)]
pub struct Ray<T>
where
    T: Scalar,
{
    origin: Point<T>,
    direction: Vector3<T>,
}

impl<T> Ray<T>
where
    T: Scalar,
{
    pub fn new(origin: Point<T>, direction: Vector3<T>) -> Self {
        Ray { origin, direction }
    }
}

impl<T> Ray<T>
where
    T: Scalar + Add<Output = T> + Mul<Output = T>,
{
    pub fn position(&self, time: T) -> Point<T> {
        self.origin + self.direction * time
    }

    pub fn transform(self, transform: Matrix4<T>) -> Ray<T> {
        Ray {
            origin: transform * self.origin,
            direction: transform * self.direction,
        }
    }
}

impl<T> Ray<T>
where
    T: Scalar
        + Add<Output = T>
        + Float
        + From<u8>
        + Mul<Output = T>
        + Sub<Output = T>
        + Sum<T>
        + Zero,
    f64: From<T>,
{
    pub fn intersect(
        self,
        object: &Sphere<T>,
    ) -> Result<Intersections<T>, matrix::NotInvertableError> {
        let ray = self.transform(object.transform.inverse()?);
        let object_to_ray = ray.origin - Point::default();
        let a = ray.direction.dot(ray.direction);
        let b = Into::<T>::into(2u8) * ray.direction.dot(object_to_ray);
        let c = object_to_ray.dot(object_to_ray) - Into::<T>::into(1);
        let discriminant = b.powi(2) - Into::<T>::into(4) * a * c;

        let mut intersections = vec![];

        if discriminant >= T::zero() {
            let t1 = (-b - discriminant.sqrt()) / (Into::<T>::into(2) * a);
            let t2 = (-b + discriminant.sqrt()) / (Into::<T>::into(2) * a);
            intersections.push(Intersection { time: t1, object });
            intersections.push(Intersection { time: t2, object });
        }

        Ok(Intersections { intersections })
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Intersection<'a, T>
where
    T: Scalar + Sub<Output = T>,
    f64: From<T>,
{
    time: T,
    object: &'a Sphere<T>,
}

#[derive(Debug)]
pub struct Intersections<'a, T>
where
    T: Scalar + Sub<Output = T>,
    f64: From<T>,
{
    intersections: Vec<Intersection<'a, T>>,
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
    fn test_creating_ray() {
        let origin = Point::new(1, 2, 3);
        let direction = Vector3::new(4, 5, 6);
        let r = Ray::new(origin, direction);
        assert_eq!(origin, r.origin);
        assert_eq!(direction, r.direction);
    }

    #[test]
    fn test_computing_point_from_distance() {
        let r = Ray::new(Point::new(2.0, 3.0, 4.0), Vector3::new(1.0, 0.0, 0.0));
        assert_eq!(Point::new(2.0, 3.0, 4.0), r.position(0.0));
        assert_eq!(Point::new(3.0, 3.0, 4.0), r.position(1.0));
        assert_eq!(Point::new(1.0, 3.0, 4.0), r.position(-1.0));
        assert_eq!(Point::new(4.5, 3.0, 4.0), r.position(2.5));
    }

    #[test]
    fn test_ray_intersects_sphere_at_two_points() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let mut xs = r.intersect(&s).unwrap().into_iter();
        assert_eq!(4.0, xs.next().unwrap().time);
        assert_eq!(6.0, xs.next().unwrap().time);
        assert!(xs.next().is_none());
    }

    #[test]
    fn test_ray_intersects_sphere_at_tangent() {
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let mut xs = r.intersect(&s).unwrap().into_iter();
        assert_eq!(5.0, xs.next().unwrap().time);
        assert_eq!(5.0, xs.next().unwrap().time);
        assert!(xs.next().is_none());
    }

    #[test]
    fn test_ray_misses_sphere() {
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = r.intersect(&s).unwrap().into_iter();
        assert_eq!(0, xs.count());
    }

    #[test]
    fn test_ray_originates_inside_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let mut xs = r.intersect(&s).unwrap().into_iter();
        assert_eq!(-1.0, xs.next().unwrap().time);
        assert_eq!(1.0, xs.next().unwrap().time);
        assert!(xs.next().is_none());
    }

    #[test]
    fn test_sphere_behind_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector3::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let mut xs = r.intersect(&s).unwrap().into_iter();
        assert_eq!(-6.0, xs.next().unwrap().time);
        assert_eq!(-4.0, xs.next().unwrap().time);
        assert!(xs.next().is_none());
    }

    #[test]
    fn test_intersection_has_time_and_object() {
        let s = Sphere::new();
        let i = Intersection {
            time: 3.5,
            object: &s,
        };

        assert_eq!(3.5, i.time);
        assert_eq!(&s, i.object);
    }

    #[test]
    fn test_intersect_sets_objects() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let mut xs = r.intersect(&s).unwrap().into_iter();
        assert_eq!(&s, xs.next().unwrap().object);
        assert_eq!(&s, xs.next().unwrap().object);
        assert!(xs.next().is_none());
    }

    #[test]
    fn test_hit_with_all_positive_times() {
        let s = Sphere::new();
        let i1 = Intersection {
            time: 1,
            object: &s,
        };
        let i2 = Intersection {
            time: 2,
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
            time: -1,
            object: &s,
        };
        let i2 = Intersection {
            time: 1,
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
            time: -2,
            object: &s,
        };
        let i2 = Intersection {
            time: -1,
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
            time: 5,
            object: &s,
        };
        let i2 = Intersection {
            time: 7,
            object: &s,
        };
        let i3 = Intersection {
            time: -3,
            object: &s,
        };
        let i4 = Intersection {
            time: 2,
            object: &s,
        };
        let xs = Intersections {
            intersections: vec![i1, i2, i3, i4],
        };
        let i = xs.hit().unwrap();
        assert_eq!(i4, i);
    }

    #[test]
    fn test_translating_ray() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector3::new(0.0, 1.0, 0.0));
        let m = Matrix4::translation(3.0, 4.0, 5.0);
        let r2 = r.transform(m);
        assert_eq!(Point::new(4.0, 6.0, 8.0), r2.origin);
        assert_eq!(Vector3::new(0.0, 1.0, 0.0), r2.direction);
    }

    #[test]
    fn test_scaling_ray() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector3::new(0.0, 1.0, 0.0));
        let m = Matrix4::scaling(2.0, 3.0, 4.0);
        let r2 = r.transform(m);
        assert_eq!(Point::new(2.0, 6.0, 12.0), r2.origin);
        assert_eq!(Vector3::new(0.0, 3.0, 0.0), r2.direction);
    }

    #[test]
    fn test_intersecting_scaled_sphere_with_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.transform = Matrix4::scaling(2.0, 2.0, 2.0);
        let mut xs = r.intersect(&s).unwrap().into_iter();
        assert_eq!(3.0, xs.next().unwrap().time);
        assert_eq!(7.0, xs.next().unwrap().time);
        assert!(xs.next().is_none());
    }

    #[test]
    fn test_intersecting_translated_sphere_with_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.transform = Matrix4::translation(5.0, 0.0, 0.0);
        let mut xs = r.intersect(&s).unwrap().into_iter();
        assert!(xs.next().is_none());
    }
}
