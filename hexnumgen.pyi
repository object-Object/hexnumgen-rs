from datetime import timedelta

class Bounds:
    q: int
    r: int
    s: int
    def __init__(self, q: int, r: int, s: int) -> None: ...
    @property
    def largest_dimension(self) -> int: ...
    @property
    def quasi_area(self) -> int: ...

class GeneratedNumber:
    @property
    def direction(self) -> str: ...
    @property
    def pattern(self) -> str: ...
    @property
    def bounds(self) -> Bounds: ...
    @property
    def num_points(self) -> int: ...
    @property
    def num_segments(self) -> int: ...

class BeamOptions:
    bounds: Bounds
    carryover: int
    def __init__(self, bounds: Bounds, carryover: int) -> None: ...

class BeamPoolOptions:
    bounds: Bounds
    carryover: int
    num_threads: int
    def __init__(self, bounds: Bounds, carryover: int, num_threads: int) -> None: ...

class BeamSplitOptions:
    bounds: Bounds
    carryover: int
    num_threads: int
    def __init__(self, bounds: Bounds, carryover: int, num_threads: int) -> None: ...

class AStarOptions:
    timeout: timedelta | None
    def __init__(self, timeout: timedelta | None = None) -> None: ...

class AStarSplitOptions:
    num_threads: int
    def __init__(self, num_threads: int) -> None: ...

Options = (
    BeamOptions | BeamPoolOptions | BeamSplitOptions | AStarOptions | AStarSplitOptions
)

def generate_number_pattern(
    target: int | tuple[int, int],
    trim_larger: bool,
    allow_fractions: bool,
    options: Options,
) -> GeneratedNumber | None: ...
