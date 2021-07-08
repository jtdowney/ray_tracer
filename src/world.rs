use crate::{color, Color, Computations, Intersection, Intersections, PointLight, Ray, Sphere};
use derive_builder::Builder;
use ord_subset::OrdSubsetSliceExt;

#[cfg(test)]
pub fn default_world() -> World {
    use crate::{point, point_light, transformations, MaterialBuilder, SphereBuilder};

    WorldBuilder::default()
        .light(point_light(point(-10.0, 10.0, -10.0), color::WHITE))
        .object(
            SphereBuilder::default()
                .material(
                    MaterialBuilder::default()
                        .color(color(0.8, 1.0, 0.6))
                        .diffuse(0.7)
                        .specular(0.2)
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .object(
            SphereBuilder::default()
                .transform(transformations::scaling(0.5, 0.5, 0.5))
                .build()
                .unwrap(),
        )
        .build()
        .unwrap()
}

#[derive(Debug, Clone, Builder, Default)]
pub struct World {
    #[builder(setter(strip_option), default)]
    light: Option<PointLight>,
    #[builder(default)]
    objects: Vec<Sphere>,
}

impl WorldBuilder {
    pub fn object(&mut self, object: Sphere) -> &mut WorldBuilder {
        if let Some(ref mut objects) = self.objects {
            objects.push(object);
        } else {
            self.objects = Some(vec![object]);
        }
        self
    }
}

impl World {
    pub fn intersect(&self, ray: Ray) -> Intersections {
        let mut xs: Vec<Intersection> = self
            .objects
            .iter()
            .flat_map(|object| object.intersect(ray))
            .collect();
        xs.ord_subset_sort_by_key(|i| i.time);
        xs.into()
    }

    pub fn shade_hit<'o>(&'o self, comps: Computations<'o>) -> Color {
        if let Some(light) = self.light {
            comps.object.material.lighting(
                &light,
                comps.point,
                comps.eye_vector,
                comps.normal_vector,
            )
        } else {
            color::BLACK
        }
    }

    pub fn color_at(&self, ray: Ray) -> Color {
        let xs = self.intersect(ray);
        if let Some(i) = xs.hit() {
            let comps = i.prepare_computations(ray);
            self.shade_hit(comps)
        } else {
            color::BLACK
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        color, intersection, point, point_light, ray, transformations, vector, MaterialBuilder,
        SphereBuilder,
    };
    use approx::assert_abs_diff_eq;

    #[test]
    fn creating_a_world() {
        let w = WorldBuilder::default().build().unwrap();
        assert!(w.light.is_none());
        assert!(w.objects.is_empty());
    }

    #[test]
    fn the_default_world() {
        let w = default_world();
        assert_eq!(
            w.light,
            Some(point_light(point(-10.0, 10.0, -10.0), color::WHITE))
        );
        assert_eq!(
            w.objects[0],
            SphereBuilder::default()
                .material(
                    MaterialBuilder::default()
                        .color(color(0.8, 1.0, 0.6))
                        .diffuse(0.7)
                        .specular(0.2)
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap()
        );
        assert_eq!(
            w.objects[1],
            SphereBuilder::default()
                .transform(transformations::scaling(0.5, 0.5, 0.5))
                .build()
                .unwrap(),
        );
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = default_world();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut xs = w.intersect(r).into_iter();
        assert_eq!(4.0, xs.next().unwrap().time);
        assert_eq!(4.5, xs.next().unwrap().time);
        assert_eq!(5.5, xs.next().unwrap().time);
        assert_eq!(6.0, xs.next().unwrap().time);
        assert!(xs.next().is_none());
    }

    #[test]
    fn shading_an_intersection() {
        let w = default_world();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = &w.objects[0];
        let i = intersection(4.0, shape);
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps);
        assert_abs_diff_eq!(c, color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = default_world();
        w.light = Some(point_light(point(0.0, 0.25, 0.0), color(1.0, 1.0, 1.0)));

        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = &w.objects[1];
        let i = intersection(0.5, shape);
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps);
        assert_abs_diff_eq!(c, color(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn color_when_ray_misses() {
        let w = default_world();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0));
        let c = w.color_at(r);
        assert_abs_diff_eq!(c, color::BLACK);
    }

    #[test]
    fn color_when_ray_hits() {
        let w = default_world();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let c = w.color_at(r);
        assert_abs_diff_eq!(c, color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_with_an_intersection_behind_the_ray() {
        let mut w = default_world();
        let outer = &mut w.objects[0];
        outer.material.ambient = 1.0;
        let mut inner = &mut w.objects[1];
        inner.material.ambient = 1.0;

        let inner = &w.objects[1];
        let r = ray(point(0.0, 0.0, 0.75), vector(0.0, 0.0, -1.0));
        let c = w.color_at(r);
        assert_abs_diff_eq!(c, inner.material.color);
    }
}
