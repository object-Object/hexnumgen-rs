import json
from pathlib import Path
from timeit import default_timer as timer
from typing import TypedDict

from hexnumgen import (
    AStarOptions,
    BeamOptions,
    BeamPoolOptions,
    BeamSplitOptions,
    Bounds,
    generate_number_pattern,
)


class NumberStats(TypedDict):
    time: float


class SuccessfulNumberStats(NumberStats):
    points: int
    largest_dim: int
    quasi_area: int


if __name__ == "__main__":
    (out_dir := Path("out")).mkdir(exist_ok=True)

    # num_threads = 8
    # carryover = 50
    # options = BeamSplitOptions(Bounds(8, 8, 8), carryover, num_threads)
    # out_path = out_dir / f"c{carryover}_{type(options).__name__}_t{num_threads}.json"

    options = AStarOptions()
    out_path = out_dir / "AStarOptions.json"

    data: list[NumberStats] = []
    for n in range(1001):
        if n % 50 == 0:
            print(n)

        start = timer()
        number = generate_number_pattern(n, False, False, options)
        time = timer() - start

        if number is None:
            data.append(NumberStats(time=time))
        else:
            data.append(
                SuccessfulNumberStats(
                    time=time,
                    points=number.num_points,
                    largest_dim=number.bounds.largest_dimension,
                    quasi_area=number.bounds.quasi_area,
                )
            )

    with out_path.open("w") as f:
        json.dump(data, f)
