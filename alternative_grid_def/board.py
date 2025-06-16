import re
from component import Pad, SquarePad, RectPad
import collision

class good_board:
    supported_list = {'wire', 'pad', 'square_pad', 'rect_pad'}

    def __init__(self, file):
        self.components = {}
        self.supported_list_activated = set()
        self.parse_file(file)

    def parse_file(self, file):
        with open(file, "r") as fo:
            for line in fo:
                line = line.strip()
                if not line:
                    continue
                try:
                    name, rest = line.split(":")
                    name = name.strip()
                    type_and_params = rest.split(",", 1)
                    comp_type = type_and_params[0].strip()

                    if comp_type not in self.supported_list:
                        print(f"Unsupported component type: {comp_type}")
                        continue

                    self.supported_list_activated.add(comp_type)

                    if comp_type == 'pad':
                        match = re.match(r"\s*\(([^)]+)\)\s*,\s*(.+)", type_and_params[1])
                        if not match:
                            raise ValueError("Invalid pad format")
                        pos_str, radius_str = match.groups()
                        x, y = map(float, pos_str.split(","))
                        radius = float(radius_str)
                        self.components[name] = Pad(name, (x, y), radius)

                    elif comp_type == 'square_pad':
                        match = re.match(r"\s*\(([^)]+)\)\s*,\s*([-\d.]+)\s*,\s*([-\d.]+)", type_and_params[1])
                        if not match:
                            raise ValueError("Invalid square_pad format")
                        pos_str, side_str, angle_str = match.groups()
                        x, y = map(float, pos_str.split(","))
                        side = float(side_str)
                        angle = float(angle_str)
                        self.components[name] = SquarePad(name, (x, y), side, angle)

                    elif comp_type == 'rect_pad':
                        match = re.match(r"\s*\(([^)]+)\)\s*,\s*([-\d.]+)\s*,\s*([-\d.]+)\s*,\s*([-\d.]+)", type_and_params[1])
                        if not match:
                            raise ValueError("Invalid rect_pad format")
                        pos_str, l_str, w_str, angle_str = match.groups()
                        x, y = map(float, pos_str.split(","))
                        l = float(l_str)
                        w = float(w_str)
                        angle = float(angle_str)
                        self.components[name] = RectPad(name, (x, y), l, w, angle)

                except Exception as e:
                    print(f"Failed to parse line: '{line}', error: {e}")

    def check_collision(self, obj1, obj2):
        t1 = obj1.type
        t2 = obj2.type

        # 用 tuple 做類型對應（不區分順序）
        type_pair = tuple(sorted([t1, t2]))

        if type_pair == ('pad', 'pad'):
            return collision.collision_circle_circle(obj1, obj2)
        elif 'pad' in type_pair and 'rect_pad' in type_pair:
            return collision.collision_circle_polygon(obj1, obj2)
        elif 'pad' in type_pair and 'square_pad' in type_pair:
            return collision.collision_circle_polygon(obj1, obj2)
        elif t1 in {'rect_pad', 'square_pad'} and t2 in {'rect_pad', 'square_pad'}:
            return collision.collision_polygon_polygon(obj1, obj2)
        elif 'wire' in type_pair:
            return collision.collision_with_wire(obj1, obj2)
        else:
            raise NotImplementedError(f"Collision detection not implemented for: {type_pair}")
        
    def print_components(self):
        print("Current components on board:")
        for name, obj in self.components.items():
            print(f"  {name}: {type(obj).__name__} at {obj.position}")
