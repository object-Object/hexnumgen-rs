use anyhow::{Context, Result};
use clap::Parser;
use hexnumgen::{pattern_to_points, Direction, PatternPlotter};
use itertools::Itertools;
use tiny_skia::{Color, Transform};

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
    let pixels = points.iter().map(|p| p.pixel()).collect_vec();
    let (min_x, max_x) = pixels.iter().map(|p| p.0).minmax().into_option().unwrap();
    let (min_y, max_y) = pixels.iter().map(|p| p.1).minmax().into_option().unwrap();

    let width = max_x - min_x;
    let height = max_y - min_y;

    let mut scale = 400. / width;
    if scale * height > 400. {
        scale = 400. / height;
    }

    let line_width = 0.05;
    let point_width = 2. * line_width;
    let margin = point_width / 2.;

    let transform = Transform::from_scale(scale, scale).pre_translate(-min_x + margin, -min_y + margin);
    let plot_width = (width * scale) as u32;
    let plot_height = (height * scale) as u32;

    // haha pp
    let mut pp = PatternPlotter::new(
        plot_width + (2. * margin * scale).ceil() as u32,
        plot_height + (2. * margin * scale).ceil() as u32,
        Some(scale),
    )
    .context(format!("Failed to create PatternPlotter"))?;

    pp.plot_monochrome_line(&points, line_width, Color::from_rgba8(168, 30, 227, 255), Some(transform))
        .context("Failed to plot line")?;
    pp.plot_monochrome_points(&points, point_width, Color::WHITE, Some(transform)).context("Failed to plot points")?;

    Ok(pp.save_png("out.png")?)
}
