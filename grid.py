from collections import defaultdict
from dataclasses import dataclass, field
from functools import total_ordering
from typing import Dict, Set, List, Tuple, Optional
import heapq
import math


@total_ordering
@dataclass(frozen=True)
class Point:
    x: int
    y: int

    def __lt__(self, other: 'Point') -> bool:
        return (self.x, self.y) < (other.x, other.y)


@total_ordering
@dataclass(frozen=True)
class PointPair:
    start: Point
    end: Point

    @staticmethod
    def new(point1: Point, point2: Point) -> 'PointPair':
        if point1 < point2:
            return PointPair(start=point1, end=point2)
        else:
            return PointPair(start=point2, end=point1)

    def __lt__(self, other: 'PointPair') -> bool:
        return (self.start, self.end) < (other.start, other.end)

@dataclass(frozen=True)
class Net:
    pad_c: str
    route_c: str


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
    
    def to_char_matrix(self) -> List[List[str]]:
        grid_string = [[' ' for _ in range(self.width)] for _ in range(self.height)]

        for net, points in self.pads.items():
            for point in points:
                assert 0 <= point.x < self.width and 0 <= point.y < self.height, "Point out of bounds"
                grid_string[point.y][point.x] = net.pad_c

        for net, points in self.traces.items():
            for point in points:
                # print(self.width, self.height, point)
                assert 0 <= point.x < self.width and 0 <= point.y < self.height, "Point out of bounds"
                grid_string[point.y][point.x] = net.route_c

        return grid_string

    @staticmethod
    def build_grid_string(char_matrix: List[List[str]]) -> str:
        width = len(char_matrix[0])
        horizontal_wall = '#' * (width + 2)
        lines = [horizontal_wall]

        for row in char_matrix:
            lines.append('#' + ''.join(row) + '#')

        lines.append(horizontal_wall)
        return '\n'.join(lines) + '\n'

    def __str__(self) -> str:
        return self.build_grid_string(self.to_char_matrix())

    def print(self):
        print(self.__str__())

    @classmethod
    def from_string(cls, s: str) -> 'Grid':
        lines = s.strip().split('\n')
        assert len(lines) >= 3, "Grid must have at least top/bottom walls and one content row"

        width = len(lines[0]) - 2
        content_lines = lines[1:-1]  # remove top/bottom wall
        height = len(content_lines)

        pads = defaultdict(set)

        for y, line in enumerate(content_lines):
            for x, c in enumerate(line):
                if 0 < x < len(line) - 1 and c not in (' ', '#'):
                    point = Point(x=x - 1, y=y)
                    net = Net(pad_c=c, route_c=c.lower())
                    pads[net].add(point)

        return cls(pads=dict(pads), traces=defaultdict(set), diagonal_traces=defaultdict(set), width=width, height=height)

