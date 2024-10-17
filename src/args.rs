use clap::{Parser, ValueEnum};

use crate::colormaps;

#[derive(Parser)]
#[command(name = "mbrot")]
#[command(version = "1.0")]
#[command(author = "seedse")]
#[command(about = "A program to generate random and customizable mandelbrot fractals.")]
pub struct Cli {
    #[arg(long, short, num_args = 2, default_values_t = [1920, 1080])]
    pub dimensions: Vec<u32>,

    #[arg(long, short, num_args = 2)]
    pub image_center: Option<Vec<f64>>,

    #[arg(long, short, num_args = 2)]
    pub view_size: Option<Vec<u32>>,

    #[arg(long, short, num_args = 2, default_values_t = [super::MIN_STEPS, super::MAX_STEPS])]
    pub step_limits: Vec<u32>,

    #[arg(long, short)]
    pub colormap: Option<Colormap>,

    #[arg(long, short)]
    pub rng_seed: Option<u64>,

    #[arg(long, short)]
    pub threads: Option<usize>,

    #[arg(long, short, default_value_t = super::BAILOUT_NUM)]
    pub bailout_num: f64,

    #[arg(long, short, default_value = "out.png")]
    pub file_name: String,

    #[arg(long, short)]
    pub no_info: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Colormap {
    Acton,
    Bamako,
    Batlow,
    Berlin,
    Bilbao,
    Broc,
    Broco,
    Buda,
    Cork,
    Corko,
    Davos,
    Devon,
    Grayc,
    Hawaii,
    Imola,
    Lajolla,
    Lapaz,
    Lisbon,
    Nuuk,
    Oleron,
    Oslo,
    Roma,
    Romao,
    Tofino,
    Tokyo,
    Turku,
    Vik,
    Viko,
}

pub const COLORMAP_CHOICES: &[Colormap] = &[
    Colormap::Acton,
    Colormap::Bamako,
    Colormap::Batlow,
    Colormap::Berlin,
    Colormap::Bilbao,
    Colormap::Broc,
    Colormap::Broco,
    Colormap::Buda,
    Colormap::Cork,
    Colormap::Corko,
    Colormap::Davos,
    Colormap::Devon,
    Colormap::Grayc,
    Colormap::Hawaii,
    Colormap::Imola,
    Colormap::Lajolla,
    Colormap::Lapaz,
    Colormap::Lisbon,
    Colormap::Nuuk,
    Colormap::Oleron,
    Colormap::Oslo,
    Colormap::Roma,
    Colormap::Romao,
    Colormap::Tofino,
    Colormap::Tokyo,
    Colormap::Turku,
    Colormap::Vik,
    Colormap::Viko,
];

impl Colormap {
    pub fn to_colormap(&self) -> &[u8] {
        match self {
            Colormap::Acton => colormaps::ACTON,
            Colormap::Bamako => colormaps::BAMAKO,
            Colormap::Batlow => colormaps::BATLOW,
            Colormap::Berlin => colormaps::BERLIN,
            Colormap::Bilbao => colormaps::BILBAO,
            Colormap::Broc => colormaps::BROC,
            Colormap::Broco => colormaps::BROCO,
            Colormap::Buda => colormaps::BUDA,
            Colormap::Cork => colormaps::CORK,
            Colormap::Corko => colormaps::CORKO,
            Colormap::Davos => colormaps::DAVOS,
            Colormap::Devon => colormaps::DEVON,
            Colormap::Grayc => colormaps::GRAYC,
            Colormap::Hawaii => colormaps::HAWAII,
            Colormap::Imola => colormaps::IMOLA,
            Colormap::Lajolla => colormaps::LAJOLLA,
            Colormap::Lapaz => colormaps::LAPAZ,
            Colormap::Lisbon => colormaps::LISBON,
            Colormap::Nuuk => colormaps::NUUK,
            Colormap::Oleron => colormaps::OLERON,
            Colormap::Oslo => colormaps::OSLO,
            Colormap::Roma => colormaps::ROMA,
            Colormap::Romao => colormaps::ROMAO,
            Colormap::Tofino => colormaps::TOFINO,
            Colormap::Tokyo => colormaps::TOKYO,
            Colormap::Turku => colormaps::TURKU,
            Colormap::Vik => colormaps::VIK,
            Colormap::Viko => colormaps::VIKO,
        }
    }
}
