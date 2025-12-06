use bon::Builder;

use crate::{
    Color, Point, PointLight, Vector,
    color::{BLACK, WHITE},
};

#[must_use]
pub fn material() -> Material {
    Material::builder().build()
}

#[derive(Builder, Clone, Copy, Debug, PartialEq)]
#[builder(on(f64, into), derive(Into))]
pub struct Material {
    #[builder(default = WHITE)]
    pub color: Color,
    #[builder(default = 0.1)]
    pub ambient: f64,
    #[builder(default = 0.9)]
    pub diffuse: f64,
    #[builder(default = 0.9)]
    pub specular: f64,
    #[builder(default = 200.0)]
    pub shininess: f64,
}

impl Material {
    #[must_use]
    pub fn lighting(
        &self,
        light: &PointLight,
        point: Point,
        eyev: Vector,
        normalv: Vector,
    ) -> Color {
        let effective_color = self.color * light.intensity;
        let lightv = (light.position - point).normalize();
        let ambient = effective_color * self.ambient;

        let light_dot_normal = lightv.dot(&normalv);
        let (diffuse, specular) = if light_dot_normal < 0.0 {
            (BLACK, BLACK)
        } else {
            let diffuse = effective_color * self.diffuse * light_dot_normal;

            let reflectv = (-lightv).reflect(&normalv);
            let reflect_dot_eye = reflectv.dot(&eyev);

            let specular = if reflect_dot_eye <= 0.0 {
                BLACK
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                light.intensity * self.specular * factor
            };

            (diffuse, specular)
        };

        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;
    use crate::{EPSILON, color, point, point_light, vector};

    #[test]
    fn default_material() {
        let m = material();
        assert_eq!(m.color, color(1, 1, 1));
        assert_relative_eq!(m.ambient, 0.1, epsilon = EPSILON);
        assert_relative_eq!(m.diffuse, 0.9, epsilon = EPSILON);
        assert_relative_eq!(m.specular, 0.9, epsilon = EPSILON);
        assert_relative_eq!(m.shininess, 200.0, epsilon = EPSILON);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let m = material();
        let position = point(0, 0, 0);
        let eyev = vector(0, 0, -1);
        let normalv = vector(0, 0, -1);
        let light = point_light(point(0, 0, -10), color(1, 1, 1));
        let result = m.lighting(&light, position, eyev, normalv);
        assert_relative_eq!(result.red, 1.9, epsilon = EPSILON);
        assert_relative_eq!(result.green, 1.9, epsilon = EPSILON);
        assert_relative_eq!(result.blue, 1.9, epsilon = EPSILON);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_eye_offset_45_degrees() {
        let m = material();
        let position = point(0, 0, 0);
        let sqrt2_over_2 = 2.0_f64.sqrt() / 2.0;
        let eyev = vector(0.0, sqrt2_over_2, -sqrt2_over_2);
        let normalv = vector(0, 0, -1);
        let light = point_light(point(0, 0, -10), color(1, 1, 1));
        let result = m.lighting(&light, position, eyev, normalv);
        assert_relative_eq!(result.red, 1.0, epsilon = EPSILON);
        assert_relative_eq!(result.green, 1.0, epsilon = EPSILON);
        assert_relative_eq!(result.blue, 1.0, epsilon = EPSILON);
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45_degrees() {
        let m = material();
        let position = point(0, 0, 0);
        let eyev = vector(0, 0, -1);
        let normalv = vector(0, 0, -1);
        let light = point_light(point(0, 10, -10), color(1, 1, 1));
        let result = m.lighting(&light, position, eyev, normalv);
        assert_relative_eq!(result.red, 0.7364, epsilon = EPSILON);
        assert_relative_eq!(result.green, 0.7364, epsilon = EPSILON);
        assert_relative_eq!(result.blue, 0.7364, epsilon = EPSILON);
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection_vector() {
        let m = material();
        let position = point(0, 0, 0);
        let sqrt2_over_2 = 2.0_f64.sqrt() / 2.0;
        let eyev = vector(0.0, -sqrt2_over_2, -sqrt2_over_2);
        let normalv = vector(0, 0, -1);
        let light = point_light(point(0, 10, -10), color(1, 1, 1));
        let result = m.lighting(&light, position, eyev, normalv);
        assert_relative_eq!(result.red, 1.6364, epsilon = EPSILON);
        assert_relative_eq!(result.green, 1.6364, epsilon = EPSILON);
        assert_relative_eq!(result.blue, 1.6364, epsilon = EPSILON);
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        let m = material();
        let position = point(0, 0, 0);
        let eyev = vector(0, 0, -1);
        let normalv = vector(0, 0, -1);
        let light = point_light(point(0, 0, 10), color(1, 1, 1));
        let result = m.lighting(&light, position, eyev, normalv);
        assert_relative_eq!(result.red, 0.1, epsilon = EPSILON);
        assert_relative_eq!(result.green, 0.1, epsilon = EPSILON);
        assert_relative_eq!(result.blue, 0.1, epsilon = EPSILON);
    }
}
