import heapq
import math
from typing import List, Tuple, Optional, Callable
from grid import Grid, Net, Point, PointPair
from dataclasses import dataclass, field

@dataclass
class Direction:
    x: float
    y: float

def octile_distance(a: Point, b: Point) -> float:
    """Octile distance heuristic for 8-direction movement."""
    dx = abs(a.x - b.x)
    dy = abs(a.y - b.y)
    return max(dx, dy) + (math.sqrt(2) - 1) * min(dx, dy)

def get_neighbors(
    point: Point,
    stride: float,
    goal: Point,
    goal_aligned: bool
) -> List[Point]:
    """Get 8 neighboring points that are stride-aligned."""
    neighbors = []
    
    # Generate all 8 directions
    for dx in [-stride, 0, stride]:
        for dy in [-stride, 0, stride]:
            if dx == 0 and dy == 0:
                continue 
            new_x = point.x + dx
            new_y = point.y + dy
            neighbors.append(Point(new_x, new_y))
    
    # Add goal if it's not aligned to ensure we can reach it
    if not goal_aligned and goal not in neighbors:
        neighbors.append(goal)
    
    return neighbors

def is_collision_free(
    start: Point,
    end: Point,
    width: float,
    collision_check_fn: Callable[[Point, Point, float], bool]
) -> bool:
    """Check if a capsule-shaped trace between points is collision-free."""
    return not collision_check_fn(start, end, width)

def binary_search_max_length(
    start: Point,
    direction: Direction,
    max_length: float,
    width: float,
    collision_check_fn: Callable[[Point, Point, float], bool],
    EPSILON: float
) -> float:
    """Find maximum length in given direction that's collision-free."""
    low = 0.0
    high = max_length
    best_length = 0.0
    for _ in range(20):  # limit iterations
        mid = (low + high) / 2
        end = Point(start.x + direction.x * mid, start.y + direction.y * mid)
        if is_collision_free(start, end, width, collision_check_fn):
            best_length = math.hypot(direction.x * mid, direction.y * mid)
            low = mid
        else:
            high = mid
        
        if high - low < EPSILON:
            break
    
    return best_length

def a_star_implicit_grid(
    start: Point,
    goal: Point,
    stride: float,
    trace_width: float,
    collision_check_fn: Callable[[Point, Point, float], bool],
    max_expansion_length: float = float('inf')
) -> Optional[List[Point]]:
    """A* algorithm for implicit grid with capsule-shaped traces."""
    # Check if start or goal are in collision
    if collision_check_fn(start, start, trace_width) or collision_check_fn(goal, goal, trace_width):
        return None
    
    open_set = []
    heapq.heappush(open_set, (octile_distance(start, goal), start))
    
    came_from = {}
    g_score = {start: 0.0}
    f_score = {start: octile_distance(start, goal)}
    open_set_hash = {start}
    
    # Floating point comparison tolerance
    EPSILON = stride * 0.1
    
    while open_set:
        current = heapq.heappop(open_set)[1]
        open_set_hash.remove(current)
        
        if current == goal:
            path = [current]
            while current in came_from:
                current = came_from[current]
                path.append(current)
            path.reverse()
            return path

        # Alignment check
        def is_aligned(p):
            return (round(p.x / stride) * stride == p.x) and \
                   (round(p.y / stride) * stride == p.y)
        
        current_aligned = is_aligned(current)
        goal_aligned = is_aligned(goal)
        
        neighbors = get_neighbors(current, stride, goal, goal_aligned)
        
        for neighbor in neighbors:
            dx = neighbor.x - current.x
            dy = neighbor.y - current.y
            dist = math.hypot(dx, dy)
            
            if dist < EPSILON:
                continue
                
            direction = Direction(dx/dist, dy/dist)
            
            if is_collision_free(current, neighbor, trace_width, collision_check_fn):
                tentative_g_score = g_score[current] + dist
                actual_end = neighbor
            else:
                max_length = min(dist, max_expansion_length)
                actual_length = binary_search_max_length(
                    current, direction, max_length, trace_width, collision_check_fn, EPSILON
                )
                
                if actual_length < EPSILON:  # Minimum meaningful expansion
                    continue
                
                actual_end = Point(
                    current.x + direction.x * actual_length,
                    current.y + direction.y * actual_length
                )
                tentative_g_score = g_score[current] + actual_length
            
            # Check if reach the goal
            if math.hypot(actual_end.x - goal.x, actual_end.y - goal.y) < EPSILON:
                actual_end = goal
            
            if actual_end not in g_score or tentative_g_score < g_score[actual_end]:
                came_from[actual_end] = current
                g_score[actual_end] = tentative_g_score
                f_score[actual_end] = tentative_g_score + octile_distance(actual_end, goal)
                if actual_end not in open_set_hash:
                    heapq.heappush(open_set, (f_score[actual_end], actual_end))
                    open_set_hash.add(actual_end)
    
    return None