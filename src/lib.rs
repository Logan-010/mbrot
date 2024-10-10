pub mod args;
pub mod colormaps;

const MIN_STEPS: u32 = 1 << 7;
const MAX_STEPS: u32 = 1 << 11;
const BAILOUT_NUM: f64 = 4.0;

pub struct MandelbrotConfig {
    pub min_steps: u32,
    pub max_steps: u32,
    pub bailout_num: f64,
}

impl Default for MandelbrotConfig {
    fn default() -> Self {
        Self {
            min_steps: MIN_STEPS,
            max_steps: MAX_STEPS,
            bailout_num: BAILOUT_NUM,
        }
    }
}

pub fn rand_range(min: f64, max: f64) -> f64 {
    let u = fastrand::f64();
    lerp(min, max, u)
}

pub fn lerp(a: f64, b: f64, u: f64) -> f64 {
    (a) * (1.0 - (u)) + (b) * (u)
}

pub fn mandelbrot(input: (f64, f64), cfg: &MandelbrotConfig) -> u32 {
    let mut r = input.0;
    let mut i = input.1;
    let mut mag_sq = r * r + i * i;
    let mut steps = 0;
    while steps < cfg.max_steps && mag_sq <= cfg.bailout_num {
        let rr = r * r - i * i + input.0;
        i = 2.0 * r * i + input.1;
        r = rr;
        mag_sq = r * r + i * i;
        steps += 1;
    }
    steps
}

pub fn choose_center(x: &mut f64, y: &mut f64, cfg: &MandelbrotConfig) -> u32 {
    let mut steps = 0;
    while !(cfg.min_steps..cfg.max_steps).contains(&steps) {
        *x = rand_range(-1.5, 1.0);
        *y = rand_range(0.0, 1.0);
        steps = mandelbrot((*x, *y), cfg);
    }
    steps
}
