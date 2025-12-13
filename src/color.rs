use std::ops::{Add, Mul, Sub};

use num_traits::AsPrimitive;
use wide::f32x4;

pub const BLACK: Color = Color {
    data: f32x4::new([0.0, 0.0, 0.0, 0.0]),
};

pub const WHITE: Color = Color {
    data: f32x4::new([1.0, 1.0, 1.0, 0.0]),
};

pub const RED: Color = Color {
    data: f32x4::new([1.0, 0.0, 0.0, 0.0]),
};

pub fn color(
    red: impl AsPrimitive<f32>,
    green: impl AsPrimitive<f32>,
    blue: impl AsPrimitive<f32>,
) -> Color {
    Color {
        data: f32x4::new([red.as_(), green.as_(), blue.as_(), 0.0]),
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub(crate) data: f32x4,
}

impl Default for Color {
    fn default() -> Self {
        BLACK
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.red() == other.red() && self.green() == other.green() && self.blue() == other.blue()
    }
}

impl Color {
    #[must_use]
    pub fn red(&self) -> f32 {
        self.data.as_array()[0]
    }

    #[must_use]
    pub fn green(&self) -> f32 {
        self.data.as_array()[1]
    }

    #[must_use]
    pub fn blue(&self) -> f32 {
        self.data.as_array()[2]
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color {
            data: self.data + rhs.data,
        }
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        Color {
            data: self.data - rhs.data,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        Color {
            data: self.data * f32x4::splat(rhs),
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color {
            data: self.data * rhs.data,
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn colors_are_red_green_blue_tuples() {
        let c = color(-0.5, 0.4, 1.7);
        assert_relative_eq!(c.red(), -0.5);
        assert_relative_eq!(c.green(), 0.4);
        assert_relative_eq!(c.blue(), 1.7);
    }

    #[test]
    fn adding_colors() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);
        let result = c1 + c2;
        assert_relative_eq!(result.red(), 1.6);
        assert_relative_eq!(result.green(), 0.7);
        assert_relative_eq!(result.blue(), 1.0);
    }

    #[test]
    fn subtracting_colors() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);
        let result = c1 - c2;
        assert_relative_eq!(result.red(), 0.2);
        assert_relative_eq!(result.green(), 0.5);
        assert_relative_eq!(result.blue(), 0.5);
    }

    #[test]
    fn multiplying_color_by_scalar() {
        let c = color(0.2, 0.3, 0.4);
        let result = c * 2.0;
        assert_relative_eq!(result.red(), 0.4);
        assert_relative_eq!(result.green(), 0.6);
        assert_relative_eq!(result.blue(), 0.8);
    }

    #[test]
    fn multiplying_colors() {
        let c1 = color(1, 0.2, 0.4);
        let c2 = color(0.9, 1, 0.1);
        let result = c1 * c2;
        assert_relative_eq!(result.red(), 0.9);
        assert_relative_eq!(result.green(), 0.2);
        assert_relative_eq!(result.blue(), 0.04);
    }
}
