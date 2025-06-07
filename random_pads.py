import random
from typing import Dict, Set
from collections import defaultdict

from grid import Grid, Net, Point, PointPair

def generate_random_grid(width: int, height: int, net_count: int, max_pads_per_net: int) -> Grid:
    grid = Grid(pads=defaultdict(set), traces=defaultdict(set), diagonal_traces=defaultdict(set), width=width, height=height)
    
    nets = []
    for i in range(net_count):
        pad_c = chr(ord('A') + i)
        route_c = chr(ord('a') + i)
        nets.append(Net(pad_c, route_c))
    
    # Generate all possible positions
    all_positions = [Point(x, y) for x in range(width) for y in range(height)]
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
    times = int(input("times = "))
    width = int(input("width = "))
    height = int(input("height = "))
    net_count = int(input("net_count = "))
    max_pads_per_net = int(input("max_pads_per_net = "))
    for i in range(times):
        random.seed(20*i) 
        grid = generate_random_grid(width, height, net_count, max_pads_per_net)
        filename = f"test_data/test_naive_route{5+i}.txt"
        with open(filename, 'w') as f:
            f.write("input:\n")
            f.write(grid.__str__().strip())
            f.write("\noutput:\n")
            f.close()
    