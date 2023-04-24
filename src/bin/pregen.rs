use clap::Parser;
use hexnumgen::{generate_number_pattern, AStarOptions, Direction, GeneratedNumber};

use rand::{seq::SliceRandom, thread_rng};
use regex::Regex;
use std::{
    collections::BTreeMap,
    fs,
    sync::mpsc::{self, Sender},
    thread,
};

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

        let GeneratedNumber { pattern, .. } =
            generate_number_pattern(target.into(), false, false, hexnumgen::GeneratorOptions::AStar(AStarOptions {}))
                .unwrap();

        data.insert(target, re.replace(&pattern, "").to_string());
    }

    data
}

fn worker(targets: Vec<i64>, tx: Sender<BTreeMap<i64, String>>) {
    tx.send(find_patterns(targets)).unwrap();
}

#[derive(Parser)]
#[command(author = "Gamma Delta", version = "1", about = "generates hexcasting number literals")]
struct Cli {
    /// Largest number to generate a literal for.
    max: u64,

    /// Only generate the "tail" of the number literal (skip aqaa/dedd).
    #[arg(short, long)]
    only_tail: bool,

    /// Number of threads to use.
    threads: Option<usize>,
}

fn main() {
    let cli = Cli::parse();

    let mut all_targets = Vec::from_iter(0..=(cli.max as i64));
    let all_data = match cli.threads {
        Some(threads) => {
            all_targets.shuffle(&mut thread_rng());

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

    fs::write(
        format!("numers_{}.json", cli.max),
        if cli.only_tail {
            serde_json::to_string_pretty(&all_data).unwrap()
        } else {
            let mut pos_neg_values: BTreeMap<i64, (String, String)> = BTreeMap::new();
            for (k, v) in all_data {
                if k == 0 {
                    continue;
                }
                pos_neg_values.insert(k, (Direction::SouthEast.to_string(), format!("aqaa{v}")));
                pos_neg_values.insert(-k, (Direction::NorthEast.to_string(), format!("dedd{v}")));
            }
            serde_json::to_string_pretty(&pos_neg_values).unwrap()
        },
    )
    .unwrap()
}
