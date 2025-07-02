from grid import Point
import heapq
import math
from typing import List, Optional, Callable
from dataclasses import dataclass
from typing import Tuple
import numpy as np

@dataclass
class Direction:
    x: float
    y: float

def octile_distance(a: Point, b: Point) -> float:
    dx = abs(a.x - b.x)
    dy = abs(a.y - b.y)
    return max(dx, dy) + (math.sqrt(2) - 1) * min(dx, dy)

def is_collision_free(
    start: Point,
    end: Point,
    width: float,
    collision_check_fn: Callable[[Point, Point, float], bool]
) -> bool:
    return not collision_check_fn(start, end, width)

def binary_search_max_length(
    start: Point,
    direction: Direction,
    max_length: float,
    width: float,
    collision_check_fn: Callable[[Point, Point, float], bool],
    epsilon: float
) -> float:
    low, high = 0.0, max_length
    best = 0.0
    for _ in range(20):
        mid = (low + high) / 2
        end = Point(start.x + direction.x * mid, start.y + direction.y * mid)
        if is_collision_free(start, end, width, collision_check_fn):
            best = mid
            low = mid
        else:
            high = mid
        if high - low < epsilon:
            break
    return best

def get_standard_direction(dx: float, dy: float) -> Direction:
    angle = math.atan2(dy, dx)
    std_angles = [0, math.pi/4, math.pi/2, 3*math.pi/4,
                 math.pi, 5*math.pi/4, 3*math.pi/2, 7*math.pi/4]
    closest_angle = min(std_angles, key=lambda a: abs(a - angle))
    return Direction(math.cos(closest_angle), math.sin(closest_angle))

def optimize_path(path: List[Point], 
                             trace_width: float,
                             collision_check_fn: Callable[[Point, Point, float], bool],
                             stride: float, epsilon) -> List[Point]:

    if len(path) < 4:
        return path
    
    optimized = path.copy()
    i = 0
    while i < len(optimized) - 3:
        seg1 = (optimized[i], optimized[i+1])   
        seg2 = (optimized[i+2], optimized[i+3])
        

        dx1 = seg1[1].x - seg1[0].x
        dy1 = seg1[1].y - seg1[0].y
        dx2 = seg2[1].x - seg2[0].x
        dy2 = seg2[1].y - seg2[0].y
        
        if (abs(dx1 - dx2) < epsilon and abs(dy1 - dy2) < epsilon):
            success = False
            new_point1 = Point(seg1[0].x + seg2[0].x - seg1[1].x, seg1[0].y + seg2[0].y - seg1[1].y)
            new_point2 = Point(seg2[1].x - seg2[0].x + seg1[1].x, seg2[1].y - seg2[0].y + seg1[1].y)
            flag1 = is_collision_free(optimized[i], new_point1, trace_width, collision_check_fn) and \
                is_collision_free(new_point1, optimized[i+2], trace_width, collision_check_fn)
            flag2 = is_collision_free(optimized[i+1], new_point2, trace_width, collision_check_fn) and \
                is_collision_free(new_point2, optimized[i+3], trace_width, collision_check_fn)

            if (flag1):
                optimized[i+1] = new_point1
                success = True
            elif (flag2): 
                optimized[i+2] = new_point2
                success = True
            
            if success:
                i += 3  
                continue
        i += 1

    i = 1
    while i < len(optimized) - 2:
        p0 = optimized[i - 1]
        p1 = optimized[i]
        p2 = optimized[i + 1]
        p3 = optimized[i + 2]

        d01 = (p0.x - p1.x, p0.y - p1.y)
        d12 = (p2.x - p1.x, p2.y - p1.y)
        d23 = (p3.x - p2.x, p3.y - p2.y)

        if is_axis(d01) and is_diagonal(d12) and is_axis(d23) and (d01[0] == 0 or d23[0] == 0) :

            for step in np.arange(1.0, 0, -0.1):
                new_point1 = Point(
                    p1.x + d01[0] * step * stride,
                    p1.y + d01[1] * step * stride
                )
                new_point2 = Point(
                    p2.x + d23[0] * step * stride,
                    p2.y + d23[1] * step * stride
                )

                if (is_collision_free(p0, new_point1, trace_width, collision_check_fn) and
                    is_collision_free(new_point1, new_point2, trace_width, collision_check_fn) and 
                    is_collision_free(new_point2, p3, trace_width, collision_check_fn)):
                    optimized[i] = new_point1
                    optimized[i + 1] = new_point2
                    break
        i += 1
    
    return optimized

def is_axis(d: Tuple[float, float]) -> bool:
    return (abs(d[0]) < 1e-6 and abs(d[1]) > 1e-6) or \
           (abs(d[1]) < 1e-6 and abs(d[0]) > 1e-6)

