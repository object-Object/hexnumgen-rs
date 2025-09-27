from datetime import timedelta

from hexnumgen import AStarOptions, BeamOptions, Bounds, generate_number_pattern

print(generate_number_pattern(100, False, False, BeamOptions(Bounds(8, 8, 8), 25)))

print(
    generate_number_pattern(
        target=100,
        trim_larger=False,
        allow_fractions=False,
        options=AStarOptions(),
    )
)

print(
    generate_number_pattern(
        target=240519132949,
        trim_larger=False,
        allow_fractions=False,
        options=AStarOptions(
            timeout=timedelta(seconds=1),
        ),
    )
)
