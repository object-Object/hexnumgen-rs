use clap::Parser;
use hexnumgen::{generate_number_pattern_astar, Direction, GeneratedNumber};
use num_rational::Ratio;
use num_traits::Zero;
use rand::{seq::SliceRandom, thread_rng};
use regex::Regex;
use std::{
    collections::HashMap,
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

fn worker(targets: Vec<Ratio<i64>>, tx: Sender<HashMap<Ratio<i64>, (String, String)>>) {
    let mut data = HashMap::new();
    let re = Regex::new(r"^aqaa").unwrap();

    for (i, &target) in targets.iter().enumerate() {
        println!("{}/{}", i + 1, targets.len());

        let GeneratedNumber { direction, pattern, .. } = generate_number_pattern_astar(target, false, false).unwrap();

        if !target.is_zero() {
            let negative_pattern = re.replace(&pattern, "dedd").to_string();
            data.insert(-target, (Direction::NorthEast.to_string(), negative_pattern));
        }

        data.insert(target, (direction, pattern));
    }

    tx.send(data).unwrap();
}

#[derive(Parser)]
struct Cli {
    /// Largest number to generate a literal for
    max: u64,
}

fn main() {
    let max = Cli::parse().max;

    let mut all_targets = Vec::from_iter((0..=max).map(|n| (n as i64).into()));
    all_targets.shuffle(&mut thread_rng());

    let cpus = thread::available_parallelism().unwrap().get() - 1;

    let (tx, rx) = mpsc::channel();

    for targets in n_groups(all_targets, cpus) {
        let tx = tx.clone();
        thread::spawn(move || worker(targets, tx));
    }
    drop(tx);

    let mut all_data = HashMap::new();
    while let Ok(data) = rx.recv() {
        all_data.extend(data);
    }

    fs::write(format!("numbers_{max}.json"), serde_json::to_string(&all_data).unwrap()).unwrap();
}
