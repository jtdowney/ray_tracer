use std::fmt::Write;

use anyhow::{Result, bail};

use crate::{Color, clamp, color::BLACK};

#[must_use]
pub fn canvas(width: usize, height: usize) -> Canvas {
    Canvas {
        width,
        height,
        pixels: vec![BLACK; width * height],
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    /// # Errors
    /// Returns an error if coordinates are out of bounds.
    pub fn write_pixel(&mut self, x: usize, y: usize, pixel: Color) -> Result<()> {
        if x >= self.width || y >= self.height {
            bail!("out of bounds pixel write");
        }

        let index = y * self.width + x;
        self.pixels[index] = pixel;
        Ok(())
    }

    #[must_use]
    pub fn pixel_at(&self, x: usize, y: usize) -> Option<Color> {
        let index = y * self.width + x;
        self.pixels.get(index).copied()
    }

    /// # Errors
    /// Returns an error if writing to the output string fails.
    pub fn to_ppm(&self) -> Result<String> {
        let mut output = String::new();
        writeln!(output, "P3")?;
        writeln!(output, "{} {}", self.width, self.height)?;
        writeln!(output, "255")?;

        for y in 0..self.height {
            let offset = y * self.width;
            let codes = self.pixels[offset..]
                .iter()
                .take(self.width)
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
    use crate::color;

    #[test]
    fn creating_a_canvas() {
        let c = canvas(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        dbg!(&c);
        for y in 0..20 {
            for x in 0..10 {
                assert_eq!(c.pixel_at(x, y).unwrap(), color(0, 0, 0));
            }
        }
    }

    #[test]
    fn writing_pixels_to_canvas() {
        let mut c = canvas(10, 20);
        let red = color(1, 0, 0);
        c.write_pixel(2, 3, red).unwrap();
        assert_eq!(c.pixel_at(2, 3).unwrap(), red);
    }

    #[test]
    fn constructing_ppm_header() {
        let c = canvas(5, 3);
        let ppm = c.to_ppm().unwrap();
        let lines: Vec<&str> = ppm.lines().collect();
        assert_eq!(lines[0], "P3");
        assert_eq!(lines[1], "5 3");
        assert_eq!(lines[2], "255");
    }

    #[test]
    fn constructing_ppm_pixel_data() {
        let mut c = canvas(5, 3);
        let c1 = color(1.5, 0, 0);
        let c2 = color(0, 0.5, 0);
        let c3 = color(-0.5, 0, 1);
        c.write_pixel(0, 0, c1).unwrap();
        c.write_pixel(2, 1, c2).unwrap();
        c.write_pixel(4, 2, c3).unwrap();
        let ppm = c.to_ppm().unwrap();
        let lines: Vec<&str> = ppm.lines().collect();
        assert_eq!(lines[3], "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
        assert_eq!(lines[4], "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
        assert_eq!(lines[5], "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
    }

    #[test]
    fn splitting_long_lines_in_ppm() {
        let mut c = canvas(10, 2);
        let col = color(1, 0.8, 0.6);
        for y in 0..2 {
            for x in 0..10 {
                c.write_pixel(x, y, col).unwrap();
            }
        }
        let ppm = c.to_ppm().unwrap();
        let lines: Vec<&str> = ppm.lines().collect();
        assert_eq!(
            lines[3],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
        );
        assert_eq!(
            lines[4],
            "153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
        assert_eq!(
            lines[5],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
        );
        assert_eq!(
            lines[6],
            "153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
    }

    #[test]
    fn ppm_files_terminated_by_newline() {
        let c = canvas(5, 3);
        let ppm = c.to_ppm().unwrap();
        assert!(ppm.ends_with('\n'));
    }
}
