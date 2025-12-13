use bon::Builder;

use crate::{
    Color, Point, PointLight, Shape, Vector,
    color::{BLACK, WHITE},
    pattern::Pattern,
};

#[must_use]
pub fn material() -> Material {
    Material::builder().build()
}

#[derive(Builder, Clone)]
#[builder(derive(Into))]
pub struct Material {
    #[builder(default = WHITE)]
    pub color: Color,
    #[builder(default = 0.1)]
    pub ambient: f32,
    #[builder(default = 0.9)]
    pub diffuse: f32,
    #[builder(default = 0.9)]
    pub specular: f32,
    #[builder(default = 200.0)]
    pub shininess: f32,
    #[builder(default = 0.0)]
    pub reflective: f32,
    #[builder(default = 0.0)]
    pub transparency: f32,
    #[builder(default = 1.0)]
    pub refractive_index: f32,
    pub pattern: Option<Pattern>,
}

impl std::fmt::Debug for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Material")
            .field("color", &self.color)
            .field("ambient", &self.ambient)
            .field("diffuse", &self.diffuse)
            .field("specular", &self.specular)
            .field("shininess", &self.shininess)
            .field("reflective", &self.reflective)
            .field("transparency", &self.transparency)
            .field("refractive_index", &self.refractive_index)
            .field("pattern", &self.pattern.as_ref().map(|_| "Pattern"))
            .finish()
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
            && (self.ambient - other.ambient).abs() < f32::EPSILON
            && (self.diffuse - other.diffuse).abs() < f32::EPSILON
            && (self.specular - other.specular).abs() < f32::EPSILON
            && (self.shininess - other.shininess).abs() < f32::EPSILON
    }
}

impl Material {
    #[must_use]
    pub fn lighting(
        &self,
        object: &Shape,
        light: &PointLight,
        point: Point,
        eyev: Vector,
        normalv: Vector,
        in_shadow: bool,
    ) -> Color {
        let color = self
            .pattern
            .as_ref()
            .map_or(self.color, |p| p.pattern_at_shape(object, point));
        let effective_color = color * light.intensity;
        let lightv = (light.position - point).normalize();
        let ambient = effective_color * self.ambient;

        if in_shadow {
            return ambient;
        }

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
    use crate::{EPSILON, color, pattern::stripe_pattern, point, point_light, sphere, vector};

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
    fn reflectivity_for_default_material() {
        let m = material();
        assert_relative_eq!(m.reflective, 0.0, epsilon = EPSILON);
    }

    #[test]
    fn transparency_and_refractive_index_for_default_material() {
        let m = material();
        assert_relative_eq!(m.transparency, 0.0, epsilon = EPSILON);
        assert_relative_eq!(m.refractive_index, 1.0, epsilon = EPSILON);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let m = material();
        let object = sphere().build();
        let position = point(0, 0, 0);
        let eyev = vector(0, 0, -1);
        let normalv = vector(0, 0, -1);
        let light = point_light(point(0, 0, -10), color(1, 1, 1));
        let result = m.lighting(&object, &light, position, eyev, normalv, false);
        assert_relative_eq!(result.red, 1.9, epsilon = EPSILON);
        assert_relative_eq!(result.green, 1.9, epsilon = EPSILON);
        assert_relative_eq!(result.blue, 1.9, epsilon = EPSILON);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_eye_offset_45_degrees() {
        let m = material();
        let object = sphere().build();
        let position = point(0, 0, 0);
        let sqrt2_over_2 = 2.0_f32.sqrt() / 2.0;
        let eyev = vector(0.0, sqrt2_over_2, -sqrt2_over_2);
        let normalv = vector(0, 0, -1);
        let light = point_light(point(0, 0, -10), color(1, 1, 1));
        let result = m.lighting(&object, &light, position, eyev, normalv, false);
        assert_relative_eq!(result.red, 1.0, epsilon = EPSILON);
        assert_relative_eq!(result.green, 1.0, epsilon = EPSILON);
        assert_relative_eq!(result.blue, 1.0, epsilon = EPSILON);
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45_degrees() {
        let m = material();
        let object = sphere().build();
        let position = point(0, 0, 0);
        let eyev = vector(0, 0, -1);
        let normalv = vector(0, 0, -1);
        let light = point_light(point(0, 10, -10), color(1, 1, 1));
        let result = m.lighting(&object, &light, position, eyev, normalv, false);
        assert_relative_eq!(result.red, 0.7364, epsilon = EPSILON);
        assert_relative_eq!(result.green, 0.7364, epsilon = EPSILON);
        assert_relative_eq!(result.blue, 0.7364, epsilon = EPSILON);
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection_vector() {
        let m = material();
        let object = sphere().build();
        let position = point(0, 0, 0);
        let sqrt2_over_2 = 2.0_f32.sqrt() / 2.0;
        let eyev = vector(0.0, -sqrt2_over_2, -sqrt2_over_2);
        let normalv = vector(0, 0, -1);
        let light = point_light(point(0, 10, -10), color(1, 1, 1));
        let result = m.lighting(&object, &light, position, eyev, normalv, false);
        assert_relative_eq!(result.red, 1.6364, epsilon = EPSILON);
        assert_relative_eq!(result.green, 1.6364, epsilon = EPSILON);
        assert_relative_eq!(result.blue, 1.6364, epsilon = EPSILON);
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        let m = material();
        let object = sphere().build();
        let position = point(0, 0, 0);
        let eyev = vector(0, 0, -1);
        let normalv = vector(0, 0, -1);
        let light = point_light(point(0, 0, 10), color(1, 1, 1));
        let result = m.lighting(&object, &light, position, eyev, normalv, false);
        assert_relative_eq!(result.red, 0.1, epsilon = EPSILON);
        assert_relative_eq!(result.green, 0.1, epsilon = EPSILON);
        assert_relative_eq!(result.blue, 0.1, epsilon = EPSILON);
    }

    #[test]
    fn lighting_with_surface_in_shadow() {
        let m = material();
        let object = sphere().build();
        let position = point(0, 0, 0);
        let eyev = vector(0, 0, -1);
        let normalv = vector(0, 0, -1);
        let light = point_light(point(0, 0, -10), color(1, 1, 1));
        let in_shadow = true;
        let result = m.lighting(&object, &light, position, eyev, normalv, in_shadow);
        assert_relative_eq!(result.red, 0.1, epsilon = EPSILON);
        assert_relative_eq!(result.green, 0.1, epsilon = EPSILON);
        assert_relative_eq!(result.blue, 0.1, epsilon = EPSILON);
    }

    #[test]
    fn lighting_with_pattern_applied() {
        let m = Material::builder()
            .pattern(stripe_pattern(color(1, 1, 1), color(0, 0, 0)).build())
            .ambient(1.0)
            .diffuse(0.0)
            .specular(0.0)
            .build();
        let object = sphere().build();
        let eyev = vector(0, 0, -1);
        let normalv = vector(0, 0, -1);
        let light = point_light(point(0, 0, -10), color(1, 1, 1));
        let c1 = m.lighting(&object, &light, point(0.9, 0, 0), eyev, normalv, false);
        let c2 = m.lighting(&object, &light, point(1.1, 0, 0), eyev, normalv, false);
        assert_eq!(c1, color(1, 1, 1));
        assert_eq!(c2, color(0, 0, 0));
    }
}
