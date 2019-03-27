use crate::matrix;
use crate::{Material, Matrix4, Point, Ray, Scalar, Vector3};
use num_traits::{Float, One, Zero};
use std::iter::Sum;
use std::ops::{Add, Mul, Sub};
use std::vec;

#[derive(Copy, Clone, Debug)]
pub struct Sphere<T: Scalar> {
    pub transform: Matrix4<T>,
    pub material: Material<T>,
}

impl<T> Sphere<T>
where
    T: Scalar + Float + From<f32> + Sub<Output = T> + One,
{
    pub fn new() -> Self {
        Sphere {
            transform: Matrix4::identity(),
            material: Material::default(),
        }
    }
}

impl<T> Sphere<T>
where
    T: Scalar + Float + Sub<Output = T> + Sum<T>,
{
    pub fn normal_at(
        &self,
        world_point: Point<T>,
    ) -> Result<Vector3<T>, matrix::NotInvertableError> {
        let object_point = self.transform.inverse()? * world_point;
        let object_normal = object_point - Point::default();
        let world_normal = self.transform.inverse()?.transpose() * object_normal;
        Ok(world_normal.normalize())
    }
}

impl<T> Sphere<T>
where
    T: Scalar
        + Add<Output = T>
        + Float
        + From<u16>
        + Mul<Output = T>
        + Sub<Output = T>
        + Sum<T>
        + Zero,
    f64: From<T>,
{
    pub fn intersect(&self, ray: Ray<T>) -> Result<Intersections<T>, matrix::NotInvertableError> {
        let ray = ray.transform(self.transform.inverse()?);
        let object_to_ray = ray.origin - Point::default();
        let a = ray.direction.dot(ray.direction);
        let b = Into::<T>::into(2) * ray.direction.dot(object_to_ray);
        let c = object_to_ray.dot(object_to_ray) - 1.into();
        let discriminant = b.powi(2) - Into::<T>::into(4) * a * c;

        let mut intersections = vec![];

        if discriminant >= T::zero() {
            let t1 = (-b - discriminant.sqrt()) / (Into::<T>::into(2) * a);
            let t2 = (-b + discriminant.sqrt()) / (Into::<T>::into(2) * a);
            intersections.push(Intersection {
                time: t1,
                object: self,
            });
            intersections.push(Intersection {
                time: t2,
                object: self,
            });
        }

        Ok(Intersections { intersections })
    }
}

impl<T> PartialEq for Sphere<T>
where
    T: Scalar + Sub<Output = T>,
    f64: From<T>,
{
    fn eq(&self, other: &Sphere<T>) -> bool {
        self.transform.eq(&other.transform)
    }
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
    use std::f32::consts::PI;

    #[test]
    fn test_spheres_default_transformation() {
        let s = Sphere::<f32>::new();
        assert_eq!(Matrix4::identity(), s.transform);
    }

    #[test]
    fn test_normal_on_sphere_at_x_axis() {
        let s = Sphere::new();
        assert_eq!(
            Vector3::new(1.0, 0.0, 0.0),
            s.normal_at(Point::new(1.0, 0.0, 0.0)).unwrap()
        );
    }

    #[test]
    fn test_normal_on_sphere_at_y_axis() {
        let s = Sphere::new();
        assert_eq!(
            Vector3::new(0.0, 1.0, 0.0),
            s.normal_at(Point::new(0.0, 1.0, 0.0)).unwrap()
        );
    }

    #[test]
    fn test_normal_on_sphere_at_z_axis() {
        let s = Sphere::new();
        assert_eq!(
            Vector3::new(0.0, 0.0, 1.0),
            s.normal_at(Point::new(0.0, 0.0, 1.0)).unwrap()
        );
    }

    #[test]
    fn test_normal_on_sphere_at_nonaxial_point() {
        let s = Sphere::new();
        assert_eq!(
            Vector3::new(3.0.sqrt() / 3.0, 3.0.sqrt() / 3.0, 3.0.sqrt() / 3.0),
            s.normal_at(Point::new(
                3.0.sqrt() / 3.0,
                3.0.sqrt() / 3.0,
                3.0.sqrt() / 3.0
            ))
            .unwrap()
        );
    }

    #[test]
    fn test_normal_is_a_normalized_vector() {
        let s = Sphere::new();
        let n = s
            .normal_at(Point::new(
                3.0.sqrt() / 3.0,
                3.0.sqrt() / 3.0,
                3.0.sqrt() / 3.0,
            ))
            .unwrap();
        assert_eq!(n, n.normalize());
    }

    #[test]
    fn test_computing_normal_on_translated_sphere() {
        let mut s = Sphere::new();
        s.transform = Matrix4::translation(0.0, 1.0, 0.0);
        assert_eq!(
            Vector3::new(0.0, 0.70711, -0.70711),
            s.normal_at(Point::new(0.0, 1.70711, -0.70711)).unwrap()
        );
    }

    #[test]
    fn test_computing_normal_on_transformed_sphere() {
        let mut s = Sphere::new();
        s.transform = Matrix4::scaling(1.0, 0.5, 1.0) * Matrix4::rotation_z(PI / 5.0);
        assert_eq!(
            Vector3::new(0.0, 0.97014, -0.24254),
            s.normal_at(Point::new(0.0, 2.0.sqrt() / 2.0, -2.0.sqrt() / 2.0))
                .unwrap()
        );
    }

    #[test]
    fn test_ray_intersects_sphere_at_two_points() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let mut xs = s.intersect(r).unwrap().into_iter();
        assert_eq!(4.0, xs.next().unwrap().time);
        assert_eq!(6.0, xs.next().unwrap().time);
        assert!(xs.next().is_none());
    }

    #[test]
    fn test_ray_intersects_sphere_at_tangent() {
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let mut xs = s.intersect(r).unwrap().into_iter();
        assert_eq!(5.0, xs.next().unwrap().time);
        assert_eq!(5.0, xs.next().unwrap().time);
        assert!(xs.next().is_none());
    }

    #[test]
    fn test_ray_misses_sphere() {
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r).unwrap().into_iter();
        assert_eq!(0, xs.count());
    }

    #[test]
    fn test_ray_originates_inside_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let mut xs = s.intersect(r).unwrap().into_iter();
        assert_eq!(-1.0, xs.next().unwrap().time);
        assert_eq!(1.0, xs.next().unwrap().time);
        assert!(xs.next().is_none());
    }

    #[test]
    fn test_sphere_behind_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector3::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let mut xs = s.intersect(r).unwrap().into_iter();
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
        let mut xs = s.intersect(r).unwrap().into_iter();
        assert_eq!(&s, xs.next().unwrap().object);
        assert_eq!(&s, xs.next().unwrap().object);
        assert!(xs.next().is_none());
    }

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
    fn test_intersecting_scaled_sphere_with_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.transform = Matrix4::scaling(2.0, 2.0, 2.0);
        let mut xs = s.intersect(r).unwrap().into_iter();
        assert_eq!(3.0, xs.next().unwrap().time);
        assert_eq!(7.0, xs.next().unwrap().time);
        assert!(xs.next().is_none());
    }

    #[test]
    fn test_intersecting_translated_sphere_with_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.transform = Matrix4::translation(5.0, 0.0, 0.0);
        let mut xs = s.intersect(r).unwrap().into_iter();
        assert!(xs.next().is_none());
    }
}