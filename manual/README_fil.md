# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

Isang moderno, high-performance, magaan at mababang memory na dashboard para sa pamamahala ng mga WSL (Windows Subsystem for Linux) na instance. Naka-built gamit ang Rust at Slint para sa pinakamahusay na native na karanasan.

---

```diff
Paunawa:

- Ang WSL Dashboard ay hindi ipinamamahagi sa pamamagitan ng Microsoft Store.
- Ang anumang application na nakalista doon sa pangalang "WSL Dashboard" ay hindi awtorisado at maaaring peke.
- Mangyaring huwag itong i-download upang maiwasan ang mga posibleng pandaraya.
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

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | Filipino | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 Talaan ng Nilalaman
- [🌍 Suportang Wika](#-suportang-wika)
- [🚀 Mga Pangunahing Tampok at Paggamit](#-mga-pangunahing-tampok-at-paggamit)
- [⚙️ Konpigurasyon at Log](#️-konpigurasyon-at-log)
- [🖼️ Mga Screenshot](#️-mga-screenshot)
- [🎬 Demo ng Operasyon](#-demo-ng-operasyon)
- [💻 Mga Kinakailangan sa Sistema](#-mga-kinakailangan-sa-sistema)
- [📦 Gabay sa Pag-install](#-gabay-sa-pag-install)
- [🛠️ Tech Stack at Performance](#️-tech-stack-at-performance)
- [🤝 Suporta ng Komunidad](#-suporta-ng-komunidad)
- [❤️ Suportahan ang Proyektong Ito](#️-suportahan-ang-proyektong-ito)
- [⭐️ Bituin ng Suporta](#️-bituin-ng-suporta)
- [📄 Lisensyang Open Source](#-lisensyang-open-source)

---

## 🌍 Suportang Wika

Ingles, Pinasimpleng Tsino, Tradisyonal na Tsino, Hindi, Espanyol, Pranses, Arabe, Bengali, Portuges, Ruso, Urdu, Indones, Aleman, Hapon, Turko, Koreano, Italyano, Olandes, Sweko, Tseko, Griyego, Hungariano, Hebrew, Norweyano, Danes, Pinlandes, Slovak, Sloveno, Islandes, Vietnamese, Telugu, Javanese, Thai, Tamil, Filipino, Punjabi, Malay, Polish, Ukrainian, Persian, Kannada, Marathi, Hausa, Myanmar, Uzbek, Azerbaijani, Cebuano, Malayalam, Sindhi, Amharic

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="Ingles" alt="Ingles" />
  <img src="../assets/flags/cn.svg" width="32" title="Pinasimpleng Tsino" alt="Pinasimpleng Tsino" />
  <img src="../assets/flags/tw.svg" width="32" title="Tradisyonal na Tsino" alt="Tradisyonal na Tsino" />
  <img src="../assets/flags/in.svg" width="32" title="Hindi" alt="Hindi" />
  <img src="../assets/flags/es.svg" width="32" title="Espanyol" alt="Espanyol" />
  <img src="../assets/flags/fr.svg" width="32" title="Pranses" alt="Pranses" />
  <img src="../assets/flags/sa.svg" width="32" title="Arabe" alt="Arabe" />
  <img src="../assets/flags/bd.svg" width="32" title="Bengali" alt="Bengali" />
  <img src="../assets/flags/pt.svg" width="32" title="Portuges" alt="Portuges" />
  <img src="../assets/flags/ru.svg" width="32" title="Ruso" alt="Ruso" />
  <img src="../assets/flags/pk.svg" width="32" title="Urdu" alt="Urdu" />
  <img src="../assets/flags/id.svg" width="32" title="Indones" alt="Indones" />
  <img src="../assets/flags/de.svg" width="32" title="Aleman" alt="Aleman" />
  <img src="../assets/flags/jp.svg" width="32" title="Hapon" alt="Hapon" />
  <img src="../assets/flags/tr.svg" width="32" title="Turko" alt="Turko" />
  <img src="../assets/flags/kr.svg" width="32" title="Koreano" alt="Koreano" />
  <img src="../assets/flags/it.svg" width="32" title="Italyano" alt="Italyano" />
  <img src="../assets/flags/nl.svg" width="32" title="Olandes" alt="Olandes" />
  <img src="../assets/flags/se.svg" width="32" title="Sweko" alt="Sweko" />
  <img src="../assets/flags/cz.svg" width="32" title="Tseko" alt="Tseko" />
  <img src="../assets/flags/gr.svg" width="32" title="Griyego" alt="Griyego" />
  <img src="../assets/flags/hu.svg" width="32" title="Hungariano" alt="Hungariano" />
  <img src="../assets/flags/il.svg" width="32" title="Hebrew" alt="Hebrew" />
  <img src="../assets/flags/no.svg" width="32" title="Norweyano" alt="Norweyano" />
  <img src="../assets/flags/dk.svg" width="32" title="Danes" alt="Danes" />
  <img src="../assets/flags/fi.svg" width="32" title="Pinlandes" alt="Pinlandes" />
  <img src="../assets/flags/sk.svg" width="32" title="Slovak" alt="Slovak" />
  <img src="../assets/flags/si.svg" width="32" title="Sloveno" alt="Sloveno" />
  <img src="../assets/flags/is.svg" width="32" title="Islandes" alt="Islandes" />
  <img src="../assets/flags/vn.svg" width="32" title="Vietnamese" alt="Vietnamese" />
  <img src="../assets/flags/in.svg" width="32" title="Telugu" alt="Telugu" />
  <img src="../assets/flags/id.svg" width="32" title="Javanese" alt="Javanese" />
  <img src="../assets/flags/th.svg" width="32" title="Thai" alt="Thai" />
  <img src="../assets/flags/in.svg" width="32" title="Tamil" alt="Tamil" />
  <img src="../assets/flags/ph.svg" width="32" title="Filipino" alt="Filipino" />
  <img src="../assets/flags/pk.svg" width="32" title="Punjabi" alt="Punjabi" />
  <img src="../assets/flags/my.svg" width="32" title="Malay" alt="Malay" />
  <img src="../assets/flags/pl.svg" width="32" title="Polish" alt="Polish" />
  <img src="../assets/flags/ua.svg" width="32" title="Ukrainian" alt="Ukrainian" />
  <img src="../assets/flags/ir.svg" width="32" title="Persian" alt="Persian" />
  <img src="../assets/flags/in.svg" width="32" title="Kannada" alt="Kannada" />
  <img src="../assets/flags/in.svg" width="32" title="Marathi" alt="Marathi" />
  <img src="../assets/flags/ng.svg" width="32" title="Hausa" alt="Hausa" />
  <img src="../assets/flags/mm.svg" width="32" title="Burmese" alt="Burmese" />
  <img src="../assets/flags/uz.svg" width="32" title="Uzbek" alt="Uzbek" />
  <img src="../assets/flags/az.svg" width="32" title="Azerbaijani" alt="Azerbaijani" />
  <img src="../assets/flags/ph.svg" width="32" title="Cebuano" alt="Cebuano" />
  <img src="../assets/flags/in.svg" width="32" title="Malayalam" alt="Malayalam" />
  <img src="../assets/flags/pk.svg" width="32" title="Sindhi" alt="Sindhi" />
  <img src="../assets/flags/et.svg" width="32" title="Amharic" alt="Amharic" />
</p>


## 🚀 Mga Pangunahing Tampok at Paggamit

- **Modernong Native na UI**: Intuitive na GUI na may suporta sa dark/light mode, makinis na animation at high-performance rendering na pinapagana ng **Skia**.
- **System Tray Integration**: Buong tray support (~10MB memory usage), double-click para ipakita/itago at full-featured na right-click menu.
- **Smart Startup**: Support sa startup sa boot, minimize sa tray (silent start gamit ang `/silent` parameter) at automatic na pagsasara ng distro kapag umalis.
- **Komprehensibong Instance Control**: Simulan, itigil, pilitin ang pagtigil at i-deregister sa isang click. Real-time na status monitoring, detalyadong disk usage at file location.
- **Distro Management**: Itakda bilang default, pisikal na migration (ilipat VHDX sa ibang disk) at mag-export/clone bilang `.tar` o `.tar.gz`.
- **Quick Integration**: Buksan ang terminal, VS Code o file manager sa isang click, may suporta sa custom working directory at launch script hooks.
- **Distro Installation**: Mag-install ng mga Linux distribution sa pamamagitan ng Microsoft Store, GitHub, lokal na mga file (RootFS/VHDX), o mga Online Mirror (na may auto speed-test para piliin ang pinakamabilis na mirror at built-in na RootFS download helper).
- **Global Security**: Mutex para sa kaligtasan ng simultaneous migration/backup operations at automatic Appx package cleanup kapag nag-aalis.
- **Napakababang Memory Usage**: Highly optimized na resource efficiency. Silent start (system tray) ay gumagamit lamang ng humigit-kumulang **10MB** memory. Sa window mode, gumagamit ng humigit-kumulang **18MB** (standard na wika tulad ng Ingles, Aleman, atbp.) hanggang **38MB** (malaking character set tulad ng CJK) depende sa complexity ng font.
- **Advanced Network Management**: Seamless na pamamahala ng port forwarding (automatic na paggawa ng firewall rules) at global HTTP proxy configuration para sa unified na karanasan sa koneksyon.
- **USB Device Management**: Malalim na integration sa `usbipd-win`, na nagbibigay-daan sa madaling pag-bind, pag-attach at pamamahala ng mga lokal na USB device para sa lahat ng WSL instance nang direkta mula sa dashboard UI.


## ⚙️ Konpigurasyon at Log

Lahat ng konpigurasyon ay pinamamahalaan sa pamamagitan ng "Settings" view:

- Pumili ng default installation directory para sa mga bagong WSL instance.
- I-configure ang log directory at log level (Error / Warn / Info / Debug / Trace).
- Pumili ng UI language o sundin ang system language.
- I-toggle ang dark mode at kung ang app ay awtomatikong isasara ang WSL pagkatapos ng operasyon.
- I-configure ang frequency ng pag-check ng update (araw-araw, lingguhan, bawat dalawang linggo, buwan-buwan).
- I-enable ang startup sa boot (may automatic path repair feature).
- I-set ang minimize sa tray sa startup.
- I-configure ang behavior ng close button (minimize sa tray sa halip na isara ang programa).
- I-customize ang sidebar sa pamamagitan ng pag-toggle ng visibility ng mga partikular na feature tab.

Ang mga log file ay isinusulat sa na-configure na log directory at maaaring isama kapag nag-uulat ng mga isyu.


## 🖼️ Mga Screenshot

### Home Screen (Dark & Light Mode)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB & Collapsed Menu
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### Network Management
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### Add Instance & Settings
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>
<p align="center">
  <img src="../assets/screenshot/settings-advanced.png" width="48%" />
  <img src="../assets/screenshot/settings-interface.png" width="48%" />
</p>

### Tungkol sa & Donasyon
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/donate.png" width="48%" />
</p>

## 🎬 Demo ng Operasyon

[Tulungan kaming mapabuti! Panoorin ang aming introductory video at ibahagi ang iyong mga saloobin.](https://github.com/owu/wsl-dashboard/discussions/9)



## 💻 Mga Kinakailangan sa Sistema

- Windows 10 o Windows 11 na may naka-enable na WSL (inirerekumenda ang WSL 2).
- Hindi bababa sa isang naka-install na WSL distro, o pahintulot na mag-install ng bago.
- 64-bit na CPU; inirerekumenda ang 4 GB RAM o higit pa para sa makinis na paggamit ng maraming distro.

## 📦 Gabay sa Pag-install

### Paraan 1: I-download ang Pre-compiled na Binary

Ang pinakamadaling paraan para magsimula ay gamitin ang compiled na bersyon:

1. Pumunta sa [GitHub Releases](https://github.com/owu/wsl-dashboard/releases) page.
2. I-download ang pinakabagong `wsldashboard` executable para sa Windows.
3. I-extract (kung archive) at patakbuhin ang `wsldashboard.exe`.

Walang kailangang i-install — ang app ay isang portable na programa na iisang file.

### Paraan 2: I-build mula sa Source Code

Siguraduhin na naka-install ang Rust toolchain (Rust 1.92+ o mas bago).

1. I-clone ang repository:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. I-build at patakbuhin:

   - Para sa development at debugging:

     ```powershell
     cargo run
     ```
   - Optimized na release build gamit ang build script:

     > Ang build script ay nangangailangan ng `x86_64-pc-windows-msvc` toolchain.

     ```powershell
     .\build\portable\build.ps1
     ```


## 🛠️ Tech Stack at Performance

- **Core**: Na-implement sa Rust para sa memory safety at zero-cost abstraction.
- **UI Framework**: Slint na may high-performance **Skia** rendering backend.
- **Async runtime**: Tokio, para sa non-blocking system commands at I/O.
- **Performance Highlights**:
  - **Response Speed**: Halos instant na startup, real-time na WSL status monitoring.
  - **Resource Efficiency**: Napakababang resource consumption (tingnan ang [Core Features](#-mga-pangunahing-tampok-at-paggamit)).
  - **Portability**: Ang optimized na release build ay gumagawa ng iisang compact na executable.



## 🤝 Suporta ng Komunidad

Taos-pusong pasasalamat sa mga sumusunod na komunidad para sa kanilang suporta:

- [Rust Programming Language](https://www.rust-lang.org) - Makapangyarihan at ligtas na programming language
- [Slint | Declarative GUI for Rust, C++, JavaScript & Python](https://slint.dev) - Modernong UI framework
- [WSL: Windows Subsystem for Linux](https://github.com/microsoft/WSL) - Napakagandang Windows Subsystem for Linux
- [Tokio - An asynchronous Rust runtime](https://tokio.rs) - Mahusay na async runtime
- [Windows Developer Community](https://developer.microsoft.com/en-us/windows/community) - Patuloy na pagpapahusay ng platform
- [Reddit](https://www.reddit.com) - Global na komunidad na diskusyon at suporta
- [Hacker News](https://news.ycombinator.com) - Global na komunidad na diskusyon at suporta
- [Linux.do](https://linux.do) - Sikat na komunidad ng mga IT propesyonal
- [V2EX](https://www.v2ex.com) - Diskusyon sa Chinese tech community

Ang iyong mga kontribusyon at feedback ang nagpapakilala sa proyektong ito!


## ❤️ Suportahan ang Proyektong Ito

- Ang proyektong ito ay naka-lisensya sa ilalim ng GPL-3.0 at available nang libre para sa lahat ng user.
- Mula sa pagbuo ng feature, araw-araw na testing hanggang sa pag-aayos ng bug — lahat ng trabaho ay ginagawa sa libreng oras. Ang daan ng open source ay hindi madaling lakarin nang mag-isa. Ang iyong pagkilala at suporta ang nagbibigay ng lakas ng loob sa proyekto na magpatuloy.
- Kung ang tool na ito ay talagang nakatulong sa iyo, mangyaring isaalang-alang ang pagsuporta. Lahat ng donasyon ay mapupunta sa server cost, pagbuo ng bersyon at pagpapahusay ng feature, upang ang proyekto ay patuloy na ma-update at matatag na umunlad.
- Bawat maliit na kabutihan ay isang sinag ng bituin. Muli, salamat sa iyong pag-unawa at kagandahang-loob!

Bisitahin ang aming donation page: [https://www.wslui.com/donate/](https://www.wslui.com/donate/)


## ⭐️ Bituin ng Suporta

Kung sa tingin mo ay kapaki-pakinabang ang proyektong ito, lubos akong nagpapasalamat kung maaari kang magbigay ng bituin sa GitHub. Ang iyong suporta ay makakatulong sa proyekto na maabot ang mas maraming user at lubos kong pinahahalagahan ito. Ang paghihikayat na ito ang nagtutulak sa akin na magpatuloy.


## 📄 Lisensyang Open Source

Ang proyektong ito ay naka-lisensya sa ilalim ng GPL-3.0 – tingnan ang file na [LICENSE](../LICENSE) para sa mga detalye.


---

Built with ❤️ for the WSL Community.