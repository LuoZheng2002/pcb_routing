# 📘 README — PCB Component Collision Checker

這個專案模擬 PCB 板上的元件擺放與碰撞偵測，支援 `pad`、`square_pad`、`rect_pad` 等類型元件，並以 `.txt` 格式讀取元件參數初始化後進行分析。

---

## 📦 專案目錄結構
```
your_project/
├── board.py # 主板件管理邏輯（component loading + collision interface）
├── component.py # 定義 Pad / SquarePad / RectPad 類別
├── collision.py # 撞擊計算與幾何工具（SAT 等）
├── test.py # 測試腳本，使用 matplotlib 畫圖與顯示碰撞結果
├── foo.txt # 測試元件輸入檔案
└── README.md # 本說明文件
```
## 📝 輸入格式說明（`foo.txt`）

每行定義一個元件，格式如下：
```
<name>: <type>, (<x>, <y>), <其他參數>
```

### ✅ 支援的元件類型與範例格式：

| 類型         | 說明                           | 範例格式                                        |
|--------------|--------------------------------|-------------------------------------------------|
| `pad`        | 圓形 pad，需提供座標與半徑     | `pad1: pad, (-10, 5), 4`                         |
| `square_pad` | 旋轉的正方形 pad               | `sq1: square_pad, (-1, -1.5), 6, 45`             |
| `rect_pad`   | 旋轉的矩形 pad（長與寬不同）   | `rect1: rect_pad, (6, 0), 6, 3, 30`              |

## 🚀 執行方法

在終端機輸入以下指令：

```bash
python3 test.py
```

## 🔧 初始化測試元件（非讀檔方式）
你也可以直接在 `test.py` 中手動定義元件，不透過 `foo.txt`：
```python
from component import Pad, SquarePad

pad1 = Pad("pad1", (-20, -20), 5)
sq1 = SquarePad("sq1", (6, 0), 6, 60)
```
再將其註冊到 `board` 內：
```python
board.components = {"pad1": pad1, "sq1": sq1}
```
## 🔍 碰撞檢查輸出範例
```yaml
Current components on board:
  pad1: Pad at (-20.0, -20.0)
  sq1: SquarePad at (6.0, 0.0)
>> No Collision.
```
若有碰撞會顯示 `>> Collision Detected!`，並以紅色圓形表示；否則顯示綠色。

## 📌 備註
所有座標單位為浮點數（`float`），角度單位為度（degree）。

`wire` 類型目前尚未實作碰撞邏輯。

所有碰撞邏輯基於 **分離軸定理(Separating Axis Theorem, SAT)** 實作。