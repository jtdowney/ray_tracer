use crate::EPSILON;
use approx::relative_eq;
use std::iter::Sum;
use std::ops::{Add, Mul, Sub};

pub const BLACK: Color = Color::new(0.0, 0.0, 0.0);
pub const WHITE: Color = Color::new(1.0, 1.0, 1.0);

#[derive(Copy, Clone, Debug, Default)]
pub struct Color {
    red: f32,
    green: f32,
    blue: f32,
}

impl Color {
    pub const fn new(red: f32, green: f32, blue: f32) -> Color {
        Color { red, green, blue }
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, other: Color) -> Self::Output {
        Color::new(
            self.red + other.red,
            self.green + other.green,
            self.blue + other.blue,
        )
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Self::Output {
        Color::new(
            self.red * other.red,
            self.green * other.green,
            self.blue * other.blue,
        )
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, other: f32) -> Self::Output {
        Color::new(self.red * other, self.green * other, self.blue * other)
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, other: f64) -> Self::Output {
        self * (other as f32)
    }
}

impl Sub<Color> for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Self::Output {
        Color::new(
            self.red - other.red,
            self.green - other.green,
            self.blue - other.blue,
        )
    }
}

impl Sum<Color> for Color {
    fn sum<I: Iterator<Item = Color>>(iter: I) -> Self {
        iter.fold(Color::default(), |a, b| a + b)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        relative_eq!(self.red, other.red, epsilon = EPSILON as f32)
            && relative_eq!(self.green, other.green, epsilon = EPSILON as f32)
            && relative_eq!(self.blue, other.blue, epsilon = EPSILON as f32)
    }
}

impl IntoIterator for Color {
    type Item = u8;
    type IntoIter = ColorIterator;

    fn into_iter(self) -> Self::IntoIter {
        ColorIterator {
            color: self,
            index: 0,
        }
    }
}

pub struct ColorIterator {
    color: Color,
    index: u8,
}

impl Iterator for ColorIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        let color = match self.index {
            1 => self.color.red,
            2 => self.color.green,
            3 => self.color.blue,
            _ => return None,
        };

        let value = (color.min(1.0).max(0.0) * 255.0).round();
        Some(value as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        assert_eq!(Color::new(1.9, 1.2, 0.5), c1 + c2);
    }

    #[test]
    fn multiplying_color_by_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);
        assert_eq!(Color::new(0.4, 0.6, 0.8), c * 2.0);
    }

    #[test]
    fn multiplying_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        assert_eq!(Color::new(0.9, 0.2, 0.04), c1 * c2);
    }

    #[test]
    fn iterator_clamps() {
        let c = Color::new(1.5, 0.5, 0.1);
        let mut iter = c.into_iter();
        assert_eq!(Some(255), iter.next());
        let c = Color::new(-1.5, 0.5, 0.1);
        let mut iter = c.into_iter();
        assert_eq!(Some(0), iter.next());
    }

    #[test]
    fn iterator() {
        let c = Color::new(0.5, 0.3, 0.1);
        let mut iter = c.into_iter();
        assert_eq!(Some(128), iter.next());
        assert_eq!(Some(77), iter.next());
        assert_eq!(Some(26), iter.next());
        assert_eq!(None, iter.next());
    }
}
