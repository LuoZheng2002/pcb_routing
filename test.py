import unittest
from typing import List, Tuple, Optional
from Astar import a_star_implicit_grid
from grid import Grid, Net, Point, PointPair  

class TestAStarImplicitGrid(unittest.TestCase):
    def setUp(self):
        # Define some rectangular obstacles (xmin, ymin, xmax, ymax)
        self.obstacles = [
            (1.0, 1.0, 2.0, 3.0),  # Rectangle obstacle 1
            (3.0, 2.0, 4.0, 4.0),  # Rectangle obstacle 2
            (1.5, 4.0, 3.5, 5.0)   # Rectangle obstacle 3
        ]
        
        # Collision check function for rectangular obstacles
        def collision_check_fn(start: Point, end: Point, width: float) -> bool:
            # Check if either point is inside an obstacle
            for (xmin, ymin, xmax, ymax) in self.obstacles:
                # Check start point
                if (xmin <= start.x <= xmax) and (ymin <= start.y <= ymax):
                    return True
                # Check end point
                if (xmin <= end.x <= xmax) and (ymin <= end.y <= ymax):
                    return True
                
                # Check line segment intersection with padding for width
                padding = width/2
                if line_rect_intersection(
                    (start.x, start.y), (end.x, end.y),
                    (xmin - padding, ymin - padding),
                    (xmax + padding, ymax + padding)
                ):
                    return True
            
            return False
        
        self.collision_check_fn = collision_check_fn

    def test_unobstructed_path(self):
        """Test a simple path with no obstacles."""
        start = Point(0.0, 0.0)
        goal = Point(5.0, 5.0)
        stride = 0.1
        trace_width = 0.2
        
        path = a_star_implicit_grid(
            start, goal, stride, trace_width, self.collision_check_fn
        )
        
        self.assertIsNotNone(path)
        self.assertEqual(path[0], start)
        self.assertEqual(path[-1], goal)
        print("Unobstructed path found:", [(p.x, p.y) for p in path])

    def test_obstructed_path(self):
        """Test a path that needs to navigate around obstacles."""
        start = Point(0.0, 0.0)
        goal = Point(5.0, 5.0)
        stride = 0.1
        trace_width = 0.2
        
        path = a_star_implicit_grid(
            start, goal, stride, trace_width, self.collision_check_fn
        )
        
        self.assertIsNotNone(path)
        self.assertEqual(path[0], start)
        self.assertEqual(path[-1], goal)
        print("Obstructed path found:", [(p.x, p.y) for p in path])

    def test_narrow_passage(self):
        """Test a path through a narrow gap between obstacles."""
        # Add two close obstacles with a small gap
        self.obstacles.append((2.0, 2.0, 2.5, 4.0))
        self.obstacles.append((2.5, 1.0, 3.0, 2.5))
        
        start = Point(2.0, 0.0)
        goal = Point(2.75, 3.0)  # Goal in the narrow passage
        stride = 0.1
        trace_width = 0.1  # Smaller width to fit through gap
        
        path = a_star_implicit_grid(
            start, goal, stride, trace_width, self.collision_check_fn
        )
        
        self.assertIsNotNone(path)
        self.assertEqual(path[0], start)
        self.assertEqual(path[-1], goal)
        print("Narrow passage path found:", [(p.x, p.y) for p in path])

    def test_no_path(self):
        """Test when no valid path exists."""
        # Create a wall of obstacles
        self.obstacles = [(x, 2.0, x+1.0, 3.0) for x in [0.0, 1.0, 2.0, 3.0, 4.0]]
        
        start = Point(2.5, 0.0)
        goal = Point(2.5, 5.0)  # Blocked by the wall
        stride = 0.1
        trace_width = 0.2
        
        path = a_star_implicit_grid(
            start, goal, stride, trace_width, self.collision_check_fn
        )
        
        self.assertIsNone(path)
        print("Correctly returned None when no path exists")

def line_rect_intersection(line_start, line_end, rect_min, rect_max):
    """Check if line segment intersects with rectangle (for testing)."""
    # Implement Liang-Barsky or use simple segment-rectangle intersection
    # This is a simplified version for testing
    x0, y0 = line_start
    x1, y1 = line_end
    xmin, ymin = rect_min
    xmax, ymax = rect_max
    
    # Check if either point is inside rectangle
    if (xmin <= x0 <= xmax and ymin <= y0 <= ymax) or \
       (xmin <= x1 <= xmax and ymin <= y1 <= ymax):
        return True
    
    # Check line segment against rectangle edges
    def ccw(A, B, C):
        return (C[1]-A[1])*(B[0]-A[0]) > (B[1]-A[1])*(C[0]-A[0])
    
    def intersect(A, B, C, D):
        return ccw(A,C,D) != ccw(B,C,D) and ccw(A,B,C) != ccw(A,B,D)
    
    # Rectangle edges
    edges = [
        ((xmin, ymin), (xmax, ymin)),
        ((xmax, ymin), (xmax, ymax)),
        ((xmax, ymax), (xmin, ymax)),
        ((xmin, ymax), (xmin, ymin))
    ]
    
    for edge in edges:
        if intersect(line_start, line_end, edge[0], edge[1]):
            return True
    
    return False

if __name__ == "__main__":
    unittest.main()