

from typing import List, Dict, Set
from collections import defaultdict
from ordered_set import OrderedSet  # optional if order matters
import heapq

from dijkstra import DijkstraModel
from grid import Grid, Net, Point
from prim_mst import prim_mst




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