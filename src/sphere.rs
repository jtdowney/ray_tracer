use crate::{intersection, point, Intersections, Matrix4, Ray};
use num::{Float, Num};
use point::Point;
use std::{iter::Sum, rc::Rc};

pub fn sphere<T>() -> Rc<Sphere<T>>
where
    T: Num + Copy,
{
    Rc::new(Sphere::default())
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Sphere<T>
where
    T: Copy,
{
    pub transform: Matrix4<T>,
}

impl<T> Default for Sphere<T>
where
    T: Num + Copy,
{
    fn default() -> Self {
        let transform = Matrix4::identity();
        Sphere { transform }
    }
}

impl<T> Sphere<T>
where
    T: Float + PartialOrd + Sum + Copy,
{
    pub fn intersect(self: Rc<Self>, ray: Ray<T>) -> Intersections<T> {
        let ray = ray.transform(self.transform.inverse());
        let diameter = T::one() + T::one();
        let two_diameter = diameter + diameter;
        let sphere_to_ray = ray.origin - Point::origin();
        let a = ray.direction.dot(ray.direction);
        let b = diameter * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - T::one();
        let discriminant = (b * b) - two_diameter * a * c;

        if discriminant.is_sign_negative() {
            Intersections::empty()
        } else {
            let discriminant_root = Float::sqrt(discriminant);
            let t1 = (-b - discriminant_root) / (diameter * a);
            let t2 = (-b + discriminant_root) / (diameter * a);

            let intersections = vec![intersection(t1, self.clone()), intersection(t2, self)];

            intersections.into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{point, ray, transformations, vector};

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let s = sphere();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut xs = s.intersect(r).into_iter();
        assert_eq!(4.0, xs.next().unwrap().time);
        assert_eq!(6.0, xs.next().unwrap().time);
        assert!(xs.next().is_none());
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let s = sphere();
        let r = ray(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut xs = s.intersect(r).into_iter();
        assert_eq!(5.0, xs.next().unwrap().time);
        assert_eq!(5.0, xs.next().unwrap().time);
        assert!(xs.next().is_none());
    }

    #[test]
    fn ray_misses_sphere() {
        let s = sphere();
        let r = ray(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = s.intersect(r).into_iter();
        assert_eq!(0, xs.count());
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let s = sphere();
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let mut xs = s.intersect(r).into_iter();
        assert_eq!(-1.0, xs.next().unwrap().time);
        assert_eq!(1.0, xs.next().unwrap().time);
        assert!(xs.next().is_none());
    }

    #[test]
    fn sphere_behind_ray() {
        let s = sphere();
        let r = ray(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let mut xs = s.intersect(r).into_iter();
        assert_eq!(-6.0, xs.next().unwrap().time);
        assert_eq!(-4.0, xs.next().unwrap().time);
        assert!(xs.next().is_none());
    }

    #[test]
    fn spheres_default_transformation() {
        let s = sphere::<i32>();
        assert_eq!(Matrix4::identity(), s.transform);
    }

    #[test]
    fn intersect_scaled_ray_with_sphere() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = sphere();
        Rc::make_mut(&mut s).transform = transformations::scaling(2.0, 2.0, 2.0);

        let mut xs = s.intersect(r).into_iter();
        assert_eq!(3.0, xs.next().unwrap().time);
        assert_eq!(7.0, xs.next().unwrap().time);
        assert!(xs.next().is_none());
    }

    #[test]
    fn intersect_translated_ray_with_sphere() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = sphere();
        Rc::make_mut(&mut s).transform = transformations::translation(5.0, 0.0, 0.0);

        let mut xs = s.intersect(r).into_iter();
        assert!(xs.next().is_none());
    }
}
