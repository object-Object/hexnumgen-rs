# pyright: reportUnknownMemberType=information

import json

import matplotlib.pyplot as plt
import pandas as pd
from matplotlib.axes import Axes
from measure_perf import PerfDump

DfDump = tuple[str, pd.DataFrame]


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


def plot_series(
    time_ax: Axes | None,
    segments_ax: Axes,
    largest_dim_ax: Axes,
    quasi_area_ax: Axes,
    label: str,
    df: pd.DataFrame,
):
    if time_ax:
        time_ax.plot(df.time, label=label)
    segments_ax.plot(df.segments, label=label)
    largest_dim_ax.plot(df.largest_dim, label=label)
    quasi_area_ax.plot(df.quasi_area, label=label)


def num_sub(a: pd.DataFrame, b: pd.DataFrame) -> pd.DataFrame:
    return a.select_dtypes("number") - b.select_dtypes("number")


if __name__ == "__main__":
    fig, (ax1, ax2, ax3, ax4) = plt.subplots(4, 1, sharex=True)

    # plot 1a
    # filenames = ["Beam_c200", "BeamPool_c200_t4", "BeamSplit_c50_t4"]

    # plot 1b
    # baseline_filename = "Beam_c200"
    # filenames = ["BeamPool_c200_t4", "BeamSplit_c50_t4"]

    # plot 2
    # baseline_filename = "AStar"
    # filenames = ["AStarSplit_t2", "AStarSplit_t4", "AStarSplit_t6", "AStarSplit_t8"]
    # ax4.set_yticks([0])

    # plot 3a
    filenames = ["BeamPool_c200_t4", "BeamSplit_c200_t4", "AStarSplit_t4"]

    # read dump files
    dfds = [read_dump_file(f"out/{filename}.json") for filename in filenames]

    # plot data
    for label, df in dfds:
        plot_series(ax1, ax2, ax3, ax4, label, df)
    ax2.set_title("Number of segments")
    ax3.set_title("Size of largest dimension")
    ax4.set_title("Quasi-area (q*r*s)")

    # plot data relative to baseline
    # baseline_label, baseline_df = read_dump_file(f"out/{baseline_filename}.json")
    # ax1.plot(baseline_df.time, label=baseline_label)
    # plot_series(None, ax2, ax3, ax4, baseline_label, num_sub(baseline_df, baseline_df))
    # for label, df in dfds:
    #     ax1.plot(df.time, label=label)
    #     plot_series(None, ax2, ax3, ax4, label, num_sub(df, baseline_df))
    # ax2.set_title("Number of segments vs. sequential")
    # ax3.set_title("Size of largest dimension vs. sequential")
    # ax4.set_title("Quasi-area (q*r*s) vs. sequential")

    # decorate plots
    ax1.set_title("Time to generate")

    ax1.legend(loc="upper left")
    # ax2.legend(loc="upper left")
    # ax3.legend(loc="upper left")
    # ax4.legend(loc="upper left")

    ax1.set_xmargin(0.01)

    fig.tight_layout()
    plt.show()
