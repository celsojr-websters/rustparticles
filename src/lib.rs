use std::sync::Mutex;
use wasm_bindgen::prelude::*;
#[macro_use]
extern crate lazy_static;

mod particle;
mod particles;
mod utils;

use particles::Particles;

lazy_static! {
    static ref PARTICLES: Mutex<Particles> = Mutex::new(Particles::new());
}

#[wasm_bindgen]
extern "C" {
    fn draw_particle(x: f64, y: f64, s: &str, size: i32);
}

#[wasm_bindgen]
pub fn create_particles(w: f64, h: f64, num: i32) {
    let mut particles = PARTICLES.lock().unwrap();
    particles.canvas_width = w;
    particles.canvas_height = h;
    particles.create(num);
}

#[wasm_bindgen]
pub fn render_particles() {
    let mut particles = PARTICLES.lock().unwrap();
    particles.draw();
    particles.update();
}

#[wasm_bindgen]
pub fn adjust_canvas_size(w: f64, h: f64) {
    let mut particles = PARTICLES.lock().unwrap();
    particles.adjust_to_canvas(w, h);
}
