use ord_subset::OrdSubsetSliceExt;

use crate::{
    color, hit,
    intersection::{Computations, Intersection},
    point, point_light, sphere,
    transform::scaling,
    Color, PointLight, Ray, Sphere, BLACK,
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

#[derive(Clone, Debug, Default)]
pub struct World {
    pub light: Option<PointLight>,
    pub objects: Vec<Sphere>,
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

    pub fn shade_hit(&self, comps: Computations) -> Color {
        if let Some(light) = self.light {
            comps.object.material.lighting(
                light,
                comps.point,
                comps.eye_vector,
                comps.normal_vector,
            )
        } else {
            BLACK
        }
    }

    pub fn color_at(&self, ray: Ray) -> Color {
        let xs = self.intersect(ray);
        if let Some(i) = hit(xs) {
            let comps = i.prepare_computations(ray);
            self.shade_hit(comps)
        } else {
            BLACK
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use crate::{intersection, ray, vector};

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
        let shape = w.objects[0];
        let i = intersection(4, &shape);
        let comps = i.prepare_computations(r);
        assert_abs_diff_eq!(color(0.38066, 0.47583, 0.2855), w.shade_hit(comps));
    }

    #[test]
    fn shading_intersection_from_inside() {
        let mut w = default_world();
        w.light = Some(point_light(point(0.0, 0.25, 0.0), color(1, 1, 1)));
        let r = ray(point(0, 0, 0), vector(0, 0, 1));
        let shape = w.objects[1];
        let i = intersection(0.5, &shape);
        let comps = i.prepare_computations(r);
        assert_abs_diff_eq!(color(0.90498, 0.90498, 0.90498), w.shade_hit(comps));
    }

    #[test]
    fn color_when_ray_misses() {
        let w = default_world();
        let r = ray(point(0, 0, -5), vector(0, 1, 0));
        assert_eq!(BLACK, w.color_at(r));
    }

    #[test]
    fn color_when_ray_hits() {
        let w = default_world();
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        assert_abs_diff_eq!(color(0.38066, 0.47583, 0.2855), w.color_at(r));
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
        let inner = w.objects[1];
        let r = ray(point(0.0, 0.0, 0.75), vector(0, 0, -1));
        assert_eq!(inner.material.color, w.color_at(r));
    }
}
