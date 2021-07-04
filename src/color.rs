use crate::EPSILON;
use approx::AbsDiffEq;
use std::ops::{Add, Mul, Sub};

pub const BLACK: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
};

pub const WHITE: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
};

pub fn color(r: f64, g: f64, b: f64) -> Color {
    Color { r, g, b }
}

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl AbsDiffEq for Color {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        f64::abs_diff_eq(&self.r, &other.r, epsilon)
            && f64::abs_diff_eq(&self.g, &other.g, epsilon)
            && f64::abs_diff_eq(&self.b, &other.b, epsilon)
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        color(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        color(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        color(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        color(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl IntoIterator for Color {
    type Item = f64;
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
    index: usize,
}

impl Iterator for ColorIterator {
    type Item = f64;

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
        assert_abs_diff_eq!(c1 - c2, color(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiplying_color_by_scalar() {
        let c = color(0.2, 0.3, 0.4);
        assert_abs_diff_eq!(c * 2.0, color(0.4, 0.6, 0.8));
    }

    #[test]
    fn multiplying_colors() {
        let c1 = color(1.0, 0.2, 0.4);
        let c2 = color(0.9, 1.0, 0.1);
        assert_abs_diff_eq!(c1 * c2, color(0.9, 0.2, 0.04));
    }
}
