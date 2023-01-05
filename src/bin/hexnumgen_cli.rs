use clap::Parser;
use hexnumgen::{generate_number_pattern_astar, generate_number_pattern_beam, Bounds, GeneratedNumber};

#[derive(Parser)]
struct Cli {
    /// Target number to generate a literal for
    target: i32,
    /// Whether to make the target negative
    #[arg(short, long)]
    negative: bool,
    /// Whether to use the A* algorithm instead of beam search
    #[arg(short, long)]
    astar: bool,
    /// Maximum size of the generated pattern (overridden by q_size, r_size, and/or s_size if set)
    #[arg(long, default_value_t = 8)]
    size: u32,
    /// Maximum size of the generated pattern in the q direction (northeast/southwest)
    #[arg(short, long)]
    q_size: Option<u32>,
    /// Maximum size of the generated pattern in the r direction (north/south)
    #[arg(short, long)]
    r_size: Option<u32>,
    /// Maximum size of the generated pattern in the s direction (northwest/southeast)
    #[arg(short, long)]
    s_size: Option<u32>,
    /// Number of possible paths kept between steps
    #[arg(short, long, default_value_t = 25)]
    carryover: usize,
    /// Whether generated paths larger than the target value should be kept or discarded (generates slower but may give better results)
    #[arg(short, long)]
    keep_larger: bool,
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    let target = if cli.negative { -cli.target } else { cli.target };
    let bounds =
        Bounds::new(cli.q_size.unwrap_or(cli.size), cli.r_size.unwrap_or(cli.size), cli.s_size.unwrap_or(cli.size));

    let GeneratedNumber { direction, pattern, .. } = if cli.astar {
        generate_number_pattern_astar(target, bounds, !cli.keep_larger)
    } else {
        generate_number_pattern_beam(target, bounds, cli.carryover, !cli.keep_larger)
    }
    .ok_or_else(|| format!("No pattern found for {target}"))?;

    println!("{direction} {pattern}");
    Ok(())
}
