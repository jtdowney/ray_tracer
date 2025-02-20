use std::f64::consts::PI;

use ray_tracer::{
    WHITE, camera, color, plane, point, point_light, sphere,
    transform::{scaling, translation, view_transform},
    vector, world,
};

fn main() -> anyhow::Result<()> {
    let mut world = world();

    let floor = plane();
    world.objects.push(floor);

    let mut middle = sphere();
    middle.transform = translation(-0.5, 1.0, 0.5);
    middle.material.color = color(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    world.objects.push(middle);

    let mut right = sphere();
    right.transform = translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5);
    right.material.color = color(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;
    world.objects.push(right);

    let mut left = sphere();
    left.transform = translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33);
    left.material.color = color(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;
    world.objects.push(left);

    world.light = Some(point_light(point(-10, 10, -10), WHITE));

    let width = 1024;
    let height = 768;
    let mut camera = camera(width, height, PI / 3.0);
    camera.transform = view_transform(
        point(0.0, 1.5, -5.0),
        point(0.0, 1.0, 0.0),
        vector(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(world)?;
    print!("{}", canvas.to_ppm()?);

    Ok(())
}
