# <div align="center"> Sub-RS 视频字幕压制工具 </div>

<div align="center"> <em> 一个简洁高效的视频字幕压制工具，支持多种编码器与自定义压制参数 </em> </div>

<div align="center"> <b> 基于 FFmpeg 与 Rust 构建 </b> </div>

<br>

<div align="center">
  <a href="#快速开始">快速开始</a> • 
  <a href="#功能特点">功能特点</a> • 
  <a href="#使用说明">使用说明</a> • 
  <a href="#高级选项">高级选项</a> • 
  <a href="#故障排除">故障排除</a>
</div>

<br>

<div align="center">
  <a href="https://www.rust-lang.org/">
    <img src="https://img.shields.io/badge/Language-Rust-DEA584?style=for-the-badge&logo=rust&logoColor=white" alt="Language"></a>
  <a href="https://ffmpeg.org/">
    <img src="https://img.shields.io/badge/Powered_By-FFmpeg-007808?style=for-the-badge&logo=ffmpeg&logoColor=white" alt="Platform"></a>
  <a href="https://developer.nvidia.com/nvidia-video-codec-sdk">
    <img src="https://img.shields.io/badge/Support-NVENC-76B900?style=for-the-badge&logo=nvidia&logoColor=white" alt="Target Site"></a>
</div>

<div align="center">
  <a href="#智能文件识别">
    <img src="https://img.shields.io/badge/Feature-Smart File Detection-4CAF50?style=for-the-badge" alt="Feature"></a>
  <a href="#多种压制模式">
    <img src="https://img.shields.io/badge/Feature-Multiple Encoding Modes-2196F3?style=for-the-badge" alt="Feature"></a>
  <a href="#硬件加速">
    <img src="https://img.shields.io/badge/Feature-Hardware Acceleration-FF9800?style=for-the-badge" alt="Permissions"></a>
</div>

<br>

---

## 快速开始

### 系统要求

- Windows 操作系统
- 已安装 FFmpeg (需确保 `ffmpeg` 命令可在命令行中使用)
- 如需使用 NVENC 功能，需要 NVIDIA 显卡且已安装最新驱动

### 安装方法

1. 下载最新的发布版本
2. 解压到任意文件夹
3. 无需安装，直接使用

### 基本用法

只需将**视频文件**和**字幕文件**拖拽到 `sub-rs.exe` 上即可开始使用。程序会自动识别文件类型并引导您完成压制过程。

## 功能特点

### 智能文件识别

Sub-RS 能够智能识别拖入的视频和字幕文件，即使顺序颠倒也能自动调整，无需担心操作失误。

### 多种编码器支持

- **CPU 编码 (libx264)** - 通用但较慢
- **NVIDIA GPU H264 编码 (h264_nvenc)** - 仅限 NVIDIA 显卡，速度快
- **NVIDIA GPU HEVC 编码 (hevc_nvenc)** - 更高压缩率，部分播放器可能不支持

### 多种压制模式

- **高质量保真模式** - 保留原视频最大质量，无码率限制
- **平衡模式** - 在质量和文件大小之间平衡
- **高压缩模式** - 更小的文件大小，适合分享或节省空间

### 硬件加速

针对 NVIDIA 显卡进行了特别优化，可利用 NVENC 编码器大幅提升压制速度。

### 完全自定义选项

- 可自定义视频质量参数 (CRF/CQ 值)
- 可选择不同的编码速度预设
- 自动为输出文件生成包含处理信息的文件名

## 使用说明

### 1. 启动程序

将视频文件和字幕文件拖拽到 `sub-rs.exe` 上，程序会自动启动。

### 2. 选择编码器

```
请选择视频编码器:
1. CPU 编码 (libx264) - 通用但较慢
2. NVIDIA GPU 编码 (h264_nvenc) - 仅限NVIDIA显卡，速度快
3. NVIDIA GPU HEVC编码 (hevc_nvenc) - 更高压缩率，部分播放器可能不支持
请输入选项 (1-3):
```

### 3. 选择压制模式

```
请选择压制模式:
1. 高质量保真模式 - 保留原视频最大质量，无码率限制
2. 平衡模式 - 在质量和文件大小之间平衡
3. 高压缩模式 - 更小的文件大小，适合分享或节省空间
请输入选项 (1-3):
```

### 4. 设置质量参数

根据您选择的编码器，程序会提示您设置 CRF (CPU 编码) 或 CQ (NVIDIA 编码) 值：

```
请输入质量值 (CQ, 0-51，值越小质量越高，推荐15-25):
```

### 5. 选择编码预设

根据您选择的编码器，程序会提供不同的预设选项：

**NVIDIA 预设**:

```
1. p1 - 最高质量，最慢速度
2. p2 - 高质量
3. p3 - 平衡质量和速度
4. p4 - 快速编码
5. p5 - 平衡质量和速度的最佳选择
6. p6 - 更快速度
7. p7 - 最快速度，最低质量
```

**CPU 预设**:

```
1. veryslow - 最高质量，最慢速度
2. slow - 高质量
3. medium - 平衡质量和速度
4. fast - 快速编码
5. veryfast - 更快速度
6. ultrafast - 最快速度，最低质量
```

### 6. 开始压制

程序会显示压制信息并开始处理。完成后，输出文件会被保存在与源视频相同的目录中。

## 高级选项

### 推荐设置

**最佳质量**:

- 编码器: NVIDIA GPU (h264_nvenc)
- 压制模式: 高质量保真模式
- 质量值: 15-18
- 预设: p1-p3

**平衡质量和速度**:

- 编码器: NVIDIA GPU (h264_nvenc)
- 压制模式: 平衡模式
- 质量值: 20-23
- 预设: p5

**最快速度**:

- 编码器: NVIDIA GPU (h264_nvenc)
- 压制模式: 高压缩模式
- 质量值: 28
- 预设: p7

### 输出文件命名

输出文件会自动添加处理信息到文件名:
`原文件名_subbed_编码器_模式.扩展名`

例如: `video_subbed_h264_nvenc_highq.mp4`

## 故障排除

### 常见问题

**问题**: 程序报错 "Failed to execute ffmpeg"
**解决方案**: 确保已正确安装 FFmpeg 并将其添加到系统 PATH 中

**问题**: NVENC 编码失败
**解决方案**:

1. 确保您有 NVIDIA 显卡且安装了最新驱动
2. 尝试更新显卡驱动到最新版本
3. 检查是否有其他程序正在使用 NVENC 编码器

**问题**: 处理特定格式视频 (如 AV1) 失败
**解决方案**:

1. 使用 `-vf format=yuv420p` 参数确保正确的色彩空间转换
2. 对于某些格式，可能需要先转换为更兼容的格式

### 自定义 FFmpeg 命令

如果您需要更高级的控制，可以在程序执行时查看完整的 FFmpeg 命令并手动修改以满足特定需求。

## 许可证

本项目基于 MIT 许可证。详见 [LICENSE](LICENSE) 文件。

## 致谢

- [FFmpeg](https://ffmpeg.org/) - 强大的多媒体框架
- [Rust](https://www.rust-lang.org/) - 系统编程语言
- [clap](https://github.com/clap-rs/clap) - 命令行参数解析库
