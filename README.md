# hexnumgen-rs

## Installation

* [Install Rust](https://www.rust-lang.org/tools/install).
* [Install Python](https://wiki.python.org/moin/BeginnersGuide/Download) if you don't already have a reasonably recent version installed.
* Clone/download this repo and enter it.
* Run `cargo fetch`.
* [Create and activate a venv](https://packaging.python.org/en/latest/guides/installing-using-pip-and-virtual-environments/#creating-a-virtual-environment).
* Run `pip install -r scripts/requirements.txt`.

## Usage (CLI)

```sh
# generate a single number
cargo run --release -- --help

# pregenerate a range of numbers
cargo run --release --bin pregen -- --help
```

## Usage (Python)

Remember to activate your venv before running these commands.

```sh
# build and locally install the Python package
maturin develop --release

# small demo script
python scripts/example.py

# generate performance data for several configurations of each algorithm
# note: this is very slow!
python scripts/measure_perf.py

# display graphs of performance data
# you'll need to edit these scripts manually, there's no CLI options
python scripts/display_perf.py
python scripts/efficiency.py
```

https://pyo3.rs/v0.17.3/getting_started

https://github.com/PyO3/maturin

## Attribution

Sequential algorithms are derived from https://github.com/DaComputerNerd717/Hex-Casting-Generator. Used with permission.
