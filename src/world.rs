use bon::Builder;
use ord_subset::OrdSubsetSliceExt;

use crate::{
    Color, Intersection, Material, Point, PointLight, Ray, Shape, color,
    color::{BLACK, WHITE},
    hit,
    intersection::Computations,
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
    pub fn shade_hit(&self, comps: &Computations<'_>) -> Color {
        if let Some(light) = self.light {
            let shadowed = self.is_shadowed(comps.over_point);
            comps.object.material.lighting(
                &light,
                comps.over_point,
                comps.eyev,
                comps.normalv,
                shadowed,
            )
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
    pub fn color_at(&self, ray: Ray) -> Color {
        let xs = self.intersect(ray);
        if let Some(i) = hit(&xs) {
            let comps = i.prepare_computations(ray);
            self.shade_hit(&comps)
        } else {
            BLACK
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;
    use crate::{
        EPSILON, Material, color, intersection, point, point_light, ray, transform, vector,
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
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(&comps);
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
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(&comps);
        assert_relative_eq!(c.red, 0.90498, epsilon = EPSILON);
        assert_relative_eq!(c.green, 0.90498, epsilon = EPSILON);
        assert_relative_eq!(c.blue, 0.90498, epsilon = EPSILON);
    }

    #[test]
    fn color_when_ray_misses() {
        let w = default_world();
        let r = ray(point(0, 0, -5), vector(0, 1, 0));
        let c = w.color_at(r);
        assert_eq!(c, color(0, 0, 0));
    }

    #[test]
    fn color_when_ray_hits() {
        let w = default_world();
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let c = w.color_at(r);
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
        let c = w.color_at(r);
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
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(&comps);
        assert_relative_eq!(c.red, 0.1, epsilon = EPSILON);
        assert_relative_eq!(c.green, 0.1, epsilon = EPSILON);
        assert_relative_eq!(c.blue, 0.1, epsilon = EPSILON);
    }
}
