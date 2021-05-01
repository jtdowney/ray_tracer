use crate::{color, Color, Point, PointLight, Vector};
use derive_builder::Builder;
use num::Float;

pub fn material<T>() -> Material<T>
where
    T: Float + Copy,
{
    MaterialBuilder::default().build().unwrap()
}

#[derive(Builder, Debug, PartialEq, Clone, Copy)]
pub struct Material<T>
where
    T: Float + Copy,
{
    #[builder(default = "self.default_color()")]
    pub color: Color<T>,
    #[builder(default = "self.default_ambient()?")]
    pub ambient: T,
    #[builder(default = "self.default_diffuse()?")]
    pub diffuse: T,
    #[builder(default = "self.default_specular()?")]
    pub specular: T,
    #[builder(default = "self.default_shininess()?")]
    pub shininess: T,
}

impl<T> MaterialBuilder<T>
where
    T: Float + Copy,
{
    fn default_color(&self) -> Color<T> {
        color(T::one(), T::one(), T::one())
    }

    fn default_ambient(&self) -> Result<T, String> {
        T::from(0.1).ok_or(format!("unable to convert ambient"))
    }

    fn default_diffuse(&self) -> Result<T, String> {
        T::from(0.9).ok_or(format!("unable to convert diffuse"))
    }

    fn default_specular(&self) -> Result<T, String> {
        T::from(0.9).ok_or(format!("unable to convert specular"))
    }

    fn default_shininess(&self) -> Result<T, String> {
        T::from(200).ok_or(format!("unable to convert shininess"))
    }
}

impl<T> Material<T>
where
    T: Float + Copy,
{
    pub fn lighting(
        &self,
        light: &PointLight<T>,
        position: Point<T>,
        eye_vector: Vector<T>,
        normal_vector: Vector<T>,
    ) -> Color<T> {
        let effective_color = self.color * light.intensity;
        let light_vector = (light.position - position).normalize();
        let ambient = effective_color * self.ambient;
        let light_dot_normal = light_vector.dot(normal_vector);
        let black = color(T::zero(), T::zero(), T::zero());
        let diffuse: Color<T>;
        let specular: Color<T>;
        if light_dot_normal.is_sign_negative() {
            diffuse = black;
            specular = black;
        } else {
            diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflect_vector = &(-light_vector).reflect(normal_vector);
            let reflect_dot_eye = reflect_vector.dot(eye_vector);
            if reflect_dot_eye <= T::zero() {
                specular = black;
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
    use crate::{point, point_light, vector, EPSILON};
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
        let position = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 0.0, -10.0), color::WHITE);
        assert_eq!(
            color(1.9, 1.9, 1.9),
            m.lighting(&light, position, eyev, normalv)
        );
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_with_eye_offset() {
        let m = material();
        let position = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 0.0, -10.0), color::WHITE);
        assert_eq!(color::WHITE, m.lighting(&light, position, eyev, normalv));
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_with_light_offset() {
        let m = material();
        let position = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 10.0, -10.0), color::WHITE);
        assert_abs_diff_eq!(
            color(0.7364, 0.7364, 0.7364),
            m.lighting(&light, position, eyev, normalv),
            epsilon = EPSILON
        );
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection() {
        let m = material();
        let position = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, -f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 10.0, -10.0), color::WHITE);
        assert_abs_diff_eq!(
            color(1.63639, 1.63639, 1.63639),
            m.lighting(&light, position, eyev, normalv),
            epsilon = EPSILON
        );
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        let m = material();
        let position = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 0.0, 10.0), color::WHITE);
        assert_eq!(
            color(0.1, 0.1, 0.1),
            m.lighting(&light, position, eyev, normalv)
        );
    }
}
