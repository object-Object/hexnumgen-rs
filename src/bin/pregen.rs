use clap::Parser;
use hexnumgen::{generate_number_pattern, AStarOptions, Direction, GeneratedNumber};
use num_traits::Zero;
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

fn find_patterns(targets: Vec<i64>, only_tail: bool) -> BTreeMap<i64, (Option<String>, String)> {
    let mut data = BTreeMap::new();
    let re = Regex::new(r"^aqaa").unwrap();

    for (i, &target) in targets.iter().enumerate() {
        println!("{}/{}", i + 1, targets.len());

        let GeneratedNumber { direction, pattern, .. } =
            generate_number_pattern(target.into(), false, false, hexnumgen::GeneratorOptions::AStar(AStarOptions {}))
                .unwrap();

        if only_tail {
            data.insert(target, (None, re.replace(&pattern, "").to_string()));
            continue;
        }

        if !target.is_zero() {
            let negative_pattern = re.replace(&pattern, "dedd").to_string();
            data.insert(-target, (Some(Direction::NorthEast.to_string()), negative_pattern));
        }

        data.insert(target, (Some(direction), pattern));
    }

    data
}

fn worker(targets: Vec<i64>, only_tail: bool, tx: Sender<BTreeMap<i64, (Option<String>, String)>>) {
    tx.send(find_patterns(targets, only_tail)).unwrap();
}

#[derive(Parser)]
#[command(author = "Gamma Delta", version = "1", about = "generates hexcasting number literals")]
struct Cli {
    /// Largest number to generate a literal for.
    max: u64,

    /// only generate the "tail" of the number literal
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
                thread::spawn(move || worker(targets, cli.only_tail.clone(), tx));
            }
            drop(tx);

            let mut all_data = BTreeMap::new();
            while let Ok(data) = rx.recv() {
                all_data.extend(data);
            }
            all_data
        }
        None => find_patterns(all_targets, cli.only_tail.clone()),
    };

    fs::write(format!("numbers_{}.json", cli.max), serde_json::to_string_pretty(&all_data).unwrap()).unwrap();
}
