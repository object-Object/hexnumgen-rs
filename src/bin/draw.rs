use anyhow::{Context, Result};
use clap::Parser;
use hexnumgen::{pattern_to_points, Direction, PatternPlotter};
use itertools::Itertools;
use tiny_skia::Color;
use tiny_skia_path::Transform;

#[derive(Parser)]
struct Cli {
    /// Starting direction
    direction: Direction,

    /// Angle signature
    #[arg(default_value_t = String::new())]
    pattern: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let pixel_size = 40.;
    let line_width = 4.;
    let point_width = 12.;

    // this is nasty
    let points = pattern_to_points(cli.direction, &cli.pattern)?;
    let pixels = points.iter().map(|p| p.pixel(pixel_size)).collect_vec();
    let (min_x, max_x) = pixels.iter().map(|p| p.0).minmax().into_option().unwrap();
    let (min_y, max_y) = pixels.iter().map(|p| p.1).minmax().into_option().unwrap();

    let width = max_x - min_x;
    let height = max_y - min_y;

    let margin = point_width / 2.;

    // haha pp
    let mut pp = PatternPlotter::new(
        (width + 2. * margin).ceil() as u32,
        (height + 2. * margin).ceil() as u32,
    )
    .context("Failed to create PatternPlotter".to_string())?;

    let transform = Transform::from_translate(-min_x + margin, -min_y + margin);
    pp.plot_monochrome_line(
        &points,
        pixel_size,
        line_width,
        Color::from_rgba8(168, 30, 227, 255),
        Some(transform),
    )
    .context("Failed to plot line")?;
    pp.plot_monochrome_points(
        &points,
        pixel_size,
        point_width,
        Color::WHITE,
        Some(transform),
    )
    .context("Failed to plot points")?;

    Ok(pp.save_png("out.png")?)
}
