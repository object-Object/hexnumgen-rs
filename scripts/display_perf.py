import json

import matplotlib.pyplot as plt
import pandas as pd
from benchmark import moving_average, nanmax
from matplotlib.axes import Axes
from matplotlib.figure import Figure
from measure_perf import PerfDump

DfDump = tuple[str, pd.DataFrame]

_colormap: list[str] = plt.rcParams["axes.prop_cycle"].by_key()["color"]
_index = 0


def plot_moving_average(ax: Axes, label: str, data: list | pd.Series, n: int):
    ax.plot(moving_average(data, n), label=label)
    ax.axhline(nanmax(data), linestyle="--", color=_colormap[_index % len(_colormap)])


def get_dump_label(dump: PerfDump) -> str:
    label = [dump["algorithm"]]
    if dump["carryover"] is not None:
        label.append(f"{dump['carryover']} carryover")
    if dump["num_threads"] is not None:
        label.append(f"{dump['num_threads']} threads")
    return ", ".join(label)


def read_dump_file(filename: str) -> DfDump:
    with open(filename, "r") as f:
        dump = PerfDump(**json.load(f))

    label = get_dump_label(dump)
    if "old_" in filename:
        label = "Old" + label

    return (label, pd.DataFrame(dump["data"]))


def agg_data(data: list | pd.Series) -> str:
    return f"min/max/avg/tot: {min(data):.4f}/{max(data):.4f}/{sum(data)/len(data):.4f}/{sum(data):.4f}"


def plot_series(
    time_ax: Axes | None,
    segments_ax: Axes,
    largest_dim_ax: Axes,
    quasi_area_ax: Axes,
    label: str,
    df: pd.DataFrame,
    label_pad: int,
):
    global _index

    print(f"{label: >{label_pad}}: {agg_data(df.time)}")

    if time_ax:
        time_ax.plot(df.time, label=label)

    segments_ax.plot(df.segments, label=label)
    # plot_moving_average(segments_ax, label, df.segments, 3)

    largest_dim_ax.plot(df.largest_dim, label=label)
    # plot_moving_average(largest_dim_ax, label, df.largest_dim, 5)

    quasi_area_ax.plot(df.quasi_area, label=label)
    # plot_moving_average(quasi_area_ax, label, df.quasi_area, 3)

    _index += 1


if __name__ == "__main__":
    fig, (ax1, ax2, ax3, ax4) = plt.subplots(4, 1)

    astar_label, astar_df = read_dump_file("out/AStar.json")
    dfds = [
        read_dump_file(f"out/{filename}.json")
        for filename in [
            # "Beam_c50",
            # "Beam_c100",
            # "Beam_c200",
            # "BeamSplit_c200_t4",
            # "BeamSplit_c100_t8",
            # "BeamSplit_c65_t12",
        ]
    ]
    label_pad = max(len(label) for label, *_ in dfds + [(astar_label,)])

    for label, df in dfds:
        plot_series(ax1, ax2, ax3, ax4, label, df, label_pad)
    # plot_series(ax1, ax2, ax3, ax4, astar_label, astar_df, label_pad)

    ax1.set_title("Time to generate")
    ax2.set_title("Number of segments")
    ax3.set_title("Size of largest dimension")
    ax4.set_title("Quasi-area (q*r*s)")

    ax1.legend(loc="upper left")
    ax2.legend(loc="upper left")
    ax3.legend(loc="upper left")
    ax4.legend(loc="upper left")

    fig.set_tight_layout(True)
    plt.show()
