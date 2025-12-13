use approx::relative_eq;
use bon::Builder;
use ord_subset::OrdSubsetSliceExt;

use crate::{
    Color, EPSILON, Intersection, Material, Point, PointLight, Ray, Shape, color,
    color::{BLACK, WHITE},
    hit,
    intersection::{Computations, schlick},
    point, point_light, ray, sphere, transform,
};

#[must_use]
pub fn default_world() -> World {
    World::builder()
        .light(point_light(point(-10, 10, -10), WHITE))
        .objects(bon::vec![
            sphere().material(
                Material::builder()
                    .color(color(0.8, 1.0, 0.6))
                    .diffuse(0.7)
                    .specular(0.2)
            ),
            sphere().transform(transform::scaling(0.5, 0.5, 0.5)),
        ])
        .build()
}

#[derive(Builder, Default)]
pub struct World {
    pub light: Option<PointLight>,
    pub objects: Vec<Shape>,
}

impl World {
    #[must_use]
    pub fn intersect(&self, ray: Ray) -> Vec<Intersection<'_>> {
        let mut xs = self
            .objects
            .iter()
            .flat_map(|o| o.intersect(ray))
            .collect::<Vec<Intersection>>();

        xs.ord_subset_sort_by_key(|i| i.time);
        xs
    }

    #[must_use]
    pub fn shade_hit(&self, comps: &Computations<'_>, remaining: usize) -> Color {
        if let Some(light) = self.light {
            let shadowed = self.is_shadowed(comps.over_point);
            let surface = comps.object.material.lighting(
                comps.object,
                &light,
                comps.over_point,
                comps.eyev,
                comps.normalv,
                shadowed,
            );

            let reflected = self.reflected_color(comps, remaining);
            let refracted = self.refracted_color(comps, remaining);

            let material = &comps.object.material;
            if material.reflective.abs() >= EPSILON && material.transparency.abs() >= EPSILON {
                let reflectance = schlick(comps);
                surface + reflected * reflectance + refracted * (1.0 - reflectance)
            } else {
                surface + reflected + refracted
            }
        } else {
            BLACK
        }
    }

    #[must_use]
    pub fn is_shadowed(&self, point: Point) -> bool {
        if let Some(light) = self.light {
            let v = light.position - point;
            let distance = v.magnitude();
            let direction = v.normalize();

            let ray = ray(point, direction);
            let xs = self.intersect(ray);
            if let Some(i) = hit(&xs) {
                i.time < distance
            } else {
                false
            }
        } else {
            false
        }
    }

    #[must_use]
    pub fn color_at(&self, ray: Ray, remaining: usize) -> Color {
        let xs = self.intersect(ray);
        if let Some(i) = hit(&xs) {
            let comps = i.prepare_computations(ray, &xs);
            self.shade_hit(&comps, remaining)
        } else {
            BLACK
        }
    }

    #[must_use]
    pub fn reflected_color(&self, comps: &Computations<'_>, remaining: usize) -> Color {
        if remaining == 0 || comps.object.material.reflective.abs() < EPSILON {
            BLACK
        } else {
            let reflect_ray = ray(comps.over_point, comps.reflectv);
            self.color_at(reflect_ray, remaining - 1) * comps.object.material.reflective
        }
    }

    #[must_use]
    pub fn refracted_color(&self, comps: &Computations<'_>, remaining: usize) -> Color {
        if remaining == 0
            || relative_eq!(comps.object.material.transparency, 0.0, epsilon = EPSILON)
        {
            return BLACK;
        }

        let n_ratio = comps.n1 / comps.n2;
        let cos_i = comps.eyev.dot(&comps.normalv);
        let sin2_t = n_ratio.powi(2) * (1.0 - cos_i.powi(2));

        if sin2_t > 1.0 {
            return BLACK;
        }

        let cos_t = (1.0 - sin2_t).sqrt();
        let direction = comps.normalv * (n_ratio * cos_i - cos_t) - comps.eyev * n_ratio;

        let refract_ray = ray(comps.under_point, direction);
        self.color_at(refract_ray, remaining - 1) * comps.object.material.transparency
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;
    use crate::{
        EPSILON, Material, color, intersection, pattern::test_pattern, point, point_light, ray,
        shape::plane, transform, vector,
    };

    #[test]
    fn creating_a_world() {
        let w = World::default();
        assert!(w.objects.is_empty());
        assert!(w.light.is_none());
    }

    #[test]
    fn default_world_configuration() {
        let light = point_light(point(-10, 10, -10), color(1, 1, 1));
        let s1_material = Material::builder()
            .color(color(0.8, 1.0, 0.6))
            .diffuse(0.7)
            .specular(0.2)
            .build();

        let w = default_world();

        assert_eq!(w.light, Some(light));
        assert_eq!(w.objects.len(), 2);
        assert_eq!(w.objects[0].material, s1_material);
        assert_eq!(w.objects[1].transform, transform::scaling(0.5, 0.5, 0.5));
    }

    #[test]
    fn intersect_world_with_ray() {
        let w = default_world();
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let xs = w.intersect(r);
        assert_eq!(xs.len(), 4);
        assert_relative_eq!(xs[0].time, 4.0, epsilon = EPSILON);
        assert_relative_eq!(xs[1].time, 4.5, epsilon = EPSILON);
        assert_relative_eq!(xs[2].time, 5.5, epsilon = EPSILON);
        assert_relative_eq!(xs[3].time, 6.0, epsilon = EPSILON);
    }

    #[test]
    fn shading_an_intersection() {
        let w = default_world();
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let shape = &w.objects[0];
        let i = intersection(4, shape);
        let comps = i.prepare_computations(r, &[i]);
        let c = w.shade_hit(&comps, 5);
        assert_relative_eq!(c.red, 0.38066, epsilon = EPSILON);
        assert_relative_eq!(c.green, 0.47583, epsilon = EPSILON);
        assert_relative_eq!(c.blue, 0.2855, epsilon = EPSILON);
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = default_world();
        w.light = Some(point_light(point(0, 0.25, 0), color(1, 1, 1)));
        let r = ray(point(0, 0, 0), vector(0, 0, 1));
        let shape = &w.objects[1];
        let i = intersection(0.5, shape);
        let comps = i.prepare_computations(r, &[i]);
        let c = w.shade_hit(&comps, 5);
        assert_relative_eq!(c.red, 0.90498, epsilon = EPSILON);
        assert_relative_eq!(c.green, 0.90498, epsilon = EPSILON);
        assert_relative_eq!(c.blue, 0.90498, epsilon = EPSILON);
    }

    #[test]
    fn color_when_ray_misses() {
        let w = default_world();
        let r = ray(point(0, 0, -5), vector(0, 1, 0));
        let c = w.color_at(r, 5);
        assert_eq!(c, color(0, 0, 0));
    }

    #[test]
    fn color_when_ray_hits() {
        let w = default_world();
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let c = w.color_at(r, 5);
        assert_relative_eq!(c.red, 0.38066, epsilon = EPSILON);
        assert_relative_eq!(c.green, 0.47583, epsilon = EPSILON);
        assert_relative_eq!(c.blue, 0.2855, epsilon = EPSILON);
    }

    #[test]
    fn color_with_intersection_behind_ray() {
        let mut w = default_world();
        w.objects[0].material.ambient = 1.0;
        w.objects[1].material.ambient = 1.0;
        let r = ray(point(0, 0, 0.75), vector(0, 0, -1));
        let c = w.color_at(r, 5);
        assert_eq!(c, w.objects[1].material.color);
    }

    #[test]
    fn no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = default_world();
        let p = point(0, 10, 0);
        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn shadow_when_object_is_between_point_and_light() {
        let w = default_world();
        let p = point(10, -10, 10);
        assert!(w.is_shadowed(p));
    }

    #[test]
    fn no_shadow_when_object_is_behind_light() {
        let w = default_world();
        let p = point(-20, 20, -20);
        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn no_shadow_when_object_is_behind_point() {
        let w = default_world();
        let p = point(-2, 2, -2);
        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn shade_hit_given_intersection_in_shadow() {
        let s1 = sphere();
        let s2 = sphere().transform(transform::translation(0, 0, 10));
        let w = World::builder()
            .light(point_light(point(0, 0, -10), color(1, 1, 1)))
            .objects(bon::vec![s1, s2])
            .build();
        let r = ray(point(0, 0, 5), vector(0, 0, 1));
        let i = intersection(4, &w.objects[1]);
        let comps = i.prepare_computations(r, &[i]);
        let c = w.shade_hit(&comps, 5);
        assert_relative_eq!(c.red, 0.1, epsilon = EPSILON);
        assert_relative_eq!(c.green, 0.1, epsilon = EPSILON);
        assert_relative_eq!(c.blue, 0.1, epsilon = EPSILON);
    }

    // Chapter 11: Reflection and Refraction tests

    #[test]
    fn reflected_color_for_nonreflective_material() {
        let mut w = default_world();
        w.objects[1].material.ambient = 1.0;
        let r = ray(point(0, 0, 0), vector(0, 0, 1));
        let shape = &w.objects[1];
        let i = intersection(1, shape);
        let comps = i.prepare_computations(r, &[i]);
        let c = w.reflected_color(&comps, 5);
        assert_eq!(c, color(0, 0, 0));
    }

    #[test]
    fn reflected_color_for_reflective_material() {
        let sqrt2_over_2 = 2.0_f64.sqrt() / 2.0;
        let mut w = default_world();
        let shape = plane()
            .material(Material::builder().reflective(0.5))
            .transform(transform::translation(0, -1, 0))
            .build();
        w.objects.push(shape);
        let r = ray(point(0, 0, -3), vector(0.0, -sqrt2_over_2, sqrt2_over_2));
        let i = intersection(2.0_f64.sqrt(), &w.objects[2]);
        let comps = i.prepare_computations(r, &[i]);
        let c = w.reflected_color(&comps, 5);
        assert_relative_eq!(c.red, 0.19032, epsilon = EPSILON);
        assert_relative_eq!(c.green, 0.2379, epsilon = EPSILON);
        assert_relative_eq!(c.blue, 0.14274, epsilon = EPSILON);
    }

    #[test]
    fn shade_hit_with_reflective_material() {
        let sqrt2_over_2 = 2.0_f64.sqrt() / 2.0;
        let mut w = default_world();
        let shape = plane()
            .material(Material::builder().reflective(0.5))
            .transform(transform::translation(0, -1, 0))
            .build();
        w.objects.push(shape);
        let r = ray(point(0, 0, -3), vector(0.0, -sqrt2_over_2, sqrt2_over_2));
        let i = intersection(2.0_f64.sqrt(), &w.objects[2]);
        let comps = i.prepare_computations(r, &[i]);
        let c = w.shade_hit(&comps, 5);
        assert_relative_eq!(c.red, 0.87677, epsilon = EPSILON);
        assert_relative_eq!(c.green, 0.92436, epsilon = EPSILON);
        assert_relative_eq!(c.blue, 0.82918, epsilon = EPSILON);
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let w = World::builder()
            .light(point_light(point(0, 0, 0), color(1, 1, 1)))
            .objects(bon::vec![
                plane()
                    .material(Material::builder().reflective(1))
                    .transform(transform::translation(0, -1, 0)),
                plane()
                    .material(Material::builder().reflective(1))
                    .transform(transform::translation(0, 1, 0)),
            ])
            .build();
        let r = ray(point(0, 0, 0), vector(0, 1, 0));
        // This should terminate without stack overflow
        let _ = w.color_at(r, 5);
    }

    #[test]
    fn reflected_color_at_maximum_recursive_depth() {
        let sqrt2_over_2 = 2.0_f64.sqrt() / 2.0;
        let mut w = default_world();
        let shape = plane()
            .material(Material::builder().reflective(0.5))
            .transform(transform::translation(0, -1, 0))
            .build();
        w.objects.push(shape);
        let r = ray(point(0, 0, -3), vector(0.0, -sqrt2_over_2, sqrt2_over_2));
        let i = intersection(2.0_f64.sqrt(), &w.objects[2]);
        let comps = i.prepare_computations(r, &[i]);
        let c = w.reflected_color(&comps, 0);
        assert_eq!(c, color(0, 0, 0));
    }

    #[test]
    fn refracted_color_with_opaque_surface() {
        let w = default_world();
        let shape = &w.objects[0];
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let xs = [intersection(4, shape), intersection(6, shape)];
        let comps = xs[0].prepare_computations(r, &xs);
        let c = w.refracted_color(&comps, 5);
        assert_eq!(c, color(0, 0, 0));
    }

    #[test]
    fn refracted_color_at_maximum_recursive_depth() {
        let mut w = default_world();
        w.objects[0].material.transparency = 1.0;
        w.objects[0].material.refractive_index = 1.5;
        let shape = &w.objects[0];
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let xs = [intersection(4, shape), intersection(6, shape)];
        let comps = xs[0].prepare_computations(r, &xs);
        let c = w.refracted_color(&comps, 0);
        assert_eq!(c, color(0, 0, 0));
    }

    #[test]
    fn refracted_color_under_total_internal_reflection() {
        let sqrt2_over_2 = 2.0_f64.sqrt() / 2.0;
        let mut w = default_world();
        w.objects[0].material.transparency = 1.0;
        w.objects[0].material.refractive_index = 1.5;
        let shape = &w.objects[0];
        let r = ray(point(0.0, 0.0, sqrt2_over_2), vector(0, 1, 0));
        let xs = [
            intersection(-sqrt2_over_2, shape),
            intersection(sqrt2_over_2, shape),
        ];
        // Inside the sphere, so look at xs[1]
        let comps = xs[1].prepare_computations(r, &xs);
        let c = w.refracted_color(&comps, 5);
        assert_eq!(c, color(0, 0, 0));
    }

    #[test]
    fn refracted_color_with_refracted_ray() {
        let mut w = default_world();
        w.objects[0].material.ambient = 1.0;
        w.objects[0].material.pattern = Some(test_pattern());
        w.objects[1].material.transparency = 1.0;
        w.objects[1].material.refractive_index = 1.5;
        let r = ray(point(0.0, 0.0, 0.1), vector(0, 1, 0));
        let xs = [
            intersection(-0.9899, &w.objects[0]),
            intersection(-0.4899, &w.objects[1]),
            intersection(0.4899, &w.objects[1]),
            intersection(0.9899, &w.objects[0]),
        ];
        let comps = xs[2].prepare_computations(r, &xs);
        let c = w.refracted_color(&comps, 5);
        assert_relative_eq!(c.red, 0.0, epsilon = EPSILON);
        assert_relative_eq!(c.green, 0.99888, epsilon = EPSILON);
        assert_relative_eq!(c.blue, 0.04725, epsilon = EPSILON);
    }

    #[test]
    fn shade_hit_with_transparent_material() {
        let sqrt2_over_2 = 2.0_f64.sqrt() / 2.0;
        let mut w = default_world();
        let floor = plane()
            .transform(transform::translation(0, -1, 0))
            .material(Material::builder().transparency(0.5).refractive_index(1.5))
            .build();
        let ball = sphere()
            .material(Material::builder().color(color(1, 0, 0)).ambient(0.5))
            .transform(transform::translation(0.0, -3.5, -0.5))
            .build();
        w.objects.push(floor);
        w.objects.push(ball);
        let r = ray(point(0, 0, -3), vector(0.0, -sqrt2_over_2, sqrt2_over_2));
        let xs = [intersection(2.0_f64.sqrt(), &w.objects[2])];
        let comps = xs[0].prepare_computations(r, &xs);
        let c = w.shade_hit(&comps, 5);
        assert_relative_eq!(c.red, 0.93642, epsilon = EPSILON);
        assert_relative_eq!(c.green, 0.68642, epsilon = EPSILON);
        assert_relative_eq!(c.blue, 0.68642, epsilon = EPSILON);
    }

    #[test]
    fn shade_hit_with_reflective_transparent_material() {
        let sqrt2_over_2 = 2.0_f64.sqrt() / 2.0;
        let mut w = default_world();
        let floor = plane()
            .transform(transform::translation(0, -1, 0))
            .material(
                Material::builder()
                    .reflective(0.5)
                    .transparency(0.5)
                    .refractive_index(1.5),
            )
            .build();
        let ball = sphere()
            .material(Material::builder().color(color(1, 0, 0)).ambient(0.5))
            .transform(transform::translation(0.0, -3.5, -0.5))
            .build();
        w.objects.push(floor);
        w.objects.push(ball);
        let r = ray(point(0, 0, -3), vector(0.0, -sqrt2_over_2, sqrt2_over_2));
        let xs = [intersection(2.0_f64.sqrt(), &w.objects[2])];
        let comps = xs[0].prepare_computations(r, &xs);
        let c = w.shade_hit(&comps, 5);
        assert_relative_eq!(c.red, 0.93391, epsilon = EPSILON);
        assert_relative_eq!(c.green, 0.69643, epsilon = EPSILON);
        assert_relative_eq!(c.blue, 0.69243, epsilon = EPSILON);
    }
}
