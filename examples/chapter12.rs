use std::f64::consts::PI;

use ray_tracer::{
    camera, checkers_pattern, color, cube, point, point_light, sphere,
    transform::{rotation_y, scaling, translation, view_transform},
    vector, world, BLACK, WHITE,
};

fn main() -> anyhow::Result<()> {
    let mut world = world();
    world.light = Some(point_light(point(-10.0, 10.0, -10.0), WHITE));

    let mut room = cube();
    room.transform = rotation_y(PI / 3.5) * translation(0, 12, 0) * scaling(15, 12, 15);
    room.material.pattern = Some({
        let mut pattern = checkers_pattern(WHITE, BLACK);
        pattern.transform = scaling(1.0 / 15.0, 1.0 / 12.0, 1.0 / 15.0);
        pattern
    });
    room.material.specular = 0.0;
    world.objects.push(room);

    let mut back = sphere();
    back.transform = translation(0.7, 0.75, 3.5) * scaling(0.75, 0.75, 0.75);
    back.material.color = color(0.5, 0.1, 0.2);
    back.material.diffuse = 0.2;
    back.material.specular = 0.2;
    back.material.reflective = 0.9;
    back.material.shininess = 400.0;
    world.objects.push(back);

    let mut back_cube = cube();
    back_cube.transform =
        translation(-2.7, 0.5, 3.0) * scaling(0.5, 0.5, 0.5) * rotation_y(PI / 5.0);
    back_cube.material.color = color(0.1, 0.1, 0.6);
    back_cube.material.diffuse = 0.6;
    back_cube.material.specular = 0.4;
    back_cube.material.reflective = 0.2;
    back_cube.material.shininess = 200.0;
    world.objects.push(back_cube);

    let mut right = sphere();
    right.transform = translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5);
    right.material.color = color(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;
    world.objects.push(right);

    let mut middle = sphere();
    middle.transform = translation(-0.5, 1.0, 0.5);
    middle.material.color = WHITE;
    middle.material.diffuse = 0.3;
    middle.material.ambient = 0.2;
    middle.material.specular = 0.2;
    middle.material.transparency = 0.3;
    middle.material.reflective = 0.4;
    middle.material.shininess = 300.0;
    world.objects.push(middle);

    let mut left = sphere();
    left.transform = translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33);
    left.material.color = color(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;
    world.objects.push(left);

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
