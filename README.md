# WSL Dashboard

<p align="center">
  <img src="assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

A modern, high-performance, lightweight, and low-memory WSL (Windows Subsystem for Linux) instance management dashboard. Built with Rust and Slint for a premium native experience.

```diff
Notice:​

- WSL Dashboard is not distributed through the Microsoft Store.
- Any application listed there under the name "WSL Dashboard"​ is unauthorized and may be counterfeit.
- Please do not download it to avoid potential scams.
```

---

<p align="left">
  <a href="https://www.rust-lang.org" target="_blank"><img src="https://img.shields.io/badge/rust-v1.92+-orange.svg" alt="Rust" /></a>
  <a href="https://slint.dev" target="_blank"><img src="https://img.shields.io/badge/UI-Slint-blue.svg" alt="Slint" /></a>
  <a href="https://tokio.rs" target="_blank"><img src="https://img.shields.io/badge/Async-Tokio-000000.svg" alt="Tokio" /></a>
  <a href="https://github.com/microsoft/windows-rs" target="_blank"><img src="https://img.shields.io/badge/OS-Windows-0078D6.svg" alt="Windows" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-GPL--3.0-blue.svg" alt="License" /></a>
  <a href="https://hellogithub.com/repository/owu/wsl-dashboard" target="_blank"><img src="https://api.hellogithub.com/v1/widgets/recommend.svg?rid=cb1edc45846e475da1dae615a4b4f71c&claim_uid=mWIRuYqZo1FUrjE&theme=small" alt="Featured｜HelloGitHub" /></a>
</p>

README I18N :  English | [简体中文](./manual/README_zh_CN.md) | [繁體中文](./manual/README_zh_TW.md) | [हिन्दी](./manual/README_hi.md) | [Español](./manual/README_es.md) | [Français](./manual/README_fr.md) | [العربية](./manual/README_ar.md) | [বাংলা](./manual/README_bn.md) | [Português](./manual/README_pt.md) | [Русский](./manual/README_ru.md) | [اردو](./manual/README_ur.md) | [Bahasa Indonesia](./manual/README_id.md) | [Deutsch](./manual/README_de.md) | [日本語](./manual/README_ja.md) | [Türkçe](./manual/README_tr.md) | [한국어](./manual/README_ko.md) | [Italiano](./manual/README_it.md) | [Nederlands](./manual/README_nl.md) | [Svenska](./manual/README_sv.md) | [Čeština](./manual/README_cs.md) | [Ελληνικά](./manual/README_el.md) | [Magyar](./manual/README_hu.md) | [עברית](./manual/README_he.md) | [Norsk](./manual/README_no.md) | [Dansk](./manual/README_da.md) | [Suomi](./manual/README_fi.md) | [Slovenčina](./manual/README_sk.md) | [Slovenščina](./manual/README_sl.md) | [Íslenska](./manual/README_is.md) | [Tiếng Việt](./manual/README_vi.md) | [తెలుగు](./manual/README_te.md) | [Basa Jawa](./manual/README_jv.md) | [ภาษาไทย](./manual/README_th.md) | [தமிழ்](./manual/README_ta.md) | [Filipino](./manual/README_fil.md) | [ਪੰਜਾਬੀ](./manual/README_pa.md) | [Bahasa Melayu](./manual/README_ms.md) | [Polski](./manual/README_pl.md) | [Українська](./manual/README_uk.md) | [فارسی](./manual/README_fa.md) | [ಕನ್ನಡ](./manual/README_kn.md) | [मराठी](./manual/README_mr.md) | [Hausa](./manual/README_ha.md) | [မြန်မာ](./manual/README_my.md) | [Oʻzbek](./manual/README_uz.md) | [Azərbaycan](./manual/README_az.md) | [Cebuano](./manual/README_ceb.md) | [മലയാളം](./manual/README_ml.md) | [سنڌي](./manual/README_sd.md) | [አማርኛ](./manual/README_am.md)

---

