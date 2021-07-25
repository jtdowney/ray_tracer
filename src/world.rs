use crate::{
    color, ray, Color, Computations, Intersection, Intersections, Point, PointLight, Ray, Shape,
};
use derive_builder::Builder;
use num::Zero;
use ord_subset::OrdSubsetSliceExt;
use std::{rc::Rc, usize};

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

#[derive(Clone, Builder)]
pub struct World {
    #[builder(setter(strip_option), default)]
    light: Option<PointLight>,
    #[builder(default)]
    objects: Vec<Rc<dyn Shape>>,
}

impl WorldBuilder {
    pub fn object<S>(&mut self, object: S) -> &mut WorldBuilder
    where
        S: Shape + 'static,
    {
        let object = Rc::new(object) as Rc<dyn Shape>;
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

    pub fn shade_hit<'o>(&'o self, comps: Computations<'o>, remaining: usize) -> Color {
        if let Some(light) = self.light {
            let in_shadow = self.is_shadowed(comps.over_point);
            let material = comps.object.material();
            let surface = material.lighting(
                &light,
                comps.over_point,
                comps.eye_vector,
                comps.normal_vector,
                in_shadow,
            );

            let reflected = self.reflected_color(&comps, remaining);
            let refracted = self.refracted_color(&comps, remaining);

            if material.reflective > 0.0 && material.transparency > 0.0 {
                let reflectance = comps.schlick();
                surface + reflected * reflectance + refracted * (1.0 - reflectance)
            } else {
                surface + reflected + refracted
            }
        } else {
            color::BLACK
        }
    }

    pub fn color_at(&self, ray: Ray, remaining: usize) -> Color {
        if remaining == 0 {
            return color::BLACK;
        }

        let xs = self.intersect(ray);
        if let Some(i) = xs.hit() {
            let comps = i.prepare_computations(ray, &xs);
            self.shade_hit(comps, remaining)
        } else {
            color::BLACK
        }
    }

