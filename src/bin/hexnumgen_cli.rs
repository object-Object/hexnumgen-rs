use std::str::FromStr;

use anyhow::Error;
use clap::Parser;
use hexnumgen::{generate_number_pattern_astar, generate_number_pattern_beam, Bounds, GeneratedNumber};
use num_rational::Ratio;

#[derive(Clone)]
struct ParsedRatio(Ratio<i64>);

impl ParsedRatio {
    fn new(numer: i64, denom: i64) -> Self {
        Self(Ratio::new(numer, denom))
    }
}

impl From<Ratio<i64>> for ParsedRatio {
    fn from(value: Ratio<i64>) -> Self {
        Self(value)
    }
}

impl FromStr for ParsedRatio {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.split_once('.') {
            // parse decimal, eg. 1.25 -> (125, 100)
            Some((numer, decimal)) => {
                let numer: i64 = numer.parse()?;
                let scale = 10_i64.pow(decimal.len() as u32);
                let decimal: i64 = decimal.parse()?;

                Self::new(numer * scale + decimal, scale)
            }

            // fall back to default Ratio parser
            None => Self(Ratio::from_str(s)?),
        })
    }
}

#[derive(Parser)]
struct Cli {
    /// Target number to generate a literal for
    target: ParsedRatio,

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

    /// If fractional targets and intermediate values should be allowed
    #[arg(short, long, default_value_t = false)]
    fractions: bool,
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    let target = if cli.negative { -cli.target.0 } else { cli.target.0 };
    if !cli.fractions && !target.is_integer() {
        return Err("Tried to generate non-integer number without enabling fractions".into());
    }

    let bounds =
        Bounds::new(cli.q_size.unwrap_or(cli.size), cli.r_size.unwrap_or(cli.size), cli.s_size.unwrap_or(cli.size));

    let GeneratedNumber { direction, pattern, .. } = if cli.astar {
        generate_number_pattern_astar(target, !cli.keep_larger, cli.fractions)
    } else {
        generate_number_pattern_beam(target, bounds, cli.carryover, !cli.keep_larger, cli.fractions)
    }
    .ok_or_else(|| format!("No pattern found for {target}"))?;

    println!("{direction} {pattern}");
    Ok(())
}
