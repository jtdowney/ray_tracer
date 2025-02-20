use ray_tracer::{Canvas, Point, Vector, color, point, vector};

#[derive(Copy, Clone)]
struct Projectile {
    position: Point,
    velocity: Vector,
}

struct Simulation {
    gravity: Vector,
    wind: Vector,
    projectile: Projectile,
}

impl Simulation {
    fn tick(&mut self) {
        self.projectile.position = self.projectile.position + self.projectile.velocity;
        self.projectile.velocity = self.projectile.velocity + self.gravity + self.wind;
    }

    fn is_running(&self) -> bool {
        self.projectile.position.y >= 0.0
    }
}

fn main() -> anyhow::Result<()> {
    let projectile = Projectile {
        position: point(0.0, 1.0, 0.0),
        velocity: vector(1.0, 1.8, 0.0).normalize() * 11.25,
    };

    let mut simulation = Simulation {
        gravity: vector(0.0, -0.1, 0.0),
        wind: vector(-0.01, 0.0, 0.0),
        projectile,
    };

    let mut canvas = Canvas::new(900, 550);
    let c = color(0.8, 0.2, 0.1);

    while simulation.is_running() {
        simulation.tick();

        let x = simulation.projectile.position.x.round() as u16;
        let y = (f64::from(canvas.height) - simulation.projectile.position.y).round() as u16;

        if x > canvas.width || y > canvas.height {
            continue;
        }

        canvas.write_pixel(x, y, c)?;
    }

    let data = canvas.to_ppm()?;
    print!("{}", data);

    Ok(())
}
