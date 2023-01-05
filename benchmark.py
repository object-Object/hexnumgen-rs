import multiprocessing
from timeit import default_timer as timer

import matplotlib.pyplot as plt

from hexnumgen import generate_number_pattern


def original_worker(n: int) -> tuple[float, int | None, int | None]:
    if n % 10 == 0:
        print(n)

    start = timer()
    number = generate_number_pattern(n)
    end = timer() - start

    if number is None:
        return end, None, None
    return end, number.num_points, number.largest_dimension


if __name__ == "__main__":
    pool = multiprocessing.Pool(processes=multiprocessing.cpu_count() - 1)
    original_times, original_points, original_sizes = zip(*pool.map(original_worker, range(1000)))
    
    print(f"Failed: {[i for i, time in enumerate(original_points) if time is None]}")

    fig, (ax1, ax2, ax3) = plt.subplots(3, 1)

    ax1.plot(original_times, label="Original")
    # ax1.legend()
    ax1.set_title("Time to generate")

    ax2.plot(original_points, label="Original")
    # ax2.legend()
    ax2.set_title("Number of points")

    ax3.plot(original_sizes, label="Original")
    # ax3.legend()
    ax3.set_title("Size of largest dimension")

    fig.tight_layout()
    plt.show()
