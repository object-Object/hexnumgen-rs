import json

import matplotlib.pyplot as plt
import pandas as pd
from benchmark import moving_average, nanmax
from matplotlib.axes import Axes
from measure_perf import PerfDump

DfDump = tuple[str, pd.DataFrame]

_colormap: list[str] = plt.rcParams["axes.prop_cycle"].by_key()["color"]
_index = 0


def plot_moving_average(ax: Axes, label: str, data: list | pd.Series, n: int):
    ax.plot(moving_average(data, n), label=label)
    ax.axhline(nanmax(data), linestyle="--", color=_colormap[_index])


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

    return (label, pd.DataFrame(dump["data"]).set_index("target"))


def agg_data(data: list | pd.Series) -> str:
    return f"min/max/avg/tot: {min(data):.4f}/{max(data):.4f}/{sum(data)/len(data):.4f}/{sum(data):.4f}"


def plot_series(
    time_ax: Axes | None,
    segments_ax: Axes,
    largest_dim_ax: Axes,
    quasi_area_ax: Axes,
    label: str,
    df: pd.DataFrame,
):
    global _index
    if time_ax:
        time_ax.plot(df.time, label=label)

    segments_ax.plot(df.segments, label=label)
    # plot_moving_average(segments_ax, label, df.segments, 3)

    largest_dim_ax.plot(df.largest_dim, label=label)
    # plot_moving_average(largest_dim_ax, label, df.largest_dim, 5)

    quasi_area_ax.plot(df.quasi_area, label=label)
    # plot_moving_average(quasi_area_ax, label, df.quasi_area, 3)

    _index = (_index + 1) % len(_colormap)


def num_sub(a: pd.DataFrame, b: pd.DataFrame) -> pd.DataFrame:
    return a.select_dtypes("number") - b.select_dtypes("number")


if __name__ == "__main__":
    fig, (ax1, ax2, ax3, ax4) = plt.subplots(4, 1)

    # read dump files
    astar_label, astar_df = read_dump_file("out/AStar_noTL.json")
    dfds = [
        read_dump_file(f"out/{filename}.json")
        for filename in [
            # "Beam_c50",
            # "Beam_c100",
            # "Beam_c200",
            # "BeamSplit_c200_t4",
            # "BeamSplit_c100_t8",
            # "BeamSplit_c65_t12",
            "AStarSplit_t2_noTL",
            "AStarSplit_t4_noTL",
            "AStarSplit_t6_noTL",
            "AStarSplit_t8_noTL",
        ]
    ]
    label_pad = max(len(label) for label, *_ in dfds + [(astar_label,)])

    # plot data
    # plot_series(ax1, ax2, ax3, ax4, astar_label, astar_df, label_pad)
    # for label, df in dfds:
    #     print(f"{label: >{label_pad}}: {agg_data(df.time)}")
    #     plot_series(ax1, ax2, ax3, ax4, label, df, label_pad)

    # plot data relative to sequential A*
    ax1.plot(astar_df.time, label=astar_label, color=_colormap[_index])
    plot_series(None, ax2, ax3, ax4, astar_label, num_sub(astar_df, astar_df))
    for label, df in dfds:
        print(f"{label: >{label_pad}}: {agg_data(df.time)}")
        ax1.plot(df.time, label=label, color=_colormap[_index])
        plot_series(None, ax2, ax3, ax4, label, num_sub(df, astar_df))

    # decorate plots
    ax1.set_title("Time to generate")

    # ax2.set_title("Number of segments")
    # ax3.set_title("Size of largest dimension")
    # ax4.set_title("Quasi-area (q*r*s)")

    ax2.set_title("Number of segments vs. sequential A*")
    ax3.set_title("Size of largest dimension vs. sequential A*")
    ax4.set_title("Quasi-area (q*r*s) vs. sequential A*")

    ax1.legend(loc="upper left")
    ax2.legend(loc="upper left")
    ax3.legend(loc="upper left")
    ax4.legend(loc="upper left")

    fig.set_tight_layout(True)
    plt.show()
