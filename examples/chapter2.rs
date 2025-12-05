use anyhow::Result;
use ray_tracer::{Point, Vector, canvas, color, point, vector};

struct Projectile {
    position: Point,
    velocity: Vector,
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;
    Projectile { position, velocity }
}

fn main() -> Result<()> {
    let start = point(0, 1, 0);
    let velocity = vector(1, 1.8, 0).normalize() * 11.25;
    let mut projectile = Projectile {
        position: start,
        velocity,
    };

    let environment = Environment {
        gravity: vector(0, -0.1, 0),
        wind: vector(-0.01, 0, 0),
    };

    let mut c = canvas(900, 550);
    let red = color(1, 0, 0);

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    while projectile.position.y > 0.0 {
        let x = projectile.position.x.round() as usize;
        let y = c
            .height
            .saturating_sub(projectile.position.y.round() as usize);

        if x < c.width && y < c.height {
            c.write_pixel(x, y, red)?;
        }

        projectile = tick(&environment, &projectile);
    }

    let ppm = c.to_ppm()?;
    print!("{ppm}");

    Ok(())
}
