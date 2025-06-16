from dataclasses import dataclass
from typing import List, Tuple, Set, Dict
from collections import defaultdict

from grid import Point, Net, Grid

@dataclass
class TraceCandidate:
    start: Point
    end: Point
    path: Set[Point] # all points in the trace
    net_id: str
    probability: float


def generate_all_traces(traces: List[TraceCandidate]) -> Dict[Tuple[Point, Point], List[TraceCandidate]]:
    traces_by_prob = defaultdict(list)

    for trace in traces:
        key = (trace.start, trace.end)
        traces_by_prob[key].append(trace)
        
    for pair in traces_by_prob:
        traces_by_prob[pair].sort(key=lambda t: t.probability, reverse=True)

    return traces_by_prob

def cross_conflict(path1: Set[Point], path2: Set[Point]) -> bool:
    for p in path1:
        # check diagonal conflict
        diag1 = Point(p.x + 1, p.y + 1)
        if diag1 in path1 and Point(p.x, p.y + 1) in path2 and Point(p.x + 1, p.y) in path2:
            return True
        diag2 = Point(p.x + 1, p.y - 1)
        if diag2 in path1 and Point(p.x, p.y - 1) in path2 and Point(p.x + 1, p.y) in path2:
            return True
    return False

def backtrack_traces(traces_by_prob: Dict[Tuple[Point, Point], List[TraceCandidate]]) -> Dict[Tuple[Point, Point], List[TraceCandidate]]:
    
    result = []
    # occupied = set()
    keys = list(traces_by_prob.keys())
    
    def backtrack(index) -> bool:
        if index == len(keys):
            return True
        
        key = keys[index]
        candidates = traces_by_prob[key] # candidates for current pair
        
        for trace in candidates:
            
            conflict = False
            
            for other_key, other_trace in result:
                # skip for same net_id
                if trace.net_id == other_trace.net_id:
                    continue

                # check occupied points
                if trace.path & other_trace.path:
                    conflict = True
                    break

                # check diagonal cross conflict
                if cross_conflict(trace.path, other_trace.path):
                    conflict = True
                    break

            if conflict:
                continue
            else: 
                result.append((key, trace))
                # occupied.update(trace.path)
                
            
            if backtrack(index + 1):
                return True
            else:
                result.pop()
                # occupied.difference_update(trace.path)
            
        return False 
    
    success = backtrack(0)
    if success:
        return dict(result)  # return dict[(start, end)] = trace
    else:
        print("No available traces!")
        return None
