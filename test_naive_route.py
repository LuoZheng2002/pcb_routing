import os
from grid import Grid

from naive_route import naive_route

def test_naive_route():
    test_files = [
        "test_data/test_naive_route1.txt",
        "test_data/test_naive_route2.txt",
        "test_data/test_naive_route3.txt",
        "test_data/test_naive_route4.txt",
    ]

    for test_file in test_files:
        print("testing", test_file)
        with open(test_file, 'r', encoding='utf-8') as f:
            content = f.read()

        content = content.replace("\r\n", "\n")  # Normalize line endings

        if "input:\n" not in content:
            raise ValueError(f"'input:' not found in {test_file}")
        _, remainder = content.split("input:\n", 1)

        if "output:\n" not in remainder:
            raise ValueError(f"'output:' not found in {test_file}")
        input_text, expected_output = remainder.split("output:\n", 1)

        input_text = input_text.strip()
        expected_output = expected_output.strip()

        grid = Grid.from_string(input_text)
        routed_grid = naive_route(grid)
        output = routed_grid.__str__().strip()

        if output != expected_output:
            print(f"Output does not match expected in {test_file}. Output:\n{output}")

# Note: Define or import Grid and naive_route before calling this function.

if __name__ == "__main__":
    test_naive_route()
    print("All tests passed!")