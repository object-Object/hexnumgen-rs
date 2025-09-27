use std::str::FromStr;

use anyhow::Error;
use clap::Parser;
use hexnumgen::{generate_number_pattern, Bounds, GeneratedNumber, GeneratorOptions};
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

    /// Whether generated paths larger than the target value should be kept or discarded (paths generate slower but may be more compact)
    #[arg(short, long)]
    keep_larger: bool,

    /// If fractional targets and intermediate values should be allowed
    #[arg(short, long, default_value_t = false)]
    fractions: bool,

    #[command(subcommand)]
    options: GeneratorOptions,
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    let target = if cli.negative {
        -cli.target.0
    } else {
        cli.target.0
    };
    if !cli.fractions && !target.is_integer() {
        return Err("Tried to generate non-integer number without enabling fractions".into());
    }

    let GeneratedNumber {
        direction,
        pattern,
        bounds,
        num_points,
        num_segments,
    } = generate_number_pattern(target, !cli.keep_larger, cli.fractions, cli.options)
        .ok_or_else(|| format!("No pattern found for {target}"))?;

    let Bounds { q, r, s } = bounds;
    println!(
        "{direction} {pattern}
    Points: {num_points}
  Segments: {num_segments}
    Bounds: {q}/{r}/{s}
Quasi-area: {}",
        bounds.quasi_area()
    );
    Ok(())
}
