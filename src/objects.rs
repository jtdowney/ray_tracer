use crate::{Matrix4, Scalar};
use num_traits::One;
use std::ops::Sub;

#[derive(Copy, Clone, Debug)]
pub struct Sphere<T: Scalar> {
    pub transform: Matrix4<T>,
}

impl<T> Sphere<T>
where
    T: Scalar + Sub<Output = T> + One,
{
    pub fn new() -> Self {
        Sphere {
            transform: Matrix4::identity(),
        }
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

    #[test]
    fn test_spheres_default_transformation() {
        let s = Sphere::<i8>::new();
        assert_eq!(Matrix4::identity(), s.transform);
    }
}
