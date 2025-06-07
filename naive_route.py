

from typing import List, Dict, Set
from collections import defaultdict
from ordered_set import OrderedSet  # optional if order matters
import heapq

from dijkstra import DijkstraModel
from grid import Grid, Net, Point, PointPair
from prim_mst import prim_mst

# Assuming these classes are defined elsewhere in your Python code
# Point, Grid, Net, PointPair, DijkstraModel, DijkstraResult

def naive_route(unrouted_grid: Grid) -> Grid:
    unrouted_grid.traces.clear()
    unrouted_grid.diagonal_traces.clear()
    
    def prepare_dijkstra_model_unrouted(net: Net, start: Point, end: Point) -> DijkstraModel:
        other_pads = unrouted_grid.pads_except(net)
        return DijkstraModel(
            width=unrouted_grid.width,
            height=unrouted_grid.height,
            obstacles=other_pads,
            diagonal_obstacles=set(),  # no diagonal obstacles in the unrouted grid
            start=start,
            end=end
        )
    
    # Prepare all the pairs of pads to route
    pad_pairs = []
    for net, points in unrouted_grid.pads.items():
        points_list = list(points)
        # Permutate all pairs and calculate their distance using Dijkstra's algorithm
        pairs = []
        for i in range(len(points_list)):
            for j in range(i + 1, len(points_list)):
                point1 = points_list[i]
                point2 = points_list[j]
                dijkstra_model = prepare_dijkstra_model_unrouted(net, point1, point2)
                try:
                    dijkstra_result = dijkstra_model.run()
                    distance = dijkstra_result.distance
                except:
                    distance = float('inf')
                pairs.append((distance, net, PointPair(point1, point2)))
        
        # Prim's algorithm (assuming prim_mst is implemented elsewhere)
        pairs = prim_mst(pairs)
        pad_pairs.extend(pairs)
    
    # Create a priority queue (min-heap)
    priority_queue = []
    items_by_distance = defaultdict(list)
    
    for distance, net, point_pair in pad_pairs:
        heapq.heappush(priority_queue, distance)
        items_by_distance[distance].append((net, point_pair))
    
    def prepare_dijkstra_model(net: Net, start: Point, end: Point) -> DijkstraModel:
        other_pads = unrouted_grid.pads_except(net)
        other_routes = unrouted_grid.routes_except(net)
        other_diagonal_routes = unrouted_grid.diagonal_routes_except(net)
        obstacles = other_pads.union(other_routes)
        return DijkstraModel(
            width=unrouted_grid.width,
            height=unrouted_grid.height,
            obstacles=obstacles,
            diagonal_obstacles=other_diagonal_routes,
            start=start,
            end=end
        )
    
    while priority_queue:
        distance = heapq.heappop(priority_queue)
        if not items_by_distance[distance]:
            continue  # skip if we've already processed all items with this distance
        net, point_pair = items_by_distance[distance].pop(0)
        
        # Construct dijkstra model for the current pair of pads
        dijkstra_model = prepare_dijkstra_model(net, point_pair.start, point_pair.end)
        
        # Run dijkstra's algorithm
        try:
            dijkstra_result = dijkstra_model.run()
            directions = dijkstra_result.directions
        except Exception as e:
            raise RuntimeError(f"Failed to run Dijkstra's algorithm: {e}")
        
        # Add the route to the grid
        current_point = point_pair.start
        if net not in unrouted_grid.traces:
            unrouted_grid.traces[net] = set()
        unrouted_grid.traces[net].add(current_point)
        
        for direction in directions:
            last_point = current_point
            current_point = Point(
                x=(current_point.x + direction.x),
                y=(current_point.y + direction.y)
            )
            assert current_point.x < unrouted_grid.width and current_point.y < unrouted_grid.height, "Point out of bounds"
            unrouted_grid.traces[net].add(current_point)
            
            if direction.x != 0 and direction.y != 0:
                # If the direction is diagonal, add the diagonal trace
                diagonal_trace_point = Point(
                    x=min(last_point.x, current_point.x),
                    y=min(last_point.y, current_point.y)
                )
                if net not in unrouted_grid.diagonal_traces:
                    unrouted_grid.diagonal_traces[net] = set()
                unrouted_grid.diagonal_traces[net].add(diagonal_trace_point)
    
    assert not priority_queue or all(not items for items in items_by_distance.values())
    return unrouted_grid