import math

# === 圓形-圓形碰撞 ===
def collision_circle_circle(c1, c2):
    dx = c1.position[0] - c2.position[0]
    dy = c1.position[1] - c2.position[1]
    distance = (dx ** 2 + dy ** 2) ** 0.5
    return distance <= c1.radius + c2.radius
# === 圓形-多邊形（SAT） ===
def collision_circle_polygon(circle_obj, poly_obj):
    if circle_obj.type not in {'pad'}:
        circle_obj, poly_obj = poly_obj, circle_obj  # swap
    circle = circle_obj
    poly = poly_obj
    corners = poly.get_corners()
    # axes = self._get_axes(corners)
    axes = [] # https://github.com/phenomLi/Blog/issues/23
    
    # 加入從圓心指向最近角點的軸 <- 這啥邏輯
    closest = min(corners, key=lambda p: (p[0]-circle.position[0])**2 + (p[1]-circle.position[1])**2)
    axis_to_circle = (closest[0] - circle.position[0], closest[1] - circle.position[1])
    axes.append(axis_to_circle) # <- circle和多邊形只需要測試一條分離軸
    
    for axis in axes:
        proj1 = project_polygon(corners, axis)
        proj2 = project_circle(circle.position, circle.radius, axis)
        if proj1[1] < proj2[0] or proj2[1] < proj1[0]:
            return False
    return True
# === 多邊形-多邊形 ===
def collision_polygon_polygon(p1, p2):
    c1 = p1.get_corners()
    c2 = p2.get_corners()
    axes = get_axes(c1) + get_axes(c2) # <- 這個 '+' 是 python list的 '+'
    for axis in axes:
        """
        在一次iteration裡面, axis的值是固定的:
        proj1 = self._project_polygon(c1, axis) <- 把一號多邊形的corner點們都投影到這個軸上面
        proj2 = self._project_polygon(c2, axis) <- 一樣
        這個軸是一條直線, 數學表示是y = ax + b, 但!!! b是誰不重要, 簡而言之: 可以投影到任一條斜率為a的直線上去
        """
        proj1 = project_polygon(c1, axis) # proj1是一個tuple, (a, b) <- a是起點, b是終點
        proj2 = project_polygon(c2, axis)
        if proj1[1] < proj2[0] or proj2[1] < proj1[0]: # 這是啥測試條件?? 如果proj1這個"線段"的終點小於proj2的起點, 代表兩個線段中間有gap阿! 根據S.A.T.兩個物體兩個object必然沒有碰撞
            return False
    return True
# === 處理 wire ===
def collision_with_wire(self, obj1, obj2):
    # 這裡你可以擴展具體邏輯，例如 wire 的中段不能碰
    print("WARNING: wire collision logic not yet implemented")
    return False
# === 工具：取得邊的法向量 ===
def get_axes(corners):
    axes = []
    for i in range(len(corners)):
        p1 = corners[i] # <- p1是point1的意思
        p2 = corners[(i + 1) % len(corners)]
        edge = (p2[0] - p1[0], p2[1] - p1[1]) # <- 這行在幹嘛? p1, p2都是一個tuple阿!!
        normal = (-edge[1], edge[0]) # <- normal怎麼取的?就是斜率相乘 == -1呀
        axes.append(normal)
    return axes
# === 工具：投影多邊形 === <- 講白了就是投影
def project_polygon(corners, axis): 
    dot = lambda p: p[0]*axis[0] + p[1]*axis[1]
    dots = [dot(p) for p in corners]
    return min(dots), max(dots) # <- min(dots) 和 max(dots) 就是這段影子的左右端點
# === 工具：投影圓形 ===
def project_circle(center, radius, axis):
    # 單位向量化
    length = (axis[0]**2 + axis[1]**2)**0.5
    norm_axis = (axis[0]/length, axis[1]/length)
    center_proj = center[0]*norm_axis[0] + center[1]*norm_axis[1]
    return center_proj - radius, center_proj + radius
def __str__(self):
    return "\n".join(str(c) for c in self.components.values())