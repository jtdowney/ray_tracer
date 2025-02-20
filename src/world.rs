use ord_subset::OrdSubsetSliceExt;

use crate::{
    BLACK, Color, Point, PointLight, Ray, Shape, color, hit,
    intersection::{Computations, Intersection},
    point, point_light, ray, sphere,
    transform::scaling,
};

pub fn world() -> World {
    World::default()
}

pub fn default_world() -> World {
    let mut world = world();
    world.light = Some(point_light(point(-10, 10, -10), color(1, 1, 1)));

    let mut s1 = sphere();
    s1.material.color = color(0.8, 1.0, 0.6);
    s1.material.diffuse = 0.7;
    s1.material.specular = 0.2;

    let mut s2 = sphere();
    s2.transform = scaling(0.5, 0.5, 0.5);

    world.objects.push(s1);
    world.objects.push(s2);

    world
}

#[derive(Debug, Default)]
pub struct World {
    pub light: Option<PointLight>,
    pub objects: Vec<Shape>,
}

impl World {
    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let mut xs = self
            .objects
            .iter()
            .flat_map(|o| o.intersect(ray))
            .collect::<Vec<Intersection>>();

        xs.ord_subset_sort_by_key(|i| i.time);
        xs
    }

    pub fn shade_hit(&self, comps: Computations, remaining: u8) -> Color {
        if let Some(light) = self.light {
            let shadowed = self.is_shadowed(comps.over_point);
            let surface = comps.object.material.lighting(
                comps.object,
                light,
                comps.over_point,
                comps.eye_vector,
                comps.normal_vector,
                shadowed,
            );

            let reflected = self.reflected_color(comps, remaining);
            let refracted = self.refracted_color(comps, remaining);

            let material = &comps.object.material;
            if material.reflective > 0.0 && material.transparency > 0.0 {
                let reflectance = comps.schlick();
                surface + reflected * reflectance + refracted * (1.0 - reflectance)
            } else {
                surface + reflected + refracted
            }
        } else {
            BLACK
        }
    }

    pub fn color_at(&self, ray: Ray, remaining: u8) -> Color {
        let xs = self.intersect(ray);
        if let Some(i) = hit(&xs) {
            let comps = i.prepare_computations(ray, &xs);
            self.shade_hit(comps, remaining)
        } else {
            BLACK
        }
    }

    fn is_shadowed(&self, point: Point) -> bool {
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

    fn reflected_color(&self, comps: Computations, remaining: u8) -> Color {
        if remaining == 0 || comps.object.material.reflective == 0.0 {
            return BLACK;
        }

        let reflected_ray = ray(comps.over_point, comps.reflect_vector);
        let color = self.color_at(reflected_ray, remaining - 1);

        color * comps.object.material.reflective
    }

    fn refracted_color(&self, comps: Computations, remaining: u8) -> Color {
        if remaining == 0 || comps.object.material.transparency == 0.0 {
            return BLACK;
        }

        let n_ratio = comps.n1 / comps.n2;
        let cos_i = comps.eye_vector.dot(comps.normal_vector);
        let sin2_t = n_ratio.powi(2) * (1.0 - cos_i.powi(2));
        if sin2_t > 1.0 {
            return BLACK;
        }

        let cos_t = (1.0 - sin2_t).sqrt();
        let direction =
            comps.normal_vector * (n_ratio * cos_i - cos_t) - comps.eye_vector * n_ratio;
        let refract_ray = ray(comps.under_point, direction);
        self.color_at(refract_ray, remaining - 1) * comps.object.material.transparency
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use crate::{
        ORIGIN, REFLECTION_DEPTH, WHITE, intersection, pattern::test_pattern, plane, ray,
        transform::translation, vector,
    };

    use super::*;

    #[test]
    fn intersecting_world_with_ray() {
        let w = default_world();
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let xs = w.intersect(r);
        assert_eq!(4, xs.len());
        assert_eq!(4.0, xs[0].time);
        assert_eq!(4.5, xs[1].time);
        assert_eq!(5.5, xs[2].time);
        assert_eq!(6.0, xs[3].time);
    }

    #[test]
    fn shading_intersection() {
        let w = default_world();
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let shape = &w.objects[0];
        let i = intersection(4, shape);
        let comps = i.prepare_computations(r, &[i]);
        assert_abs_diff_eq!(
            color(0.38066, 0.47583, 0.2855),
            w.shade_hit(comps, REFLECTION_DEPTH)
        );
    }

    #[test]
    fn shading_intersection_from_inside() {
        let mut w = default_world();
        w.light = Some(point_light(point(0.0, 0.25, 0.0), color(1, 1, 1)));
        let r = ray(point(0, 0, 0), vector(0, 0, 1));
        let shape = &w.objects[1];
        let i = intersection(0.5, shape);
        let comps = i.prepare_computations(r, &[i]);
        assert_abs_diff_eq!(
            color(0.90498, 0.90498, 0.90498),
            w.shade_hit(comps, REFLECTION_DEPTH)
        );
    }

    #[test]
    fn color_when_ray_misses() {
        let w = default_world();
        let r = ray(point(0, 0, -5), vector(0, 1, 0));
        assert_eq!(BLACK, w.color_at(r, REFLECTION_DEPTH));
    }

    #[test]
    fn color_when_ray_hits() {
        let w = default_world();
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        assert_abs_diff_eq!(
            color(0.38066, 0.47583, 0.2855),
            w.color_at(r, REFLECTION_DEPTH)
        );
    }

    #[test]
    fn color_with_intersection_behind_ray() {
        let mut w = default_world();
        {
            let outer = &mut w.objects[0];
            outer.material.ambient = 1.0;
            let inner = &mut w.objects[1];
            inner.material.ambient = 1.0;
        }
        let inner = &w.objects[1];
        let r = ray(point(0.0, 0.0, 0.75), vector(0, 0, -1));
        assert_eq!(inner.material.color, w.color_at(r, REFLECTION_DEPTH));
    }

    #[test]
    fn no_shadow_when_nothing_collinear() {
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
    fn shade_hit_given_an_intersection_in_shadow() {
        let mut w = world();
        w.light = Some(point_light(point(0, 0, -10), WHITE));
        w.objects.push(sphere());

        let mut s = sphere();
        s.transform = translation(0, 0, 10);
        w.objects.push(s);

        let r = ray(point(0, 0, 5), vector(0, 0, 1));
        let i = intersection(4, &w.objects[0]);
        let comps = i.prepare_computations(r, &[i]);

        assert_eq!(color(0.1, 0.1, 0.1), w.shade_hit(comps, REFLECTION_DEPTH));
    }

    #[test]
    fn reflected_color_for_nonreflective_material() {
        let mut w = default_world();
        let r = ray(ORIGIN, vector(0, 0, 1));
        let shape = {
            let shape = &mut w.objects[1];
            shape.material.ambient = 1.0;
            &w.objects[1]
        };
        let i = intersection(1, shape);
        let comps = i.prepare_computations(r, &[i]);
        assert_eq!(BLACK, w.reflected_color(comps, REFLECTION_DEPTH));
    }

    #[test]
    fn reflected_color_for_reflective_material() {
        let mut w = default_world();
        let r = ray(
            point(0, 0, -3),
            vector(0.0, -(2.0_f64.sqrt()) / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let mut shape = plane();
        shape.material.reflective = 0.5;
        shape.transform = translation(0, -1, 0);
        w.objects.push(shape);
        let shape = &w.objects[2];
        let i = intersection(2.0_f64.sqrt(), shape);
        let comps = i.prepare_computations(r, &[i]);
        assert_abs_diff_eq!(
            color(0.19032, 0.2379, 0.14274),
            w.reflected_color(comps, REFLECTION_DEPTH)
        );
    }

    #[test]
    fn shade_hit_with_reflective_material() {
        let mut w = default_world();
        let mut shape = plane();
        shape.material.reflective = 0.5;
        shape.transform = translation(0, -1, 0);
        w.objects.push(shape);
        let shape = &w.objects[2];
        let r = ray(
            point(0, 0, -3),
            vector(0.0, -(2.0_f64.sqrt()) / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let i = intersection(2.0_f64.sqrt(), shape);
        let comps = i.prepare_computations(r, &[i]);
        assert_abs_diff_eq!(
            color(0.87677, 0.92436, 0.82918),
            w.shade_hit(comps, REFLECTION_DEPTH)
        );
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let mut w = world();
        w.light = Some(point_light(ORIGIN, WHITE));
        let mut lower = plane();
        lower.material.reflective = 1.0;
        lower.transform = translation(0, -1, 0);
        w.objects.push(lower);
        let mut upper = plane();
        upper.material.reflective = 1.0;
        upper.transform = translation(0, 1, 0);
        w.objects.push(upper);
        let r = ray(ORIGIN, vector(0, 1, 0));
        w.color_at(r, REFLECTION_DEPTH);
    }

    #[test]
    fn reflected_color_at_max_recursion_depth() {
        let mut w = default_world();
        let mut shape = plane();
        shape.material.reflective = 0.5;
        shape.transform = translation(0, -1, 0);
        w.objects.push(shape);
        let shape = &w.objects[2];
        let r = ray(
            point(0, 0, -3),
            vector(0.0, -(2.0_f64.sqrt()) / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let i = intersection(2.0_f64.sqrt(), shape);
        let comps = i.prepare_computations(r, &[i]);
        assert_eq!(BLACK, w.reflected_color(comps, 0));
    }

    #[test]
    fn refracted_color_with_opaque_surface() {
        let w = default_world();
        let shape = &w.objects[0];
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let xs = vec![intersection(4, shape), intersection(6, shape)];
        let comps = xs[0].prepare_computations(r, &xs);
        assert_eq!(BLACK, w.refracted_color(comps, REFLECTION_DEPTH));
    }

    #[test]
    fn refracted_color_at_max_recursion_depth() {
        let mut w = default_world();
        {
            let shape = &mut w.objects[0];
            shape.material.transparency = 1.0;
            shape.material.refractive_index = 1.5;
        }
        let shape = &w.objects[0];
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let xs = vec![intersection(4, shape), intersection(6, shape)];
        let comps = xs[0].prepare_computations(r, &xs);
        assert_eq!(BLACK, w.refracted_color(comps, 0));
    }

    #[test]
    fn refracted_color_under_total_internal_reflection() {
        let mut w = default_world();
        {
            let shape = &mut w.objects[0];
            shape.material.transparency = 1.0;
            shape.material.refractive_index = 1.5;
        }
        let shape = &w.objects[0];
        let r = ray(point(0.0, 0.0, 2.0_f64.sqrt() / 2.0), vector(0, 1, 0));
        let xs = vec![
            intersection(-(2.0_f64.sqrt()) / 2.0, shape),
            intersection(2.0_f64.sqrt() / 2.0, shape),
        ];
        let comps = xs[1].prepare_computations(r, &xs);
        assert_eq!(BLACK, w.refracted_color(comps, REFLECTION_DEPTH));
    }

    #[test]
    fn refracted_color_with_reflected_ray() {
        let mut w = default_world();
        {
            let shape = &mut w.objects[0];
            shape.material.ambient = 1.0;
            shape.material.pattern = Some(test_pattern());
        }
        {
            let shape = &mut w.objects[1];
            shape.material.transparency = 1.0;
            shape.material.refractive_index = 1.5;
        }
        let a = &w.objects[0];
        let b = &w.objects[1];
        let r = ray(point(0.0, 0.0, 0.1), vector(0, 1, 0));
        let xs = vec![
            intersection(-0.9899, a),
            intersection(-0.4899, b),
            intersection(0.4899, b),
            intersection(0.9899, a),
        ];
        let comps = xs[2].prepare_computations(r, &xs);
        assert_abs_diff_eq!(
            color(0.0, 0.99888, 0.04725),
            w.refracted_color(comps, REFLECTION_DEPTH)
        );
    }

    #[test]
    fn shade_hit_with_transparent_material() {
        let mut w = default_world();
        let mut floor = plane();
        floor.transform = translation(0, -1, 0);
        floor.material.transparency = 0.5;
        floor.material.refractive_index = 1.5;
        w.objects.push(floor);

        let mut ball = sphere();
        ball.material.color = color(1, 0, 0);
        ball.material.ambient = 0.5;
        ball.transform = translation(0.0, -3.5, -0.5);
        w.objects.push(ball);

        let floor = &w.objects[2];

        let r = ray(
            point(0, 0, -3),
            vector(0.0, -(2.0_f64.sqrt()) / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let xs = vec![intersection(2.0_f64.sqrt(), floor)];
        let comps = xs[0].prepare_computations(r, &xs);
        assert_abs_diff_eq!(
            color(0.93642, 0.68642, 0.68642),
            w.shade_hit(comps, REFLECTION_DEPTH)
        );
    }

    #[test]
    fn shade_hit_with_reflective_transparent_material() {
        let mut w = default_world();
        let mut floor = plane();
        floor.transform = translation(0, -1, 0);
        floor.material.reflective = 0.5;
        floor.material.transparency = 0.5;
        floor.material.refractive_index = 1.5;
        w.objects.push(floor);

        let mut ball = sphere();
        ball.material.color = color(1, 0, 0);
        ball.material.ambient = 0.5;
        ball.transform = translation(0.0, -3.5, -0.5);
        w.objects.push(ball);

        let floor = &w.objects[2];
        let r = ray(
            point(0, 0, -3),
            vector(0.0, -(2.0_f64.sqrt()) / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let xs = vec![intersection(2.0_f64.sqrt(), floor)];
        let comps = xs[0].prepare_computations(r, &xs);
        assert_abs_diff_eq!(
            color(0.93391, 0.69643, 0.69243),
            w.shade_hit(comps, REFLECTION_DEPTH)
        );
    }
}
