# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

一款現代、高性能且輕量級的 WSL (Windows Subsystem for Linux) 實例管理面板。基於 Rust 和 Slint 構建，提供高級的原生體驗。

---

```diff
注意:

- 本軟體未通過 Microsoft Store 分發。
- 在此上架的同名應用均為未經授權的盜版軟體。
- 請勿下載，以免上當受騙。
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

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | 繁體中文 | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md)

---

## 📑 目錄
- [🌍 語言支持](#-語言支持)
- [🚀 核心功能與使用](#-核心功能與使用)
- [⚙️ 配置與日誌](#️-配置與日誌)
- [🖼️ 軟體截圖](#️-軟體截圖)
- [🎬 操作演示](#-操作演示)
- [💻 系統要求](#-系統要求)
- [📦 安裝指南](#-安裝指南)
- [🛠️ 技術棧與性能](#️-技術棧與性能)
- [⭐️ 為愛發電](#️-為愛發電)
- [📄 開源協議](#-開源協議)

---

## 🌍 語言支持

英語, 簡體中文, 繁體中文, 印地語, 西班牙語, 法語, 阿拉伯語, 孟加拉語, 葡萄牙語, 俄語, 烏爾都語, 印尼語, 德語, 日語, 土耳其語, 韓語, 義大利語, 荷蘭語, 瑞典語, 捷克語, 希臘語, 匈牙利語, 希伯來語, 挪威語, 丹麥語, 芬蘭語, 斯洛伐克語, 斯洛文尼亞語, 冰島語

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="英語" alt="英語" />
  <img src="../assets/flags/cn.svg" width="32" title="簡體中文" alt="簡體中文" />
  <img src="../assets/flags/tw.svg" width="32" title="繁體中文" alt="繁體中文" />
  <img src="../assets/flags/in.svg" width="32" title="印地語" alt="印地語" />
  <img src="../assets/flags/es.svg" width="32" title="西班牙語" alt="西班牙語" />
  <img src="../assets/flags/fr.svg" width="32" title="法語" alt="法語" />
  <img src="../assets/flags/sa.svg" width="32" title="阿拉伯語" alt="阿拉伯語" />
  <img src="../assets/flags/bd.svg" width="32" title="孟加拉語" alt="孟加拉語" />
  <img src="../assets/flags/pt.svg" width="32" title="葡萄牙語" alt="葡萄牙語" />
  <img src="../assets/flags/ru.svg" width="32" title="俄語" alt="俄語" />
  <img src="../assets/flags/pk.svg" width="32" title="烏爾都語" alt="烏爾都語" />
  <img src="../assets/flags/id.svg" width="32" title="印尼語" alt="印尼語" />
  <img src="../assets/flags/de.svg" width="32" title="德語" alt="德語" />
  <img src="../assets/flags/jp.svg" width="32" title="日語" alt="日語" />
  <img src="../assets/flags/tr.svg" width="32" title="土耳其語" alt="土耳其語" />
  <img src="../assets/flags/kr.svg" width="32" title="韓語" alt="韓語" />
  <img src="../assets/flags/it.svg" width="32" title="義大利語" alt="義大利語" />
  <img src="../assets/flags/nl.svg" width="32" title="荷蘭語" alt="荷蘭語" />
  <img src="../assets/flags/se.svg" width="32" title="瑞典語" alt="瑞典語" />
  <img src="../assets/flags/cz.svg" width="32" title="捷克語" alt="捷克語" />
  <img src="../assets/flags/gr.svg" width="32" title="希臘語" alt="希臘語" />
  <img src="../assets/flags/hu.svg" width="32" title="匈牙利語" alt="匈牙利語" />
  <img src="../assets/flags/il.svg" width="32" title="希伯來語" alt="希伯來語" />
  <img src="../assets/flags/no.svg" width="32" title="挪威語" alt="挪威語" />
  <img src="../assets/flags/dk.svg" width="32" title="丹麥語" alt="丹麥語" />
  <img src="../assets/flags/fi.svg" width="32" title="芬蘭語" alt="芬蘭語" />
  <img src="../assets/flags/sk.svg" width="32" title="斯洛伐克語" alt="斯洛伐克語" />
  <img src="../assets/flags/si.svg" width="32" title="斯洛文尼亞語" alt="斯洛文尼亞語" />
  <img src="../assets/flags/is.svg" width="32" title="冰島語" alt="冰島語" />
</p>


## 🚀 核心功能與使用

- **現代原生 UI**：直觀的 GUI，支持深色/淺色模式，流暢的動畫，由 **Skia** 驅動的高性能渲染。
- **系統托盤整合**：全方位的托盘支持（約 10MB 內存佔用），支持雙擊切換顯示/隱藏以及功能完整的右鍵菜單。
- **智慧啟動**：支持開機自啟、最小化到托盤（使用 `/silent` 參數靜默啟動），以及退出時自動關閉發行版。
- **全面的實例控制**：一鍵啟動、停止、終止和註銷。即時狀態監控，深入查看磁碟使用情況和文件位置。
- **發行版管理**：設置為預設、物理遷移（將 VHDX 移動到其他磁碟）、以及匯出/克隆為 `.tar` 或 `.tar.gz` 封存檔。
- **快速整合**：一鍵進入終端、VS Code 或文件資源管理器，支持自定義工作目錄和啟動腳本鉤子。
- **智慧安裝**：支持從 Microsoft Store、GitHub 或本地文件（RootFS/VHDX）安裝。內置 RootFS 下載助手。
- **全局安全**：使用互斥鎖確保併發遷移/備份操作的安全，並在移除時自動清理 Appx 包。
- **極低內存佔用**：高度優化的資源效率。靜默啟動（系統托盤）僅約 **10MB** 內存。視窗模式下根據字體複雜度佔用約 **18MB**（標準語言如英語、德語等）到 **38MB**（大字符集如中日韓語）。
- **高級網路管理**: 無縫管理連接埠轉發（自動建立防火牆規則）以及全局 HTTP 代理配置，實現統一的連線體驗。
- **USB 設備管理**: 與 `usbipd-win` 深度整合，可在儀表板 UI 中直接對所有 WSL 實例的本地 USB 設備進行輕鬆的綁定、附加和管理操作。


## ⚙️ 配置與日誌

所有配置均通過「設置」視圖管理：

- 選擇新 WSL 實例的默認安裝目錄。
- 配置日誌目錄和日誌級別 (Error / Warn / Info / Debug / Trace)。
- 選擇 UI 語言或跟隨系統語言。
- 切換深色模式，及應用是否在操作後自動關閉 WSL。
- 配置檢查更新的頻率（每天、每周、每兩周、每月）。
- 啟用開机自啟（帶路徑自動修復功能）。
- 設置啟動時最小化到托盤。
- 配置關閉按鈕的行为（最小化到托盤而非退出程序）。
- 透過切換特定功能分頁的可見性來自訂側邊欄。

日誌文件將寫入配置的日誌目錄，在報告問題時可以附帶日誌。


## 🖼️ 軟體截圖

### 主介面 (深色 & 淺色模式)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB & 菜單折疊
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### 網路管理
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### 添加實例 & 設置
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>

### 關於
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
</p>

## 🎬 操作演示

以下是 WSL Dashboard 的運行演示：

![WSL Dashboard Demo](../assets/screenshot/demo.gif)



## 💻 系統要求

- 啟用了 WSL 的 Windows 10 或 Windows 11（推薦使用 WSL 2）。
- 至少安裝了一個 WSL 發行版，或擁有安裝新發行版的權限。
- 64 位元 CPU；建議 4 GB 或更多 RAM 以確保多發行版使用順畅。

## 📦 安裝指南

### 方式 1: 下載預編譯二進制文件

最简单的方式是使用編譯好的版本：

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
   - 使用構建腳本構建優化後的發布版本：

     > 構建腳本需要 `x86_64-pc-windows-msvc` 工具鏈。

     ```powershell
     .\build\scripts\build.ps1
     ```


## 🛠️ 技術棧與性能

- **內核**：使用 Rust 實現，確保內存安全和零成本抽象。
- **UI 框架**：使用高性能 **Skia** 渲染後端的 Slint。
- **非同步執行階段**：Tokio，用於非阻塞系統命令和 I/O。
- **性能亮點**：
  - **響應速度**：近乎瞬時的啟動速度，並即時監控 WSL 狀態。
  - **資源效率**：極低的資源佔用（詳见 [核心功能](#-核心功能與使用)）。
  - **便攜性**：優化後的發布版本生成單個精簡的可執行文件。



## ⭐️ 為愛發電

如果您覺得這個项目對您有所幫助，我將不胜感激您能在 GitHub 上點亮一顆星。您的認可將幫助项目觸及更廣泛的用戶群體，我也深表謝意。正是这种鼓勵激勵着我不断前行。

## 📄 開源協議

本项目採用 GPL-3.0 協議授權 – 詳见 [LICENSE](../LICENSE) 文件。

---

Built with ❤️ for the WSL Community.

---

## 🤝 Community Support

A big thank you to the following communities for their support:

- [Rust Programming Language](https://www.rust-lang.org) - For the powerful and safe programming language
- [Slint | Declarative GUI for Rust, C++, JavaScript & Python](https://slint.dev) - For the modern UI framework
- [WSL: Windows Subsystem for Linux](https://github.com/microsoft/WSL) - For the amazing Windows Subsystem for Linux
- [Tokio - An asynchronous Rust runtime](https://tokio.rs) - For the efficient async runtime
- [Windows Developer Community](https://developer.microsoft.com/en-us/windows/community) - For continuous platform improvements
- [Reddit](https://www.reddit.com) - For global community discussions and support
- [Linux.do](https://linux.do) - For popular community for IT professionals
- [V2EX](https://www.v2ex.com) - For Chinese tech community discussions

Your contributions and feedback make this project possible!
