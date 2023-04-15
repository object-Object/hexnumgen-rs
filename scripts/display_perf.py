import json

import matplotlib.pyplot as plt
from benchmark import moving_average, nanmax
from matplotlib.axes import Axes
from matplotlib.figure import Figure
from measure_perf import NumberStats, SuccessfulNumberStats

_colormap: list[str] = plt.rcParams["axes.prop_cycle"].by_key()["color"]
_index = 0


def plot_moving_average(ax: Axes, data: list, n: int):
    ax.plot(moving_average(data, n), label=label)
    ax.axhline(nanmax(data), linestyle="--", color=_colormap[_index])


def plot_series(
    ax1: Axes,
    ax2: Axes,
    ax3: Axes,
    ax4: Axes,
    filename: str,
    label: str,
):
    global _index

    time = []
    points = []
    largest_dim = []
    quasi_area = []

    with open(filename, "r") as f:
        for i, raw_stats in enumerate(json.load(f)):
            if "points" in raw_stats:
                stats = SuccessfulNumberStats(**raw_stats)
                time.append(stats["time"])
                points.append(stats["points"])
                largest_dim.append(stats["largest_dim"])
                quasi_area.append(stats["quasi_area"])
            else:
                stats = NumberStats(**raw_stats)
                time.append(stats["time"])
                points.append(None)
                largest_dim.append(None)
                quasi_area.append(None)

    print(
        f"{label: <35} min/max/avg/tot: {min(time):.4f}/{max(time):.4f}/{sum(time)/len(time):.4f}/{sum(time):.4f}"
    )

    ax1.plot(time, label=label)
    # ax2.plot(points, label=label)
    plot_moving_average(ax2, points, 3)
    # ax3.plot(largest_dim, label=label)
    plot_moving_average(ax3, largest_dim, 5)
    # ax4.plot(quasi_area, label=label)
    plot_moving_average(ax4, quasi_area, 3)
    _index += 1


if __name__ == "__main__":
    fig, (ax1, ax2, ax3, ax4) = plt.subplots(4, 1)

    data_files: list[tuple[str, str]] = [
        ("out/c200_BeamOptions.json", "Beam, 200 carryover"),
        ("out/c200_BeamPoolOptions_t4.json", "BeamPool, 200 carryover, 4 threads"),
        ("out/c50_BeamSplitOptions_t4.json", "BeamSplit, 50 carryover, 4 threads"),
        ("out/AStarOptions.json", "A*"),
    ]
    for filename, label in data_files:
        plot_series(ax1, ax2, ax3, ax4, filename, label)

    ax1.set_title("Time to generate")
    ax2.set_title("Number of points (moving average)")
    ax3.set_title("Size of largest dimension (moving average)")
    ax4.set_title("Quasi-area (q*r*s, moving average)")

    ax1.legend(loc="upper left")
    ax2.legend(loc="upper left")
    ax3.legend(loc="upper left")
    ax4.legend(loc="upper left")

    fig.set_tight_layout(True)
    plt.show()
