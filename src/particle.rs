use rand::Rng;

#[derive(Debug)] // Optional: For easier debugging
pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub color: String,
}

impl Particle {
    pub fn new(x_starting_range: f64, y_starting_range: f64) -> Particle {
        let mut rng = rand::thread_rng();

        Particle {
            x: x_starting_range * rng.gen::<f64>(),
            y: y_starting_range * rng.gen::<f64>(),
            vx: 4.0 * rng.gen::<f64>() - 2.0,
            vy: 4.0 * rng.gen::<f64>() - 2.0,
            color: crate::utils::get_random_rgb(),
        }
    }
}
