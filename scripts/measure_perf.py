from __future__ import annotations

import json
from pathlib import Path
from timeit import default_timer as timer
from typing import TYPE_CHECKING, Required, TypedDict

from tqdm import tqdm, trange

from hexnumgen import (
    AStarOptions,
    AStarSplitOptions,
    BeamOptions,
    BeamPoolOptions,
    BeamSplitOptions,
    Bounds,
    generate_number_pattern,
)

if TYPE_CHECKING:
    from hexnumgen import Options


class PerfDumpItem(TypedDict, total=False):
    target: Required[int]
    time: Required[float]
    pattern: str
    points: int
    segments: int
    bounds: tuple[int, int, int]
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
        case AStarSplitOptions():
            carryover = None
            num_threads = options.num_threads

    return PerfDump(
        algorithm=type(options).__name__.replace("Options", ""),
        carryover=carryover,
        num_threads=num_threads,
        data=[],
    )


def get_dump_filename(dump: PerfDump, trim_larger: bool) -> str:
    filename = dump["algorithm"]
    if dump["carryover"] is not None:
        filename += f"_c{dump['carryover']}"
    if dump["num_threads"] is not None:
        filename += f"_t{dump['num_threads']}"
    if not trim_larger:
        filename += "_noTL"
    return filename + ".json"


def measure(out_dir: Path, trim_larger: bool, options: Options):
    dump = create_dump(options)

    filename = get_dump_filename(dump, trim_larger)
    for n in trange(1001, desc=filename):
        start = timer()
        number = generate_number_pattern(
            target=n,
            trim_larger=trim_larger,
            allow_fractions=False,
            options=options,
        )
        time = timer() - start

        item = PerfDumpItem(target=n, time=time)
        if number is None:
            tqdm.write(f"WARNING: Failed to generate {n}")
        else:
            bounds = number.bounds
            item["pattern"] = number.pattern
            item["points"] = number.num_points
            item["segments"] = number.num_segments
            item["largest_dim"] = bounds.largest_dimension
            item["bounds"] = (bounds.q, bounds.r, bounds.s)
            item["quasi_area"] = bounds.quasi_area
        dump["data"].append(item)

    out_path = out_dir / filename
    with out_path.open("w") as f:
        json.dump(dump, f)


if __name__ == "__main__":
    bounds = Bounds(8, 8, 8)

    # list of options to generate
    # note: running this entire list will take a *long* time
    optionses: list[Options] = [
        # AStarOptions(),
        BeamSplitOptions(bounds, carryover=1000, num_threads=10),
        # BeamOptions(bounds, 768),
    ]
    # for carryover in [50, 100, 200]:
    #     optionses.append(BeamOptions(bounds, carryover))
    #     for num_threads in [2, 4, 6, 8]:
    #         optionses.append(BeamPoolOptions(bounds, carryover, num_threads))
    #         optionses.append(BeamSplitOptions(bounds, carryover, num_threads))
    # for num_threads in [2, 4, 6, 8]:
    #     optionses.append(BeamSplitOptions(bounds, 768 // num_threads, num_threads))
    # for num_threads in [2, 4, 6, 8]:
    #     optionses.append(AStarSplitOptions(num_threads))

    # actually generate the patterns
    trim_larger = True
    (out_dir := Path("out")).mkdir(exist_ok=True)
    for options in tqdm(optionses, desc="Total"):
        measure(out_dir, trim_larger, options)
