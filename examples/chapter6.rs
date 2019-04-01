use ray_tracer::{
    color, Canvas, Color, MaterialBuilder, Point, PointLight, Ray, Shape, SphereBuilder,
};
use std::error;
use std::fmt::Display;

#[derive(Debug)]
struct Error {
    error: String,
}

impl<T: Display + error::Error> From<T> for Error {
    fn from(error: T) -> Self {
        Error {
            error: format!("Error: {}", error),
        }
    }
}

fn main() -> Result<(), Error> {
    let ray_origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 100;
    let pixel_size = wall_size / f64::from(canvas_pixels);
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let shape = SphereBuilder::default()
        .material(
            MaterialBuilder::default()
                .color(Color::new(1.0, 0.2, 1.0))
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let light_position = Point::new(-10.0, 10.0, -10.0);
    let light_color = color::WHITE;
    let light = PointLight::new(light_position, light_color);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * f64::from(y);

        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * f64::from(x);
            let position = Point::new(world_x, world_y, wall_z);
            let ray = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = shape.intersect(ray);
            if let Some(hit) = xs.hit() {
                let point = ray.position(hit.time);
                let normal = hit.object.normal_at(point);
                let eye = -ray.direction;
                let color = hit
                    .object
                    .material()
                    .lighting(hit.object, light, point, eye, normal, false);
                canvas.write_pixel(x, y, color);
            }
        }
    }

    print!("{}", canvas.to_ppm()?);

    Ok(())
}
