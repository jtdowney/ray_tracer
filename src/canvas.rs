use anyhow::bail;

use crate::{clamp, Color};
use std::fmt::Write;

pub fn canvas(width: u16, height: u16) -> Canvas {
    Canvas::new(width, height)
}

#[derive(Clone, Debug)]
pub struct Canvas {
    pub width: u16,
    pub height: u16,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: u16, height: u16) -> Self {
        let pixels = vec![Default::default(); width as usize * height as usize];
        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn write_pixel(&mut self, x: u16, y: u16, pixel: Color) -> anyhow::Result<()> {
        if x >= self.width || y >= self.height {
            bail!("out of bounds pixel write");
        } else {
            let index = y as usize * self.width as usize + x as usize;
            self.pixels[index] = pixel;
            Ok(())
        }
    }

    pub fn pixel_at(&self, x: u16, y: u16) -> anyhow::Result<Color> {
        if x >= self.width || y >= self.height {
            bail!("out of bounds pixel read");
        } else {
            let index = y as usize * self.width as usize + x as usize;
            let pixel = self.pixels[index];
            Ok(pixel)
        }
    }

    pub fn fill(&mut self, pixel: Color) -> anyhow::Result<()> {
        for y in 0..self.height {
            for x in 0..self.width {
                self.write_pixel(x, y, pixel)?;
            }
        }

        Ok(())
    }

    pub fn to_ppm(&self) -> anyhow::Result<String> {
        let mut output = String::new();
        writeln!(output, "P3")?;
        writeln!(output, "{} {}", self.width, self.height)?;
        writeln!(output, "255")?;

        for y in 0..self.height {
            let offset = y as usize * self.width as usize;
            let codes = self.pixels[offset..]
                .iter()
                .take(self.width as usize)
                .flat_map(|&p| [p.red, p.green, p.blue])
                .map(|n| {
                    let scaled = clamp(n * 255.0, 0.0, 255.0).round();
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
    use crate::{color, BLACK};

    #[test]
    fn creating_blank_canvas() {
        let canvas = Canvas::new(10, 20);
        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);

        for pixel in canvas.pixels {
            assert_eq!(pixel, BLACK);
        }
    }

    #[test]
    fn writing_pixels_to_canvas() {
        let mut canvas = Canvas::new(10, 20);
        let red = color(1.0, 0.0, 0.0);
        canvas.write_pixel(2, 3, red).unwrap();
        assert_eq!(canvas.pixel_at(2, 3).unwrap(), red);
    }

    #[test]
    fn constructing_ppm_header() {
        let canvas: Canvas = Canvas::new(5, 3);
        let ppm = canvas.to_ppm().unwrap();
        let mut lines = ppm.lines();
        assert_eq!(lines.next(), Some("P3"));
        assert_eq!(lines.next(), Some("5 3"));
        assert_eq!(lines.next(), Some("255"));
    }

    #[test]
    fn constructing_ppm_pixel_data() {
        let mut canvas = Canvas::new(5, 3);
        let c1 = color(1.5, 0.0, 0.0);
        let c2 = color(0.0, 0.5, 0.0);
        let c3 = color(-0.5, 0.0, 1.0);
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
        let c = color(1.0, 0.8, 0.6);
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
        let canvas: Canvas = Canvas::new(5, 3);
        let ppm = canvas.to_ppm().unwrap();
        let line = ppm.lines().last();
        assert_eq!(Some(""), line);
    }
}
