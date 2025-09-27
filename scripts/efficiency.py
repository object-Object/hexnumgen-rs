# pyright: reportUnknownMemberType=information

import matplotlib.pyplot as plt
from display_perf import read_dump_file
from matplotlib.axes import Axes


def style_ax(ax: Axes, title: str, thread_counts: list[int]):
    ax.set_title(title)
    ax.legend()

    ax.set_xticks(thread_counts)
    ax.set_xlabel("Threads")


def plot_eff(
    ax: Axes,
    label: str | None,
    seq_filename: str,
    par_filename: str | list[str],
    thread_counts: list[int],
):
    seq_time = read_dump_file(f"out/{seq_filename}.json")[1].time

    if isinstance(par_filename, str):
        par_filename = [par_filename.format(t) for t in thread_counts]

    y = list[float]()
    for threads, filename in zip(thread_counts, par_filename):
        df = read_dump_file(f"out/{filename}.json")[1]
        y.append((seq_time / (threads * df.time)).mean())

    ax.plot(thread_counts, y, label=label)


if __name__ == "__main__":
    fig, (ax1, ax2, ax3) = plt.subplots(1, 3, sharey=True)
    thread_counts = [2, 4, 6, 8]

    # BeamPool
    for c in [50, 100, 200]:
        plot_eff(
            ax1, f"{c} carryover", f"Beam_c{c}", f"BeamPool_c{c}_t{{}}", thread_counts
        )
    style_ax(ax1, "BeamPool", thread_counts)
    ax1.set_ylabel("Efficiency (speedup/threads)")

    # BeamSplit
    for c in [50, 100, 200]:
        plot_eff(
            ax2, f"{c} carryover", f"Beam_c{c}", f"BeamSplit_c{c}_t{{}}", thread_counts
        )
    plot_eff(
        ax2,
        "(768/threads) carryover",
        "Beam_c768",
        [f"BeamSplit_c{768 // t}_t{t}" for t in thread_counts],
        thread_counts,
    )
    style_ax(ax2, "BeamSplit", thread_counts)

    # AStarSplit
    plot_eff(ax3, "Trim larger", "AStar", "AStarSplit_t{}", thread_counts)
    plot_eff(ax3, "Keep larger", "AStar_noTL", "AStarSplit_t{}_noTL", thread_counts)
    style_ax(ax3, "AStarSplit", thread_counts)

    fig.tight_layout()
    plt.show()
