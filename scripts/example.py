from hexnumgen import BeamOptions, Bounds, generate_number_pattern

print(generate_number_pattern(100, False, False, BeamOptions(Bounds(8, 8, 8), 25)))
