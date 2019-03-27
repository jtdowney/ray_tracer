use crate::{Color, Point, PointLight, Scalar, Vector3};
use num_traits::{Float, One, Zero};
use std::iter::Sum;
use std::ops::{Mul, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Material<T: Scalar> {
    pub color: Color<T>,
    pub ambient: T,
    pub diffuse: T,
    pub specular: T,
    pub shininess: T,
}

impl<T> Default for Material<T>
where
    T: Scalar + Float + From<f32> + One,
{
    fn default() -> Self {
        Material {
            color: Color::new(T::one(), T::one(), T::one()),
            ambient: 0.1.into(),
            diffuse: 0.9.into(),
            specular: 0.9.into(),
            shininess: 200.0.into(),
        }
    }
}

impl<T> Material<T>
where
    T: Scalar + Float + From<u16> + Mul<Output = T> + Sub<Output = T> + Sum<T> + Zero,
{
    pub fn lighting(
        &self,
        light: PointLight<T>,
        position: Point<T>,
        eye_vector: Vector3<T>,
        normal_vector: Vector3<T>,
    ) -> Color<T> {
        let effective_color = self.color * light.intensity;
        let light_vector = (light.position - position).normalize();
        let ambient = effective_color * self.ambient;
        let light_dot_normal = light_vector.dot(normal_vector);

        let diffuse: Color<T>;
        let specular: Color<T>;
        if light_dot_normal < T::zero() {
            diffuse = Color::default();
            specular = Color::default();
        } else {
            diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflect_vector = (-light_vector).reflect(normal_vector);
            let reflect_dot_eye = reflect_vector.dot(eye_vector);

            if reflect_dot_eye <= T::zero() {
                specular = Color::default();
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }

        ambient + diffuse + specular
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
            m.lighting(light, position, eyev, normalv)
        );
    }

    #[test]
    fn test_lighting_with_eye_between_light_and_surface_with_eye_offset() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector3::new(0.0, 2.0.sqrt() / 2.0, -2.0.sqrt() / 2.0);
        let normalv = Vector3::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        assert_eq!(
            Color::new(1.0, 1.0, 1.0),
            m.lighting(light, position, eyev, normalv)
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
            m.lighting(light, position, eyev, normalv)
        );
    }

    #[test]
    fn test_lighting_with_eye_in_path_of_reflection() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector3::new(0.0, -2.0.sqrt() / 2.0, -2.0.sqrt() / 2.0);
        let normalv = Vector3::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        assert_eq!(
            Color::new(1.6364, 1.6364, 1.6364),
            m.lighting(light, position, eyev, normalv)
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
            m.lighting(light, position, eyev, normalv)
        );
    }
}
