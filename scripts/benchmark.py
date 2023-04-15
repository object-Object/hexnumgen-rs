import multiprocessing
from timeit import default_timer as timer

import matplotlib.pyplot as plt
import numpy as np
import pandas as pd

from hexnumgen import generate_number_pattern, GeneratedNumber, AStarOptions, BeamOptions, Bounds

def beam_worker(n: int) -> tuple[float, int | None, int | None]:
    if n % 10 == 0:
        print(n)

    start = timer()
    number = generate_number_pattern(n, True, False, BeamOptions(Bounds(6, 5, 6), 25))
    end = timer() - start

    if number is None:
        return end, None, None
    return end, number.num_points, number.bounds.largest_dimension

def astar_worker(n: int) -> tuple[float, int | None, int | None]:
    if n % 10 == 0:
        print(n)

    start = timer()
    number = generate_number_pattern(n, True, False, AStarOptions())
    end = timer() - start

    if number is None:
        return end, None, None
    return end, number.num_points, number.bounds.largest_dimension

def moving_average(data: list | tuple, n: int) -> list:
    return list(pd.Series(data).rolling(window=n).mean().values)

def nanmax(data: list | tuple) -> float:
    return np.nanmax(np.array(data, dtype=float))

if __name__ == "__main__":
    pool = multiprocessing.Pool(processes=multiprocessing.cpu_count() - 2)
    numbers = range(1001)

    # beam_times, beam_points, beam_sizes = zip(*pool.map(beam_worker, numbers))
    astar_times, astar_points, astar_sizes = zip(*pool.map(astar_worker, numbers, chunksize=32))
    
    # print(f"Beam failed: {[i for i, time in enumerate(beam_points) if time is None]}")
    print(f"A* failed: {[i for i, time in enumerate(astar_points) if time is None]}")

    colormap = plt.rcParams['axes.prop_cycle'].by_key()['color']

    fig, (ax1, ax2, ax3) = plt.subplots(3, 1)

    # ax1.plot(beam_times, label="Beam")
    ax1.plot(astar_times, label="A*")
    # ax1.legend(loc="upper left")
    ax1.set_title("Time to generate")

    # ax2.plot(beam_points, label="Beam")
    ax2.plot(astar_points, label="A*")
    # ax2.legend(loc="upper left")
    ax2.set_title("Number of points")

    # ax3.plot(moving_average(beam_sizes, 5), label="Beam")
    ax3.plot(moving_average(astar_sizes, 5), label="A*")
    # ax3.axhline(nanmax(beam_sizes), color=colormap[0])
    ax3.axhline(nanmax(astar_sizes), linestyle="--", color=colormap[1])
    # ax3.legend(loc="upper left")
    ax3.set_title("Size of largest dimension (moving average)")

    fig.set_tight_layout(True)
    plt.show()
