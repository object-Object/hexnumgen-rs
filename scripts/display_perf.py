import json

import matplotlib.pyplot as plt
from benchmark import moving_average, nanmax
from matplotlib.axes import Axes
from matplotlib.figure import Figure
from measure_perf import PerfDump, PerfDumpItem

_colormap: list[str] = plt.rcParams["axes.prop_cycle"].by_key()["color"]
_index = 0


def plot_moving_average(ax: Axes, label: str, data: list, n: int):
    ax.plot(moving_average(data, n), label=label)
    ax.axhline(nanmax(data), linestyle="--", color=_colormap[_index])


def get_dump_label(dump: PerfDump) -> str:
    label = [dump["algorithm"]]
    if dump["carryover"] is not None:
        label.append(f"{dump['carryover']} carryover")
    if dump["num_threads"] is not None:
        label.append(f"{dump['num_threads']} threads")
    return ", ".join(label)


def plot_series(
    ax1: Axes, ax2: Axes, ax3: Axes, ax4: Axes, filename: str, plot_time: bool = True
):
    global _index

    time = []
    segments = []
    largest_dim = []
    quasi_area = []

    with open(filename, "r") as f:
        dump = PerfDump(**json.load(f))

    label = get_dump_label(dump)
    for i, stats in enumerate(dump["data"]):
        if isinstance(stats, float):
            print(f"WARNING: No pattern data for {i} in {filename}")
            time.append(stats)
            segments.append(None)
            largest_dim.append(None)
            quasi_area.append(None)
        else:
            time.append(stats["time"])
            segments.append(stats["segments"])
            largest_dim.append(stats["largest_dim"])
            quasi_area.append(stats["quasi_area"])

    print(
        f"{label: <35} min/max/avg/tot: {min(time):.4f}/{max(time):.4f}/{sum(time)/len(time):.4f}/{sum(time):.4f}"
    )

    if plot_time:
        ax1.plot(time, label=label)

    ax2.plot(segments, label=label)
    # plot_moving_average(ax2, label, segments, 3)

    # ax3.plot(largest_dim, label=label)
    plot_moving_average(ax3, label, largest_dim, 5)

    ax4.plot(quasi_area, label=label)
    # plot_moving_average(ax4, label, quasi_area, 3)

    _index += 1


if __name__ == "__main__":
    fig, (ax1, ax2, ax3, ax4) = plt.subplots(4, 1)

    for filename in [
        # "out/Beam_c50.json",
        # "out/Beam_c100.json",
        # "out/Beam_c200.json",
        "out/BeamSplit_c200_t4.json",
        "out/BeamSplit_c100_t8.json",
        "out/BeamSplit_c65_t12.json",
    ]:
        plot_series(ax1, ax2, ax3, ax4, filename)
    plot_series(ax1, ax2, ax3, ax4, "out/AStar_2000.json", plot_time=False)

    ax1.set_title("Time to generate")
    ax2.set_title("Number of segments")
    ax3.set_title("Size of largest dimension (moving average)")
    ax4.set_title("Quasi-area (q*r*s)")

    ax1.legend(loc="upper left")
    ax2.legend(loc="upper left")
    ax3.legend(loc="upper left")
    ax4.legend(loc="upper left")

    fig.set_tight_layout(True)
    plt.show()
