use approx::AbsDiffEq;
use std::ops::{Add, Mul, Sub};

pub fn color<N>(r: N, g: N, b: N) -> Color<N>
where
    N: Copy,
{
    Color::new(r, g, b)
}

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct Color<N>
where
    N: Copy,
{
    pub r: N,
    pub g: N,
    pub b: N,
}

impl<N> Color<N>
where
    N: Copy,
{
    pub fn new(r: N, g: N, b: N) -> Self {
        Self { r, g, b }
    }
}

impl<N: AbsDiffEq> AbsDiffEq for Color<N>
where
    N: Copy,
    N::Epsilon: Copy,
{
    type Epsilon = N::Epsilon;

    fn default_epsilon() -> N::Epsilon {
        N::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: N::Epsilon) -> bool {
        N::abs_diff_eq(&self.r, &other.r, epsilon)
            && N::abs_diff_eq(&self.g, &other.g, epsilon)
            && N::abs_diff_eq(&self.b, &other.b, epsilon)
    }
}

impl<N> Add for Color<N>
where
    N: Add<Output = N> + Copy,
{
    type Output = Color<N>;

    fn add(self, rhs: Self) -> Self::Output {
        Color::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl<N> Sub for Color<N>
where
    N: Sub<Output = N> + Copy,
{
    type Output = Color<N>;

    fn sub(self, rhs: Self) -> Self::Output {
        Color::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl<N> Mul<N> for Color<N>
where
    N: Mul<Output = N> + Copy,
{
    type Output = Color<N>;

    fn mul(self, rhs: N) -> Self::Output {
        Color::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl<N> Mul for Color<N>
where
    N: Mul<Output = N> + Copy,
{
    type Output = Color<N>;

    fn mul(self, rhs: Self) -> Self::Output {
        Color::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl<N> IntoIterator for Color<N>
where
    N: Copy,
{
    type Item = N;
    type IntoIter = ColorIterator<N>;

    fn into_iter(self) -> Self::IntoIter {
        ColorIterator {
            color: self,
            index: 0,
        }
    }
}

pub struct ColorIterator<N>
where
    N: Copy,
{
    color: Color<N>,
    index: usize,
}

impl<N> Iterator for ColorIterator<N>
where
    N: Copy,
{
    type Item = N;

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
