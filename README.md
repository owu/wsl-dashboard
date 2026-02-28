# WSL Dashboard

<p align="center">
  <img src="assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

A modern, high-performance, and lightweight WSL (Windows Subsystem for Linux) instance management dashboard. Built with Rust and Slint for a premium native experience.

---

<p align="left">
  <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/rust-v1.92+-orange.svg" alt="Rust" /></a>
  <a href="https://slint.dev"><img src="https://img.shields.io/badge/UI-Slint-blue.svg" alt="Slint" /></a>
  <a href="https://tokio.rs"><img src="https://img.shields.io/badge/Async-Tokio-000000.svg" alt="Tokio" /></a>
  <a href="https://github.com/microsoft/windows-rs"><img src="https://img.shields.io/badge/OS-Windows-0078D6.svg" alt="Windows" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-GPL--3.0-blue.svg" alt="License" /></a>
</p>

I18N :  English | [简体中文](./manual/README_zh_CN.md) | [繁體中文](./manual/README_zh_TW.md) | [हिन्दी](./manual/README_hi.md) | [Español](./manual/README_es.md) | [Français](./manual/README_fr.md) | [العربية](./manual/README_ar.md) | [বাংলা](./manual/README_bn.md) | [Português](./manual/README_pt.md) | [Русский](./manual/README_ru.md) | [اردو](./manual/README_ur.md) | [Bahasa Indonesia](./manual/README_id.md) | [Deutsch](./manual/README_de.md) | [日本語](./manual/README_ja.md) | [Türkçe](./manual/README_tr.md) | [한국어](./manual/README_ko.md) | [Italiano](./manual/README_it.md) | [Nederlands](./manual/README_nl.md) | [Svenska](./manual/README_sv.md) | [Čeština](./manual/README_cs.md) | [Ελληνικά](./manual/README_el.md) | [Magyar](./manual/README_hu.md) | [עברית](./manual/README_he.md) | [Norsk](./manual/README_no.md) | [Dansk](./manual/README_da.md) | [Suomi](./manual/README_fi.md) | [Slovenčina](./manual/README_sk.md) | [Slovenščina](./manual/README_sl.md) | [Íslenska](./manual/README_is.md) | [Tiếng Việt](./manual/README_vi.md)

---

