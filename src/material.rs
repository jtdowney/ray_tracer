use crate::{color, Color, Pattern, Point, PointLight, Shape, SolidPattern, Vector3};
use derive_builder::Builder;
use std::ptr;

#[derive(Builder, Clone, Debug)]
pub struct Material {
    #[builder(default = "0.1")]
    pub ambient: f64,
    #[builder(default = "0.9")]
    pub diffuse: f64,
    #[builder(default = "0.9")]
    pub specular: f64,
    #[builder(default = "200.0")]
    pub shininess: f64,
    #[builder(default = "0.0")]
    pub reflective: f64,
    #[builder(default = "0.0")]
    pub transparency: f64,
    #[builder(default = "1.0")]
    pub refractive_index: f64,
    #[builder(setter(prefix = "boxed"))]
    #[builder(default = "Self::default_pattern()")]
    pub pattern: Box<Pattern>,
}

impl MaterialBuilder {
    pub fn color(&mut self, value: Color) -> &mut Self {
        self.pattern(SolidPattern::new(value))
    }

    pub fn pattern<P: Pattern + 'static>(&mut self, value: P) -> &mut Self {
        self.boxed_pattern(Box::new(value))
    }

    fn default_pattern() -> Box<Pattern> {
        Box::new(SolidPattern::new(color::WHITE)) as Box<Pattern>
    }
}

impl Default for Material {
    fn default() -> Self {
        MaterialBuilder::default().build().unwrap()
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Material) -> bool {
        self.ambient == other.ambient
            && self.diffuse == other.diffuse
            && self.specular == other.specular
            && self.shininess == other.shininess
            && self.reflective == other.reflective
            && self.transparency == other.transparency
            && self.refractive_index == other.refractive_index
            && ptr::eq(self.pattern.as_ref(), other.pattern.as_ref())
    }
}

impl Material {
    pub fn lighting(
        &self,
        object: &Shape,
        light: &PointLight,
        position: Point,
        eye_vector: Vector3,
        normal_vector: Vector3,
        in_shadow: bool,
    ) -> Color {
        let color = self.pattern.pattern_at_object(object, position);
        let effective_color = color * light.intensity;
        let light_vector = (light.position - position).normalize();
        let ambient = effective_color * self.ambient;
        let light_dot_normal = light_vector.dot(normal_vector);

        let diffuse: Color;
        let specular: Color;
        if light_dot_normal < 0.0 {
            diffuse = color::BLACK;
            specular = color::BLACK;
        } else {
            diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflect_vector = (-light_vector).reflect(normal_vector);
            let reflect_dot_eye = reflect_vector.dot(eye_vector);

            if reflect_dot_eye <= 0.0 {
                specular = color::BLACK;
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

    pub fn is_reflective(&self) -> bool {
        self.reflective > 0.0
    }

    pub fn is_transparent(&self) -> bool {
        self.transparency > 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Sphere, StripePattern};

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector3::new(0.0, 0.0, -1.0);
        let normalv = Vector3::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), color::WHITE);
        assert_eq!(
            Color::new(1.9, 1.9, 1.9),
            m.lighting(&Sphere::default(), &light, position, eyev, normalv, false)
        );
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_with_eye_offset() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector3::new(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0);
        let normalv = Vector3::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), color::WHITE);
        assert_eq!(
            color::WHITE,
            m.lighting(&Sphere::default(), &light, position, eyev, normalv, false)
        );
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_with_light_offset() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector3::new(0.0, 0.0, -1.0);
        let normalv = Vector3::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), color::WHITE);
        assert_eq!(
            Color::new(0.7364, 0.7364, 0.7364),
            m.lighting(&Sphere::default(), &light, position, eyev, normalv, false)
        );
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector3::new(0.0, -f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0);
        let normalv = Vector3::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), color::WHITE);
        assert_eq!(
            Color::new(1.63638, 1.63638, 1.63638),
            m.lighting(&Sphere::default(), &light, position, eyev, normalv, false)
        );
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector3::new(0.0, 0.0, -1.0);
        let normalv = Vector3::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, 10.0), color::WHITE);
        assert_eq!(
            Color::new(0.1, 0.1, 0.1),
            m.lighting(&Sphere::default(), &light, position, eyev, normalv, false)
        );
    }

    #[test]
    fn lighting_surface_in_shadow() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector3::new(0.0, 0.0, -1.0);
        let normalv = Vector3::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), color::WHITE);
        let in_shadow = true;
        assert_eq!(
            Color::new(0.1, 0.1, 0.1),
            m.lighting(
                &Sphere::default(),
                &light,
                position,
                eyev,
                normalv,
                in_shadow
            )
        );
    }

    #[test]
    fn lighting_with_pattern() {
        let m = MaterialBuilder::default()
            .pattern(StripePattern::new(color::WHITE, color::BLACK))
            .ambient(1.0)
            .diffuse(0.0)
            .specular(0.0)
            .build()
            .unwrap();
        let eyev = Vector3::new(0.0, 0.0, -1.0);
        let normalv = Vector3::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), color::WHITE);
        let c1 = m.lighting(
            &Sphere::default(),
            &light,
            Point::new(0.9, 0.0, 0.0),
            eyev,
            normalv,
            false,
        );
        let c2 = m.lighting(
            &Sphere::default(),
            &light,
            Point::new(1.1, 0.0, 0.0),
            eyev,
            normalv,
            false,
        );
        assert_eq!(color::WHITE, c1);
        assert_eq!(color::BLACK, c2);
    }
}
