mod graphics;
mod math;

use std::fmt;

use graphics::Canvas;
use math::Tuple;

#[derive(Debug)]
struct Projectile {
    pos: Tuple,
    vel: Tuple,
}

struct Environment {
    grav: Tuple,
    wind: Tuple,
}

impl fmt::Display for Projectile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} {:.2} {:.2}", self.pos.0, self.pos.1, self.pos.2)
    }
}

fn main() {
    let mut canvas = Canvas::new(900, 550);
    let mut p = Projectile {
        pos: Tuple::point(0.0, 1.0, 0.0),
        vel: Tuple::vector(1.0, 1.8, 0.0).normal() * 11.25,
    };

    let env = Environment {
        grav: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };

    let color = Tuple::color(1.0, 1.0, 1.0);
    canvas.set_pixel(
        p.pos.x().round() as usize,
        canvas.height - (p.pos.y().round() as usize),
        &color,
    );
    loop {
        p.pos = p.pos + p.vel;
        p.vel = p.vel + env.grav + env.wind;
        if p.pos.1 <= 0.0 {
            break;
        }
        canvas.set_pixel(
            p.pos.x().round() as usize,
            canvas.height - (p.pos.y().round() as usize),
            &color,
        );
    }

    canvas.write_to_file("output.ppm").ok();
}
