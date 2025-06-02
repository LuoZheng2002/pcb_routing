from typing import List, Dict, Set
from datatypes import Grid, Net, Point, DijkstraModel, DijkstraResult
from collections import defaultdict
from ordered_set import OrderedSet  # optional if order matters
import heapq

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

def prim_mst(pad_pairs: List[tuple[float, Net, Point, Point, DijkstraResult]]) -> List[tuple[float, Net, Point, Point, DijkstraResult]]:
    if not pad_pairs:
        return []

    points = set()
    for _, _, start, end, _ in pad_pairs:
        points.add(start)
        points.add(end)
    points = list(points)

    adj = {}
    for dist, net, start, end, result in pad_pairs:
        adj[(start, end)] = (dist, net, result)
        adj[(end, start)] = (dist, net, result)  

    mst_edges = []
    visited = set()
    start_point = points[0]
    visited.add(start_point)

    heap = []
    for (u, v), (dist, net, result) in adj.items():
        if u == start_point:
            heapq.heappush(heap, (dist, net, u, v, result))

    while heap and len(visited) < len(points):
        dist, net, u, v, result = heapq.heappop(heap)
        if v not in visited:
            visited.add(v)
            mst_edges.append((dist, net, u, v, result))
            for (u_new, v_new), (dist_new, net_new, result_new) in adj.items():
                if u_new == v and v_new not in visited:
                    heapq.heappush(heap, (dist_new, net_new, u_new, v_new, result_new))

    return mst_edges

def naive_route(unrouted_grid: Grid) -> Grid:

    def prepare_model(net: Net, start: Point, end: Point, pending_net:Dict[Net, Set[Point]]) -> DijkstraModel:
        return DijkstraModel(
            net=net,
            width=unrouted_grid.width,
            height=unrouted_grid.height,
            obstacles=unrouted_grid.pads_except(net),
            # diagonal_obstacles=set(),
            pending_net = pending_net,
            start=start,
            end=end,
        )

    pad_pairs = []
    pending_net = {}
    for net, points in unrouted_grid.pads.items():
        points_list = list(points)
        pad_pairs_tmp = []
        for i in range(len(points_list)):
            for j in range(i + 1, len(points_list)):
                start, end = points_list[i], points_list[j]
                model = prepare_model(net, start, end, pending_net)
                result = model.run()
                pad_pairs_tmp.append((result.distance, net, start, end, result))
        pad_pairs_tmp = prim_mst(pad_pairs_tmp)
        pad_pairs.extend(pad_pairs_tmp)


    pad_pairs = sorted(pad_pairs, key=lambda x: x[0])
    new_traces = dict(unrouted_grid.traces)

    for _, net, start, _, result in pad_pairs:
        pos = start
        if(net in new_traces.keys()):
            trace =  new_traces[net] | {pos}
        else:
            trace = {pos}

        for dir in result.directions:
            pos = Point(pos.x + dir.x, pos.y + dir.y)
            trace.add(pos)
        new_traces[net] = trace
        # print(trace)

    return Grid(
        pads=unrouted_grid.pads,
        traces=new_traces,
        diagonal_traces=unrouted_grid.diagonal_traces,
        width=unrouted_grid.width,
        height=unrouted_grid.height,
    )