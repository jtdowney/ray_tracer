use std::ops::{Add, Mul, Sub};

pub const BLACK: Color = Color {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
};

pub const WHITE: Color = Color {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
};

pub const RED: Color = Color {
    red: 1.0,
    green: 0.0,
    blue: 0.0,
};

pub fn color(red: impl Into<f64>, green: impl Into<f64>, blue: impl Into<f64>) -> Color {
    Color {
        red: red.into(),
        green: green.into(),
        blue: blue.into(),
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
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
        assert_relative_eq!(c.red, -0.5);
        assert_relative_eq!(c.green, 0.4);
        assert_relative_eq!(c.blue, 1.7);
    }

    #[test]
    fn adding_colors() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);
        let result = c1 + c2;
        assert_relative_eq!(result.red, 1.6);
        assert_relative_eq!(result.green, 0.7);
        assert_relative_eq!(result.blue, 1.0);
    }

    #[test]
    fn subtracting_colors() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);
        let result = c1 - c2;
        assert_relative_eq!(result.red, 0.2);
        assert_relative_eq!(result.green, 0.5);
        assert_relative_eq!(result.blue, 0.5);
    }

    #[test]
    fn multiplying_color_by_scalar() {
        let c = color(0.2, 0.3, 0.4);
        let result = c * 2.0;
        assert_relative_eq!(result.red, 0.4);
        assert_relative_eq!(result.green, 0.6);
        assert_relative_eq!(result.blue, 0.8);
    }

    #[test]
    fn multiplying_colors() {
        let c1 = color(1, 0.2, 0.4);
        let c2 = color(0.9, 1, 0.1);
        let result = c1 * c2;
        assert_relative_eq!(result.red, 0.9);
        assert_relative_eq!(result.green, 0.2);
        assert_relative_eq!(result.blue, 0.04);
    }
}
