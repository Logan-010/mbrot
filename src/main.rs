use clap::Parser;
use image::RgbImage;
use mbrot::{
    MandelbrotConfig,
    args::{COLORMAP_CHOICES, Cli},
    choose_center, lerp, mandelbrot, rand_range,
};
use rayon::{
    ThreadPoolBuilder, iter::IndexedParallelIterator, prelude::ParallelIterator,
    slice::ParallelSliceMut,
};
use std::time::Instant;

fn main() {
    let args = Cli::parse();

    let cfg = MandelbrotConfig {
        min_steps: args.step_limits[0],
        max_steps: args.step_limits[1],
        bailout_num: 1.0 * 10.0f64.powf(args.bailout_num),
    };

    let seed = args.rng_seed.unwrap_or_else(fastrand::get_seed);

    fastrand::seed(seed);

    let (width, height) = (args.dimensions[0], args.dimensions[1]);

    let palette = if let Some(ref colormap) = args.colormap {
        colormap.to_colormap()
    } else {
        let choice = fastrand::usize(0..COLORMAP_CHOICES.len() - 1);

        COLORMAP_CHOICES[choice].to_colormap()
    };

    fastrand::seed(seed);

    let (center, steps) = match args.image_center {
        Some(v) => {
            let point = (v[0], v[1]);
            let steps = mandelbrot(point, &cfg);
            (point, steps)
        }
        None => choose_center(&cfg),
    };

    fastrand::seed(seed);

    let (dx, dy) = if let Some(size) = args.view_size {
        (size[0] / 2.0, size[1] / 2.0)
    } else {
        let dx = (steps as f64).powf(rand_range(-2.5, -1.0));
        (dx, dx * height as f64 / width as f64)
    };

    let (xmin, xmax) = (center.0 - dx, center.0 + dx);
    let (ymin, ymax) = (center.1 - dy, center.1 + dy);

    if !args.no_info {
        println!("Starting generation with seed {}...", seed);
    }

    let mut image = RgbImage::new(width, height);

    let timer = Instant::now();

    if let Some(threads) = args.threads {
        let pool = ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .unwrap();

        pool.install(|| {
            image
                .par_chunks_mut(width as usize * 3)
                .enumerate()
                .for_each(|(y, row)| {
                    for x in 0..width {
                        let xf = x as f64 / (width as f64 - 1.0);
                        let yf = y as f64 / (height as f64 - 1.0);

                        let scaled = (lerp(xmin, xmax, xf), lerp(ymin, ymax, yf));

                        let iteration = mandelbrot(scaled, &cfg);
                        let idx = (3 * iteration as usize).clamp(0, palette.len() - 3);

                        let px = &palette[idx..idx + 3];

                        let offset = (x as usize) * 3;
                        row[offset..offset + 3].copy_from_slice(&[px[0], px[1], px[2]]);
                    }
                });
        });
    } else {
        image
            .par_chunks_mut(width as usize * 3)
            .enumerate()
            .for_each(|(y, row)| {
                for x in 0..width {
                    let xf = x as f64 / (width as f64 - 1.0);
                    let yf = y as f64 / (height as f64 - 1.0);

                    let scaled = (lerp(xmin, xmax, xf), lerp(ymin, ymax, yf));

                    let iteration = mandelbrot(scaled, &cfg);
                    let idx = (3 * iteration as usize).clamp(0, palette.len() - 3);

                    let px = &palette[idx..idx + 3];

                    let offset = (x as usize) * 3;
                    row[offset..offset + 3].copy_from_slice(&[px[0], px[1], px[2]]);
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
