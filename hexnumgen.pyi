class GeneratedNumber:
    @property
    def direction(self) -> str: ...

    @property
    def pattern(self) -> str: ...

    @property
    def largest_dimension(self) -> int: ...
    
    @property
    def num_points(self) -> int: ...

def generate_number_pattern_beam(
    target: int,
    q_size: int = 8,
    r_size: int = 8,
    s_size: int = 8,
    carryover: int = 25,
    trim_larger: bool = True,
) -> GeneratedNumber | None: ...
