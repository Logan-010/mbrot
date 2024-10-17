use num::complex::Complex64;

pub mod args;
pub mod colormaps;

const MIN_STEPS: u32 = 150;
const MAX_STEPS: u32 = 1024;
const BAILOUT_NUM: f64 = 15.0;

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
            bailout_num: 1.0 * 10.0f64.powf(BAILOUT_NUM),
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

fn abs_square(input: Complex64) -> f64 {
    input.re.powf(2.0) + input.im.powf(2.0)
}

pub fn mandelbrot(input: (f64, f64), cfg: &MandelbrotConfig) -> u32 {
    let c0 = Complex64::new(input.0, input.1);
    let mut c = c0;
    let mut dc = Complex64::new(1.0, 0.0);
    let mut dc_sum = Complex64::new(0.0, 0.0);

    for n in 1..cfg.max_steps {
        c = c.powf(2.0) + c0;
        dc = 2.0 * dc * c + 1.0;
        dc_sum += dc;

        if abs_square(dc_sum) >= cfg.bailout_num {
            return n;
        }
    }

    0
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
