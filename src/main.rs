use clap::Parser;
use image::{ImageFormat, Rgba, RgbaImage};
use mbrot::{
    args::{Cli, COLORMAP_CHOICES},
    choose_center, lerp, mandelbrot, rand_range, MandelbrotConfig,
};
use rayon::{
    prelude::{IntoParallelIterator, ParallelIterator},
    ThreadPoolBuilder,
};
use std::time::Instant;

fn main() {
    let args = Cli::parse();

    let cfg = MandelbrotConfig {
        min_steps: args.step_limits[0],
        max_steps: args.step_limits[1],
        bailout_num: 1.0 * 10.0f64.powf(args.bailout_num),
    };

    if let Some(seed) = args.rng_seed {
        fastrand::seed(seed)
    }

    let seed = fastrand::get_seed();

    let (width, height) = (args.dimensions[0], args.dimensions[1]);

    let palette = if let Some(ref colormap) = args.colormap {
        colormap.to_colormap()
    } else {
        let choice = fastrand::usize(0..COLORMAP_CHOICES.len() - 1);

        COLORMAP_CHOICES[choice].to_colormap()
    };

    let steps;

    if let Some(seed) = args.rng_seed {
        fastrand::seed(seed)
    }

    let center = match args.image_center {
        Some(v) => {
            let point = (v[0], v[1]);
            steps = mandelbrot(point, &cfg);
            point
        }
        None => {
            let (mut x, mut y) = (0.0, 0.0);
            steps = choose_center(&mut x, &mut y, &cfg);
            (x, y)
        }
    };

    if let Some(seed) = args.rng_seed {
        fastrand::seed(seed)
    }

    let dx;
    let dy;

    if let Some(size) = args.view_size {
        dx = (size[0] / 2) as f64;
        dy = (size[1] / 2) as f64;
    } else {
        dx = (steps as f64).powf(rand_range(-2.5, -1.0));
        dy = dx * height as f64 / width as f64;
    }

    let (xmin, xmax) = (center.0 - dx, center.0 + dx);
    let (ymin, ymax) = (center.1 - dy, center.1 + dy);

    let points = || {
        (0..height)
            .into_par_iter()
            .flat_map(|y| {
                let mut temp = Vec::new();

                for x in 0..width {
                    temp.push((x as f64, y as f64));
                }

                temp
            })
            .map(|point| {
                let scaled = (
                    lerp(xmin, xmax, point.0 / (width as f64 - 1.0)),
                    lerp(ymin, ymax, point.1 / (height as f64 - 1.0)),
                );

                let iteration = mandelbrot(scaled, &cfg);

                let index = (3 * iteration as usize).clamp(0, palette.len() - 3);

                let sample = &palette[index..];

                (point, Rgba::from([sample[0], sample[1], sample[2], 0xFF]))
            })
            .collect::<Vec<((f64, f64), Rgba<u8>)>>()
    };

    let mut image = RgbaImage::new(width, height);

    if !args.no_info {
        println!("Starting generation with seed {}...", seed);
    }

    let timer = Instant::now();

    if let Some(threads) = args.threads {
        let pool = ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .unwrap();

        pool.install(|| {
            for (point, color) in points() {
                image.put_pixel(point.0 as u32, point.1 as u32, color);
            }
        });
    } else {
        for (point, color) in points() {
            image.put_pixel(point.0 as u32, point.1 as u32, color);
        }
    }

    if !args.no_info {
        println!(
            "Finished generation in {:?}!\nSaving image...",
            timer.elapsed()
        );
    }

    image
        .save_with_format(args.file_name, ImageFormat::Png)
        .unwrap();

    if !args.no_info {
        println!("Done!");
    }
}
