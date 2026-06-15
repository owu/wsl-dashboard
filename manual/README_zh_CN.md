# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

一款现代、高性能、轻量级且低内存占用的 WSL (Windows Subsystem for Linux) 实例管理仪表板。基于 Rust 和 Slint 构建，提供顶级的原生体验。

---

```diff
注意:

- 本软件未通过 Microsoft Store 分发。
- 在此上架的同名应用均为未经授权的盗版软件。
- 请勿下载，以免上当受骗。
```

---

<p align="left">
  <a href="https://www.rust-lang.org" target="_blank"><img src="https://img.shields.io/badge/rust-v1.92+-orange.svg" alt="Rust" /></a>
  <a href="https://slint.dev" target="_blank"><img src="https://img.shields.io/badge/UI-Slint-blue.svg" alt="Slint" /></a>
  <a href="https://tokio.rs" target="_blank"><img src="https://img.shields.io/badge/Async-Tokio-000000.svg" alt="Tokio" /></a>
  <a href="https://github.com/microsoft/windows-rs" target="_blank"><img src="https://img.shields.io/badge/OS-Windows-0078D6.svg" alt="Windows" /></a>
  <a href="../LICENSE"><img src="https://img.shields.io/badge/license-GPL--3.0-blue.svg" alt="License" /></a>
  <a href="https://hellogithub.com/repository/owu/wsl-dashboard" target="_blank"><img src="https://api.hellogithub.com/v1/widgets/recommend.svg?rid=cb1edc45846e475da1dae615a4b4f71c&claim_uid=mWIRuYqZo1FUrjE&theme=small" alt="Featured｜HelloGitHub" /></a>
</p>

