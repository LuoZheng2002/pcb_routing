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
    axes = get_axes(corners) # https://github.com/phenomLi/Blog/issues/23
    
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
def collision_with_wire(obj1, obj2):
    """
    wire.width是線段寬度!!!!!! 所以中繼點的半徑要除以2
    """
    if obj1.type == 'wire' and obj2.type != 'wire':
        obj1, obj2 = obj2, obj1

    if (obj1.name == obj2.start_component or obj1.name == obj2.end_component):
        # print("{} is one of the component that {} is connecting!".format(obj1.name, obj2.name))
        return False
    
    if obj1.type == 'wire' and obj2.type == 'wire':
        """
        High-Level邏輯: 在過程中我們有好幾個return True, 如果obj1 & obj2能過pass這些檢測的話 那就代表沒有碰撞
        """
        wire_segments1 = obj1.get_segments()
        wire_segments2 = obj2.get_segments()
        for seg1 in wire_segments1:
            for seg2 in wire_segments2:
                collide = True

                axes = get_axes(seg1) + get_axes(seg2)
                for axis in axes:
                    proj1 = project_polygon(seg1, axis)
                    proj2 = project_polygon(seg2, axis)
                    if proj1[1] < proj2[0] or proj2[1] < proj1[0]:
                        collide = False
                        break
                
                # 如果做到這裡collide還是初始值True代表我找不到分離軸 -> 代表兩個多邊形有碰撞
                if collide == True:
                    print("Collision between {} and {} is detected!".format(obj1.name, obj2.name))
                    return True
                
        # 能做到這一行代表每一對矩形都沒有碰撞 -> 不需做任何事 ->繼續檢查
                
        # 做到這一步代表obj1, obj2這兩個wire的 "主架構沒甚麼問題了", 但中繼點還是要拎出來檢查
        relay1 = obj1.get_relay_points()
        relay2 = obj2.get_relay_points()

        # circle and circle的碰撞邏輯
        for p1 in relay1:
            for p2 in relay2:
                dx = p1[0] - p2[0]
                dy = p1[1] - p2[1]
                dist = (dx**2 + dy**2) ** 0.5
                if dist <= obj1.width / 2 + obj2.width / 2: # 寫 '<=' 因為'等於'也要觸發
                    return True
        
        # cirlce和矩形的碰撞邏輯
        for p1 in relay1:
            for seg2 in wire_segments2: # seg2 是一個矩形
            
                axes = get_axes(seg2)
                closest = min(seg2, key=lambda p: (p[0]-p1[0])**2 + (p[1]-p1[1])**2)
                axis_to_circle = (closest[0] - p1[0], closest[1] - p1[1])
                axes.append(axis_to_circle) 

                collide = True
                for axis in axes:
                    if collide == False: # 表示找到分離軸了
                        break
                    proj1 = project_polygon(seg2, axis)
                    proj2 = project_circle(p1, obj1.width / 2, axis)
                    if proj1[1] < proj2[0] or proj2[1] < proj1[0]:
                        collide = False
                        break

                # 做到這一行代表seg2矩形的每一條分離軸我都測試了, 如果collide仍為True表示有碰撞
                if collide:
                    return True 

        for p2 in relay2:
            for seg1 in wire_segments1:
                
                axes = get_axes(seg1)
                closest = min(seg1, key=lambda p: (p[0] - p2[0])**2 + (p[1] - p2[1])**2)
                axis_to_circle = (closest[0] - p2[0], closest[1] - p2[1])
                axes.append(axis_to_circle)

                collide = True
                for axis in axes:
                    if collide == False:
                        break
                    proj1 = project_polygon(seg1, axis)
                    proj2 = project_circle(p2, obj2.width / 2, axis)
                    if proj1[1] < proj2[0] or proj2[1] < proj1[0]:
                        collide = False
                        break
                
                if collide:
                    print("Collision between relay point of {} and segment of {} is detected!".format(obj2.name, obj1.name))
                    return True
        return False
    else:
        # 如果程式碼能跑到這裡代表一個是wire一個是pad <- 廣義的pad
        
        # 因為line 50的邏輯 所以必然obj1是pad, obj2是wire

        # 先判斷obj2是不是圓形的pad
        # 用switch case語句
        wire_segments = obj2.get_segments()
        relay_points = obj2.get_relay_points()

        if obj1.type == 'pad':
            # 用到的碰撞邏輯就是circle to circle & circle to polygon
            pad_center = obj1.position
            pad_radius = obj1.radius

            # circle to circle
            # obj2的 relay points 和 Pad本身
            for point in relay_points:
                dx = point[0] - pad_center[0]
                dy = point[1] - pad_center[1]

                dist = (dx ** 2 + dy ** 2) ** 0.5

                if dist <= pad_radius + obj2.width / 2: # obj2.width / 2 是relay_points們的半徑
                    print(f"Collision between pad : {obj1.name} and relay point of wire {obj2.name}")
                    return True
            
            
            # circle to polygon
            for seg in wire_segments:
                # print("pad center is {}".format(pad_center))

                # print("the 4 corners of polygon is {}".format(seg))

                closest = min(seg, key=lambda p: (p[0] - pad_center[0])**2 + (p[1] - pad_center[1])**2)

                axes = get_axes(seg)

                # print("pad_center is {}".format(pad_center))
                edge = (closest[0] - pad_center[0], closest[1] - pad_center[1])
                axes.append((-edge[1], edge[0]))
                # print("axes is {}".format(axes))

                collide = True
                for axis in axes:
                    proj1 = project_polygon(seg, axis)
                    proj2 = project_circle(pad_center, pad_radius, axis)

                    if proj1[1] < proj2[0] or proj2[1] < proj1[0]: # 只有一條軸確實可以這麼寫!
                        collide = False
                        break

                if collide:
                    return True
                
            return False
        else:
            # 這邊就保證pad是polygon了 不管是長方形還是正方形
            
            # 所以這邊我們用到的邏輯只會是circle to polygon & polygon to polygon
            pad_polygon = obj1.get_corners()
            # circle (obj2的relay points們) to polygon <- 先寫, 因為較easy
            for point in relay_points:
                axes = get_axes(pad_polygon)
                closest = min(pad_polygon, key=lambda p: (p[0] - point[0])**2 + (p[1] - point[1])**2)
                edge = (closest[0] - point[0], closest[1] - point[1])
                axes.append((-edge[1], edge[0]))

                collide = True
                for axis in axes:

                    proj1 = project_polygon(pad_polygon, axis)
                    proj2 = project_circle(point, obj2.width / 2, axis)
                    if proj1[1] < proj2[0] or proj2[1] < proj1[0]:
                        collide = False
                        break
                if collide:
                    return True
                
            for seg in wire_segments:
                collide = True
                axes = get_axes(pad_polygon) + get_axes(seg)
                for axis in axes:
                    proj1 = project_polygon(pad_polygon, axis)
                    proj2 = project_polygon(seg, axis)
                    if proj1[1] < proj2[0] or proj2[1] < proj1[0]:
                        collide = False
                        break

                # -------- 這條線以上, 測試完一對待測試多邊形的每一個可能分離軸
                # 如果仍然沒找到! 那就代表實際上有碰撞
                if collide:
                    print(f"Collision between wire segment of {obj2.name} and pad {obj1.name}")
                    return True
  
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
    """
    axis是一個tuple: (x, y)
    代表一個向量
    """
    # Project points onto normalized axis
    dot = lambda p: p[0]*axis[0] + p[1]*axis[1]
    dots = [dot(p) for p in corners]
    return min(dots), max(dots) # <- min(dots) 和 max(dots) 就是這段影子的左右端點
# === 工具：投影圓形 ===
def project_circle(center, radius, axis):
    center_proj = center[0] * axis[0] + center[1] * axis[1]
    return center_proj - radius, center_proj + radius

def __str__(self):
    return "\n".join(str(c) for c in self.components.values())