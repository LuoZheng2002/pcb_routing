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

def get_stride_aligned_neighbors(
    point: Point,
    stride: float,
    goal: Point
) -> List[Point]:
    """Get 8 neighboring points that are stride-aligned."""
    neighbors = []
    
    # Generate all 8 directions
    for dx in [-stride, 0, stride]:
        for dy in [-stride, 0, stride]:
            if dx == 0 and dy == 0:
                continue  # skip current point
            new_x = point.x + dx
            new_y = point.y + dy
            neighbors.append(Point(new_x, new_y))
    
    # Add goal if it's not aligned (to ensure we can reach it)
    if (goal.x % stride != 0 or goal.y % stride != 0) and goal not in neighbors:
        neighbors.append(goal)
    
    return neighbors

def line_rect_intersection(line_start, line_end, rect_min, rect_max):
    """Check if line segment intersects with rectangle."""
    # Liang-Barsky line clipping algorithm
    def clip(t0, t1, p, q):
        if p == 0:
            if q < 0:
                return False
        else:
            r = q / p
            if p < 0:
                if r > t1:
                    return False
                elif r > t0:
                    t0 = r
            else:
                if r < t0:
                    return False
                elif r < t1:
                    t1 = r
        return True, t0, t1

    x0, y0 = line_start
    x1, y1 = line_end
    xmin, ymin = rect_min
    xmax, ymax = rect_max
    
    t0, t1 = 0.0, 1.0
    dx = x1 - x0
    dy = y1 - y0
    
    # Left edge
    res = clip(t0, t1, -dx, x0 - xmin)
    if not res:
        return False
    t0, t1 = res[1], res[2]
    
    # Right edge
    res = clip(t0, t1, dx, xmax - x0)
    if not res:
        return False
    t0, t1 = res[1], res[2]
    
    # Bottom edge
    res = clip(t0, t1, -dy, y0 - ymin)
    if not res:
        return False
    t0, t1 = res[1], res[2]
    
    # Top edge
    res = clip(t0, t1, dy, ymax - y0)
    if not res:
        return False
    
    return True

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
    collision_check_fn: Callable[[Point, Point, float], bool]
) -> float:
    """Find maximum length in given direction that's collision-free."""
    low = 0.0
    high = max_length
    epsilon = 1e-3  # precision threshold
    
    best_length = 0.0
    for _ in range(20):  # limit iterations
        mid = (low + high) / 2
        end = (start.x + direction.x * mid, start.y + direction.y * mid)
        if is_collision_free(start, end, width, collision_check_fn):
            best_length = mid
            low = mid
        else:
            high = mid
        
        if high - low < epsilon:
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
        
        is_aligned = (current.x % stride == 0) and (current.y % stride == 0)
        
        if is_aligned:
            neighbors = get_stride_aligned_neighbors(current, stride, goal)
        else:
            neighbors = []
            base_x = round(current.x / stride) * stride
            base_y = round(current.y / stride) * stride
            
            for dx in [-stride, 0, stride]:
                for dy in [-stride, 0, stride]:
                    if dx == 0 and dy == 0:
                        continue
                    neighbor = Point(base_x + dx, base_y + dy)
                    if neighbor != current:
                        neighbors.append(neighbor)
            
            if (goal.x % stride != 0 or goal.y % stride != 0) and goal not in neighbors:
                neighbors.append(goal)
        
        for neighbor in neighbors:
            dx = neighbor.x - current.x
            dy = neighbor.y - current.y
            dist = math.hypot(dx, dy)
            direction = (dx/dist, dy/dist) if dist > 0 else (0, 0)
            
            if is_aligned and (neighbor.x % stride == 0) and (neighbor.y % stride == 0):
                if not is_collision_free(current, neighbor, trace_width, collision_check_fn):
                    continue
                tentative_g_score = g_score[current] + dist
                actual_end = neighbor
            else:
                max_length = min(dist, max_expansion_length)
                actual_length = binary_search_max_length(
                    current, direction, max_length, trace_width, collision_check_fn
                )
                
                if actual_length <= 1e-3:
                    continue
                
                actual_end = (
                    current.x + direction.x * actual_length,
                    current.y + direction.y * actual_length
                )
                tentative_g_score = g_score[current] + actual_length
            
            if actual_end not in g_score or tentative_g_score < g_score[actual_end]:
                came_from[actual_end] = current
                g_score[actual_end] = tentative_g_score
                f_score[actual_end] = g_score[actual_end] + octile_distance(actual_end, goal)
                if actual_end not in open_set_hash:
                    heapq.heappush(open_set, (f_score[actual_end], actual_end))
                    open_set_hash.add(actual_end)
    
    return None