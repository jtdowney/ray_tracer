use approx::AbsDiffEq;
use std::ops::{Add, Mul, Sub};

pub const BLACK: Color<f64> = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
};

pub const WHITE: Color<f64> = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
};

pub fn color<T>(r: T, g: T, b: T) -> Color<T>
where
    T: Copy,
{
    Color { r, g, b }
}

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct Color<T>
where
    T: Copy,
{
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T> AbsDiffEq for Color<T>
where
    T: AbsDiffEq + Copy,
    T::Epsilon: Copy,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> T::Epsilon {
        T::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: T::Epsilon) -> bool {
        T::abs_diff_eq(&self.r, &other.r, epsilon)
            && T::abs_diff_eq(&self.g, &other.g, epsilon)
            && T::abs_diff_eq(&self.b, &other.b, epsilon)
    }
}

impl<T> Add for Color<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Color<T>;

    fn add(self, rhs: Self) -> Self::Output {
        color(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl<T> Sub for Color<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Color<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        color(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl<T> Mul<T> for Color<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Color<T>;

    fn mul(self, rhs: T) -> Self::Output {
        color(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl<T> Mul for Color<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Color<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        color(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl<T> IntoIterator for Color<T>
where
    T: Copy,
{
    type Item = T;
    type IntoIter = ColorIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        ColorIterator {
            color: self,
            index: 0,
        }
    }
}

pub struct ColorIterator<T>
where
    T: Copy,
{
    color: Color<T>,
    index: usize,
}

impl<T> Iterator for ColorIterator<T>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = match self.index {
            0 => self.color.r,
            1 => self.color.g,
            2 => self.color.b,
            _ => return None,
        };

        self.index += 1;
        Some(item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EPSILON;
    use approx::assert_abs_diff_eq;

    #[test]
    fn adding_colors() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, color(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtracting_colors() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);
        assert_abs_diff_eq!(c1 - c2, color(0.2, 0.5, 0.5), epsilon = EPSILON);
    }

    #[test]
    fn multiplying_color_by_scalar() {
        let c = color(0.2, 0.3, 0.4);
        assert_abs_diff_eq!(c * 2.0, color(0.4, 0.6, 0.8), epsilon = EPSILON);
    }

    #[test]
    fn multiplying_colors() {
        let c1 = color(1.0, 0.2, 0.4);
        let c2 = color(0.9, 1.0, 0.1);
        assert_abs_diff_eq!(c1 * c2, color(0.9, 0.2, 0.04), epsilon = EPSILON);
    }
}
