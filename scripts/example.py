from hexnumgen import generate_number_pattern, BeamOptions, Bounds

print(generate_number_pattern(100, False, False, BeamOptions(Bounds(8, 8, 8), 25)))
