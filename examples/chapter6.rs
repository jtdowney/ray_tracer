use anyhow::Result;
use ray_tracer::{Material, canvas, color, hit, point, point_light, ray, shape::sphere};

fn main() -> Result<()> {
    let ray_origin = point(0, 0, -5);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels: usize = 100;

    #[allow(clippy::cast_precision_loss)]
    let pixel_size = wall_size / canvas_pixels as f32;
    let half = wall_size / 2.0;

    let mut c = canvas(canvas_pixels, canvas_pixels);

    let shape = sphere()
        .material(Material::builder().color(color(1, 0.2, 1)))
        .build();

    let light = point_light(point(-10, 10, -10), color(1, 1, 1));

    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_precision_loss
    )]
    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f32;

        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f32;
            let position = point(world_x, world_y, wall_z);
            let direction = (position - ray_origin).normalize();
            let r = ray(ray_origin, direction);
            let xs = shape.intersect(r);

            if let Some(intersection) = hit(xs) {
                let hit_point = r.position(intersection.time);
                let normal = intersection.object.normal_at(hit_point);
                let eye = -r.direction;
                let material = intersection.object.material();
                let pixel_color =
                    material.lighting(&intersection.object, &light, hit_point, eye, normal, false);
                c.write_pixel(x, y, pixel_color)?;
            }
        }
    }

    let ppm = c.to_ppm()?;
    print!("{ppm}");

    Ok(())
}