def is_diagonal(d: Tuple[float, float]) -> bool:
    return abs(abs(d[0]) - abs(d[1])) < 1e-6 and abs(d[0]) > 0 and abs(d[1]) > 0



def a_star_implicit_grid(
    start: Point,
    goal: Point,
    stride: float,
    trace_width: float,
    collision_check_fn: Callable[[Point, Point, float], bool],
    max_expansion_length: float = float('inf')
) -> Optional[List[Point]]:
    # Initial checks
    if collision_check_fn(start, start, trace_width) or \
       collision_check_fn(goal, goal, trace_width):
        return None

    # Initialize
    open_set = []
    heapq.heappush(open_set, (octile_distance(start, goal), start))
    
    came_from = {}
    g_score = {start: 0.0}
    f_score = {start: octile_distance(start, goal)}
    open_set_hash = {start}
    EPSILON = 1e-6

    # Standard directions for 8-way movement
    std_directions = [
        Direction(1, 0), Direction(-1, 0), Direction(0, 1), Direction(0, -1),
        Direction(1, 1), Direction(1, -1), Direction(-1, 1), Direction(-1, -1)
    ]

    while open_set:
        current = heapq.heappop(open_set)[1]
        open_set_hash.remove(current)

        # Check if reached goal
        if math.hypot(current.x - goal.x, current.y - goal.y) < EPSILON:
            # Reconstruct path with 8-direction constraints
            path = [current]
            while current in came_from:
                current = came_from[current]
                path.append(current)
            path.reverse()

            # Process path segments to enforce 8-direction movement
            aligned_path = [path[0]]
            for i in range(1, len(path)):
                prev = aligned_path[-1]
                curr = path[i]

                dx = curr.x - prev.x
                dy = curr.y - prev.y
                dist = math.hypot(dx, dy)

                if dist < EPSILON:
                    continue

                # Get standard direction
                direction = get_standard_direction(dx, dy)
                moved_dist = dx * direction.x + dy * direction.y  # Projection

                # Find maximum safe length
                safe_len = binary_search_max_length(
                    prev, direction, moved_dist, trace_width, collision_check_fn, EPSILON
                )

                if safe_len < EPSILON:
                    continue  # Skip unreachable points

                new_point = Point(
                    prev.x + direction.x * safe_len,
                    prev.y + direction.y * safe_len
                )
                aligned_path.append(new_point)

            # Ensure final point reaches goal
            if len(aligned_path) >= 2:
                last_seg_start = aligned_path[-2]
                if not math.isclose(aligned_path[-1].x, goal.x, abs_tol=EPSILON) or \
                   not math.isclose(aligned_path[-1].y, goal.y, abs_tol=EPSILON):
                    # Add final segment to goal if possible
                    if is_collision_free(last_seg_start, goal, trace_width, collision_check_fn):
                        aligned_path[-1] = goal
                    else:
                        # Find intermediate point
                        dx = goal.x - last_seg_start.x
                        dy = goal.y - last_seg_start.y
                        direction = get_standard_direction(dx, dy)
                        max_len = math.hypot(dx, dy)
                        safe_len = binary_search_max_length(
                            last_seg_start, direction, max_len, trace_width, collision_check_fn, EPSILON
                        )
                        if safe_len > EPSILON:
                            intermediate = Point(
                                last_seg_start.x + direction.x * safe_len,
                                last_seg_start.y + direction.y * safe_len
                            )
                            aligned_path[-1] = intermediate
                            aligned_path.append(goal)
                # if len(aligned_path) >= 4:  # long enough for optimization
                #     optimized_path = optimize_path(aligned_path, trace_width, collision_check_fn, stride,EPSILON)
                #     return optimized_path
            return aligned_path

        # Generate standard direction neighbors
        for direction in std_directions:
            neighbor = Point(
                current.x + direction.x * stride,
                current.y + direction.y * stride
            )

            # Skip if beyond max expansion length
            dist = math.hypot(direction.x * stride, direction.y * stride)
            if dist > max_expansion_length:
                continue

            # Collision check
            if not is_collision_free(current, neighbor, trace_width, collision_check_fn):
                # Try to find maximum safe length
                safe_len = binary_search_max_length(
                    current, direction, stride, trace_width, collision_check_fn, EPSILON
                )
                if safe_len < EPSILON:
                    continue
                neighbor = Point(
                    current.x + direction.x * safe_len,
                    current.y + direction.y * safe_len
                )

            # Update scores
            tentative_g = g_score[current] + math.hypot(
                neighbor.x - current.x, neighbor.y - current.y)
            
            if neighbor not in g_score or tentative_g < g_score[neighbor]:
                came_from[neighbor] = current
                g_score[neighbor] = tentative_g
                f_score[neighbor] = tentative_g + octile_distance(neighbor, goal)
                if neighbor not in open_set_hash:
                    heapq.heappush(open_set, (f_score[neighbor], neighbor))
                    open_set_hash.add(neighbor)

    return None