    pub fn is_shadowed(&self, point: Point) -> bool {
        if let Some(light) = self.light {
            let v = light.position - point;
            let distance = v.magnitude();
            let direction = v.normalize();
            let ray = ray(point, direction);
            let intersections = self.intersect(ray);

            if let Some(hit) = intersections.hit() {
                hit.time < distance
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn reflected_color<'o>(&self, comps: &Computations<'o>, remaining: usize) -> Color {
        let reflective = comps.object.material().reflective;
        if reflective.is_zero() || remaining.is_zero() {
            color::BLACK
        } else {
            let reflected_ray = ray(comps.over_point, comps.reflect_vector);
            let color = self.color_at(reflected_ray, remaining - 1);
            color * reflective
        }
    }

    pub fn refracted_color<'o>(&self, comps: &Computations<'o>, remaining: usize) -> Color {
        let transparency = comps.object.material().transparency;
        if transparency.is_zero() || remaining.is_zero() {
            return color::BLACK;
        }

        let n_ratio = comps.n1 / comps.n2;
        let cos_i = comps.eye_vector.dot(comps.normal_vector);
        let sin2_t = n_ratio.powi(2) * (1.0 - cos_i.powi(2));
        if sin2_t > 1.0 {
            return color::WHITE;
        }

        let cos_t = f64::sqrt(1.0 - sin2_t);
        let direction =
            comps.normal_vector * (n_ratio * cos_i - cos_t) - comps.eye_vector * n_ratio;
        let refract_ray = ray(comps.under_point, direction);
        self.color_at(refract_ray, remaining - 1) * transparency
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        color, intersection, point, point_light, ray, sphere, transformations, vector,
        MaterialBuilder, PlaneBuilder, SphereBuilder, REFLECTION_LIMIT,
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
        assert_eq!(w.objects.len(), 2);
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
        let i = intersection(4.0, shape.as_ref());
        let comps = i.prepare_computations(r, &Intersections::empty());
        let c = w.shade_hit(comps, REFLECTION_LIMIT);
        assert_abs_diff_eq!(c, color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = default_world();
        w.light = Some(point_light(point(0.0, 0.25, 0.0), color(1.0, 1.0, 1.0)));

        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = &w.objects[1];
        let i = intersection(0.5, shape.as_ref());
        let comps = i.prepare_computations(r, &Intersections::empty());
        let c = w.shade_hit(comps, REFLECTION_LIMIT);
        assert_abs_diff_eq!(c, color(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn color_when_ray_misses() {
        let w = default_world();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0));
        let c = w.color_at(r, REFLECTION_LIMIT);
        assert_abs_diff_eq!(c, color::BLACK);
    }

    #[test]
    fn color_when_ray_hits() {
        let w = default_world();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let c = w.color_at(r, REFLECTION_LIMIT);
        assert_abs_diff_eq!(c, color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_with_an_intersection_behind_the_ray() {
        let mut w = default_world();
        let outer = Rc::get_mut(&mut w.objects[0]).unwrap();
        let mut material = outer.material().clone();
        material.ambient = 1.0;
        outer.set_material(material);

        let inner = Rc::get_mut(&mut w.objects[1]).unwrap();
        let mut material = inner.material().clone();
        material.ambient = 1.0;
        inner.set_material(material);

        let inner = &w.objects[1];
        let r = ray(point(0.0, 0.0, 0.75), vector(0.0, 0.0, -1.0));
        let c = w.color_at(r, REFLECTION_LIMIT);
        assert_abs_diff_eq!(c, inner.material().color);
    }

    #[test]
    fn no_shadow_when_nothing_is_colinear_with_point_and_light() {
        let w = default_world();
        let p = point(0.0, 10.0, 0.0);
        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn shadow_when_an_object_is_between_point_and_light() {
        let w = default_world();
        let p = point(10.0, -10.0, 10.0);
        assert!(w.is_shadowed(p));
    }

    #[test]
    fn no_shadow_when_object_is_behind_light() {
        let w = default_world();
        let p = point(-20.0, 20.0, -20.0);
        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn no_shadow_when_object_is_behind_point() {
        let w = default_world();
        let p = point(-2.0, 2.0, -2.0);
        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn shade_hit_when_in_shadow() {
        let w = WorldBuilder::default()
            .light(point_light(point(0.0, 0.0, -10.0), color::WHITE))
            .object(sphere())
            .object(
                SphereBuilder::default()
                    .transform(transformations::translation(0.0, 0.0, 10.0))
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();

        let r = ray(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let i = intersection(4.0, w.objects[1].as_ref());
        let comps = i.prepare_computations(r, &Intersections::empty());
        assert_eq!(w.shade_hit(comps, REFLECTION_LIMIT), color(0.1, 0.1, 0.1));
    }

    #[test]
    fn reflected_color_for_a_nonreflective_surface() {
        let mut w = default_world();
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = Rc::get_mut(&mut w.objects[1]).unwrap();
        let mut material = shape.material().clone();
        material.ambient = 1.0;
        shape.set_material(material);
        let shape = &w.objects[1];

        let i = intersection(1.0, shape.as_ref());
        let comps = i.prepare_computations(r, &Intersections::empty());
        let c = w.reflected_color(&comps, REFLECTION_LIMIT);
        assert_eq!(c, color::BLACK);
    }

    #[test]
    fn reflected_color_for_a_reflective_material() {
        let mut w = default_world();
        let shape = PlaneBuilder::default()
            .transform(transformations::translation(0.0, -1.0, 0.0))
            .material(MaterialBuilder::default().reflective(0.5).build().unwrap())
            .build()
            .unwrap();
        let shape = Rc::new(shape);
        w.objects.push(shape);
        let shape = w.objects.last().unwrap();

        let r = ray(
            point(0.0, 0.0, -3.0),
            vector(0.0, -f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
        );
        let i = intersection(f64::sqrt(2.0), shape.as_ref());
        let comps = i.prepare_computations(r, &Intersections::empty());
        let c = w.reflected_color(&comps, REFLECTION_LIMIT);
        assert_abs_diff_eq!(c, color(0.19032, 0.2379, 0.14274));
    }

    #[test]
    fn shade_hit_with_a_reflective_material() {
        let mut w = default_world();
        let shape = PlaneBuilder::default()
            .transform(transformations::translation(0.0, -1.0, 0.0))
            .material(MaterialBuilder::default().reflective(0.5).build().unwrap())
            .build()
            .unwrap();
        let shape = Rc::new(shape);
        w.objects.push(shape);
        let shape = w.objects.last().unwrap();

        let r = ray(
            point(0.0, 0.0, -3.0),
            vector(0.0, -f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
        );
        let i = intersection(f64::sqrt(2.0), shape.as_ref());
        let comps = i.prepare_computations(r, &Intersections::empty());
        let c = w.shade_hit(comps, REFLECTION_LIMIT);
        assert_abs_diff_eq!(c, color(0.87677, 0.92436, 0.82918));
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let w = WorldBuilder::default()
            .light(point_light(point(0.0, 0.0, 0.0), color(1.0, 1.0, 1.0)))
            .object(
                PlaneBuilder::default()
                    .transform(transformations::translation(0.0, -1.0, 0.0))
                    .material(MaterialBuilder::default().reflective(1.0).build().unwrap())
                    .build()
                    .unwrap(),
            )
            .object(
                PlaneBuilder::default()
                    .transform(transformations::translation(0.0, 1.0, 0.0))
                    .material(MaterialBuilder::default().reflective(1.0).build().unwrap())
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 1.0, 0.0));
        w.color_at(r, REFLECTION_LIMIT);
    }

    #[test]
    fn relfected_color_at_the_maximum_recursive_depth() {
        let mut w = default_world();
        let shape = PlaneBuilder::default()
            .transform(transformations::translation(0.0, -1.0, 0.0))
            .material(MaterialBuilder::default().reflective(0.5).build().unwrap())
            .build()
            .unwrap();
        let shape = Rc::new(shape);
        w.objects.push(shape);
        let shape = w.objects.last().unwrap();

        let r = ray(
            point(0.0, 0.0, -3.0),
            vector(0.0, -f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
        );
        let i = intersection(f64::sqrt(2.0), shape.as_ref());
        let comps = i.prepare_computations(r, &Intersections::empty());
        let c = w.reflected_color(&comps, 0);
        assert_eq!(c, color::BLACK);
    }

    #[test]
    fn refracted_color_of_an_opaque_object() {
        let w = default_world();
        let shape = w.objects[0].as_ref();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs: Intersections = vec![intersection(4.0, shape), intersection(6.0, shape)].into();
        let comps = xs[0].prepare_computations(r, &xs);
        let c = w.refracted_color(&comps, 5);
        assert_eq!(c, color::BLACK);
    }

    #[test]
    fn refracted_color_at_the_maximum_recursive_depth() {
        let mut w = default_world();
        let shape = Rc::get_mut(&mut w.objects[0]).unwrap();
        let mut material = shape.material().clone();
        material.transparency = 1.0;
        material.refractive_index = 1.5;
        shape.set_material(material);
        let shape = w.objects[0].as_ref();

        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs: Intersections = vec![intersection(4.0, shape), intersection(6.0, shape)].into();
        let comps = xs[0].prepare_computations(r, &xs);
        let c = w.refracted_color(&comps, 0);
        assert_eq!(c, color::BLACK);
    }

    #[test]
    fn refracted_color_under_total_internal_reflection() {
        let mut w = default_world();
        let shape = Rc::get_mut(&mut w.objects[0]).unwrap();
        let mut material = shape.material().clone();
        material.transparency = 1.0;
        material.refractive_index = 1.5;
        shape.set_material(material);
        let shape = w.objects[0].as_ref();

        let r = ray(point(0.0, 0.0, f64::sqrt(2.0) / 2.0), vector(0.0, 1.0, 0.0));
        let xs: Intersections = vec![
            intersection(-f64::sqrt(2.0) / 2.0, shape),
            intersection(f64::sqrt(2.0) / 2.0, shape),
        ]
        .into();
        let comps = xs[1].prepare_computations(r, &xs);
        let c = w.refracted_color(&comps, 5);
        assert_eq!(c, color::WHITE);
    }

    #[test]
    fn refracted_color_with_a_refracted_ray() {
        let mut w = default_world();
        let shape = Rc::get_mut(&mut w.objects[0]).unwrap();
        let mut material = shape.material().clone();
        material.ambient = 1.0;
        material.pattern = Some(Rc::new(crate::pattern::tests::test_pattern()));
        shape.set_material(material);

        let shape = Rc::get_mut(&mut w.objects[1]).unwrap();
        let mut material = shape.material().clone();
        material.transparency = 1.0;
        material.refractive_index = 1.5;
        shape.set_material(material);

        let a = w.objects[0].as_ref();
        let b = w.objects[1].as_ref();

        let r = ray(point(0.0, 0.0, 0.1), vector(0.0, 1.0, 0.0));
        let xs: Intersections = vec![
            intersection(-0.9899, a),
            intersection(-0.4899, b),
            intersection(0.4899, b),
            intersection(0.9899, a),
        ]
        .into();
        let comps = xs[2].prepare_computations(r, &xs);
        let c = w.refracted_color(&comps, 5);
        assert_abs_diff_eq!(c, color(0.0, 0.99888, 0.04725));
    }

    #[test]
    fn shade_hit_with_a_transparent_material() {
        let mut w = default_world();
        let ball = SphereBuilder::default()
            .transform(transformations::translation(0.0, -3.5, -0.5))
            .material(
                MaterialBuilder::default()
                    .color(color(1.0, 0.0, 0.0))
                    .ambient(0.5)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        w.objects.push(Rc::new(ball));
        let floor = PlaneBuilder::default()
            .transform(transformations::translation(0.0, -1.0, 0.0))
            .material(
                MaterialBuilder::default()
                    .transparency(0.5)
                    .refractive_index(1.5)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        w.objects.push(Rc::new(floor));
        let floor = w.objects.last().unwrap().as_ref();

        let r = ray(
            point(0.0, 0.0, -3.0),
            vector(0.0, -f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
        );
        let xs: Intersections = vec![intersection(f64::sqrt(2.0), floor)].into();
        let comps = xs[0].prepare_computations(r, &xs);
        let c = w.shade_hit(comps, 5);
        assert_abs_diff_eq!(c, color(0.93642, 0.68642, 0.68642));
    }

    #[test]
    fn shade_hit_with_a_reflective_transparent_material() {
        let mut w = default_world();
        let ball = SphereBuilder::default()
            .transform(transformations::translation(0.0, -3.5, -0.5))
            .material(
                MaterialBuilder::default()
                    .color(color(1.0, 0.0, 0.0))
                    .ambient(0.5)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        w.objects.push(Rc::new(ball));
        let floor = PlaneBuilder::default()
            .transform(transformations::translation(0.0, -1.0, 0.0))
            .material(
                MaterialBuilder::default()
                    .reflective(0.5)
                    .transparency(0.5)
                    .refractive_index(1.5)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        w.objects.push(Rc::new(floor));
        let floor = w.objects.last().unwrap().as_ref();
        let r = ray(
            point(0.0, 0.0, -3.0),
            vector(0.0, -f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
        );
        let xs: Intersections = vec![intersection(f64::sqrt(2.0), floor)].into();
        let comps = xs[0].prepare_computations(r, &xs);
        let c = w.shade_hit(comps, 5);
        assert_abs_diff_eq!(c, color(0.93391, 0.69643, 0.69243));
    }
}
