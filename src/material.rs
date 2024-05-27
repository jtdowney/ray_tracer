use crate::{color, pattern::Pattern, Color, Point, PointLight, Shape, Vector, BLACK};

pub fn material() -> Material {
    Material {
        color: color(1, 1, 1),
        ambient: 0.1,
        diffuse: 0.9,
        specular: 0.9,
        shininess: 200.0,
        reflective: 0.0,
        transparency: 0.0,
        refractive_index: 1.0,
        pattern: None,
    }
}

#[derive(Clone, Debug)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub reflective: f64,
    pub transparency: f64,
    pub refractive_index: f64,
    pub pattern: Option<Pattern>,
}

impl Material {
    pub fn lighting(
        &self,
        shape: &Shape,
        light: PointLight,
        point: Point,
        eye_vector: Vector,
        normal_vector: Vector,
        in_shadow: bool,
    ) -> Color {
        let color = if let Some(pattern) = &self.pattern {
            pattern.pattern_at_shape(shape, point)
        } else {
            self.color
        };

        let effective_color = color * light.intensity;
        let light_vector = (light.position - point).normalize();
        let ambient = effective_color * self.ambient;
        if in_shadow {
            return ambient;
        }

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

    use crate::{
        intersection, point, point_light, ray,
        shapes::sphere::glass_sphere,
        sphere,
        transform::{scaling, translation},
        vector, ORIGIN, WHITE,
    };

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
            m.lighting(&sphere(), light, position, eyev, normalv, false)
        );
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_and_with_eye_offset_45() {
        let m = material();
        let position = ORIGIN;
        let eyev = vector(0.0, 2.0_f64.sqrt() / 2.0, -(2.0_f64.sqrt()) / 2.0);
        let normalv = vector(0, 0, -1);
        let light = point_light(point(0, 0, -10), color(1, 1, 1));
        assert_abs_diff_eq!(
            color(1.0, 1.0, 1.0),
            m.lighting(&sphere(), light, position, eyev, normalv, false)
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
            m.lighting(&sphere(), light, position, eyev, normalv, false)
        );
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection_vector() {
        let m = material();
        let position = ORIGIN;
        let eyev = vector(0.0, -(2.0_f64.sqrt()) / 2.0, -(2.0_f64.sqrt()) / 2.0);
        let normalv = vector(0, 0, -1);
        let light = point_light(point(0, 10, -10), color(1, 1, 1));
        assert_abs_diff_eq!(
            color(1.6364, 1.6364, 1.6364),
            m.lighting(&sphere(), light, position, eyev, normalv, false)
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
            m.lighting(&sphere(), light, position, eyev, normalv, false)
        );
    }

    #[test]
    fn lighting_with_surface_in_shadow() {
        let m = material();
        let position = ORIGIN;
        let eyev = vector(0, 0, -1);
        let normalv = vector(0, 0, -1);
        let light = point_light(point(0, 0, -10), WHITE);
        let in_shadow = true;
        assert_eq!(
            color(0.1, 0.1, 0.1),
            m.lighting(&sphere(), light, position, eyev, normalv, in_shadow)
        );
    }

    #[test]
    fn finding_n1_and_n2() {
        let mut a = glass_sphere();
        a.transform = scaling(2.0, 2.0, 2.0);
        a.material.refractive_index = 1.5;
        let mut b = glass_sphere();
        b.transform = translation(0.0, 0.0, -0.25);
        b.material.refractive_index = 2.0;
        let mut c = glass_sphere();
        c.transform = translation(0.0, 0.0, 0.25);
        c.material.refractive_index = 2.5;
        let r = ray(point(0, 0, -4), vector(0, 0, 1));
        let xs = vec![
            intersection(2, &a),
            intersection(2.75, &b),
            intersection(3.25, &c),
            intersection(4.75, &b),
            intersection(5.25, &c),
            intersection(6, &a),
        ];

        let comps = xs[0].prepare_computations(r, &xs);
        assert_eq!(1.0, comps.n1);
        assert_eq!(1.5, comps.n2);

        let comps = xs[1].prepare_computations(r, &xs);
        assert_eq!(1.5, comps.n1);
        assert_eq!(2.0, comps.n2);

        let comps = xs[2].prepare_computations(r, &xs);
        assert_eq!(2.0, comps.n1);
        assert_eq!(2.5, comps.n2);

        let comps = xs[3].prepare_computations(r, &xs);
        assert_eq!(2.5, comps.n1);
        assert_eq!(2.5, comps.n2);

        let comps = xs[4].prepare_computations(r, &xs);
        assert_eq!(2.5, comps.n1);
        assert_eq!(1.5, comps.n2);

        let comps = xs[5].prepare_computations(r, &xs);
        assert_eq!(1.5, comps.n1);
        assert_eq!(1.0, comps.n2);
    }
}
