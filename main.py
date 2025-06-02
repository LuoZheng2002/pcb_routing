from datatypes import Grid, Net, Point
from functions import print_grid, naive_route
from collections import defaultdict

def create_grid(pads_list, width=12, height=8):
    pads = defaultdict(set)
    for net, points in pads_list:
        pads[net].update(points)
    return Grid(pads=dict(pads), traces={}, diagonal_traces={}, width=width, height=height)

def main():
    print("Hello, world!")

    pad_sets = [
        (Net('A', 'a'), [Point(3, 3), Point(9, 3)]),
        (Net('B', 'b'), [Point(5, 1), Point(5, 5)]),
        (Net('C', 'c'), [Point(7, 1), Point(7, 5)]),
    ]
    grid = create_grid(pad_sets)
    print_grid(grid)
    routed = naive_route(grid)
    print_grid(routed)

    pad_sets = [
        (Net('A', 'a'), [Point(3, 3), Point(9, 3)]),
        (Net('B', 'b'), [Point(5, 1), Point(7, 5)]),
        (Net('C', 'c'), [Point(7, 1), Point(5, 5)]),
    ]
    grid = create_grid(pad_sets)
    print_grid(grid)
    routed = naive_route(grid)
    print_grid(routed)

    pad_sets = [
        (Net('A', 'a'), [Point(3, 3), Point(9, 3), Point(5, 1), Point(7, 5), Point(7, 1)])
        # Point(5, 5)
    ]
    grid = create_grid(pad_sets)
    print_grid(grid)
    routed = naive_route(grid)
    print_grid(routed)

if __name__ == '__main__':
    main()