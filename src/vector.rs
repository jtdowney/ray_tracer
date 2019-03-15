use std::ops::{Add, Div, Mul, Neg, Sub};

const EPSILON: f32 = 0.00001;

#[derive(Copy, Clone, Debug, Default)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x, y, z }
    }

    pub fn cross(&self, other: Vector) -> Vector {
        Vector::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn dot(&self, other: Vector) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vector {
        let magnitude = self.magnitude();
        Vector::new(self.x / magnitude, self.y / magnitude, self.z / magnitude)
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Self::Output {
        Vector::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Div<f32> for Vector {
    type Output = Vector;

    fn div(self, other: f32) -> Self::Output {
        Vector::new(self.x / other, self.y / other, self.z / other)
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, other: f32) -> Self::Output {
        Vector::new(self.x * other, self.y * other, self.z * other)
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self {
        Vector::new(-self.x, -self.y, -self.z)
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Self::Output {
        Vector::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        (self.x - other.x).abs() < EPSILON
            && (self.y - other.y).abs() < EPSILON
            && (self.z - other.z).abs() < EPSILON
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adding_vectors() {
        let v1 = Vector::new(3.0, -2.0, 5.0);
        let v2 = Vector::new(-2.0, 3.0, 1.0);
        assert_eq!(Vector::new(1.0, 1.0, 6.0), v1 + v2);
    }

    #[test]
    fn test_subtracting_vectors() {
        let v1 = Vector::new(3.0, 2.0, 1.0);
        let v2 = Vector::new(5.0, 6.0, 7.0);
        assert_eq!(Vector::new(-2.0, -4.0, -6.0), v1 - v2);
    }

    #[test]
    fn test_subtracting_vector_from_zero() {
        let zero = Vector::default();
        let v = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(Vector::new(-1.0, 2.0, -3.0), zero - v);
    }

    #[test]
    fn test_negating_vector() {
        let v = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(Vector::new(-1.0, 2.0, -3.0), -v);
    }

    #[test]
    fn test_multiplying_vector_by_scalar() {
        let v = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(Vector::new(3.5, -7.0, 10.5), v * 3.5);
    }

    #[test]
    fn test_multiplying_vector_by_fraction() {
        let v = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(Vector::new(0.5, -1.0, 1.5), v * 0.5);
    }

    #[test]
    fn test_dividing_vector_by_scalar() {
        let v = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(Vector::new(0.5, -1.0, 1.5), v / 2.0);
    }

    #[test]
    fn test_vector_magnitude() {
        let v = Vector::new(1.0, 0.0, 0.0);
        assert_eq!(1.0, v.magnitude());
        let v = Vector::new(0.0, 1.0, 0.0);
        assert_eq!(1.0, v.magnitude());
        let v = Vector::new(0.0, 0.0, 1.0);
        assert_eq!(1.0, v.magnitude());
        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!((14.0_f32).sqrt(), v.magnitude());
        let v = Vector::new(-1.0, -2.0, -3.0);
        assert_eq!((14.0_f32).sqrt(), v.magnitude());
    }

    #[test]
    fn test_vector_normalization() {
        let v = Vector::new(4.0, 0.0, 0.0);
        assert_eq!(Vector::new(1.0, 0.0, 0.0), v.normalize());
        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(Vector::new(0.26726, 0.53452, 0.80178), v.normalize());
    }

    #[test]
    fn test_vector_dot_product() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        assert_eq!(20.0, v1.dot(v2));
    }

    #[test]
    fn test_vector_cross_product() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        assert_eq!(Vector::new(-1.0, 2.0, -1.0), v1.cross(v2));
        assert_eq!(Vector::new(1.0, -2.0, 1.0), v2.cross(v1));
    }
}
