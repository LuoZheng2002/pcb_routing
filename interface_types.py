from dataclasses import dataclass
from typing import List


@dataclass
class Color:
    r: int  # u8 maps to int (0-255) in Python
    g: int
    b: int


@dataclass
class ColorGrid:
    grid: List[List[Color]]