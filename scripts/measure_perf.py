import json
from pathlib import Path
from timeit import default_timer as timer
from typing import TypedDict

from tqdm import tqdm

from hexnumgen import (
    AStarOptions,
    BeamOptions,
    BeamPoolOptions,
    BeamSplitOptions,
    Bounds,
    generate_number_pattern,
)

Options = BeamOptions | BeamPoolOptions | BeamSplitOptions | AStarOptions


class PerfDumpItem(TypedDict):
    time: float
    points: int
    segments: int
    largest_dim: int
    quasi_area: int


class PerfDump(TypedDict):
    algorithm: str
    carryover: int | None
    num_threads: int | None
    data: list[PerfDumpItem | float]


def create_dump(options: Options) -> PerfDump:
    match options:
        case BeamOptions():
            carryover = options.carryover
            num_threads = None
        case BeamPoolOptions() | BeamSplitOptions():
            carryover = options.carryover
            num_threads = options.num_threads
        case AStarOptions():
            carryover = None
            num_threads = None

    return PerfDump(
        algorithm=type(options).__name__.replace("Options", ""),
        carryover=carryover,
        num_threads=num_threads,
        data=[],
    )


def get_dump_filename(dump: PerfDump) -> str:
    filename = dump["algorithm"]
    if dump["carryover"] is not None:
        filename += f"_c{dump['carryover']}"
    if dump["num_threads"] is not None:
        filename += f"_t{dump['num_threads']}"
    return filename + ".json"


def measure(out_dir: Path, options: Options):
    dump = create_dump(options)

    print(filename := get_dump_filename(dump))
    for n in tqdm(range(1001)):
        start = timer()
        number = generate_number_pattern(
            target=n,
            trim_larger=True,
            allow_fractions=False,
            options=options,
        )
        time = timer() - start

        if number is None:
            tqdm.write(f"WARNING: Failed to generate {n}")
            dump["data"].append(time)
        else:
            dump["data"].append(
                PerfDumpItem(
                    time=time,
                    points=number.num_points,
                    segments=number.num_segments,
                    largest_dim=number.bounds.largest_dimension,
                    quasi_area=number.bounds.quasi_area,
                )
            )

    out_path = out_dir / filename
    with out_path.open("w") as f:
        json.dump(dump, f)


if __name__ == "__main__":
    bounds = Bounds(8, 8, 8)

    optionses: list[Options] = [
        # AStarOptions(),
        # BeamOptions(bounds, carryover=50),
        # BeamOptions(bounds, carryover=100),
        # BeamOptions(bounds, carryover=200),
        # BeamPoolOptions(bounds, carryover=50, num_threads=2),
        # BeamPoolOptions(bounds, carryover=100, num_threads=2),
        # BeamPoolOptions(bounds, carryover=200, num_threads=2),
        # BeamPoolOptions(bounds, carryover=50, num_threads=4),
        # BeamPoolOptions(bounds, carryover=100, num_threads=4),
        # BeamPoolOptions(bounds, carryover=200, num_threads=4),
        # BeamPoolOptions(bounds, carryover=50, num_threads=8),
        # BeamPoolOptions(bounds, carryover=100, num_threads=8),
        # BeamPoolOptions(bounds, carryover=200, num_threads=8),
        # BeamSplitOptions(bounds, carryover=50, num_threads=4),
        # BeamSplitOptions(bounds, carryover=100, num_threads=4),
        # BeamSplitOptions(bounds, carryover=200, num_threads=4),
        # BeamSplitOptions(bounds, carryover=100, num_threads=8),
        # BeamSplitOptions(bounds, carryover=200, num_threads=8),
        # BeamSplitOptions(bounds, carryover=65, num_threads=12),
    ]

    (out_dir := Path("out")).mkdir(exist_ok=True)
    for options in optionses:
        measure(out_dir, options)
