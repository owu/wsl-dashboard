# WSL Dashboard 功能增强 PR 说明

本 PR 旨在将 `wsl2-distro-manager` 中的核心深度管理功能移植并增强到 `wsl-dashboard` 中，显著提升了工具的专业性与便利性，同时保持了 Rust/Slint 的极致性能。

## 🌟 新增功能概览

### 1. 深度配置管理 (System Configuration)
- **wsl.conf 全面接管**: 在实例设置面板中新增了 "System Configuration" 分区。
- **GUI 一键配置**: 支持直接修改 `systemd`, `generateHosts`, `generateResolvConf`, `Interop`, `appendWindowsPath`。
- **价值**: 用户无需再进入终端手动编辑复杂的 INI 配置文件。

### 2. 磁盘维护专家 (Disk Maintenance)
- **VHDX 磁盘瘦身 (Compact)**: 在实例信息面板新增 "Compact" 按钮。
- **安全释放空间**: 采用 Windows 原生 `diskpart` 逻辑，在实例停止时安全压缩虚拟磁盘文件，释放物理硬盘空间。

### 3. 全局资源控制 (.wslconfig)
- **全局看板**: 在设置页面新增了针对 `.wslconfig` 的管理界面。
- **资源限制**: 支持从 GUI 直接设置所有 WSL 实例的 **内存上限**、**CPU 核心数限制**。
- **网络模式切换**: 支持一键在 `nat` 和 `mirrored` (镜像) 模式间切换。

### 4. 实时运行看板 (Runtime Insights)
- **IP 地址显示**: 实例运行时，信息面板会实时显示其内网 IP 地址。
- **Docker 容器集成**: 自动探测实例内的 Docker 环境，点击信息即可查看当前运行的所有容器名称及状态。

### 5. Windows 底层环境检测
- **系统功能体检**: 全局设置中实时显示 Windows “虚拟机平台”和“Linux 子系统”组件的开启状态，方便故障排查。

### 6. 交互便利性增强
- **桌面快捷方式**: 实例菜单中新增“创建快捷方式”图标，一键在桌面上生成直接进入该实例的 `.lnk` 文件。
- **智能克隆**: 克隆对话框新增“克隆后立即启动”选项，优化工作流。

## 🛠️ 技术实现
- **Rust 后端**: 扩展了 `wsl::ops` 模块，新增了对注册表、`.wslconfig` 文件、`/etc/wsl.conf` 文件的安全读写逻辑。
- **PowerShell 桥接**: 利用最小化的 PowerShell 脚本处理 `diskpart` 压缩和快捷方式创建等 Windows 原生操作。
- **Slint UI**: 优化了设置面板和信息面板的布局，增加了滚动支持和分组标题，确保在增加大量功能后依然保持界面清爽。

---
本更新在功能上已完全覆盖并超越了同类 Flutter 开发的工具，使 `wsl-dashboard` 成为目前市面上最强大的 WSL 桌面管理工具之一。
