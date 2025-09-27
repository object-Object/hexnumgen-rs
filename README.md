# hexnumgen-rs

Number literal generator for the Hex Casting mod for Minecraft.

## Installation

* [Install Rust](https://www.rust-lang.org/tools/install).
* [Install uv](https://docs.astral.sh/uv/getting-started/installation/).
* Clone/download this repo and enter it.
* Run these commands:
  ```sh
  cargo fetch
  uv sync
  ```

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
# build and locally install the Python package in release mode
maturin develop --uv --release

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

https://pyo3.rs/v0.26.0/getting_started

https://github.com/PyO3/maturin

## Attribution

Sequential algorithms are derived from https://github.com/DaComputerNerd717/Hex-Casting-Generator. Used with permission.
