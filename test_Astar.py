from Astar import a_star_implicit_grid
from grid import Grid, Net, Point, PointPair

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

def create_pcb_collision_checker(obstacles):
    """Create a collision checker function for PCB obstacles."""
    def collision_check(start, end, width):
        # Check if either endpoint is inside an obstacle
        for (x1, y1), (x2, y2) in obstacles:
            if (start.x >= x1 and start.x <= x2 and start.y >= y1 and start.y <= y2):
                return True
            if (end.x >= x1 and end.x <= x2 and end.y >= y1 and end.y <= y2):
                return True
        
        # Check if the line segment intersects any obstacle
        for (x1, y1), (x2, y2) in obstacles:
            if line_rect_intersection((start.x, start.y), (end.x, end.y), (x1, y1), (x2, y2)):
                return True
        
        return False
    return collision_check

# Define a simple PCB layout with guaranteed path
obstacles = [
    ((2, 1), (3, 2)),  # Obstacle in the middle
    ((4, 0), (5, 1))   # Small obstacle in corner
]

collision_check_fn = create_pcb_collision_checker(obstacles)

# Define parameters
start_point = Point(0.5, 0.0)
goal_point = Point(5.0, 5.0)
stride_size = 1.0
trace_width = 0.2

# Find path
path = a_star_implicit_grid(
    start=start_point,
    goal=goal_point,
    stride=stride_size,
    trace_width=trace_width,
    collision_check_fn=collision_check_fn
)

print("Found path:", path)

# Visualize the path (simple ASCII representation)
def visualize_path(start, goal, obstacles, path, stride=1.0):
    grid_size = 6
    grid = [['.' for _ in range(grid_size)] for _ in range(grid_size)]
    
    # Mark obstacles
    for (x1, y1), (x2, y2) in obstacles:
        for x in range(int(x1), int(x2)+1):
            for y in range(int(y1), int(y2)+1):
                if 0 <= x < grid_size and 0 <= y < grid_size:
                    grid[y][x] = 'X'
    
    # Mark path
    if path:
        for point in path:
            ix, iy = int(round(point.x)), int(round(point.y))
            if 0 <= ix < grid_size and 0 <= iy < grid_size:
                if grid[iy][ix] == '.':
                    grid[iy][ix] = '*'
    
    # Mark start and goal
    sx, sy = int(round(start.x)), int(round(start.y))
    gx, gy = int(round(goal.x)), int(round(goal.y))
    if 0 <= sx < grid_size and 0 <= sy < grid_size:
        grid[sy][sx] = 'S'
    if 0 <= gx < grid_size and 0 <= gy < grid_size:
        grid[gy][gx] = 'G'
    
    # Print grid
    for row in reversed(grid):
        print(' '.join(row))

visualize_path(start_point, goal_point, obstacles, path, stride_size)