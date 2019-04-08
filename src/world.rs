use crate::{
    color, intersection, transforms, Color, Intersection, Intersections, MaterialBuilder, Point,
    PointLight, Ray, Shape, SphereBuilder,
};
use std::sync::Arc;

#[derive(Default)]
pub struct WorldBuilder {
    objects: Vec<Arc<Shape + Sync + Send>>,
    lights: Vec<PointLight>,
}

impl WorldBuilder {
    pub fn light(&mut self, light: PointLight) -> &mut Self {
        self.lights.push(light);
        self
    }

    pub fn object<O, T>(&mut self, object: O) -> &mut Self
    where
        O: Into<Arc<T>>,
        T: Shape + Sync + Send,
    {
        self.objects.push(object.into());
        self
    }

    pub fn build(&self) -> World {
        World {
            objects: self.objects.clone(),
            lights: self.lights.clone(),
        }
    }
}

pub struct World {
    objects: Vec<Arc<Shape + Sync + Send>>,
    lights: Vec<PointLight>,
}

impl Default for World {
    fn default() -> Self {
        WorldBuilder::default()
            .object(
                SphereBuilder::default()
                    .material(
                        MaterialBuilder::default()
                            .color(Color::new(0.8, 1.0, 0.6))
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
                    .transform(transforms::scaling(0.5, 0.5, 0.5))
                    .build()
                    .unwrap(),
            )
            .light(PointLight::new(
                Point::new(-10.0, 10.0, -10.0),
                color::WHITE,
            ))
            .build()
    }
}

impl World {
    pub fn color_at(&self, ray: Ray, remaining: u8) -> Color {
        if remaining == 0 {
            return color::BLACK;
        }

        let intersections = self.intersect(ray);
        if let Some(hit) = intersections.hit() {
            let comps = hit.prepare_computations(ray, &intersections);
            self.shade_hit(comps, remaining)
        } else {
            color::BLACK
        }
    }

    fn intersect(&self, ray: Ray) -> Intersections {
        let mut intersections = self
            .objects
            .iter()
            .flat_map(|o| o.intersect(ray))
            .collect::<Vec<Intersection>>();
        intersections.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
        Intersections(intersections)
    }

    fn shade_hit(&self, comps: intersection::Computations, remaining: u8) -> Color {
        let surface = self
            .lights
            .iter()
            .map(|light| {
                let shadowed = self.is_shadowed(comps.over_point, light);
                comps.object.material().lighting(
                    comps.object,
                    light,
                    comps.over_point,
                    comps.eye_vector,
                    comps.normal_vector,
                    shadowed,
                )
            })
            .sum::<Color>();
        let reflected = self.reflected_color(comps, remaining);
        let refracted = self.refracted_color(comps, remaining);

        let material = comps.object.material();
        if material.is_reflective() && material.is_transparent() {
            let reflectance = comps.schlick();
            surface + reflected * reflectance + refracted * (1.0 - reflectance)
        } else {
            surface + reflected + refracted
        }
    }

    fn is_shadowed(&self, point: Point, light: &PointLight) -> bool {
        let light_vector = light.position - point;
        let distance = light_vector.magnitude();
        let direction = light_vector.normalize();

        let ray = Ray::new(point, direction);
        let intersections = self.intersect(ray);
        if let Some(hit) = intersections.hit() {
            hit.time < distance
        } else {
            false
        }
    }

    fn reflected_color(&self, comps: intersection::Computations, remaining: u8) -> Color {
        if remaining == 0 || !comps.object.material().is_reflective() {
            color::BLACK
        } else {
            let reflect_ray = Ray::new(comps.over_point, comps.reflect_vector);
            let color = self.color_at(reflect_ray, remaining - 1);
            color * comps.object.material().reflective
        }
    }

