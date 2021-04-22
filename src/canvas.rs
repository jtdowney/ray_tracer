use crate::{ByteScale, Color};
use num::Float;
use std::fmt::{self, Write};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum CanvasError {
    #[error("out out bounds pixel write at {0}, {1}")]
    PixelOutOfBounds(usize, usize),
    #[error("unable to format PPM")]
    Format(#[from] fmt::Error),
}

pub struct Canvas<T>
where
    T: Copy,
{
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color<T>>,
}

impl<T> Canvas<T>
where
    T: Copy + Default,
{
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![Default::default(); width * height];
        Self {
            width,
            height,
            pixels,
        }
    }
}

impl<T> Canvas<T>
where
    T: Copy,
{
    pub fn write_pixel(&mut self, x: usize, y: usize, pixel: Color<T>) -> Result<(), CanvasError> {
        if x >= self.width || y >= self.height {
            Err(CanvasError::PixelOutOfBounds(x, y))
        } else {
            let index = y * self.width + x;
            self.pixels[index] = pixel;
            Ok(())
        }
    }

    pub fn pixel_at(&mut self, x: usize, y: usize) -> Result<Color<T>, CanvasError> {
        if x >= self.width || y >= self.height {
            Err(CanvasError::PixelOutOfBounds(x, y))
        } else {
            let index = y * self.width + x;
            let pixel = self.pixels[index];
            Ok(pixel)
        }
    }

    pub fn fill(&mut self, pixel: Color<T>) -> Result<(), CanvasError> {
        for y in 0..self.height {
            for x in 0..self.width {
                self.write_pixel(x, y, pixel)?;
            }
        }

        Ok(())
    }
}

impl<T> Canvas<T>
where
    T: Float + ByteScale + Copy,
{
    pub fn to_ppm(&self) -> Result<String, CanvasError> {
        let mut output = String::new();
        writeln!(output, "P3")?;
        writeln!(output, "{} {}", self.width, self.height)?;
        writeln!(output, "255")?;

        for y in 0..self.height {
            let offset = y as usize * self.width as usize;
            let codes = self.pixels[offset..]
                .iter()
                .take(self.width as usize)
                .flat_map(|&p| p)
                .map(|n| {
                    let clamped = num::clamp(n, T::zero(), T::one());
                    let scaled = clamped.byte_scale();
                    scaled.to_string()
                })
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
    use crate::color;

    #[test]
    fn creating_blank_canvas() {
        let canvas = Canvas::new(10, 20);
        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);

        for pixel in canvas.pixels {
            assert_eq!(pixel, color(0, 0, 0));
        }
    }

    #[test]
    fn writing_pixels_to_canvas() {
        let mut canvas = Canvas::new(10, 20);
        let red = color(1, 0, 0);
        canvas.write_pixel(2, 3, red).unwrap();
        assert_eq!(canvas.pixel_at(2, 3), Ok(red));
    }

    #[test]
    fn constructing_ppm_header() {
        let canvas: Canvas<f32> = Canvas::new(5, 3);
        let ppm = canvas.to_ppm().unwrap();
        let mut lines = ppm.lines();
        assert_eq!(lines.next(), Some("P3"));
        assert_eq!(lines.next(), Some("5 3"));
        assert_eq!(lines.next(), Some("255"));
    }

    #[test]
    fn constructing_ppm_pixel_data() {
        let mut canvas = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        canvas.write_pixel(0, 0, c1).unwrap();
        canvas.write_pixel(2, 1, c2).unwrap();
        canvas.write_pixel(4, 2, c3).unwrap();

        let ppm = canvas.to_ppm().unwrap();
        let mut lines = ppm.lines().skip(3);
        assert_eq!(Some("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0"), lines.next());
        assert_eq!(Some("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0"), lines.next());
        assert_eq!(Some("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"), lines.next());
    }

    #[test]
    fn splitting_long_lines_in_ppm() {
        let mut canvas = Canvas::new(10, 2);
        let c = Color::new(1.0, 0.8, 0.6);
        canvas.fill(c).unwrap();

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
    fn ppm_files_are_newline_terminated() {
        let canvas: Canvas<f32> = Canvas::new(5, 3);
        let ppm = canvas.to_ppm().unwrap();
        let line = ppm.lines().last();
        assert_eq!(Some(""), line);
    }
}
