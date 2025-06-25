import math

class Component:
    def __init__(self, name, comp_type, position):
        self.name = name
        self.type = comp_type
        self.position = position

    def __repr__(self):
        return f"{self.type}({self.name}) at {self.position}"

class Pad(Component):
    def __init__(self, name, position, radius):
        super().__init__(name, 'pad', position)
        self.radius = radius

    def __repr__(self):
        return f"Pad({self.name}, pos={self.position}, r={self.radius})"

class SquarePad(Component):
    def __init__(self, name, position, side_length, angle):
        super().__init__(name, 'square_pad', position)
        self.side_length = side_length
        self.angle = angle

    def __repr__(self):
        return f"SquarePad({self.name}, pos={self.position}, side={self.side_length}, angle={self.angle})"

    def get_corners(self):
        """
        回傳旋轉後的四個角座標。
        square_pad 是正方形，因此邊長相等。
        """
        cx, cy = self.position
        half_side = self.side_length / 2
        angle_rad = math.radians(self.angle)

        # 正方形在原點的四角
        corners = [
            (-half_side, -half_side),
            (-half_side, half_side),
            (half_side, half_side),
            (half_side, -half_side)
        ]

        # 旋轉並平移
        rotated = []
        for x, y in corners:
            x_rot = x * math.cos(angle_rad) - y * math.sin(angle_rad)
            y_rot = x * math.sin(angle_rad) + y * math.cos(angle_rad)
            rotated.append((cx + x_rot, cy + y_rot))

        return rotated

class RectPad(Component):
    def __init__(self, name, position, side_length, side_width, angle):
        super().__init__(name, 'rect_pad', position)
        self.side_length = side_length  # 長邊
        self.side_width = side_width    # 短邊
        self.angle = angle              # 旋轉角度（度數）

    def __repr__(self):
        return (f"RectPad({self.name}, pos={self.position}, "
                f"L={self.side_length}, W={self.side_width}, angle={self.angle})")

    def area(self):
        return self.side_length * self.side_width

    def get_corners(self):
        """
        回傳旋轉後的四個角座標，用於畫圖或碰撞檢測等。
        預設中心為 self.position，角度為 self.angle（度）
        """
        cx, cy = self.position
        l, w = self.side_length / 2, self.side_width / 2
        angle_rad = math.radians(self.angle)

        # 長方形四角在原點時的相對位置
        corners = [
            (-l, -w), (-l, w),
            (l, w), (l, -w)
        ]

        # 旋轉 + 平移
        rotated = []
        for x, y in corners:
            x_rot = x * math.cos(angle_rad) - y * math.sin(angle_rad)
            y_rot = x * math.sin(angle_rad) + y * math.cos(angle_rad)
            rotated.append((cx + x_rot, cy + y_rot))

        return rotated

class Wire(Component):
    def __init__(self, name, start_component, end_component, width, points):
        super().__init__(name, 'wire', None)
        self.start_component = start_component
        self.end_component = end_component
        self.width = width
        self.points = points  # List of (x, y)

    def __repr__(self):
        return f"Wire({self.name}, width={self.width}, points={self.points})"

    def get_segments(self):
        """
        將線段轉換為多個寬度為 `width` 的矩形，每段是兩點之間構成。
        回傳：List of 4-point polygons（corners of rectangles）
        """
        rectangles = []
        half_w = self.width / 2
        for i in range(len(self.points) - 1):
            x1, y1 = self.points[i]
            x2, y2 = self.points[i+1]
            dx, dy = x2 - x1, y2 - y1
            length = (dx**2 + dy**2)**0.5
            # 垂直向量（法線）
            nx, ny = -dy / length * half_w, dx / length * half_w
            # 四角點
            p1 = (x1 + nx, y1 + ny)
            p2 = (x1 - nx, y1 - ny)
            p3 = (x2 - nx, y2 - ny)
            p4 = (x2 + nx, y2 + ny)
            rectangles.append([p1, p2, p3, p4])
        return rectangles
    
    def get_relay_points(self):
        return self.points[1:-1]
        