    fn refracted_color(&self, comps: intersection::Computations, remaining: u8) -> Color {
        if !comps.object.material().is_transparent() {
            return color::BLACK;
        }

        let n_ratio = comps.n1 / comps.n2;
        let cos_i = comps.eye_vector.dot(comps.normal_vector) as f32;
        let sin2_t = n_ratio.powi(2) * (1.0 - cos_i.powi(2));
        if sin2_t > 1.0 {
            return color::BLACK;
        }

        let cos_t = (1.0 - sin2_t).sqrt();
        let direction = comps.normal_vector * f64::from(n_ratio * cos_i - cos_t)
            - comps.eye_vector * f64::from(n_ratio);
        let refract_ray = Ray::new(comps.under_point, direction);

        self.color_at(refract_ray, remaining - 1) * comps.object.material().transparency
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::patterns::tests::TestPattern;
    use crate::{Pattern, PlaneBuilder, Sphere, Vector3};

    #[test]
    fn intersect_world_with_ray() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let mut xs = w.intersect(r).into_iter();
        assert_eq!(4.0, xs.next().unwrap().time);
        assert_eq!(4.5, xs.next().unwrap().time);
        assert_eq!(5.5, xs.next().unwrap().time);
        assert_eq!(6.0, xs.next().unwrap().time);
        assert!(xs.next().is_none());
    }

