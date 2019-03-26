use crate::{Color, Scalar};
use num_traits::{Float, One, Zero};
use std::fmt::{self, Write};

#[derive(Debug)]
pub struct Canvas<T: Scalar> {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color<T>>,
}

impl<T> Canvas<T>
where
    T: Scalar + Float + From<f32> + One + Zero,
{
    pub fn new(width: usize, height: usize) -> Canvas<T> {
        Canvas {
            width,
            height,
            pixels: vec![Color::default(); width * height],
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, pixel: Color<T>) {
        self.pixels[y * self.width + x] = pixel;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color<T> {
        self.pixels[y * self.width + x]
    }

    pub fn fill(&mut self, color: Color<T>) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.write_pixel(x, y, color);
            }
        }
    }

    pub fn to_ppm(&self) -> Result<String, fmt::Error> {
        let mut output = String::new();
        writeln!(output, "P3")?;
        writeln!(output, "{} {}", self.width, self.height)?;
        writeln!(output, "255")?;

        for y in 0..self.height {
            let codes = self.pixels[y * self.width..]
                .iter()
                .take(self.width)
                .flat_map(|&p| p)
                .map(|b| b.to_string())
                .collect::<Vec<String>>();
            for line in codes.chunks(17) {
                writeln!(output, "{}", line.join(" "))?;
            }
        }

        writeln!(output)?;

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creating_canvas() {
        let canvas = Canvas::new(10, 20);
        assert_eq!(10, canvas.width);
        assert_eq!(20, canvas.height);
        for pixel in canvas.pixels {
            assert_eq!(Color::new(0.0, 0.0, 0.0), pixel);
        }
    }

    #[test]
    fn test_writing_pixel_to_canvas() {
        let mut canvas = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        canvas.write_pixel(2, 3, red);
        assert_eq!(red, canvas.pixel_at(2, 3));
    }

    #[test]
    fn test_constructing_ppm_header() {
        let canvas = Canvas::<f32>::new(5, 3);
        let ppm = canvas.to_ppm().unwrap();
        let mut lines = ppm.lines();
        assert_eq!(Some("P3"), lines.next());
        assert_eq!(Some("5 3"), lines.next());
        assert_eq!(Some("255"), lines.next());
    }

    #[test]
    fn test_constructing_ppm_pixel_data() {
        let mut canvas = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        canvas.write_pixel(0, 0, c1);
        canvas.write_pixel(2, 1, c2);
        canvas.write_pixel(4, 2, c3);
        let ppm = canvas.to_ppm().unwrap();
        let mut lines = ppm.lines().skip(3);
        assert_eq!(Some("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0"), lines.next());
        assert_eq!(Some("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0"), lines.next());
        assert_eq!(Some("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"), lines.next());
    }

    #[test]
    fn test_splitting_long_lines_in_ppm() {
        let mut canvas = Canvas::new(10, 2);
        let c = Color::new(1.0, 0.8, 0.6);
        canvas.fill(c);
        let ppm = canvas.to_ppm().unwrap();
        let mut lines = ppm.lines().skip(3);
        assert_eq!(
            Some("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"),
            lines.next()
        );
        assert_eq!(
            Some("153 255 204 153 255 204 153 255 204 153 255 204 153"),
            lines.next()
        );
        assert_eq!(
            Some("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"),
            lines.next()
        );
        assert_eq!(
            Some("153 255 204 153 255 204 153 255 204 153 255 204 153"),
            lines.next()
        );
    }

    #[test]
    fn test_ppm_files_are_newline_terminated() {
        let canvas = Canvas::<f32>::new(5, 3);
        let ppm = canvas.to_ppm().unwrap();
        let line = ppm.lines().last();
        assert_eq!(Some(""), line);
    }
}
