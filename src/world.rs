use crate::{
    color, intersection, transforms, Color, Intersection, Intersections, Pattern, Point,
    PointLight, Ray, Shape, SolidPattern, Sphere,
};

pub struct World {
    objects: Vec<Box<Shape + Send + Sync>>,
    light: PointLight,
}

impl Default for World {
    fn default() -> Self {
        let mut s1 = Sphere::default();
        s1.material.pattern =
            Box::new(SolidPattern::new(Color::new(0.8, 1.0, 0.6))) as Box<Pattern + Send + Sync>;
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let mut s2 = Sphere::default();
        s2.transform = transforms::scaling(0.5, 0.5, 0.5);

        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), color::WHITE);
        let objects = vec![s1, s2]
            .into_iter()
            .map(|s| Box::new(s) as Box<Shape + Send + Sync>)
            .collect();

        World { objects, light }
    }
}

impl World {
    pub fn new<I>(light: PointLight, objects: I) -> Self
    where
        I: IntoIterator<Item = Box<Shape + Send + Sync>>,
    {
        let objects = objects.into_iter().collect();
        World { objects, light }
    }

    pub fn color_at(&self, ray: Ray, remaining: u8) -> Color {
        let intersections = self.intersect(ray);
        if let Some(hit) = intersections.hit() {
            let comps = hit.prepare_computations(ray);
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
        let shadowed = self.is_shadowed(comps.over_point);
        let surface = comps.object.material().lighting(
            comps.object,
            self.light,
            comps.over_point,
            comps.eye_vector,
            comps.normal_vector,
            shadowed,
        );
        let reflected = self.reflected_color(comps, remaining);

        surface + reflected
    }

    fn is_shadowed(&self, point: Point) -> bool {
        let v = self.light.position - point;
        let distance = v.magnitude();
        let direction = v.normalize();

        let ray = Ray::new(point, direction);
        let intersections = self.intersect(ray);
        if let Some(hit) = intersections.hit() {
            hit.time < distance
        } else {
            false
        }
    }

    fn reflected_color(&self, comps: intersection::Computations, remaining: u8) -> Color {
        if remaining == 0 || comps.object.material().reflective == 0.0 {
            color::BLACK
        } else {
            let reflect_ray = Ray::new(comps.over_point, comps.reflect_vector);
            let color = self.color_at(reflect_ray, remaining - 1);
            color * comps.object.material().reflective
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Plane, Vector3};

    #[test]
    fn test_intersect_world_with_ray() {
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
    fn test_shading_an_intersection() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let shape = &w.objects[0];
        let i = Intersection {
            time: 4.0,
            object: shape.as_ref(),
        };
        let comps = i.prepare_computations(r);
        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), w.shade_hit(comps, 5));
    }

