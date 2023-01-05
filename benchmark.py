import multiprocessing
from timeit import default_timer as timer
from typing import Callable

import matplotlib.pyplot as plt

from hexnumgen import generate_number_pattern_beam, GeneratedNumber


def worker(f: Callable[[], GeneratedNumber | None]) -> tuple[float, int | None, int | None]:
    start = timer()
    number = f()
    end = timer() - start

    if number is None:
        return end, None, None
    return end, number.num_points, number.largest_dimension


def beam_worker(n: int) -> tuple[float, int | None, int | None]:
    return worker(lambda: generate_number_pattern_beam(n, 8, 8, 8, 25, True))


if __name__ == "__main__":
    pool = multiprocessing.Pool(processes=multiprocessing.cpu_count() - 1)
    numbers = range(100)

    beam_times, beam_points, beam_sizes = zip(*pool.map(beam_worker, numbers))
    
    print(f"Failed: {[i for i, time in enumerate(beam_points) if time is None]}")

    fig, (ax1, ax2, ax3) = plt.subplots(3, 1)

    ax1.plot(beam_times, label="Beam")
    # ax1.legend()
    ax1.set_title("Time to generate")

    ax2.plot(beam_points, label="Beam")
    # ax2.legend()
    ax2.set_title("Number of points")

    ax3.plot(beam_sizes, label="Beam")
    # ax3.legend()
    ax3.set_title("Size of largest dimension")

    fig.tight_layout()
    plt.show()
