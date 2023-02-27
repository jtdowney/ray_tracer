use crate::{color, Color, Point, PointLight, Vector, BLACK};

pub fn material() -> Material {
    Material {
        color: color(1, 1, 1),
        ambient: 0.1,
        diffuse: 0.9,
        specular: 0.9,
        shininess: 200.0,
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn lighting(
        &self,
        light: PointLight,
        position: Point,
        eye_vector: Vector,
        normal_vector: Vector,
    ) -> Color {
        let effective_color = self.color * light.intensity;
        let light_vector = (light.position - position).normalize();
        let ambient = effective_color * self.ambient;
        let light_dot_normal = light_vector.dot(normal_vector);

        let diffuse;
        let specular;
        if light_dot_normal < 0.0 {
            diffuse = BLACK;
            specular = BLACK;
        } else {
            diffuse = effective_color * self.diffuse * light_dot_normal;

            let reflect_vector = (-light_vector).reflect(normal_vector);
            let refect_dot_eye = reflect_vector.dot(eye_vector);

            if refect_dot_eye <= 0.0 {
                specular = BLACK;
            } else {
                let factor = refect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }

        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use crate::{point, point_light, vector, ORIGIN};

    use super::*;

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let m = material();
        let position = ORIGIN;
        let eyev = vector(0, 0, -1);
        let normalv = vector(0, 0, -1);
        let light = point_light(point(0, 0, -10), color(1, 1, 1));
        assert_abs_diff_eq!(
            color(1.9, 1.9, 1.9),
            m.lighting(light, position, eyev, normalv)
        );
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_and_with_eye_offset_45() {
        let m = material();
        let position = ORIGIN;
        let eyev = vector(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normalv = vector(0, 0, -1);
        let light = point_light(point(0, 0, -10), color(1, 1, 1));
        assert_abs_diff_eq!(
            color(1.0, 1.0, 1.0),
            m.lighting(light, position, eyev, normalv)
        );
    }

    #[test]
    fn lighting_with_eye_opposite_surface_and_eye_offset_45() {
        let m = material();
        let position = ORIGIN;
        let eyev = vector(0, 0, -1);
        let normalv = vector(0, 0, -1);
        let light = point_light(point(0, 10, -10), color(1, 1, 1));
        assert_abs_diff_eq!(
            color(0.7364, 0.7364, 0.7364),
            m.lighting(light, position, eyev, normalv)
        );
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection_vector() {
        let m = material();
        let position = ORIGIN;
        let eyev = vector(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normalv = vector(0, 0, -1);
        let light = point_light(point(0, 10, -10), color(1, 1, 1));
        assert_abs_diff_eq!(
            color(1.6364, 1.6364, 1.6364),
            m.lighting(light, position, eyev, normalv)
        );
    }

    #[test]
    fn lighting_with_light_behind_the_surface() {
        let m = material();
        let position = ORIGIN;
        let eyev = vector(0, 0, -1);
        let normalv = vector(0, 0, -1);
        let light = point_light(point(0, 0, 10), color(1, 1, 1));
        assert_abs_diff_eq!(
            color(0.1, 0.1, 0.1),
            m.lighting(light, position, eyev, normalv)
        );
    }
}
