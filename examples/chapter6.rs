use ray_tracer::{color, hit, point, point_light, ray, sphere, Canvas, WHITE};

fn main() -> anyhow::Result<()> {
    let ray_origin = point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 100;
    let pixel_size = wall_size / f64::from(canvas_pixels as u16);
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let mut shape = sphere();
    shape.material.color = color(1.0, 0.2, 1.0);

    let light_position = point(-10.0, 10.0, -10.0);
    let light_color = WHITE;
    let light = point_light(light_position, light_color);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * f64::from(y as u16);

        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * f64::from(x as u16);
            let position = point(world_x, world_y, wall_z);
            let ray = ray(ray_origin, (position - ray_origin).normalize());
            let xs = shape.intersect(ray);
            if let Some(hit) = hit(xs) {
                let point = ray.position(hit.time);
                let normal = hit.object.normal_at(point);
                let eye = -ray.direction;
                let color = hit
                    .object
                    .material
                    .lighting(light, point, eye, normal, false);
                canvas.write_pixel(x, y, color)?;
            }
        }
    }

    print!("{}", canvas.to_ppm()?);

    Ok(())
}
