use std::{
    collections::BTreeMap,
    fs,
    sync::mpsc::{self, Sender},
    thread,
};

use anyhow::Result;
use clap::Parser;
use hexnumgen::{AStarOptions, Direction, GeneratedNumber, generate_number_pattern};
use rand::{rng, seq::SliceRandom};
use regex::Regex;
use serde::Serialize;

fn n_groups<T>(mut values: Vec<T>, n: usize) -> Vec<Vec<T>> {
    let mut groups = Vec::new();
    let len = values.len();

    for i in (0..n).rev() {
        groups.push(values.split_off(i * (len / n) + i.min(len % n)));
    }

    groups.reverse();
    groups
}

fn find_patterns(targets: Vec<i64>) -> BTreeMap<i64, String> {
    let mut data = BTreeMap::new();
    let re = Regex::new(r"^aqaa").unwrap();

    for (i, &target) in targets.iter().enumerate() {
        println!("{}/{}", i + 1, targets.len());

        let GeneratedNumber { pattern, .. } = generate_number_pattern(
            target.into(),
            false,
            false,
            hexnumgen::GeneratorOptions::AStar(AStarOptions { timeout: None }),
        )
        .unwrap();

        data.insert(target, re.replace(&pattern, "").to_string());
    }

    data
}

fn worker(targets: Vec<i64>, tx: Sender<BTreeMap<i64, String>>) {
    tx.send(find_patterns(targets)).unwrap();
}

fn write_pregen<T: Serialize>(max: u64, pretty: bool, data: T) -> Result<()> {
    Ok(fs::write(
        format!("numbers_{max}.json"),
        if pretty {
            serde_json::to_string_pretty(&data)
        } else {
            serde_json::to_string(&data)
        }?,
    )?)
}

#[derive(Parser)]
struct Cli {
    /// Largest number to generate a literal for
    max: u64,

    /// Number of threads to use
    threads: Option<usize>,

    /// Only generate the "tail" of the number literal (skip aqaa/dedd)
    #[arg(short, long, default_value_t = false)]
    only_tail: bool,

    /// If the output should be prettified
    #[arg(short, long, default_value_t = false)]
    pretty: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut all_targets = Vec::from_iter(0..=(cli.max as i64));
    let all_data = match cli.threads {
        Some(threads) => {
            all_targets.shuffle(&mut rng());

            let (tx, rx) = mpsc::channel();

            for targets in n_groups(all_targets, threads) {
                let tx = tx.clone();
                thread::spawn(move || worker(targets, tx));
            }
            drop(tx);

            let mut all_data = BTreeMap::new();
            while let Ok(data) = rx.recv() {
                all_data.extend(data);
            }
            all_data
        }
        None => find_patterns(all_targets),
    };

    if cli.only_tail {
        write_pregen(cli.max, cli.pretty, all_data)
    } else {
        let mut pos_neg_values: BTreeMap<i64, (String, String)> = BTreeMap::new();
        for (target, tail) in all_data {
            pos_neg_values.insert(
                target,
                (Direction::SouthEast.to_string(), format!("aqaa{tail}")),
            );
            if target != 0 {
                pos_neg_values.insert(
                    -target,
                    (Direction::NorthEast.to_string(), format!("dedd{tail}")),
                );
            }
        }
        write_pregen(cli.max, cli.pretty, pos_neg_values)
    }
}
