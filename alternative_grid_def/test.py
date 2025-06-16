# test.py
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


if __name__ == '__main__':
    main("input.txt")