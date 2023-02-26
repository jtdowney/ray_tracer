use std::ops::{Add, Mul, Sub};

pub fn color<T: Into<f64>>(red: T, green: T, blue: T) -> Color {
    Color {
        red: red.into(),
        green: green.into(),
        blue: blue.into(),
    }
}

#[cfg(test)]
pub const BLACK: Color = Color {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Self) -> Self::Output {
        let red = self.red + other.red;
        let green = self.green + other.green;
        let blue = self.blue + other.blue;
        Self { red, green, blue }
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Self) -> Self::Output {
        let red = self.red - other.red;
        let green = self.green - other.green;
        let blue = self.blue - other.blue;
        Self { red, green, blue }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Self) -> Self::Output {
        let red = self.red * other.red;
        let green = self.green * other.green;
        let blue = self.blue * other.blue;
        Self { red, green, blue }
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, other: f64) -> Self::Output {
        let red = self.red * other;
        let green = self.green * other;
        let blue = self.blue * other;
        Self { red, green, blue }
    }
}

#[cfg(test)]
impl quickcheck::Arbitrary for Color {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let red = f64::from(i32::arbitrary(g));
        let green = f64::from(i32::arbitrary(g));
        let blue = f64::from(i32::arbitrary(g));
        Self { red, green, blue }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use super::*;

    #[quickcheck]
    fn adding_colors(a: Color, b: Color) {
        let c = a + b;
        assert_eq!(a.red + b.red, c.red);
        assert_eq!(a.green + b.green, c.green);
        assert_eq!(a.blue + b.blue, c.blue);
    }

    #[quickcheck]
    fn subtracting_colors(a: Color, b: Color) {
        let c = a - b;
        assert_eq!(a.red - b.red, c.red);
        assert_eq!(a.green - b.green, c.green);
        assert_eq!(a.blue - b.blue, c.blue);
    }

    #[quickcheck]
    fn scalar_multiplying_color(v: Color, scale: i32) {
        let scale = f64::from(scale);
        let scaled = v * scale;
        assert_eq!(v.red * scale, scaled.red);
        assert_eq!(v.green * scale, scaled.green);
        assert_eq!(v.blue * scale, scaled.blue);
    }

    #[quickcheck]
    fn hadamard_product_colors(a: Color, b: Color) {
        let c = a * b;
        assert_eq!(a.red * b.red, c.red);
        assert_eq!(a.green * b.green, c.green);
        assert_eq!(a.blue * b.blue, c.blue);
    }
}
