import json
from pathlib import Path
from timeit import default_timer as timer
from typing import Required, TypedDict

from tqdm import tqdm, trange

from hexnumgen import (
    AStarOptions,
    BeamOptions,
    BeamPoolOptions,
    BeamSplitOptions,
    Bounds,
    generate_number_pattern,
)

Options = BeamOptions | BeamPoolOptions | BeamSplitOptions | AStarOptions


class PerfDumpItem(TypedDict, total=False):
    target: Required[int]
    time: Required[float]
    points: int
    segments: int
    largest_dim: int
    quasi_area: int


class PerfDump(TypedDict):
    algorithm: str
    carryover: int | None
    num_threads: int | None
    data: list[PerfDumpItem]


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

    filename = get_dump_filename(dump)
    for n in trange(1001, desc=filename):
        start = timer()
        number = generate_number_pattern(
            target=n,
            trim_larger=True,
            allow_fractions=False,
            options=options,
        )
        time = timer() - start

        item = PerfDumpItem(target=n, time=time)
        if number is None:
            tqdm.write(f"WARNING: Failed to generate {n}")
        else:
            item["points"] = number.num_points
            item["segments"] = number.num_segments
            item["largest_dim"] = number.bounds.largest_dimension
            item["quasi_area"] = number.bounds.quasi_area
        dump["data"].append(item)

    out_path = out_dir / filename
    with out_path.open("w") as f:
        json.dump(dump, f)


if __name__ == "__main__":
    bounds = Bounds(8, 8, 8)

    optionses: list[Options] = [
        AStarOptions(),
        BeamSplitOptions(bounds, carryover=65, num_threads=12),
        BeamSplitOptions(bounds, carryover=500, num_threads=10),
    ]
    for carryover in [50, 100, 200]:
        optionses.append(BeamOptions(bounds, carryover))
        for num_threads in [2, 4, 6, 8]:
            optionses.append(BeamPoolOptions(bounds, carryover, num_threads))
            optionses.append(BeamSplitOptions(bounds, carryover, num_threads))

    (out_dir := Path("out")).mkdir(exist_ok=True)
    for options in tqdm(optionses, desc="Total"):
        measure(out_dir, options)