## 📑 Table of Contents
- [🌍 Language Support](#-language-support)
- [🚀 Key Features & Usage](#-key-features--usage)
- [⚙️ Configuration & Logs](#️-configuration--logs)
- [🖼️ Screenshots](#️-screenshots)
- [🎬 Operation Demo](#-operation-demo)
- [💻 System Requirements](#-system-requirements)
- [📦 Installation](#-installation)
- [🛠️ Tech Stack & Performance](#️-tech-stack--performance)
- [📄 License](#-license)

---

## 🌍 Language Support

English, Simplified Chinese, Traditional Chinese, Hindi, Spanish, French, Arabic, Bengali, Portuguese, Russian, Urdu, Indonesian, German, Japanese, Turkish, Korean, Italian, Dutch, Swedish, Czech, Greek, Hungarian, Hebrew, Norwegian, Danish, Finnish, Slovak, Slovenian, Icelandic, Vietnamese

<p align="left">
  <img src="assets/flags/us.svg" width="32" title="English" alt="English" />
  <img src="assets/flags/cn.svg" width="32" title="Simplified Chinese" alt="Simplified Chinese" />
  <img src="assets/flags/tw.svg" width="32" title="Traditional Chinese" alt="Traditional Chinese" />
  <img src="assets/flags/in.svg" width="32" title="Hindi" alt="Hindi" />
  <img src="assets/flags/es.svg" width="32" title="Spanish" alt="Spanish" />
  <img src="assets/flags/fr.svg" width="32" title="French" alt="French" />
  <img src="assets/flags/sa.svg" width="32" title="Arabic" alt="Arabic" />
  <img src="assets/flags/bd.svg" width="32" title="Bengali" alt="Bengali" />
  <img src="assets/flags/pt.svg" width="32" title="Portuguese" alt="Portuguese" />
  <img src="assets/flags/ru.svg" width="32" title="Russian" alt="Russian" />
  <img src="assets/flags/pk.svg" width="32" title="Urdu" alt="Urdu" />
  <img src="assets/flags/id.svg" width="32" title="Indonesian" alt="Indonesian" />
  <img src="assets/flags/de.svg" width="32" title="German" alt="German" />
  <img src="assets/flags/jp.svg" width="32" title="Japanese" alt="Japanese" />
  <img src="assets/flags/tr.svg" width="32" title="Turkish" alt="Turkish" />
  <img src="assets/flags/kr.svg" width="32" title="Korean" alt="Korean" />
  <img src="assets/flags/it.svg" width="32" title="Italian" alt="Italian" />
  <img src="assets/flags/nl.svg" width="32" title="Dutch" alt="Dutch" />
  <img src="assets/flags/se.svg" width="32" title="Swedish" alt="Swedish" />
  <img src="assets/flags/cz.svg" width="32" title="Czech" alt="Czech" />
  <img src="assets/flags/gr.svg" width="32" title="Greek" alt="Greek" />
  <img src="assets/flags/hu.svg" width="32" title="Hungarian" alt="Hungarian" />
  <img src="assets/flags/il.svg" width="32" title="Hebrew" alt="Hebrew" />
  <img src="assets/flags/no.svg" width="32" title="Norwegian" alt="Norwegian" />
  <img src="assets/flags/dk.svg" width="32" title="Danish" alt="Danish" />
  <img src="assets/flags/fi.svg" width="32" title="Finnish" alt="Finnish" />
  <img src="assets/flags/sk.svg" width="32" title="Slovak" alt="Slovak" />
  <img src="assets/flags/si.svg" width="32" title="Slovenian" alt="Slovenian" />
  <img src="assets/flags/is.svg" width="32" title="Icelandic" alt="Icelandic" />
  <img src="assets/flags/vn.svg" width="32" title="Tiếng Việt" alt="Tiếng Việt" />
</p>


## 🚀 Key Features & Usage

- **Modern Native UI**: Intuitive GUI with Dark/Light mode support, smooth animations, and high-performance rendering powered by **Skia**.
- **System Tray Integration**: Full support for system tray minimizing (~10MB RAM usage), double-click to toggle, and a functional right-click menu.
- **Intelligent Startup**: Configure the dashboard to start with Windows, minimize to tray (silent mode with `/silent`), and auto-shutdown distributions on exit.
- **Comprehensive Instance Control**: One-click Start, Stop, Terminate, and Unregister. Real-time status monitoring and detailed insights into disk usage and file locations.
- **Distro Management**: Set as default, migration (Move VHDX to other drives), and export/clone to `.tar` or `.tar.gz` archives.
- **Quick Integration**: Instant launch into Terminal, VS Code, or File Explorer with customizable working directories and startup script hooks.
- **Smart Installation**: Install from Microsoft Store, GitHub, or local files (RootFS/VHDX). Includes a built-in RootFS download helper.
- **Global Safety**: Mutex locks for safe concurrent migration/backup operations and automatic Appx cleanup on removal.
- **Ultra-Low Memory Footprint**: Highly optimized for efficiency. Silent startup (system tray) uses only **~10MB** RAM. Windowed mode usage varies by font complexity: **~18MB** for standard languages (English, German, Spanish, etc.) and **~38MB** for large font languages (Chinese, Japanese, Korean, etc.).


## ⚙️ Configuration & Logs

All configuration is managed through the Settings view:

- Choose the default installation directory for new WSL instances.
- Configure the log directory and log level (Error / Warn / Info / Debug / Trace).
- Pick the UI language or let it follow the system language.
- Toggle dark mode and whether the app can auto-shutdown WSL after operations.
- Configure how often the app checks for updates (daily, weekly, biweekly, monthly).
- Enable automatic startup on system boot (with automatic path repair).
- Set the app to minimize to the system tray on startup for a distraction-free experience.
- Configure the close button to minimize to the system tray instead of exiting.

Log files are written to the configured log directory and can be attached when reporting issues.


## 🖼️ Screenshots

### Home (Light & Dark Mode)
<p align="center">
  <img src="assets/screenshot/home.png" width="48%" />
  <img src="assets/screenshot/home-dark.png" width="48%" />
</p>

### Add Instance & Settings
<p align="center">
  <img src="assets/screenshot/add.png" width="48%" />
  <img src="assets/screenshot/settings.png" width="48%" />
</p>

### About & Collapse menu
<p align="center">
  <img src="assets/screenshot/about.png" width="48%" />
  <img src="assets/screenshot/collapsed.png" width="48%" />
</p>

## 🎬 Operation Demo

Below is a demonstration of the WSL Dashboard in action:

![WSL Dashboard Demo](assets/screenshot/demo.gif)



## 💻 System Requirements

- Windows 10 or Windows 11 with WSL enabled (WSL 2 recommended).
- At least one WSL distribution installed, or permission to install new ones.
- 64-bit CPU; 4 GB RAM or more recommended for smooth multi-distro usage.

## 📦 Installation

### Option 1: Download prebuilt binary

The easiest way to get started is to use the precompiled release:

1. Go to the [GitHub Releases](https://github.com/owu/wsl-dashboard/releases) page.
2. Download the latest `wsldashboard` executable for Windows.
3. Extract (if packaged) and run `wsldashboard.exe`.

No installer is required; the app is a single portable binary.

### Option 2: Build from source

Ensure you have the Rust toolchain (Rust 1.92+ or newer) installed.

1. Clone the repository:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. Build and run:

   - For development:

     ```powershell
     cargo run
     ```
   - Optimized release build, using the build script:

     > The build script requires the `x86_64-pc-windows-msvc` toolchain.

     ```powershell
     .\build\scripts\build.ps1
     ```


## 🛠️ Tech Stack & Performance

- **Core**: Implemented in Rust for memory safety and zero-cost abstractions.
- **UI Framework**: Slint with high-performance **Skia** rendering backend.
- **Async Runtime**: Tokio for non-blocking system commands and I/O.
- **Performance Highlights**:
  - **Responsiveness**: Near-instant startup and real-time WSL status monitoring.
  - **Efficiency**: Ultra-low resource usage (see [Key Features](#-key-features--usage) for details).
  - **Portability**: Optimized release build produces a single compact executable.



## 📄 License

This project is licensed under the GPL-3.0 – see the [LICENSE](LICENSE) file for details.

---

Built with ❤️ for the WSL Community.