    #[test]
    fn test_color_when_ray_misses() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 1.0, 0.0));
        assert_eq!(Color::new(0.0, 0.0, 0.0), w.color_at(r, 5));
    }

    #[test]
    fn test_color_when_ray_hits() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), w.color_at(r, 5));
    }

    #[test]
    fn test_color_with_intersection_behind_ray() {
        let mut w = World::default();
        let mut s1 = w.objects[0].as_any_mut().downcast_mut::<Sphere>().unwrap();
        s1.material.ambient = 1.0;
        let mut s2 = w.objects[1].as_any_mut().downcast_mut::<Sphere>().unwrap();
        s2.material.ambient = 1.0;

        let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector3::new(0.0, 0.0, -1.0));
        assert_eq!(color::WHITE, w.color_at(r, 5));
    }

    #[test]
    fn test_no_shadow_with_no_collinear_objects() {
        let w = World::default();
        assert_eq!(false, w.is_shadowed(Point::new(0.0, 10.0, 0.0)));
    }

    #[test]
    fn test_shadow_when_object_between_point_and_light() {
        let w = World::default();
        assert_eq!(true, w.is_shadowed(Point::new(10.0, -10.0, 10.0)));
    }

    #[test]
    fn test_no_shadow_when_object_behind_light() {
        let w = World::default();
        assert_eq!(false, w.is_shadowed(Point::new(-20.0, 20.0, -20.0)));
    }

    #[test]
    fn test_no_shadow_when_object_behind_point() {
        let w = World::default();
        assert_eq!(false, w.is_shadowed(Point::new(-2.0, 2.0, -2.0)));
    }

    #[test]
    fn test_shade_hit_when_in_shadow() {
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), color::WHITE);
        let s1 = Sphere::default();
        let mut s2 = Sphere::default();
        s2.transform = transforms::translation(0.0, 0.0, 10.0);

        let w = World::new(
            light,
            vec![
                Box::new(s1) as Box<Shape + Send + Sync>,
                Box::new(s2) as Box<Shape + Send + Sync>,
            ],
        );

        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector3::new(0.0, 0.0, 1.0));
        let i = Intersection {
            time: 4.0,
            object: w.objects[1].as_ref(),
        };
        let comps = i.prepare_computations(r);
        assert_eq!(Color::new(0.1, 0.1, 0.1), w.shade_hit(comps, 5));
    }

    #[test]
    fn test_reflected_color_for_nonreflective_material() {
        let mut w = World::default();
        let mut shape = w.objects[1].as_any_mut().downcast_mut::<Sphere>().unwrap();
        shape.material.ambient = 1.0;

        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let i = Intersection {
            time: 1.0,
            object: w.objects[1].as_ref(),
        };
        let comps = i.prepare_computations(r);
        assert_eq!(color::BLACK, w.reflected_color(comps, 5));
    }

    #[test]
    fn test_reflected_color_for_reflective_material() {
        let mut w = World::default();
        let mut shape = Plane::default();
        shape.material.reflective = 0.5;
        shape.transform = transforms::translation(0.0, -1.0, 0.0);
        w.objects.push(Box::new(shape) as Box<Shape + Send + Sync>);

        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector3::new(0.0, -f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
        );
        let i = Intersection {
            time: f64::sqrt(2.0),
            object: w.objects[2].as_ref(),
        };
        let comps = i.prepare_computations(r);
        assert_eq!(
            Color::new(0.19032, 0.2379, 0.14274),
            w.reflected_color(comps, 5)
        );
    }

    #[test]
    fn test_shade_hit_with_reflective_material() {
        let mut w = World::default();
        let mut shape = Plane::default();
        shape.material.reflective = 0.5;
        shape.transform = transforms::translation(0.0, -1.0, 0.0);
        w.objects.push(Box::new(shape) as Box<Shape + Send + Sync>);

        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector3::new(0.0, -f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
        );
        let i = Intersection {
            time: f64::sqrt(2.0),
            object: w.objects[2].as_ref(),
        };
        let comps = i.prepare_computations(r);
        assert_eq!(Color::new(0.87677, 0.92436, 0.82918), w.shade_hit(comps, 5));
    }

    #[test]
    fn test_color_at_with_mutually_reflective_surfaces() {
        let light = PointLight::new(Point::new(0.0, 0.0, 0.0), color::WHITE);
        let mut w = World::new(light, vec![]);

        let mut lower = Plane::default();
        lower.material.reflective = 1.0;
        lower.transform = transforms::translation(0.0, -1.0, 0.0);

        let mut upper = Plane::default();
        upper.material.reflective = 1.0;
        upper.transform = transforms::translation(0.0, 1.0, 0.0);

        w.objects.push(Box::new(lower) as Box<Shape + Send + Sync>);
        w.objects.push(Box::new(upper) as Box<Shape + Send + Sync>);

        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        w.color_at(r, 5);
    }

    #[test]
    fn test_reflected_color_at_the_maximum_recurive_depth() {
        let mut w = World::default();
        let mut shape = Plane::default();
        shape.material.reflective = 0.5;
        shape.transform = transforms::translation(0.0, -1.0, 0.0);
        w.objects.push(Box::new(shape) as Box<Shape + Send + Sync>);

        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector3::new(0.0, -f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
        );
        let i = Intersection {
            time: f64::sqrt(2.0),
            object: w.objects[2].as_ref(),
        };
        let comps = i.prepare_computations(r);
        assert_eq!(color::BLACK, w.reflected_color(comps, 0));
    }
}
