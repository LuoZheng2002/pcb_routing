
from typing import List, Dict, Set
from collections import defaultdict
from ordered_set import OrderedSet  # optional if order matters
import heapq

from dijkstra import DijkstraResult
from grid import Net, Point




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