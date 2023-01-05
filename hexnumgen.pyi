def generate_number_pattern(
    target: int,
    q_size: int = 8,
    r_size: int = 8,
    s_size: int = 8,
    carryover: int = 25,
    trim_larger: bool = True,
) -> tuple[str, str] | None: ...