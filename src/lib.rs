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

pub fn mandelbrot(input: (f64, f64), cfg: &MandelbrotConfig) -> u32 {
    // c0 = x + iy
    let (c0_re, c0_im) = input;

    // c = c0
    let mut c_re = c0_re;
    let mut c_im = c0_im;

    // dc = 1 + 0i
    let mut dc_re = 1.0;
    let mut dc_im = 0.0;

    // dc_sum = 0 + 0i
    let mut sum_re = 0.0;
    let mut sum_im = 0.0;

    for n in 1..cfg.max_steps {
        // c = c^2 + c0
        let c_re2 = c_re * c_re - c_im * c_im;
        let c_im2 = 2.0 * c_re * c_im;
        c_re = c_re2 + c0_re;
        c_im = c_im2 + c0_im;

        // dc = 2 * dc * c + 1
        let new_dc_re = 2.0 * (dc_re * c_re - dc_im * c_im) + 1.0;
        let new_dc_im = 2.0 * (dc_re * c_im + dc_im * c_re);
        dc_re = new_dc_re;
        dc_im = new_dc_im;

        // dc_sum += dc
        sum_re += dc_re;
        sum_im += dc_im;

        // bailout: |dc_sum|^2 >= bailout_num
        let mag2 = sum_re * sum_re + sum_im * sum_im;
        if mag2 >= cfg.bailout_num {
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
