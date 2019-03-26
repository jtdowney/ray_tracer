use failure::Error;
use ray_tracer::{Canvas, Color, Point, PointLight, Ray, Sphere};

fn main() -> Result<(), Error> {
    let ray_origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 100;
    let pixel_size = wall_size / canvas_pixels as f32;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let mut shape = Sphere::new();
    shape.material.color = Color::new(1.0, 0.2, 1.0);

    let light_position = Point::new(-10.0, 10.0, -10.0);
    let light_color = Color::new(1.0, 1.0, 1.0);
    let light = PointLight::new(light_position, light_color);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f32;

        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f32;
            let position = Point::new(world_x, world_y, wall_z);
            let ray = Ray::new(ray_origin, (position - ray_origin).normalize());

            if let Ok(xs) = ray.intersect(&shape) {
                if let Some(hit) = xs.hit() {
                    let point = ray.position(hit.time);
                    let normal = hit.object.normal_at(point).unwrap();
                    let eye = -ray.direction;
                    let color = hit.object.material.lighting(light, point, eye, normal);
                    canvas.write_pixel(x, y, color);
                }
            }
        }
    }

    print!("{}", canvas.to_ppm()?);

    Ok(())
}
