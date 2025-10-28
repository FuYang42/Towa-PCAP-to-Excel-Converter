# Cepton LiDAR PCAP to Excel Converter

一个用 Rust 编写的高性能工具，用于从 Cepton LiDAR PCAP 文件中提取点云数据并导出到 Excel 文件。

支持两种数据格式：
- **Normal 模式**：标准 10 字节点数据（144 点/包）
- **Debug 模式**：扩展 17 字节点数据（72 点/包）

## 使用方法

### 1. 编译项目

```bash
cargo build --release
```

编译后的可执行文件位于：`target/release/pcap_xyz_extractor.exe`

### 2. 运行程序

```bash
cargo run --release
```

或者直接运行：
```bash
.\target\release\pcap_xyz_extractor.exe
```

### 3. 使用流程

#### 步骤 0：选择数据格式
```
[Step 0/5] Select data format:
  1. Normal mode  (10 bytes/point, 144 points/packet)
  2. Debug mode   (17 bytes/point, 72 points/packet)

Your selection [1]:
```

选择对应的模式：
- 输入 `1` 或直接按回车：使用 **Normal 模式**（标准 PCAP 文件）
- 输入 `2`：使用 **Debug 模式**（带额外调试信息的 PCAP 文件）

#### 步骤 1：输入 PCAP 文件路径
```
Enter PCAP file path [***.pcap]:
```

输入 PCAP 文件路径，或直接按回车使用默认文件。

#### 步骤 2：查看通道统计
程序会自动扫描文件并显示所有通道的点数：
```
Found 8 channels:

  Channel  0:    12000 points
  Channel  5:    11500 points
  Channel 10:    12200 points
  ...

  Total:       95000 points
```

#### 步骤 3：选择通道
```
[Step 2/5] Select channels to extract:
  Options:
    - Enter channel numbers separated by commas (e.g., 0,5,10)
    - Enter 'all' to extract all channels
    - Enter a range (e.g., 0-10)

Your selection:
```

输入通道选择：
- 输入 `all` 选择所有通道
- 输入通道号（用逗号分隔）：`0,5,10`
- 输入范围：`0-10`

#### 步骤 4：提取数据
程序会显示进度条：
```
[Step 3/5] Extracting XYZ coordinates...
[00:01:23] [=======>-----------------] 12500/95000 (13%)
```

#### 步骤 5：完成导出
```
✓ Export complete!

Output file: ***_xyz.xlsx

=======================================================
Summary:
  Channel 10: 12200 points extracted
  Channel 15: 11800 points extracted
=======================================================
```

## Excel 输出格式

### 文件结构
- 每个通道一个工作表（Sheet）
- 工作表命名：`Channel_0`, `Channel_5`, `Channel_10`, ...

### 数据列

#### Normal 模式（5列）

| 列名 | 说明 | 格式 |
|------|------|------|
| X (m) | X 坐标（米） | 保留4位小数 |
| Y (m) | Y 坐标（米） | 保留4位小数 |
| Z (m) | Z 坐标（米） | 保留4位小数 |
| Reflectivity | 反射率 | 整数 (0-255) |
| Flags | 状态标志 | 整数 (0-255) |

#### Debug 模式（8列）

| 列名 | 说明 | 格式 |
|------|------|------|
| X (m) | X 坐标（米） | 保留4位小数 |
| Y (m) | Y 坐标（米） | 保留4位小数 |
| Z (m) | Z 坐标（米） | 保留4位小数 |
| Reflectivity | 反射率 | 整数 (0-255) |
| Flags | 状态标志 | 整数 (0-255) |
| **Distance** | **距离值** | **整数** |
| **Intensity** | **强度值** | **整数** |
| **Power Level** | **功率等级** | **整数** |

> 📝 **注意**：Debug 模式包含额外的 3 列调试信息，用于详细分析激光雷达性能。

## 数据格式说明

### Normal 模式 vs Debug 模式

| 特性 | Normal 模式 | Debug 模式 |
|------|-------------|-----------|
| 点数据大小 | 10 字节 | 17 字节 |
| 每包点数 | 144 个点 | 72 个点 |
| 基础数据 | XYZ 坐标、反射率、标志 | ✅ |
| 额外数据 | - | Distance、Intensity、Power Level |
| 适用场景 | 标准数据采集 | 性能分析、调试 |

### 点数据结构

#### Normal 模式（10 字节）
```
0-1:   int16_t x                    (X坐标，0.5cm分辨率)
2-3:   int16_t y                    (Y坐标，0.5cm分辨率)
4-5:   int16_t z                    (Z坐标，0.5cm分辨率)
6:     uint8_t reflectivity         (反射率 0-255)
7:     uint8_t timestamp_offset     (时间戳偏移)
8:     uint8_t laser_id             (激光通道ID)
9:     uint8_t flags                (状态标志)
```

#### Debug 模式（17 字节）
```
0-9:   [Normal 模式的 10 字节]
10-13: uint32_t distance            (距离值)
14-15: uint16_t intensity           (强度值)
16:    uint8_t power_level          (功率等级)
```

## 项目结构

```
Towa-PCAP-to-Excel-Converter/
├── src/
│   ├── main.rs           # 主程序入口，模式选择
│   ├── cepton.rs         # Cepton STDV 数据结构定义
│   ├── pcap_reader.rs    # PCAP 文件解析（支持双模式）
│   └── excel_exporter.rs # Excel 导出功能（自动识别模式）
├── Cargo.toml            # 项目配置
└── README.md             # 本文档
```
