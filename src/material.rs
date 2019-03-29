use crate::{Color, Point, PointLight, Vector3};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

impl Material {
    pub fn lighting(
        &self,
        light: PointLight,
        position: Point,
        eye_vector: Vector3,
        normal_vector: Vector3,
        in_shadow: bool,
    ) -> Color {
        let effective_color = self.color * light.intensity;
        let light_vector = (light.position - position).normalize();
        let ambient = effective_color * self.ambient;
        let light_dot_normal = light_vector.dot(normal_vector);

        let diffuse: Color;
        let specular: Color;
        if light_dot_normal < 0.0 {
            diffuse = Color::default();
            specular = Color::default();
        } else {
            diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflect_vector = (-light_vector).reflect(normal_vector);
            let reflect_dot_eye = reflect_vector.dot(eye_vector);

            if reflect_dot_eye <= 0.0 {
                specular = Color::default();
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

    #[test]
    fn test_lighting_with_eye_between_light_and_surface() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector3::new(0.0, 0.0, -1.0);
        let normalv = Vector3::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        assert_eq!(
            Color::new(1.9, 1.9, 1.9),
            m.lighting(light, position, eyev, normalv, false)
        );
    }

    #[test]
    fn test_lighting_with_eye_between_light_and_surface_with_eye_offset() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector3::new(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0);
        let normalv = Vector3::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        assert_eq!(
            Color::new(1.0, 1.0, 1.0),
            m.lighting(light, position, eyev, normalv, false)
        );
    }

    #[test]
    fn test_lighting_with_eye_between_light_and_surface_with_light_offset() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector3::new(0.0, 0.0, -1.0);
        let normalv = Vector3::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        assert_eq!(
            Color::new(0.7364, 0.7364, 0.7364),
            m.lighting(light, position, eyev, normalv, false)
        );
    }

    #[test]
    fn test_lighting_with_eye_in_path_of_reflection() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector3::new(0.0, -f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0);
        let normalv = Vector3::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        assert_eq!(
            Color::new(1.63638, 1.63638, 1.63638),
            m.lighting(light, position, eyev, normalv, false)
        );
    }

    #[test]
    fn test_lighting_with_light_behind_surface() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector3::new(0.0, 0.0, -1.0);
        let normalv = Vector3::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));
        assert_eq!(
            Color::new(0.1, 0.1, 0.1),
            m.lighting(light, position, eyev, normalv, false)
        );
    }

    #[test]
    fn test_lighting_surface_in_shadow() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector3::new(0.0, 0.0, -1.0);
        let normalv = Vector3::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let in_shadow = true;
        assert_eq!(
            Color::new(0.1, 0.1, 0.1),
            m.lighting(light, position, eyev, normalv, in_shadow)
        );
    }
}
