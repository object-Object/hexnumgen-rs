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

    // this is nasty
    let points = pattern_to_points(cli.direction, &cli.pattern)?;
    let pixels = points.iter().map(|p| p.pixel(20.)).collect_vec();
    let (min_x, max_x) = pixels.iter().map(|p| p.0).minmax().into_option().unwrap();
    let (min_y, max_y) = pixels.iter().map(|p| p.1).minmax().into_option().unwrap();

    let width = max_x - min_x;
    let height = max_y - min_y;

    let line_width = 3.;
    let point_width = 7.;
    let margin = point_width / 2.;

    // haha pp
    let mut pp = PatternPlotter::new((width + 2. * margin).ceil() as u32, (height + 2. * margin).ceil() as u32)
        .context(format!("Failed to create PatternPlotter"))?;

    let transform = Transform::from_translate(-min_x + margin, -min_y + margin);
    pp.plot_monochrome_line(&points, line_width, Color::from_rgba8(168, 30, 227, 255), Some(transform))
        .context("Failed to plot line")?;
    pp.plot_monochrome_points(&points, point_width, Color::WHITE, Some(transform)).context("Failed to plot points")?;

    Ok(pp.save_png("out.png")?)
}
