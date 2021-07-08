use crate::{color, Color, Point, PointLight, Vector};
use derive_builder::Builder;

pub fn material() -> Material {
    MaterialBuilder::default().build().unwrap()
}

#[derive(Builder, Debug, PartialEq, Clone, Copy)]
pub struct Material {
    #[builder(default = "color::WHITE")]
    pub color: Color,
    #[builder(default = "0.1")]
    pub ambient: f64,
    #[builder(default = "0.9")]
    pub diffuse: f64,
    #[builder(default = "0.9")]
    pub specular: f64,
    #[builder(default = "200.0")]
    pub shininess: f64,
}

impl Material {
    pub fn lighting(
        &self,
        light: &PointLight,
        position: Point,
        eye_vector: Vector,
        normal_vector: Vector,
        in_shadow: bool,
    ) -> Color {
        let effective_color = self.color * light.intensity;
        let light_vector = (light.position - position).normalize();
        let ambient = effective_color * self.ambient;
        let light_dot_normal = light_vector.dot(normal_vector);
        let black = color(0.0, 0.0, 0.0);
        let diffuse: Color;
        let specular: Color;
        if light_dot_normal.is_sign_negative() {
            diffuse = black;
            specular = black;
        } else {
            diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflect_vector = &(-light_vector).reflect(normal_vector);
            let reflect_dot_eye = reflect_vector.dot(eye_vector);
            if reflect_dot_eye <= 0.0 {
                specular = black;
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }

        if in_shadow {
            ambient
        } else {
            ambient + diffuse + specular
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{color, point, point_light, vector};
    use approx::assert_abs_diff_eq;

    #[test]
    fn default_material() {
        let m = material();
        assert_eq!(m.color, color(1.0, 1.0, 1.0));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let m = material();
        let position = point::ORIGIN;
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 0.0, -10.0), color::WHITE);
        assert_eq!(
            color(1.9, 1.9, 1.9),
            m.lighting(&light, position, eyev, normalv, false)
        );
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_with_eye_offset() {
        let m = material();
        let position = point::ORIGIN;
        let eyev = vector(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 0.0, -10.0), color::WHITE);
        assert_eq!(
            color::WHITE,
            m.lighting(&light, position, eyev, normalv, false)
        );
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_with_light_offset() {
        let m = material();
        let position = point::ORIGIN;
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 10.0, -10.0), color::WHITE);
        assert_abs_diff_eq!(
            color(0.7364, 0.7364, 0.7364),
            m.lighting(&light, position, eyev, normalv, false)
        );
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection() {
        let m = material();
        let position = point::ORIGIN;
        let eyev = vector(0.0, -f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 10.0, -10.0), color::WHITE);
        assert_abs_diff_eq!(
            color(1.63639, 1.63639, 1.63639),
            m.lighting(&light, position, eyev, normalv, false)
        );
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        let m = material();
        let position = point::ORIGIN;
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 0.0, 10.0), color::WHITE);
        assert_eq!(
            color(0.1, 0.1, 0.1),
            m.lighting(&light, position, eyev, normalv, false)
        );
    }

    #[test]
    fn lighting_with_the_surface_in_shadow() {
        let m = material();
        let position = point::ORIGIN;
        let eye_vector = vector(0.0, 0.0, -1.0);
        let normal_vector = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 0.0, -10.0), color(1.0, 1.0, 1.0));
        let in_shadow = true;
        let c = m.lighting(&light, position, eye_vector, normal_vector, in_shadow);
        assert_abs_diff_eq!(c, color(0.1, 0.1, 0.1));
    }
}
