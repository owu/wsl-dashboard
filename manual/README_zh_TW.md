# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" />
</p>

一款現代、高性能且輕量級的 WSL (Windows Subsystem for Linux) 實例管理面板。基於 Rust 和 Slint 構建，提供高級的原生體驗。

---

[![Rust](https://img.shields.io/badge/rust-v1.92+-orange.svg)](https://www.rust-lang.org)
[![Slint](https://img.shields.io/badge/UI-Slint-blue.svg)](https://slint.dev)
[![License](https://img.shields.io/badge/license-GPL--3.0-blue.svg)](LICENSE)

I18N: [简体中文](./README_zh_CN.md) | 繁體中文 | [English](../README.md) | [日本語](./README_ja.md) | [Français](./README_fr.md) | [Español](./README_es.md) | [Русский](./README_ru.md) | [Português](./README_pt.md) | [Deutsch](./README_de.md) | [Italiano](./README_it.md) | [Türkçe](./README_tr.md) | [Bahasa Indonesia](./README_id.md) | [हिन्दी](./README_hi.md) | [বাংলা](./README_bn.md)

---

## 🖼️ 軟體截圖

### 主介面 (明亮 & 暗黑模式)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

### 添加實例 & 設置
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>

### 關於 & 菜單折疊
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

## 🎬 操作演示

以下是 WSL Dashboard 的運行演示：

![WSL Dashboard Demo](../assets/screenshot/demo.gif)

## 🚀 核心功能

- 直觀的 GUI，支持暗黑模式和流暢的動畫。
- 一鍵管理所有 WSL 發行版（啟動、停止、終止、註冊）。
- 快速訪問發行版終端、VS Code 和文件資源管理器。
- 即時監聽並顯示 WSL 實例狀態。
- 匯出並備份為 `.tar` 或壓縮的 `.tar.gz` 封存檔。
- 從備份或現有發行版匯入並克隆實例。
- 遷移大型 WSL 實例（VHDX 遷移）到其他磁碟，節省 C 碟空間。
- 從 Microsoft Store 或 GitHub 智慧安裝發行版。
- 內置 RootFS 下載輔助工具，用於手動安裝。
- 深入查看 VHDX 文件位置、虛擬磁碟大小及實際磁碟使用情況。

## 系統要求

- 啟用了 WSL 的 Windows 10 或 Windows 11（推薦使用 WSL 2）。
- 至少安裝了一個 WSL 發行版，或擁有安裝新發行版的權限。
- 64 位元 CPU；建議 4 GB 或更多 RAM 以確保多發行版使用順暢。

## 📦 安装

### 方式 1: 下載預編譯二進制文件

最簡單的方式是使用編譯好的版本：

1. 前往 [GitHub Releases](https://github.com/owu/wsl-dashboard/releases) 頁面。
2. 下載適用於 Windows 的最新 `wsldashboard` 可執行文件。
3. 解壓（如果是壓縮包）並運行 `wsldashboard.exe`。

無需安裝，本應用為單文件便攜式程序。

### 方式 2: 從源碼構建

請確保已安裝 Rust 工具鏈（Rust 1.92+ 或更新版本）。

1. 克隆倉庫：

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. 構建並運行：

   - 開發調試：

     ```powershell
     cargo run
     ```

   - 優化後的發布版本構建：

     ```powershell
     cargo run --release
     ```

   - 使用構建腳本（推薦用於生產發布版本）：

     > 構建腳本需要 `x86_64-pc-windows-gnu` 工具鏈。

     ```powershell
     .\build\scripts\build.ps1
     ```

## 🧭 使用概覽

- **管理現有發行版**：在主視圖中啟動、停止、終止或註冊任何 WSL 發行版。
- **快速開啟工具**：點擊即可在終端、VS Code 或文件資源管理器中打開發行版。
- **創建新實例**：使用「添加實例」視圖從 Microsoft Store 安裝、下載 RootFS 鏡像或克隆現有發行版。
- **備份與恢復**：將發行版匯出為 `.tar` / `.tar.gz` 封存檔，稍後或在另一台機器上匯入。
- **遷移存儲位置**：將大型 VHDX 文件遷移到不同分區，以節省 C 碟空間。
- **狀態監控**：在 WSL Dashboard 運行時保持對即時發行版狀態和存儲使用的監控。

## ⚙️ 配置與日誌

所有配置均通過「設置」視圖管理：

- 選擇新 WSL 實例的默認安裝目錄。
- 配置日誌目錄和日誌級別 (Error / Warn / Info / Debug / Trace)。
- 選擇 UI 語言或跟隨系統語言。
- 切換暗黑模式，以及應用是否可以在操作後自動關閉 WSL。
- 配置檢查更新的頻率（每天、每周、每兩周、每月）。

日誌文件將寫入配置的日誌目錄，在報告問題時可以附帶日誌。

## 🛠️ 技術棧與性能

- **內核**：使用 Rust 實現，確保內存安全和零成本抽象。
- **UI 框架**：Slint，現代的 GPU 加速 UI 工具包（後端：`winit`）。
- **非同步執行階段**：Tokio，用於高併發、非阻塞的系統命令和 I/O。
- **性能評估**：
  - **內存占用**：典型情況下約為 60–80 MB RAM。
  - **響應速度**：近乎瞬時的啟動速度，並使用流式技術即時更新 WSL 狀態。
  - **二進制大小**：優化後的發布版本生成單個精簡的可執行文件。

## 🌍 語言支持

本應用提供以下語言的完整國際化支持：

| 語言 | 代碼 | Emoji |
| :--- | :---: | :---: |
| 简体中文 | `zh-CN` | 🇨🇳 |
| 繁體中文 | `zh-TW` | 🇭🇰 / 🇹🇼 |
| 英語 | `en` | 🇺🇸 |
| 日語 | `ja` | 🇯🇵 |
| 法語 | `fr` | 🇫🇷 |
| 西班牙語 | `es` | 🇪🇸 |
| 俄語 | `ru` | 🇷🇺 |
| 葡萄牙語 | `pt` | 🇵🇹 |
| 德語 | `de` | 🇩🇪 |
| 義大利語 | `it` | 🇮🇹 |
| 土耳其語 | `tr` | 🇹🇷 |
| 印度尼西亞語 | `id` | 🇮🇩 |
| 印地語 | `hi` | 🇮🇳 |
| 孟加拉語 | `bn` | 🇧🇩 |

## 📄 開源協議

本项目採用 GPL-3.0 協議授權 – 詳見 [LICENSE](LICENSE) 文件。

---

Built with ❤️ for the WSL Community.
