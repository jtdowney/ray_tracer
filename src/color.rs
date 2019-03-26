use crate::Scalar;
use num_traits::{Float, One, Zero};
use std::ops::{Add, Mul, Sub};

#[derive(Copy, Clone, Debug, Default)]
pub struct Color<T> {
    red: T,
    green: T,
    blue: T,
}

impl<T: Scalar> Color<T> {
    pub fn new(red: T, green: T, blue: T) -> Color<T> {
        Color { red, green, blue }
    }
}

impl<T> Add<Color<T>> for Color<T>
where
    T: Scalar + Add<Output = T>,
{
    type Output = Color<T>;

    fn add(self, other: Color<T>) -> Self::Output {
        Color::new(
            self.red + other.red,
            self.green + other.green,
            self.blue + other.blue,
        )
    }
}

impl<T> Mul<Color<T>> for Color<T>
where
    T: Scalar + Mul<Output = T>,
{
    type Output = Color<T>;

    fn mul(self, other: Color<T>) -> Self::Output {
        Color::new(
            self.red * other.red,
            self.green * other.green,
            self.blue * other.blue,
        )
    }
}

impl<T> Mul<T> for Color<T>
where
    T: Scalar + Mul<Output = T>,
{
    type Output = Color<T>;

    fn mul(self, other: T) -> Self::Output {
        Color::new(self.red * other, self.green * other, self.blue * other)
    }
}

impl<T> PartialEq for Color<T>
where
    T: Scalar + Sub<Output = T>,
    f64: From<T>,
{
    fn eq(&self, other: &Color<T>) -> bool {
        const EPSILON: f64 = 0.00001;
        f64::from(self.red - other.red).abs() < EPSILON
            && f64::from(self.green - other.green).abs() < EPSILON
            && f64::from(self.blue - other.blue).abs() < EPSILON
    }
}

impl<T> IntoIterator for Color<T>
where
    T: Scalar + Float + From<f32> + One + Zero,
{
    type Item = u8;
    type IntoIter = ColorIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        ColorIterator {
            color: self,
            index: 0,
        }
    }
}

pub struct ColorIterator<T: Scalar> {
    color: Color<T>,
    index: u8,
}

impl<T> Iterator for ColorIterator<T>
where
    T: Scalar + Float + From<f32> + One + Zero,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        let color = match self.index {
            1 => self.color.red,
            2 => self.color.green,
            3 => self.color.blue,
            _ => return None,
        };

        (color.min(T::one()).max(T::zero()) * Into::<T>::into(255.0))
            .round()
            .to_u8()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adding_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        assert_eq!(Color::new(1.9, 1.2, 0.5), c1 + c2);
    }

    #[test]
    fn test_multiplying_color_by_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);
        assert_eq!(Color::new(0.4, 0.6, 0.8), c * 2.0);
    }

    #[test]
    fn test_multiplying_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        assert_eq!(Color::new(0.9, 0.2, 0.04), c1 * c2);
    }

    #[test]
    fn test_iterator_clamps() {
        let c = Color::new(1.5, 0.5, 0.1);
        let mut iter = c.into_iter();
        assert_eq!(Some(255), iter.next());
        let c = Color::new(-1.5, 0.5, 0.1);
        let mut iter = c.into_iter();
        assert_eq!(Some(0), iter.next());
    }

    #[test]
    fn test_iterator() {
        let c = Color::new(0.5, 0.3, 0.1);
        let mut iter = c.into_iter();
        assert_eq!(Some(128), iter.next());
        assert_eq!(Some(77), iter.next());
        assert_eq!(Some(26), iter.next());
        assert_eq!(None, iter.next());
    }
}
