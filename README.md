# PCAP to Excel Converter

一个用 Rust 编写的高性能工具，用于从 PCAP 文件中提取 XYZ 坐标数据并导出到 Excel 文件。

## 使用方法

### 1. 编译项目

```bash
cargo build --release
```

编译后的可执行文件位于：`target/release/pcap_xyz_extractor.exe`

### 2. 运行程序

```bash
cd "C:\Users\FuYou\Desktop\Tool for Data"
cargo run --release
```

或者直接运行：
```bash
.\target\release\pcap_xyz_extractor.exe
```

### 3. 使用流程

#### 步骤 1：输入 PCAP 文件路径
```
Enter PCAP file path:
```

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
使用空格键选择，Enter 确认：
```
[Step 2/4] Select channels to extract:
[ ] Channel 0 (12000 points)
[ ] Channel 5 (11500 points)
[x] Channel 10 (12200 points)
...
```

#### 步骤 4：等待处理
程序会显示进度条：
```
[00:01:23] [=======>-----------------] 12500/95000 (13%)
```

#### 步骤 5：完成
```
✓ Export complete!

Output file: ch_28 (1)_xyz.xlsx

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

## 项目结构

```
Tool for Data/
├── src/
│   ├── main.rs           # 主程序入口
│   ├── cepton.rs         # Cepton 数据结构定义
│   ├── pcap_reader.rs    # PCAP 文件解析
│   └── excel_exporter.rs # Excel 导出功能
├── Cargo.toml            # 项目配置
├── README.md             # 本文档
└── ch_28 (1).pcap        # 示例数据文件
```
