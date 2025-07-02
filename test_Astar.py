from Astar import a_star_implicit_grid, optimize_path
from grid import Grid, Net, Point, PointPair
import matplotlib.pyplot as plt
import matplotlib.patches as patches
from matplotlib.collections import LineCollection


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
goal_point = Point(4.0, 4.0)
stride_size = 1.0
trace_width = 0.1

# Find path
path = a_star_implicit_grid(
    start=start_point,
    goal=goal_point,
    stride=stride_size,
    trace_width=trace_width,
    collision_check_fn=collision_check_fn
)

print("Found path:", path)

def visualize_path(start, goal, obstacles, path, trace_width=0.2):
    """Visualize the path with obstacles on real coordinate axes using matplotlib."""
    fig, ax = plt.subplots(figsize=(10, 10))
    
    # Set axis limits with some padding
    all_x = [p.x for p in path] + [start.x, goal.x]
    all_y = [p.y for p in path] + [start.y, goal.y]
    ax.set_xlim(min(all_x) - 1, max(all_x) + 1)
    ax.set_ylim(min(all_y) - 1, max(all_y) + 1)
    ax.set_aspect('equal')
    ax.grid(True, which='both', linestyle='--', alpha=0.7)
    
    # Draw obstacles (as rectangles)
    for (x1, y1), (x2, y2) in obstacles:
        width = x2 - x1
        height = y2 - y1
        rect = patches.Rectangle(
            (x1, y1), width, height,
            linewidth=1, edgecolor='r', facecolor='red', alpha=0.5
        )
        ax.add_patch(rect)
    
    # Draw the path (as a thick line with rounded ends)
    if path:
        # Convert Point objects to coordinate arrays
        segments = []
        for i in range(len(path)-1):
            segments.append([
                [path[i].x, path[i].y],
                [path[i+1].x, path[i+1].y]
            ])
        
        lc = LineCollection(
            segments,
            linewidths=trace_width*10,  # Scale for visibility
            colors='blue',
            capstyle='round',
            joinstyle='round',
            alpha=0.8
        )
        ax.add_collection(lc)
        
        # Add points at each path node
        path_x = [p.x for p in path]
        path_y = [p.y for p in path]
        ax.scatter(path_x, path_y, color='blue', s=20, zorder=3)
    
    # Mark start and goal points
    ax.scatter([start.x], [start.y], color='green', s=100, label='Start', zorder=4)
    ax.scatter([goal.x], [goal.y], color='purple', s=100, label='Goal', zorder=4)
    
    # Add labels and legend
    ax.set_xlabel('X coordinate (inches)')
    ax.set_ylabel('Y coordinate (inches)')
    ax.set_title('PCB Routing Path with Obstacles')
    ax.legend()
    
    plt.show()

# Run the visualization with your path
visualize_path(start_point, goal_point, obstacles, path, trace_width)


path = [
    Point(1.0, 0.5),   # P0
    Point(1.0, 1.5),   # ↑ P1
    Point(2.0, 2.5),   # ↗ P2
    Point(3.0, 2.5),   # → P3
    Point(4.0, 2.5)    # → P4
]
optimized = optimize_path(
    path=path,
    trace_width=0.1,
    collision_check_fn=collision_check_fn,
    stride=1.0,
    epsilon=1e-6
)

print("Original path:")
for p in path:
    print(f"({p.x:.2f}, {p.y:.2f})")

print("\nOptimized path:")
for p in optimized:
    print(f"({p.x:.2f}, {p.y:.2f})")

visualize_path(path[0], path[-1], obstacles, path)
visualize_path(optimized[0], optimized[-1], obstacles, optimized)