I18N :  [English](../README.md) | 简体中文 | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 目录
- [🌍 语言支持](#-语言支持)
- [🚀 核心功能与使用](#-核心功能与使用)
- [⚙️ 配置与日志](#️-配置与日志)
- [🖼️ 软件截图](#️-软件截图)
- [🎬 操作演示](#-操作演示)
- [💻 系统要求](#-系统要求)
- [📦 安装指南](#-安装指南)
- [🛠️ 技术栈与性能](#️-技术栈与性能)
- [🤝 社区支持](#-社区支持)
- [❤️ 支持本项目](#️-支持本项目)
- [⭐️ 为爱发电](#️-为爱发电)
- [📄 开源协议](#-开源协议)

---

## 🌍 语言支持

英语, 简体中文, 繁体中文, 印地语, 西班牙语, 法语, 阿拉伯语, 孟加拉语, 葡萄牙语, 俄语, 乌尔都语, 印尼语, 德语, 日语, 土耳其语, 韩语, 意大利语, 荷兰语, 瑞典语, 捷克语, 希腊语, 匈牙利语, 希伯来语, 挪威语, 丹麦语, 芬兰语, 斯洛伐克语, 斯洛文尼亚语, 冰岛语、越南语、泰卢固语、爪哇语、泰语、泰米尔语、菲律宾语、旁遮普语、马来语、波兰语、乌克兰语、波斯语、卡纳达语、马拉地语、豪萨语、缅甸语、乌兹别克语、阿塞拜疆语、宿务语、马拉雅拉姆语、信德语、阿姆哈拉语

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="英语" alt="英语" />
  <img src="../assets/flags/cn.svg" width="32" title="简体中文" alt="简体中文" />
  <img src="../assets/flags/tw.svg" width="32" title="繁体中文" alt="繁体中文" />
  <img src="../assets/flags/in.svg" width="32" title="印地语" alt="印地语" />
  <img src="../assets/flags/es.svg" width="32" title="西班牙语" alt="西班牙语" />
  <img src="../assets/flags/fr.svg" width="32" title="法语" alt="法语" />
  <img src="../assets/flags/sa.svg" width="32" title="阿拉伯语" alt="阿拉伯语" />
  <img src="../assets/flags/bd.svg" width="32" title="孟加拉语" alt="孟加拉语" />
  <img src="../assets/flags/pt.svg" width="32" title="葡萄牙语" alt="葡萄牙语" />
  <img src="../assets/flags/ru.svg" width="32" title="俄语" alt="俄语" />
  <img src="../assets/flags/pk.svg" width="32" title="乌尔都语" alt="乌尔都语" />
  <img src="../assets/flags/id.svg" width="32" title="印尼语" alt="印尼语" />
  <img src="../assets/flags/de.svg" width="32" title="德语" alt="德语" />
  <img src="../assets/flags/jp.svg" width="32" title="日语" alt="日语" />
  <img src="../assets/flags/tr.svg" width="32" title="土耳其语" alt="土耳其语" />
  <img src="../assets/flags/kr.svg" width="32" title="韩语" alt="韩语" />
  <img src="../assets/flags/it.svg" width="32" title="意大利语" alt="意大利语" />
  <img src="../assets/flags/nl.svg" width="32" title="荷兰语" alt="荷兰语" />
  <img src="../assets/flags/se.svg" width="32" title="瑞典语" alt="瑞典语" />
  <img src="../assets/flags/cz.svg" width="32" title="捷克语" alt="捷克语" />
  <img src="../assets/flags/gr.svg" width="32" title="希腊语" alt="希腊语" />
  <img src="../assets/flags/hu.svg" width="32" title="匈牙利语" alt="匈牙利语" />
  <img src="../assets/flags/il.svg" width="32" title="希伯来语" alt="希伯来语" />
  <img src="../assets/flags/no.svg" width="32" title="挪威语" alt="挪威语" />
  <img src="../assets/flags/dk.svg" width="32" title="丹麦语" alt="丹麦语" />
  <img src="../assets/flags/fi.svg" width="32" title="芬兰语" alt="芬兰语" />
  <img src="../assets/flags/sk.svg" width="32" title="斯洛伐克语" alt="斯洛伐克语" />
  <img src="../assets/flags/si.svg" width="32" title="斯洛文尼亚语" alt="斯洛文尼亚语" />
  <img src="../assets/flags/is.svg" width="32" title="冰岛语" alt="冰岛语" />
  <img src="../assets/flags/vn.svg" width="32" title="越南语" alt="越南语" />
  <img src="../assets/flags/in.svg" width="32" title="泰卢固语" alt="泰卢固语" />
  <img src="../assets/flags/id.svg" width="32" title="爪哇语" alt="爪哇语" />
  <img src="../assets/flags/th.svg" width="32" title="泰语" alt="泰语" />
  <img src="../assets/flags/in.svg" width="32" title="泰米尔语" alt="泰米尔语" />
  <img src="../assets/flags/ph.svg" width="32" title="菲律宾语" alt="菲律宾语" />
  <img src="../assets/flags/pk.svg" width="32" title="旁遮普语" alt="旁遮普语" />
  <img src="../assets/flags/my.svg" width="32" title="马来语" alt="马来语" />
  <img src="../assets/flags/pl.svg" width="32" title="波兰语" alt="波兰语" />
  <img src="../assets/flags/ua.svg" width="32" title="乌克兰语" alt="乌克兰语" />
  <img src="../assets/flags/ir.svg" width="32" title="波斯语" alt="波斯语" />
  <img src="../assets/flags/in.svg" width="32" title="卡纳达语" alt="卡纳达语" />
  <img src="../assets/flags/in.svg" width="32" title="马拉地语" alt="马拉地语" />
  <img src="../assets/flags/ng.svg" width="32" title="豪萨语" alt="豪萨语" />
  <img src="../assets/flags/mm.svg" width="32" title="缅甸语" alt="缅甸语" />
  <img src="../assets/flags/uz.svg" width="32" title="乌兹别克语" alt="乌兹别克语" />
  <img src="../assets/flags/az.svg" width="32" title="阿塞拜疆语" alt="阿塞拜疆语" />
  <img src="../assets/flags/ph.svg" width="32" title="宿务语" alt="宿务语" />
  <img src="../assets/flags/in.svg" width="32" title="马拉雅拉姆语" alt="马拉雅拉姆语" />
  <img src="../assets/flags/pk.svg" width="32" title="信德语" alt="信德语" />
  <img src="../assets/flags/et.svg" width="32" title="阿姆哈拉语" alt="阿姆哈拉语" />
</p>


## 🚀 核心功能与使用

- **现代原生 UI**：直观的 GUI，支持深色/浅色模式，流畅的动画，由 **Skia** 驱动的高性能渲染。
- **系统托盘集成**：全方位的托盘支持（约 10MB 内存占用），支持双击切换显示/隐藏以及功能完整的右键菜单。
- **智能启动**：支持开机自启、最小化到托盘（使用 `/silent` 参数静默启动），以及退出时自动关闭发行版。
- **全面的实例控制**：一键启动、停止、终止和注销。实时状态监控，深入查看磁盘使用情况和文件位置。
- **发行版管理**：设置为默认、物理迁移（将 VHDX 移动到其他磁盘）、以及导出/克隆为 `.tar` 或 `.tar.gz` 存档。
- **快速集成**：一键进入终端、VS Code 或文件资源管理器，支持自定义工作目录和启动脚本钩子。
- **发行版安装**：支持通过 Microsoft Store、GitHub、本地文件（RootFS/VHDX）或在线镜像源安装 Linux 发行版（自动测速选择最快镜像，内置 RootFS 下载助手）。
- **全局安全**：使用互斥锁确保并发迁移/备份操作的安全，并在移除时自动清理 Appx 包。
- **极低内存占用**：高度优化的资源效率。静默启动（系统托盘）仅约 **10MB** 内存。窗口模式下根据字体复杂度占用约 **18MB**（标准语言如英语、德语等）到 **38MB**（大字符集如中日韩语）。
- **高级网络管理**: 无缝管理端口转发（自动创建防火墙规则）以及全局 HTTP 代理配置，实现统一的连接体验。
- **USB 设备管理**: 与 `usbipd-win` 深度集成，可在仪表板 UI 中直接对所有 WSL 实例的本地 USB 设备进行轻松的绑定、附加和管理操作。


## ⚙️ 配置与日志

所有配置均通过“设置”视图管理：

- 选择新 WSL 实例的默认安装目录。
- 配置日志目录和日志级别 (Error / Warn / Info / Debug / Trace)。
- 选择 UI 语言或跟随系统语言。
- 切换深色模式，及应用是否在操作后自动关闭 WSL。
- 配置检查更新的频率（每天、每周、每两周、每月）。
- 启动开机自启（带路径自动修复功能）。
- 设置启动时最小化到托盘。
- 配置关闭按钮的行为（最小化到托盘而非退出程序）。
- 通过切换特定功能选项卡的可见性来自定义侧边栏。

日志文件将写入配置的日志目录，在报告问题时可以附带日志。


## 🖼️ 软件截图

### 主界面 (深色 & 浅色模式)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB & 菜单折叠
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### 网络管理
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### 添加实例 & 设置
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>
<p align="center">
  <img src="../assets/screenshot/settings-advanced.png" width="48%" />
  <img src="../assets/screenshot/settings-interface.png" width="48%" />
</p>

### 关于 & 捐赠
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/donate.png" width="48%" />
</p>

## 🎬 操作演示

[帮助我们改进！观看我们的介绍视频并分享您的想法。](https://github.com/owu/wsl-dashboard/discussions/9)



## 💻 系统要求

- 启用了 WSL 的 Windows 10 或 Windows 11（推荐使用 WSL 2）。
- 至少安装了一个 WSL 发行版，或拥有安装新发行版的权限。
- 64 位 CPU；建议 4 GB 或更多 RAM 以确保多发行版使用顺畅。

## 📦 安装指南

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
   - 使用构建脚本构建优化后的发布版本：

     > 构建脚本需要 `x86_64-pc-windows-msvc` 工具链。

     ```powershell
     .\build\portable\build.ps1
     ```


## 🛠️ 技术栈与性能

- **内核**：使用 Rust 实现，确保内存安全和零成本抽象。
- **UI 框架**：使用高性能 **Skia** 渲染后端的 Slint。
- **异步运行时**：Tokio，用于非阻塞系统命令和 I/O。
- **性能亮点**：
  - **响应速度**：近乎瞬时的启动速度，并实时监控 WSL 状态。
  - **资源效率**：极低的资源占用（详见 [核心功能](#-核心功能与使用)）。
  - **便携性**：优化后的发布版本生成单个精简的可执行文件。



## 🤝 社区支持

衷心感谢以下社区的支持：

- [Rust Programming Language](https://www.rust-lang.org) - 强大且安全的编程语言
- [Slint | Declarative GUI for Rust, C++, JavaScript & Python](https://slint.dev) - 现代化的 UI 框架
- [WSL: Windows Subsystem for Linux](https://github.com/microsoft/WSL) - 出色的 Windows Subsystem for Linux
- [Tokio - An asynchronous Rust runtime](https://tokio.rs) - 高效的异步运行时
- [Windows Developer Community](https://developer.microsoft.com/en-us/windows/community) - 持续的平台改进
- [Reddit](https://www.reddit.com) - 全球社区讨论与支持
- [Hacker News](https://news.ycombinator.com) - 全球社区讨论与支持
- [Linux.do](https://linux.do) - IT 专业人士的热门社区
- [V2EX](https://www.v2ex.com) - 中文技术社区讨论

您的贡献和反馈使这个项目成为可能！


## ❤️ 支持本项目

- 本项目采用 GPL-3.0 开源协议，面向所有用户免费使用。
- 从功能开发、日常测试到问题修复，所有工作都源于业余时间的坚持。开源之路独行不易，您的认可与支持，是项目长久走下去的底气。
- 如果您觉得这款工具切实帮到了您，不妨伸出援手。所有捐赠将用于服务器开销、版本迭代与功能优化，让项目持续更新、稳步前行。
- 点滴善意，皆为星光。再次感谢您的理解与慷慨！

访问我们的捐赠页面：[https://www.wslui.com/donate/](https://www.wslui.com/donate/)


## ⭐️ 为爱发电

如果您觉得这个项目对您有所帮助，我将不胜感激您能在 GitHub 上点亮一颗星。您的认可将帮助项目触及更广泛的用户群体，我也深表谢意。正是这种鼓励激励着我不断前行。


## 📄 开源协议

本项目采用 GPL-3.0 协议授权 – 详见 [LICENSE](../LICENSE) 文件。


---

Built with ❤️ for the WSL Community.

