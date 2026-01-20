# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" />
</p>

一款现代、高性能且轻量级的 WSL (Windows Subsystem for Linux) 实例管理面板。基于 Rust 和 Slint 构建，提供高级的原生体验。

---

[![Rust](https://img.shields.io/badge/rust-v1.92+-orange.svg)](https://www.rust-lang.org)
[![Slint](https://img.shields.io/badge/UI-Slint-blue.svg)](https://slint.dev)
[![License](https://img.shields.io/badge/license-GPL--3.0-blue.svg)](LICENSE)

I18N: 简体中文 | [繁體中文](./README_zh_TW.md) | [English](../README.md) | [日本語](./README_ja.md) | [Français](./README_fr.md) | [Español](./README_es.md) | [Русский](./README_ru.md) | [Português](./README_pt.md) | [Deutsch](./README_de.md) | [Italiano](./README_it.md) | [Türkçe](./README_tr.md) | [Bahasa Indonesia](./README_id.md) | [हिन्दी](./README_hi.md) | [বাংলা](./README_bn.md)

---

## 🖼️ 软件截图

### 主界面 (明亮 & 暗黑模式)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

### 添加实例 & 设置
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>

### 关于 & 菜单折叠
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

## 🎬 操作演示

以下是 WSL Dashboard 的运行演示：

![WSL Dashboard Demo](../assets/screenshot/demo.gif)

## 🚀 核心功能

- 直观的 GUI，支持暗黑模式和流畅的动画。
- 一键管理所有 WSL 发行版（启动、停止、终止、注销）。
- 快速访问发行版终端、VS Code 和文件资源管理器。
- 实时监听并显示 WSL 实例状态。
- 导出并备份为 `.tar` 或压缩的 `.tar.gz` 存档。
- 从备份或现有发行版导入并克隆实例。
- 迁移大型 WSL 实例（VHDX 迁移）到其他磁盘，节省 C 盘空间。
- 从 Microsoft Store 或 GitHub 智能安装发行版。
- 内置 RootFS 下载辅助工具，用于手动安装。
- 深入查看 VHDX 文件位置、虚拟磁盘大小及实际磁盘使用情况。

## 系统要求

- 启用了 WSL 的 Windows 10 或 Windows 11（推荐使用 WSL 2）。
- 至少安装了一个 WSL 发行版，或拥有安装新发行版的权限。
- 64 位 CPU；建议 4 GB 或更多 RAM 以确保多发行版使用顺畅。

## 📦 安装

### 方式 1: 下载预编译二进制文件

最简单的方式是使用编译好的版本：

1. 前往 [GitHub Releases](https://github.com/owu/wsl-dashboard/releases) 页面。
2. 下载适用于 Windows 的最新 `wsldashboard` 可执行文件。
3. 解压（如果是压缩包）并运行 `wsldashboard.exe`。

无需安装，本应用为单文件便携式程序。

### 方式 2: 从源码构建

请确保已安装 Rust 工具链（Rust 1.92+ 或更新版本）。

1. 克隆仓库：

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. 构建并运行：

   - 开发调试：

     ```powershell
     cargo run
     ```

   - 优化后的发布版本构建：

     ```powershell
     cargo run --release
     ```

   - 使用构建脚本（推荐用于生产发布版本）：

     > 构建脚本需要 `x86_64-pc-windows-gnu` 工具链。

     ```powershell
     .\build\scripts\build.ps1
     ```

## 🧭 使用概览

- **管理现有发行版**：在主视图中启动、停止、终止或注销任何 WSL 发行版。
- **快速开启工具**：点击即可在终端、VS Code 或文件资源管理器中打开发行版。
- **创建新实例**：使用“添加实例”视图从 Microsoft Store 安装、下载 RootFS 镜像或克隆现有发行版。
- **备份与恢复**：将发行版导出为 `.tar` / `.tar.gz` 存档，稍后或在另一台机器上导入。
- **迁移存储位置**：将大型 VHDX 文件迁移到不同分驱，以节省 C 盘空间。
- **状态监控**：在 WSL Dashboard 运行时保持对实时发行版状态和存储使用的监控。

## ⚙️ 配置与日志

所有配置均通过“设置”视图管理：

- 选择新 WSL 实例的默认安装目录。
- 配置日志目录和日志级别 (Error / Warn / Info / Debug / Trace)。
- 选择 UI 语言或跟随系统语言。
- 切换暗黑模式，以及应用是否可以在操作后自动关闭 WSL。
- 配置检查更新的频率（每天、每周、每两周、每月）。

日志文件将写入配置的日志目录，在报告问题时可以附带日志。

## 🛠️ 技术栈与性能

- **内核**：使用 Rust 实现，确保内存安全和零成本抽象。
- **UI 框架**：Slint，现代的 GPU 加速 UI 工具包（后端：`winit`）。
- **异步运行时**：Tokio，用于高并发、非阻塞的系统命令和 I/O。
- **性能评估**：
  - **内存占用**：典型情况下约为 60–80 MB RAM。
  - **响应速度**：近乎瞬时的启动速度，并使用流式技术实时更新 WSL 状态。
  - **二进制大小**：优化后的发布版本生成单个精简的可执行文件。

## 🌍 语言支持

本应用提供以下语言的完整国际化支持：

| 语言 | 代码 | Emoji |
| :--- | :---: | :---: |
| 简体中文 | `zh-CN` | 🇨🇳 |
| 繁体中文 | `zh-TW` | 🇭🇰 / 🇹🇼 |
| 英语 | `en` | 🇺🇸 |
| 日语 | `ja` | 🇯🇵 |
| 法语 | `fr` | 🇫🇷 |
| 西班牙语 | `es` | 🇪🇸 |
| 俄语 | `ru` | 🇷🇺 |
| 葡萄牙语 | `pt` | 🇵🇹 |
| 德语 | `de` | 🇩🇪 |
| 意大利语 | `it` | 🇮🇹 |
| 土耳其语 | `tr` | 🇹🇷 |
| 印度尼西亚语 | `id` | 🇮🇩 |
| 印地语 | `hi` | 🇮🇳 |
| 孟加拉语 | `bn` | 🇧🇩 |

## 📄 开源协议

本项目采用 GPL-3.0 协议授权 – 详见 [LICENSE](LICENSE) 文件。

---

Built with ❤️ for the WSL Community.
