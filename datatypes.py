from dataclasses import dataclass, field
from typing import Dict, Set, List, Tuple, Optional
import heapq
import math

@dataclass(frozen=True)
class Net:
    pad_c: str
    route_c: str

@dataclass(frozen=True, order=True)
class Point:
    x: int
    y: int

@dataclass
class Direction:
    x: int
    y: int

@dataclass
class Grid:
    pads: Dict[Net, Set[Point]]
    traces: Dict[Net, Set[Point]]
    diagonal_traces: Dict[Net, Set[Point]]
    width: int
    height: int

    def pads_except(self, net: Net) -> Set[Point]:
        return {p for n, ps in self.pads.items() if n != net for p in ps}

    def routes_except(self, net: Net) -> Set[Point]:
        return {p for n, ps in self.traces.items() if n != net for p in ps}

    def diagonal_routes_except(self, net: Net) -> Set[Point]:
        return {p for n, ps in self.diagonal_traces.items() if n != net for p in ps}

@dataclass
class DijkstraResult:
    start: Point
    directions: List[Direction]
    distance: float

@dataclass
class DijkstraModel:
    width: int
    height: int
    obstacles: Set[Point]
    diagonal_obstacles: Set[Point]
    start: Point
    end: Point

    def run(self) -> DijkstraResult:
        class State:
            def __init__(self, cost: float, position: Point):
                self.cost = cost
                self.position = position
            def __lt__(self, other):
                return self.cost < other.cost

        heap = [State(0.0, self.start)]
        dist = {self.start: 0.0}
        prev = {}

        cardinal_dirs = [(0, -1), (0, 1), (-1, 0), (1, 0)]
        diagonal_dirs = [(1, 1), (1, -1), (-1, 1), (-1, -1)]

        while heap:
            state = heapq.heappop(heap)
            cost, position = state.cost, state.position

            if position == self.end:
                break

            if dist.get(position, float('inf')) < cost:
                continue

            for dx, dy in cardinal_dirs:
                next_point = self.offset_point(position, dx, dy)
                if next_point and next_point not in self.obstacles:
                    new_cost = cost + 1.0
                    if new_cost < dist.get(next_point, float('inf')):
                        dist[next_point] = new_cost
                        prev[next_point] = position
                        heapq.heappush(heap, State(new_cost, next_point))

            for dx, dy in diagonal_dirs:
                next_point = self.offset_point(position, dx, dy)
                if next_point:
                    top_left = self.offset_point(position, min(dx, 0), min(dy, 0))
                    if (next_point not in self.obstacles and
                        top_left and top_left not in self.diagonal_obstacles):
                        new_cost = cost + math.sqrt(2)
                        if new_cost < dist.get(next_point, float('inf')):
                            dist[next_point] = new_cost
                            prev[next_point] = position
                            heapq.heappush(heap, State(new_cost, next_point))

        directions = []
        current = self.end
        while current != self.start:
            if current not in prev:
                return DijkstraResult(self.start, [], float('inf'))
            prev_point = prev[current]
            directions.append(Direction(current.x - prev_point.x, current.y - prev_point.y))
            current = prev_point

        directions.reverse()
        return DijkstraResult(self.start, directions, dist.get(self.end, float('inf')))

    def offset_point(self, point: Point, dx: int, dy: int) -> Optional[Point]:
        nx, ny = point.x + dx, point.y + dy
        if 0 <= nx < self.width and 0 <= ny < self.height:
            return Point(nx, ny)
        return None