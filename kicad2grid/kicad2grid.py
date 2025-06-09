import numpy as np
import re
import math
import random
from collections import defaultdict

from grid import Point, Net, Grid


file_path = "tutorial.kicad_pcb"
grid_size = 2.54



def read_kicad_pcb(file_path):

    with open(file_path, 'r', encoding='utf-8') as f:
        return f.read()
    
def extract_footprints(content):
    """extract the position and the rotation of footprints"""
    footprints = []
    start = 0

    while True:
        start = content.find("(footprint ", start)
        if start == -1:
            break

        depth = 0
        end = start
        while end < len(content):
            if content[end] == '(':
                depth += 1
            elif content[end] == ')':
                depth -= 1
                if depth == 0:
                    break
            end += 1

        footprint_block = content[start:end+1]

        # extract name, position and rotation
        name_match = re.search(r'\(footprint\s+"(.*?)"', footprint_block)
        at_match = re.search(r'\(at\s+([\d\.-]+)\s+([\d\.-]+)(?:\s+([\d\.-]+))?', footprint_block)

        if name_match and at_match:
            name = name_match.group(1)
            x = float(at_match.group(1))
            y = float(at_match.group(2))
            rotation = float(at_match.group(3)) if at_match.group(3) else 0.0

            footprints.append({
                "name": name,
                "x": x,
                "y": y,
                "rotation": rotation,
                "body": footprint_block  
            })

        start = end + 1

    return footprints

def extract_pads(footprints):
    pads = []
    pattern = r'\(pad\s+"?\d+"?.*?\(at\s+([\d\.-]+)\s+([\d\.-]+)(?:\s+([\d\.-]+))?\).*?\(net\s+(\d)+\s+".*?"\)'
    for match in re.finditer(pattern, footprints["body"], re.DOTALL):
        xp, yp = float(match.group(1)), float(match.group(2))
        pad_angle = float(match.group(3)) if match.group(3) else 0.0
        net = match.group(4)

        # compute absolute position
        rotate = math.radians(footprints["rotation"])
        x_abs = footprints["x"] + xp * math.cos(rotate) - yp * math.sin(rotate)
        y_abs = footprints["y"] + xp * math.sin(rotate) + yp * math.cos(rotate)

        pads.append({"x": round(x_abs, 4), "y": round(y_abs, 4), "net": net, "footprint": footprints["name"]})
    return pads


def extract_vias(content):
    """extract position and net num of vias"""
    vias = []
    pattern = r'\(via\s+.*?\(at\s+([\d\.-]+)\s+([\d\.-]+)\)\s+.*?\(net\s+(\d+)\)?'
    for match in re.finditer(pattern, content, re.DOTALL):
        x, y = float(match.group(1)), float(match.group(2))
        net = match.group(3)
        vias.append({"x": x, "y": y, "net": net})
    return vias

'''def extract_segments(content):
    """提取所有 segment 数据"""
    segments = []
    pattern = r'\(segment (.*?)\)'
    for match in re.finditer(pattern, content, re.DOTALL):
        seg_block = match.group(1)
        # 提取起点、终点和网络
        start_match = re.search(r'\(start ([\d.-]+) ([\d.-]+)', seg_block)
        end_match = re.search(r'\(end ([\d.-]+) ([\d.-]+)', seg_block)
        net_match = re.search(r'\(net (\d+) "(.*?)"\)', seg_block)
        if start_match and end_match:
            x1, y1 = float(start_match.group(1)), float(start_match.group(2))
            x2, y2 = float(end_match.group(1)), float(end_match.group(2))
            net = net_match.group(2) if net_match else None
            segments.append({
                "type": "segment",
                "x1": x1,
                "y1": y1,
                "x2": x2,
                "y2": y2,
                "net": net
            })
    return segments
'''



def align_to_grid(x, y, grid_size):

    x_aligned = round(round(x / grid_size) * grid_size, 2)
    y_aligned = round(round(y / grid_size) * grid_size, 2)
    return x_aligned, y_aligned

def align_pcb_elements(pads, vias, grid_size):
    '''
    # 对齐 footprints
    for fp in footprints:
        fp["x"], fp["y"] = align_to_grid(fp["x"], fp["y"], grid_size)
    '''

    # align pads
    for pad in pads:
        pad["x"], pad["y"] = align_to_grid(pad["x"], pad["y"], grid_size)

    # align vias
    for via in vias:
        via["x"], via["y"] = align_to_grid(via["x"], via["y"], grid_size)
    
    '''
    # 对齐 segments 的起点和终点
    for seg in segments:
        seg["x1"], seg["y1"] = align_to_grid(seg["x1"], seg["y1"], grid_size)
        seg["x2"], seg["y2"] = align_to_grid(seg["x2"], seg["y2"], grid_size)
    '''
        
    return pads, vias

def generate_net_colors(net_ids):
    net_colors = {}
    for net_id in net_ids:
        random.seed(int(net_id))
        r = random.randint(0, 255)
        g = random.randint(0, 255)
        b = random.randint(0, 255)
        net_colors[net_id] = (r, g, b)
    return net_colors

def convert_to_grid(pads, vias, grid_size):
    """
    将 pads 和 vias 转换为 Grid 结构。
    - net_colors: {"1": (255, 0, 0)}
    """
    pad_points = defaultdict(set)

    net_ids = set(pad["net"] for pad in pads) | set(via["net"] for via in vias)
    net_colors = generate_net_colors(net_ids)

    for item in pads + vias:
        net_id = item["net"]
        x, y = item["x"], item["y"]
        
        rgb = net_colors.get(net_id, (0, 0, 0))  # default
        net_obj = Net(pad_c = rgb, route_c = rgb)
        pad_points[net_obj].add(Point(x, y))

    all_points = []
    for point_set in pad_points.values():
        for pt in point_set:
            all_points.append(pt)
    max_x = (max(pt.x for pt in all_points) + grid_size) if all_points else 0
    max_y = (max(pt.y for pt in all_points) + grid_size) if all_points else 0
    min_x = (min(pt.x for pt in all_points) - grid_size) if all_points else 0
    min_y = (min(pt.y for pt in all_points) - grid_size) if all_points else 0

    return Grid(pads = pad_points, width = max_x - min_x, height = max_y - min_y)


def save_aligned_pcb(file_path, grid_size):
    pcb_content = read_kicad_pcb(file_path)
    footprints = extract_footprints(pcb_content)
    pads = []
    for fp in footprints:
        fp_pads = extract_pads(fp)
        pads.extend(fp_pads)
    vias = extract_vias(pcb_content)

    aligned_pads, aligned_vias = align_pcb_elements(pads, vias, grid_size)
    grid = convert_to_grid(aligned_pads, aligned_vias, grid_size)
    return grid



