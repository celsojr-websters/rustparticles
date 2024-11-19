use rand::Rng;

pub fn get_random_rgb() -> String {
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    let mut rng = rand::thread_rng();

    while r < 100 && g < 100 && b < 100 {
        r = rng.gen_range(0..256);
        g = rng.gen_range(0..256);
        b = rng.gen_range(0..256);
    }
    format!("rgb({},{},{})", r, g, b)
}
