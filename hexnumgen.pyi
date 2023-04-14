class GeneratedNumber:
    @property
    def direction(self) -> str: ...

    @property
    def pattern(self) -> str: ...

    @property
    def largest_dimension(self) -> int: ...
    
    @property
    def num_points(self) -> int: ...

class Bounds:
    def __init__(self, q: int, r: int, s: int) -> None: ...

class BeamOptions:
    def __init__(self, bounds: Bounds, carryover: int) -> None: ...

class BeamPoolOptions:
    def __init__(self, bounds: Bounds, carryover: int, num_threads: int) -> None: ...

class BeamSplitOptions:
    def __init__(self, bounds: Bounds, carryover: int, num_threads: int) -> None: ...

class AStarOptions:
    def __init__(self) -> None: ...

def generate_number_pattern_beam(
    target: int | tuple[int, int],
    trim_larger: bool,
    allow_fractions: bool,
    options: BeamOptions | BeamPoolOptions | BeamSplitOptions | AStarOptions,
) -> GeneratedNumber | None: ...
