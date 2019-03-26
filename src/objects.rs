use crate::matrix;
use crate::{Material, Matrix4, Point, Scalar, Vector3};
use num_traits::{Float, One};
use std::iter::Sum;
use std::ops::Sub;

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

impl<T> PartialEq for Sphere<T>
where
    T: Scalar + Sub<Output = T>,
    f64: From<T>,
{
    fn eq(&self, other: &Sphere<T>) -> bool {
        self.transform.eq(&other.transform)
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
}
