use crate::{
    intersection, matrix, transforms, Color, Intersection, Intersections, Point, PointLight, Ray,
    Scalar, Sphere,
};
use num_traits::{Float, One, Zero};
use std::iter::Sum;
use std::ops::{Add, Mul, Sub};

#[derive(Debug)]
pub struct World<T: Scalar> {
    objects: Vec<Sphere<T>>,
    light: PointLight<T>,
}

impl<T> World<T>
where
    T: Scalar,
{
    pub fn new<I>(light: PointLight<T>, objects: I) -> Self
    where
        I: IntoIterator<Item = Sphere<T>>,
    {
        let objects = objects.into_iter().collect();
        World { objects, light }
    }
}

impl<T> Default for World<T>
where
    T: Scalar + Float + From<f32> + Sub<Output = T> + One,
{
    fn default() -> Self {
        let mut s1 = Sphere::default();
        s1.material.color = Color::new(0.8.into(), 1.0.into(), 0.6.into());
        s1.material.diffuse = 0.7.into();
        s1.material.specular = 0.2.into();

        let mut s2 = Sphere::default();
        s2.transform = transforms::scaling(0.5.into(), 0.5.into(), 0.5.into());

        let objects = vec![s1, s2];
        let light = PointLight::new(
            Point::new((-10.0).into(), 10.0.into(), (-10.0).into()),
            Color::new(T::one(), T::one(), T::one()),
        );

        World { objects, light }
    }
}

impl<T> World<T>
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
        let mut intersections = self
            .objects
            .iter()
            .flat_map(|o| o.intersect(ray).map(|i| i.into_iter()))
            .flat_map(|i| i)
            .collect::<Vec<Intersection<T>>>();
        intersections.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
        Ok(Intersections { intersections })
    }

    pub fn color_at(&self, ray: Ray<T>) -> Result<Color<T>, matrix::NotInvertableError> {
        let intersections = self.intersect(ray)?;
        if let Some(hit) = intersections.hit() {
            let comps = hit.prepare_computations(ray)?;
            Ok(self.shade_hit(comps))
        } else {
            Ok(Color::default())
        }
    }
}

impl<T> World<T>
where
    T: Scalar + Add<Output = T> + Float + From<u16> + Sub<Output = T> + Sum<T>,
    f64: From<T>,
{
    pub fn shade_hit(&self, comps: intersection::Computations<T>) -> Color<T> {
        comps.object.material.lighting(
            self.light,
            comps.point,
            comps.eye_vector,
            comps.normal_vector,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Vector3;

    #[test]
    fn test_intersect_world_with_ray() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let mut xs = w.intersect(r).unwrap().into_iter();
        assert_eq!(4.0, xs.next().unwrap().time);
        assert_eq!(4.5, xs.next().unwrap().time);
        assert_eq!(5.5, xs.next().unwrap().time);
        assert_eq!(6.0, xs.next().unwrap().time);
        assert!(xs.next().is_none());
    }

    #[test]
    fn test_shading_an_intersection() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let shape = w.objects[0];
        let i = Intersection {
            time: 4.0,
            object: &shape,
        };
        let comps = i.prepare_computations(r).unwrap();
        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), w.shade_hit(comps));
    }

    #[test]
    fn test_color_when_ray_misses() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 1.0, 0.0));
        assert_eq!(Color::new(0.0, 0.0, 0.0), w.color_at(r).unwrap());
    }

    #[test]
    fn test_color_when_ray_hits() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), w.color_at(r).unwrap());
    }

    #[test]
    fn test_color_with_intersection_behind_ray() {
        let mut w = World::default();
        w.objects[0].material.ambient = 1.0;
        w.objects[1].material.ambient = 1.0;
        let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector3::new(0.0, 0.0, -1.0));
        assert_eq!(w.objects[1].material.color, w.color_at(r).unwrap());
    }
}