## 📑 Table of Contents
[🌍 Language Support](#-language-support) | [🚀 Key Features & Usage](#-key-features--usage) | [🖼️ Screenshots](#️-screenshots) | [📦 Installation](#-installation) | [📢 Official Community Groups](#-official-community-groups) | [❤️ Support this project](#️-support-this-project) | [⭐️ Labor of love](#️-labor-of-love) | [📄 License](#-license)

---

## 🌍 Language Support

English, Simplified Chinese, Traditional Chinese, Hindi, Spanish, French, Arabic, Bengali, Portuguese, Russian, Urdu, Indonesian, German, Japanese, Turkish, Korean, Italian, Dutch, Swedish, Czech, Greek, Hungarian, Hebrew, Norwegian, Danish, Finnish, Slovak, Slovenian, Icelandic, Vietnamese, Telugu, Javanese, Thai, Tamil, Filipino, Punjabi, Malay, Polish, Ukrainian, Persian, Kannada, Marathi, Hausa, Burmese, Uzbek, Azerbaijani, Cebuano, Malayalam, Sindhi, Amharic

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
  <img src="assets/flags/vn.svg" width="32" title="Vietnamese" alt="Vietnamese" />
  <img src="assets/flags/in.svg" width="32" title="Telugu" alt="Telugu" />
  <img src="assets/flags/id.svg" width="32" title="Javanese" alt="Javanese" />
  <img src="assets/flags/th.svg" width="32" title="Thai" alt="Thai" />
  <img src="assets/flags/in.svg" width="32" title="Tamil" alt="Tamil" />
  <img src="assets/flags/ph.svg" width="32" title="Filipino" alt="Filipino" />
  <img src="assets/flags/pk.svg" width="32" title="Punjabi" alt="Punjabi" />
  <img src="assets/flags/my.svg" width="32" title="Malay" alt="Malay" />
  <img src="assets/flags/pl.svg" width="32" title="Polish" alt="Polish" />
  <img src="assets/flags/ua.svg" width="32" title="Ukrainian" alt="Ukrainian" />
  <img src="assets/flags/ir.svg" width="32" title="Persian" alt="Persian" />
  <img src="assets/flags/in.svg" width="32" title="Kannada" alt="Kannada" />
  <img src="assets/flags/in.svg" width="32" title="Marathi" alt="Marathi" />
  <img src="assets/flags/ng.svg" width="32" title="Hausa" alt="Hausa" />
  <img src="assets/flags/mm.svg" width="32" title="Burmese" alt="Burmese" />
  <img src="assets/flags/uz.svg" width="32" title="Uzbek" alt="Uzbek" />
  <img src="assets/flags/az.svg" width="32" title="Azerbaijani" alt="Azerbaijani" />
  <img src="assets/flags/ph.svg" width="32" title="Cebuano" alt="Cebuano" />
  <img src="assets/flags/in.svg" width="32" title="Malayalam" alt="Malayalam" />
  <img src="assets/flags/pk.svg" width="32" title="Sindhi" alt="Sindhi" />
  <img src="assets/flags/et.svg" width="32" title="Amharic" alt="Amharic" />
</p>

## 🚀 Key Features & Usage

- **Modern Native UI**: Intuitive GUI with Dark/Light mode support, smooth animations, and high-performance rendering powered by **Skia**.
- **System Tray Integration**: Full support for system tray minimizing (~10MB RAM usage), double-click to toggle, and a functional right-click menu.
- **Intelligent Startup**: Configure the dashboard to start with Windows, minimize to tray (silent mode with `/silent`), and auto-shutdown distributions on exit.
- **Task Scheduler**: Flexible task scheduler management, supporting automatic execution of scripts and commands in WSL instances at specified times or intervals for automated operations.
- **Comprehensive Instance Control**: One-click Start, Stop, Terminate, and Unregister. Real-time status monitoring and detailed insights into disk usage and file locations.
- **Distro Management**: Set as default, migration (Move VHDX to other drives), and export/clone to `.tar` or `.tar.gz` archives.
- **Disk Mount**: Directly mount and manage WSL instance VHDX virtual disks, supporting mount, unmount, and disk management operations, making it easy to transfer files between the host and WSL.
- **Quick Integration**: Instant launch into Terminal, VS Code, or File Explorer with customizable working directories and startup script hooks.
- **Distro Installation**: Install Linux distributions via Microsoft Store, GitHub, local files (RootFS/VHDX), or Online Mirrors (with auto speed-test to pick the fastest mirror and built-in RootFS download helper).
- **Global Safety**: Mutex locks for safe concurrent migration/backup operations and automatic Appx cleanup on removal.
- **Ultra-Low Memory Footprint**: Highly optimized for efficiency. Silent startup (system tray) uses only **~10MB** RAM. Windowed mode usage varies by font complexity: **~18MB** for standard languages (English, German, Spanish, etc.) and **~38MB** for large font languages (Chinese, Japanese, Korean, etc.).
- **Advanced Networking**: Seamless port forwarding management (with automatic firewall rule creation) and global HTTP proxy configuration for unified connectivity.
- **USB Device Management**: Full integration with `usbipd-win` for effortless binding, attaching, and managing of local USB devices across your WSL instances directly from the dashboard UI.
- **Configuration & Logs**: Manage the default install directory, log directory and level, UI language, dark mode, update-check frequency, autostart and close behavior, plus sidebar visibility, all from one Settings view.
- **Distro Keep-Alive**: Start a distro with one click to keep it running instead of shutting down after ~60 seconds of idle; or enable "Start this distro when Windows starts" for always-on use.
- **Global Shortcuts**: Define global hotkeys to show/hide the main window or open a chosen distro's terminal instantly.


## 🖼️ Screenshots
<p align="center">
  <img src="assets/screenshot/home.png" width="48%" />
  <img src="assets/screenshot/home-dark.png" width="48%" />
</p>

Want to see the dashboard in action? Visit the [official website](https://www.wslui.com/) for the full screenshot gallery.

## 📦 Installation

### Option 1: Visit the project website (Recommended)

We recommend visiting the official website to download, as it offers multiple mirror links for a smoother experience:

Go to the [Download page](https://www.wslui.com/download/) and choose the mirror suitable for your region.

### Option 2: Install via winget

You can install WSLDashboard directly from the Windows Package Manager (winget), using either the moniker or the full package identifier:

```powershell
# Search (case-insensitive)
winget search wsl-dashboard
# or
winget search WSLDashboard

# Install (pick one)
winget install wsl-dashboard
# or
winget install Owu.WSLDashboard
```

> The winget package identifier is `Owu.WSLDashboard` and the moniker is `wsl-dashboard` (case-insensitive). Either works.

For more information, visit the [WinGet community repository](https://github.com/microsoft/winget-pkgs/tree/master/manifests/o/Owu/WSLDashboard).

### Option 3: Build from source

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
     .\build\portable\build.ps1 -Arch x64
     ```


## 📢 Official Community Groups

Welcome to the official WSL UI community groups! Join our channels to discuss usage tips, report issues, get the latest updates, and connect with the developers and other users.

<div align="center">

| Global Group (Telegram) | Chinese Group (WeChat) |
| :---: | :---: |
| <img src="https://cdn1.wslui.com/static/images/group/group-telegram.png" width="240" alt="Telegram Group QR Code" /> | <img src="https://cdn1.wslui.com/static/images/group/group-wechat.png" width="240" alt="WeChat Group QR Code" /> |
| Scan to join | Scan to join |

</div>


## ❤️ Support this project

- This project is licensed under GPL-3.0 and is free for all users.
- From feature development and daily testing to bug fixes, all work is done in spare time. The road of open source is not easy alone — your recognition and support give the project the confidence to keep going.
- If this tool has genuinely helped you, consider lending a hand. All donations go toward server costs, version iterations, and feature improvements, keeping the project continuously updated and steadily progressing.
- Every bit of kindness is a ray of starlight. Thank you again for your understanding and generosity! [Visit our donation page](https://www.wslui.com/donate/).


## ⭐️ Labor of love

If you have found this project useful, I would be grateful if you could leave a star on GitHub. Your endorsement helps it reach a wider audience and is deeply appreciated. It is this encouragement that motivates me to keep building.


## 📄 License

This project is licensed under the GPL-3.0 – see the [LICENSE](LICENSE) file for details.


---

❤️ Thanks to these communities: [Rust Programming Language](https://www.rust-lang.org), [Slint](https://slint.dev), [WSL](https://github.com/microsoft/WSL), [Tokio](https://tokio.rs), [Windows Developer Community](https://developer.microsoft.com/en-us/windows/community), [Reddit](https://www.reddit.com), [Hacker News](https://news.ycombinator.com), [Linux.do](https://linux.do), [V2EX](https://www.v2ex.com).
