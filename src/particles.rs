use crate::particle::Particle;

pub struct Particles {
    pub particles: Vec<Particle>,
    pub canvas_width: f64,
    pub canvas_height: f64,
}

impl Particles {
    pub fn new() -> Self {
        Particles {
            particles: Vec::new(),
            canvas_width: 0.0,
            canvas_height: 0.0,
        }
    }

    pub fn create(&mut self, num: i32) {
        for _ in 0..num {
            self.particles.push(Particle::new(self.canvas_width, self.canvas_height));
        }
    }

    pub fn draw(&self) {
        for p in self.particles.iter() {
            crate::draw_particle(p.x, p.y, &p.color, 2);
        }
    }

    pub fn update(&mut self) {
        for p in self.particles.iter_mut() {
            p.x += p.vx;
            p.y += p.vy;

            if p.x < 0.0 || p.x > self.canvas_width {
                p.vx = -p.vx;
            }

            if p.y < 0.0 || p.y > self.canvas_height {
                p.vy = -p.vy;
            }
        }
    }

    pub fn adjust_to_canvas(&mut self, new_width: f64, new_height: f64) {
        self.canvas_width = new_width;
        self.canvas_height = new_height;

        for p in self.particles.iter_mut() {
            // Keep particles within the new boundaries
            if p.x > new_width {
                p.x = new_width;
            }
            if p.y > new_height {
                p.y = new_height;
            }

            // Adjust velocities to ensure they stay consistent
            if p.vx.abs() > new_width / 50.0 {
                p.vx = 4.0 * rand::random::<f64>() - 2.0;
            }
            if p.vy.abs() > new_height / 50.0 {
                p.vy = 4.0 * rand::random::<f64>() - 2.0;
            }
        }
    }
}