    #[test]
    fn shading_an_intersection() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let shape = &w.objects[0];
        let i = Intersection {
            time: 4.0,
            object: shape.as_ref(),
        };
        let xs = Intersections(vec![i]);
        let comps = i.prepare_computations(r, &xs);
        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), w.shade_hit(comps, 5));
    }

    #[test]
    fn color_when_ray_misses() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 1.0, 0.0));
        assert_eq!(Color::new(0.0, 0.0, 0.0), w.color_at(r, 5));
    }

    #[test]
    fn color_when_ray_hits() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), w.color_at(r, 5));
    }

    #[test]
    fn color_with_intersection_behind_ray() {
        let mut w = World::default();
        let mut s1 = Arc::get_mut(&mut w.objects[0])
            .unwrap()
            .as_any_mut()
            .downcast_mut::<Sphere>()
            .unwrap();
        s1.material.ambient = 1.0;
        let mut s2 = Arc::get_mut(&mut w.objects[1])
            .unwrap()
            .as_any_mut()
            .downcast_mut::<Sphere>()
            .unwrap();
        s2.material.ambient = 1.0;

        let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector3::new(0.0, 0.0, -1.0));
        assert_eq!(color::WHITE, w.color_at(r, 5));
    }

    #[test]
    fn no_shadow_with_no_collinear_objects() {
        let w = World::default();
        let light = w.lights[0];
        assert_eq!(false, w.is_shadowed(Point::new(0.0, 10.0, 0.0), &light));
    }

    #[test]
    fn shadow_when_object_between_point_and_light() {
        let w = World::default();
        let light = w.lights[0];
        assert_eq!(true, w.is_shadowed(Point::new(10.0, -10.0, 10.0), &light));
    }

    #[test]
    fn no_shadow_when_object_behind_light() {
        let w = World::default();
        let light = w.lights[0];
        assert_eq!(false, w.is_shadowed(Point::new(-20.0, 20.0, -20.0), &light));
    }

    #[test]
    fn no_shadow_when_object_behind_point() {
        let w = World::default();
        let light = w.lights[0];
        assert_eq!(false, w.is_shadowed(Point::new(-2.0, 2.0, -2.0), &light));
    }

    #[test]
    fn shade_hit_when_in_shadow() {
        let w = WorldBuilder::default()
            .light(PointLight::new(Point::new(0.0, 0.0, -10.0), color::WHITE))
            .object(Sphere::default())
            .object(
                SphereBuilder::default()
                    .transform(transforms::translation(0.0, 0.0, 10.0))
                    .build()
                    .unwrap(),
            )
            .build();

        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector3::new(0.0, 0.0, 1.0));
        let i = Intersection {
            time: 4.0,
            object: w.objects[1].as_ref(),
        };
        let xs = Intersections(vec![i]);
        let comps = i.prepare_computations(r, &xs);
        assert_eq!(Color::new(0.1, 0.1, 0.1), w.shade_hit(comps, 5));
    }

    #[test]
    fn reflected_color_for_nonreflective_material() {
        let mut w = World::default();
        let mut shape = Arc::get_mut(&mut w.objects[1])
            .unwrap()
            .as_any_mut()
            .downcast_mut::<Sphere>()
            .unwrap();
        shape.material.ambient = 1.0;

        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let i = Intersection {
            time: 1.0,
            object: w.objects[1].as_ref(),
        };
        let xs = Intersections(vec![i]);
        let comps = i.prepare_computations(r, &xs);
        assert_eq!(color::BLACK, w.reflected_color(comps, 5));
    }

    #[test]
    fn reflected_color_for_reflective_material() {
        let mut w = World::default();
        let shape = PlaneBuilder::default()
            .transform(transforms::translation(0.0, -1.0, 0.0))
            .material(MaterialBuilder::default().reflective(0.5).build().unwrap())
            .build()
            .unwrap();
        w.objects.push(Arc::new(shape) as Arc<Shape + Sync + Send>);

        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector3::new(0.0, -f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
        );
        let i = Intersection {
            time: f64::sqrt(2.0),
            object: w.objects[2].as_ref(),
        };
        let xs = Intersections(vec![i]);
        let comps = i.prepare_computations(r, &xs);
        assert_eq!(
            Color::new(0.19034, 0.23793, 0.14276),
            w.reflected_color(comps, 5)
        );
    }

    #[test]
    fn shade_hit_with_reflective_material() {
        let mut w = World::default();
        let shape = PlaneBuilder::default()
            .transform(transforms::translation(0.0, -1.0, 0.0))
            .material(MaterialBuilder::default().reflective(0.5).build().unwrap())
            .build()
            .unwrap();
        w.objects.push(Arc::new(shape) as Arc<Shape + Sync + Send>);

        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector3::new(0.0, -f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
        );
        let i = Intersection {
            time: f64::sqrt(2.0),
            object: w.objects[2].as_ref(),
        };
        let xs = Intersections(vec![i]);
        let comps = i.prepare_computations(r, &xs);
        assert_eq!(Color::new(0.87677, 0.92436, 0.82918), w.shade_hit(comps, 5));
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let w = WorldBuilder::default()
            .light(PointLight::new(Point::new(0.0, 0.0, 0.0), color::WHITE))
            .object(
                PlaneBuilder::default()
                    .transform(transforms::translation(0.0, -1.0, 0.0))
                    .material(MaterialBuilder::default().reflective(1.0).build().unwrap())
                    .build()
                    .unwrap(),
            )
            .object(
                PlaneBuilder::default()
                    .transform(transforms::translation(0.0, 1.0, 0.0))
                    .material(MaterialBuilder::default().reflective(1.0).build().unwrap())
                    .build()
                    .unwrap(),
            )
            .build();

        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        w.color_at(r, 5);
    }

    #[test]
    fn reflected_color_at_the_maximum_recurive_depth() {
        let mut w = World::default();
        let shape = PlaneBuilder::default()
            .transform(transforms::translation(0.0, -1.0, 0.0))
            .material(MaterialBuilder::default().reflective(0.5).build().unwrap())
            .build()
            .unwrap();
        w.objects.push(Arc::new(shape) as Arc<Shape + Sync + Send>);

        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector3::new(0.0, -f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
        );
        let i = Intersection {
            time: f64::sqrt(2.0),
            object: w.objects[2].as_ref(),
        };
        let xs = Intersections(vec![i]);
        let comps = i.prepare_computations(r, &xs);
        assert_eq!(color::BLACK, w.reflected_color(comps, 0));
    }

    #[test]
    fn refracted_color_with_opaque_surface() {
        let w = World::default();
        let shape = &w.objects[0];
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let i1 = Intersection {
            time: 4.0,
            object: shape.as_ref(),
        };
        let i2 = Intersection {
            time: 6.0,
            object: shape.as_ref(),
        };
        let xs = Intersections(vec![i1, i2]);
        let comps = i1.prepare_computations(r, &xs);
        assert_eq!(color::BLACK, w.refracted_color(comps, 5));
    }

    #[test]
    fn refracted_color_under_total_internal_reflection() {
        let mut w = World::default();
        let mut shape = Arc::get_mut(&mut w.objects[0])
            .unwrap()
            .as_any_mut()
            .downcast_mut::<Sphere>()
            .unwrap();
        shape.material.transparency = 1.0;
        shape.material.refractive_index = 1.5;

        let r = Ray::new(
            Point::new(0.0, 0.0, f64::sqrt(2.0) / 2.0),
            Vector3::new(0.0, 1.0, 0.0),
        );
        let i1 = Intersection {
            time: -f64::sqrt(2.0) / 2.0,
            object: w.objects[0].as_ref(),
        };
        let i2 = Intersection {
            time: f64::sqrt(2.0) / 2.0,
            object: w.objects[0].as_ref(),
        };
        let xs = Intersections(vec![i1, i2]);
        let comps = i2.prepare_computations(r, &xs);
        assert_eq!(color::BLACK, w.refracted_color(comps, 5));
    }

    #[test]
    fn refracted_color_with_refracted_ray() {
        let mut w = World::default();
        {
            let mut s1 = Arc::get_mut(&mut w.objects[0])
                .unwrap()
                .as_any_mut()
                .downcast_mut::<Sphere>()
                .unwrap();
            s1.material.ambient = 1.0;
            s1.material.pattern = Box::new(TestPattern::default()) as Box<Pattern + Sync + Send>;
        }

        {
            let mut s2 = Arc::get_mut(&mut w.objects[1])
                .unwrap()
                .as_any_mut()
                .downcast_mut::<Sphere>()
                .unwrap();
            s2.material.transparency = 1.0;
            s2.material.refractive_index = 1.5;
        }

        let r = Ray::new(Point::new(0.0, 0.0, 0.1), Vector3::new(0.0, 1.0, 0.0));
        let i1 = Intersection {
            time: -0.9899,
            object: w.objects[0].as_ref(),
        };
        let i2 = Intersection {
            time: -0.4899,
            object: w.objects[1].as_ref(),
        };
        let i3 = Intersection {
            time: 0.4899,
            object: w.objects[1].as_ref(),
        };
        let i4 = Intersection {
            time: 0.9899,
            object: w.objects[0].as_ref(),
        };
        let xs = Intersections(vec![i1, i2, i3, i4]);
        let comps = i3.prepare_computations(r, &xs);
        assert_eq!(
            Color::new(0.0, 0.99878, 0.04724),
            w.refracted_color(comps, 5)
        );
    }

    #[test]
    fn shade_hit_with_transparent_material() {
        let mut w = World::default();
        let floor = PlaneBuilder::default()
            .transform(transforms::translation(0.0, -1.0, 0.0))
            .material(
                MaterialBuilder::default()
                    .transparency(0.5)
                    .refractive_index(1.5)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        let ball = SphereBuilder::default()
            .transform(transforms::translation(0.0, -3.5, -0.5))
            .material(
                MaterialBuilder::default()
                    .color(Color::new(1.0, 0.0, 0.0))
                    .ambient(0.5)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();

        w.objects.push(Arc::new(floor) as Arc<Shape + Sync + Send>);
        w.objects.push(Arc::new(ball) as Arc<Shape + Sync + Send>);

        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector3::new(0.0, -f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
        );
        let i = Intersection {
            time: f64::sqrt(2.0),
            object: w.objects[2].as_ref(),
        };
        let xs = Intersections(vec![i]);
        let comps = i.prepare_computations(r, &xs);
        assert_eq!(Color::new(0.93642, 0.68642, 0.68642), w.shade_hit(comps, 5));
    }

    #[test]
    fn shade_hit_with_reflective_transparent_material() {
        let mut w = World::default();
        let floor = PlaneBuilder::default()
            .transform(transforms::translation(0.0, -1.0, 0.0))
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
        let ball = SphereBuilder::default()
            .transform(transforms::translation(0.0, -3.5, -0.5))
            .material(
                MaterialBuilder::default()
                    .color(Color::new(1.0, 0.0, 0.0))
                    .ambient(0.5)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();

        w.objects.push(Arc::new(floor) as Arc<Shape + Sync + Send>);
        w.objects.push(Arc::new(ball) as Arc<Shape + Sync + Send>);

        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector3::new(0.0, -f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
        );
        let i = Intersection {
            time: f64::sqrt(2.0),
            object: w.objects[2].as_ref(),
        };
        let xs = Intersections(vec![i]);
        let comps = i.prepare_computations(r, &xs);
        assert_eq!(Color::new(0.93391, 0.69643, 0.69243), w.shade_hit(comps, 5));
    }
}
