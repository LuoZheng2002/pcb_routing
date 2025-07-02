from component import Wire, RectPad, Pad, SquarePad
import collision
import board

def collision_detection(board):
    component_name = list(board.components.keys())
    
    for component in component_name:
        # print("the current component in hand is {}".format(component))
        test_list = list(board.components.keys()) # make a copy of component
        test_list.remove(component)
        
        # print("test_list is {}".format(test_list))

        for test_obj in test_list:
            collided = board.check_collision(board.components[test_obj], board.components[component])

            if collided:
                return True

    return False # 能做到這代表兩兩檢查都沒有碰撞
        
def main():
    myboard = board.good_board('foo2.txt')
    if not collision_detection(myboard):
        print("Congradulation!!!!! no collisoin!!!!! it's a valid instance of pcb board")
    else:
        print("Collision Detected!!!!")


if __name__ == '__main__':
    main()