import os
from grid import Grid, Net, Point, PointPair
from collections import defaultdict
import random

def generate_random_grid_plus(test_file, net_count: int, max_pads_per_net: int) -> Grid:
    with open(f"test_data/{test_file}", 'r', encoding='utf-8') as f:
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
    nets = []
    for i in range(net_count):
        pad_c = chr(ord('A') + i)
        route_c = chr(ord('a') + i)
        nets.append(Net(pad_c, route_c))
    
    # Generate all possible positions
    positions = [Point(x, y) for x in range(grid.width) for y in range(grid.height)]
    current_positions = [p for n, ps in grid.pads.items() for p in ps]
    all_positions = [x for x in positions if x not in current_positions]
    random.shuffle(all_positions)
    
    # Assign pads to nets
    position_index = 0
    for net in nets:
        pad_count = random.randint(2, max_pads_per_net)
        
        # Assign positions
        for _ in range(pad_count):
            if position_index >= len(all_positions):
                raise ValueError("Not enough positions for all pads")
            
            grid.pads[net].add(all_positions[position_index])
            position_index += 1
    
    return grid

if __name__ == "__main__":
    test_files = input("files = ")
    times = int(input("times = "))
    net_count = int(input("net_count = "))
    max_pads_per_net = int(input("max_pads_per_net = "))
    for i in range(times):
        random.seed(20*i) 
        grid = generate_random_grid_plus(test_files, net_count, max_pads_per_net)
        filename = f"test_data/test_naive_route{5+i}.txt"
        with open(filename, 'w') as f:
            f.write("input:\n")
            f.write(grid.__str__().strip())
            f.write("\noutput:\n")
            f.close()
