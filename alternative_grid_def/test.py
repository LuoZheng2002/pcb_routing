# test.py
import collision
from component import Pad, SquarePad
from board import good_board

def main(str):
    # === 建立 board 並測試碰撞 ===
    board = good_board(str) 
    board.print_components()
    collision = board.check_collision(board.components["rect1"], board.components["sq1"])
    # collision = board.check_collision(square1, rect1)
    print("✅ Collision detected!" if collision else "❌ No collision.")
    # collision = board.check_collision(pad1, square1)
    # print("Collision detected:" if collision else "No collision detected.")

def main2(str): 
    board = good_board(str)
    temp_collide_semaphore = collision.collision_with_wire(board.components['wire1'], board.components['pad4'])

    if temp_collide_semaphore:
        print("collision detected!")
    else:
        print("no collision")

if __name__ == '__main__':
    # main("input.txt") 
    main2("foo2.txt") # <- 測試wire class