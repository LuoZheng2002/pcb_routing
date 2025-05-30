from typing import List, Dict, Set
from datatypes import Grid, Net, Point, DijkstraModel, DijkstraResult
from collections import defaultdict
from ordered_set import OrderedSet  # optional if order matters

def grid_to_string(grid: Grid) -> List[List[str]]:
    grid_string = [[' ' for _ in range(grid.width)] for _ in range(grid.height)]

    for net, points in grid.pads.items():
        for p in points:
            grid_string[p.y][p.x] = net.pad_c

    for net, points in grid.traces.items():
        for p in points:
            grid_string[p.y][p.x] = net.route_c

    return grid_string

def print_grid_string(grid_string: List[List[str]]) -> None:
    border = '#' * (len(grid_string[0]) + 2)
    print(border)
    for row in grid_string:
        print('#' + ''.join(row) + '#')
    print(border)

def print_grid(grid: Grid) -> None:
    grid_string = grid_to_string(grid)
    print_grid_string(grid_string)

def naive_route(unrouted_grid: Grid) -> Grid:

    def prepare_model(net: Net, start: Point, end: Point) -> DijkstraModel:
        return DijkstraModel(
            width=unrouted_grid.width,
            height=unrouted_grid.height,
            obstacles=unrouted_grid.pads_except(net),
            diagonal_obstacles=set(),
            start=start,
            end=end,
        )

    pad_pairs = []
    for net, points in unrouted_grid.pads.items():
        points_list = list(points)
        for i in range(len(points_list)):
            for j in range(i + 1, len(points_list)):
                start, end = points_list[i], points_list[j]
                model = prepare_model(net, start, end)
                result = model.run()
                pad_pairs.append((result.distance, net, start, end, result))

    pad_pairs = sorted(pad_pairs, key=lambda x: x[0])

    new_traces = dict(unrouted_grid.traces)
    for _, net, start, _, result in pad_pairs:
        pos = start
        trace = {pos}
        for dir in result.directions:
            pos = Point(pos.x + dir.x, pos.y + dir.y)
            trace.add(pos)
        new_traces[net] = trace

    return Grid(
        pads=unrouted_grid.pads,
        traces=new_traces,
        diagonal_traces=unrouted_grid.diagonal_traces,
        width=unrouted_grid.width,
        height=unrouted_grid.height,
    )