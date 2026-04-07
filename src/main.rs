use clap::Parser;
use image::RgbaImage;
use mbrot::{
    args::{Cli, COLORMAP_CHOICES},
    choose_center, lerp, mandelbrot, rand_range, MandelbrotConfig,
};
use rayon::{
    iter::IndexedParallelIterator, prelude::ParallelIterator, slice::ParallelSliceMut,
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

    let seed = args.rng_seed.unwrap_or_else(|| fastrand::get_seed());

    fastrand::seed(seed);

    let (width, height) = (args.dimensions[0], args.dimensions[1]);

    let palette = if let Some(ref colormap) = args.colormap {
        colormap.to_colormap()
    } else {
        let choice = fastrand::usize(0..COLORMAP_CHOICES.len() - 1);

        COLORMAP_CHOICES[choice].to_colormap()
    };

    let steps;

    fastrand::seed(seed);

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

    fastrand::seed(seed);

    let dx;
    let dy;

    if let Some(size) = args.view_size {
        dx = size[0] / 2.0;
        dy = size[1] / 2.0;
    } else {
        dx = (steps as f64).powf(rand_range(-2.5, -1.0));
        dy = dx * height as f64 / width as f64;
    }

    let (xmin, xmax) = (center.0 - dx, center.0 + dx);
    let (ymin, ymax) = (center.1 - dy, center.1 + dy);

    if !args.no_info {
        println!("Starting generation with seed {}...", seed);
    }

    let mut image = RgbaImage::new(width, height);

    let timer = Instant::now();

    if let Some(threads) = args.threads {
        let pool = ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .unwrap();

        pool.install(|| {
            image
                .par_chunks_mut(width as usize * 4)
                .enumerate()
                .for_each(|(y, row)| {
                    for x in 0..width {
                        let xf = x as f64 / (width as f64 - 1.0);
                        let yf = y as f64 / (height as f64 - 1.0);

                        let scaled = (lerp(xmin, xmax, xf), lerp(ymin, ymax, yf));

                        let iteration = mandelbrot(scaled, &cfg);
                        let idx = (3 * iteration as usize).clamp(0, palette.len() - 3);

                        let px = &palette[idx..idx + 3];

                        let offset = (x as usize) * 4;
                        row[offset..offset + 4].copy_from_slice(&[px[0], px[1], px[2], 0xFF]);
                    }
                });
        });
    } else {
        image
            .par_chunks_mut(width as usize * 4)
            .enumerate()
            .for_each(|(y, row)| {
                for x in 0..width {
                    let xf = x as f64 / (width as f64 - 1.0);
                    let yf = y as f64 / (height as f64 - 1.0);

                    let scaled = (lerp(xmin, xmax, xf), lerp(ymin, ymax, yf));

                    let iteration = mandelbrot(scaled, &cfg);
                    let idx = (3 * iteration as usize).clamp(0, palette.len() - 3);

                    let px = &palette[idx..idx + 3];

                    let offset = (x as usize) * 4;
                    row[offset..offset + 4].copy_from_slice(&[px[0], px[1], px[2], 0xFF]);
                }
            });
    }

    if !args.no_info {
        println!(
            "Finished generation in {:?}!\nSaving image...",
            timer.elapsed()
        );
    }

    image.save(args.file_name).expect("Failed to save image");

    if !args.no_info {
        println!("Done!");
    }
}
