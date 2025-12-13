use ray_tracer::{Point, Vector, point, vector};

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

fn main() {
    let mut projectile = Projectile {
        position: point(0, 1, 0),
        velocity: vector(1, 1, 0).normalize(),
    };

    let environment = Environment {
        gravity: vector(0, -0.1, 0),
        wind: vector(-0.01, 0, 0),
    };

    let mut ticks = 0;
    while projectile.position.y() > 0.0 {
        println!(
            "Tick {}: position ({:.4}, {:.4}, {:.4})",
            ticks, projectile.position.x(), projectile.position.y(), projectile.position.z()
        );
        projectile = tick(&environment, &projectile);
        ticks += 1;
    }

    println!(
        "Projectile hit the ground after {} ticks at x = {:.4}",
        ticks, projectile.position.x()
    );
}
