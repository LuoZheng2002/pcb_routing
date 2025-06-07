
from typing import List, Dict, Set
from collections import defaultdict
from ordered_set import OrderedSet  # optional if order matters
import heapq
from grid import Net, Point, PointPair




def prim_mst(pad_pairs: List[tuple[float, Net, PointPair]]) -> List[tuple[float, Net, PointPair]]:
    if not pad_pairs:
        return []

    points = set()
    for _, _, point_pair in pad_pairs:
        points.add(point_pair.start)
        points.add(point_pair.end)
    points = list(points)

    adj = {}
    for dist, net, point_pair in pad_pairs:
        start = point_pair.start
        end = point_pair.end
        adj[(start, end)] = (dist, net)
        adj[(end, start)] = (dist, net)  

    mst_edges = []
    visited = set()
    start_point = points[0]
    visited.add(start_point)

    heap = []
    for (u, v), (dist, net) in adj.items():
        if u == start_point:
            heapq.heappush(heap, (dist, net, u, v))

    while heap and len(visited) < len(points):
        dist, net, u, v = heapq.heappop(heap)
        if v not in visited:
            visited.add(v)
            mst_edges.append((dist, net, PointPair.new(u, v)))
            for (u_new, v_new), (dist_new, net_new) in adj.items():
                if u_new == v and v_new not in visited:
                    heapq.heappush(heap, (dist_new, net_new, u_new, v_new))

    return mst_edges