# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

Usa ka moderno, high-performance, gaan ug ubos ang memory usage nga dashboard sa pagdumala sa WSL (Windows Subsystem for Linux) instances. Gitukod gamit ang Rust ug Slint, naghatag ug labing maayong native nga kasinatian.

```diff
Pahibalo:

- Ang WSL Dashboard wala gipanghatag pinaagi sa Microsoft Store.
- Bisan unsang aplikasyon nga gilista didto uban sa ngalan nga "WSL Dashboard" wala giuyonan ug mahimong peke.
- Palihug ayaw kini i-download aron malikayan ang posibleng mga pagpanglimbong.
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

README I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | Cebuano | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 Talaan sa Sulod
[🌍 Pagsuporta sa Pinulongan](#-pagsuporta-sa-pinulongan) | [🚀 Mga Panguna nga Bahin ug Paggamit](#-mga-panguna-nga-bahin-ug-paggamit) | [🖼️ Mga Screenshot sa Software](#️-mga-screenshot-sa-software) | [📦 Giya sa Pag-install](#-giya-sa-pag-install) | [📢 Opisyal na Komunidad Grupo](#-opisyal-na-komunidad-grupo) | [❤️ Suportaha Kini nga Proyekto](#️-suportaha-kini-nga-proyekto) | [⭐️ Alang sa Gugma](#️-alang-sa-gugma) | [📄 Lisensya sa Open Source](#-lisensya-sa-open-source)

---

## 🌍 Pagsuporta sa Pinulongan

English, Simplified Chinese, Traditional Chinese, Hindi, Spanish, French, Arabic, Bengali, Portuguese, Russian, Urdu, Indonesian, German, Japanese, Turkish, Korean, Italian, Dutch, Swedish, Czech, Greek, Hungarian, Hebrew, Norwegian, Danish, Finnish, Slovak, Slovenian, Icelandic, Vietnamese, Telugu, Javanese, Thai, Tamil, Filipino, Punjabi, Malay, Polish, Ukrainian, Persian, Kannada, Marathi, Hausa, Myanmar, Uzbek, Azerbaijani, Cebuano, Malayalam, Sindhi, Amharic

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="English" alt="English" />
  <img src="../assets/flags/cn.svg" width="32" title="Simplified Chinese" alt="Simplified Chinese" />
  <img src="../assets/flags/tw.svg" width="32" title="Traditional Chinese" alt="Traditional Chinese" />
  <img src="../assets/flags/in.svg" width="32" title="Hindi" alt="Hindi" />
  <img src="../assets/flags/es.svg" width="32" title="Spanish" alt="Spanish" />
  <img src="../assets/flags/fr.svg" width="32" title="French" alt="French" />
  <img src="../assets/flags/sa.svg" width="32" title="Arabic" alt="Arabic" />
  <img src="../assets/flags/bd.svg" width="32" title="Bengali" alt="Bengali" />
  <img src="../assets/flags/pt.svg" width="32" title="Portuguese" alt="Portuguese" />
  <img src="../assets/flags/ru.svg" width="32" title="Russian" alt="Russian" />
  <img src="../assets/flags/pk.svg" width="32" title="Urdu" alt="Urdu" />
  <img src="../assets/flags/id.svg" width="32" title="Indonesian" alt="Indonesian" />
  <img src="../assets/flags/de.svg" width="32" title="German" alt="German" />
  <img src="../assets/flags/jp.svg" width="32" title="Japanese" alt="Japanese" />
  <img src="../assets/flags/tr.svg" width="32" title="Turkish" alt="Turkish" />
  <img src="../assets/flags/kr.svg" width="32" title="Korean" alt="Korean" />
  <img src="../assets/flags/it.svg" width="32" title="Italian" alt="Italian" />
  <img src="../assets/flags/nl.svg" width="32" title="Dutch" alt="Dutch" />
  <img src="../assets/flags/se.svg" width="32" title="Swedish" alt="Swedish" />
  <img src="../assets/flags/cz.svg" width="32" title="Czech" alt="Czech" />
  <img src="../assets/flags/gr.svg" width="32" title="Greek" alt="Greek" />
  <img src="../assets/flags/hu.svg" width="32" title="Hungarian" alt="Hungarian" />
  <img src="../assets/flags/il.svg" width="32" title="Hebrew" alt="Hebrew" />
  <img src="../assets/flags/no.svg" width="32" title="Norwegian" alt="Norwegian" />
  <img src="../assets/flags/dk.svg" width="32" title="Danish" alt="Danish" />
  <img src="../assets/flags/fi.svg" width="32" title="Finnish" alt="Finnish" />
  <img src="../assets/flags/sk.svg" width="32" title="Slovak" alt="Slovak" />
  <img src="../assets/flags/si.svg" width="32" title="Slovenian" alt="Slovenian" />
  <img src="../assets/flags/is.svg" width="32" title="Icelandic" alt="Icelandic" />
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

## 🚀 Mga Panguna nga Bahin ug Paggamit

- **Modernong Native UI**: Intuitive nga GUI nga adunay dark/light mode, hapsay nga animation, ug high-performance rendering nga gidagan sa **Skia**.
- **System Tray Integration**: Bug-os nga pagsuporta sa tray (~10MB memory usage), nga adunay toggle sa display/hide pinaagi sa double-click ug fully functional right-click menu.
- **Smart Startup**: Pagsuporta sa auto-start sa boot, minimize sa tray (gamit ang `/silent` parameter sa pag-start nga walay tunog), ug auto-close sa distro sa paggawas.
- **Pag-iskedyul sa Taksa**: Flexible nga pagdumala sa pag-iskedyul sa mga taksa, nagsuporta sa awtomatikong pag-execute sa mga script ug commands sa WSL instances sa gitakda nga oras o interval para sa automated nga operasyon.
- **Bug-os nga Pagkontrol sa Instance**: Sugdi, hunong, patya, ug i-unregister sa usa ka pag-klik. Real-time nga pag-monitor sa status, pagtan-aw sa paggamit sa disk ug lokasyon sa file.
- **Pagdumala sa Distro**: I-set isip default, pisikal nga pagbalhin (ibalhin ang VHDX sa lain disk), ug pag-export/clone isip `.tar` o `.tar.gz` archive.
- **Disk Mount**: Direktang i-mount ug i-manage ang WSL instance VHDX virtual disk, nagsuporta sa mount, unmount, ug disk management operations, mas sayon ang pagbalhin sa mga file tali sa host ug WSL.
- **Dali nga Pag-integrate**: Usa ka pag-click sa pag-abli sa terminal, VS Code, o File Explorer, nga adunay pagsuporta sa custom working directory ug startup script hooks.
- **Distro Installation**: Pag-install sa mga Linux distribution pinaagi sa Microsoft Store, GitHub, lokal nga file (RootFS/VHDX), o mga Online Mirror (nga adunay auto speed-test aron mapili ang pinakadali nga mirror ug built-in nga RootFS download helper).
- **Global Security**: Paggamit sa mutex aron masiguro ang seguridad sa sabay nga pag-migrate/backup operations, ug auto-cleanup sa Appx packages sa pagtangtang.
- **Grabe ka Uban ang Memory Usage**: Taas nga optimized resource efficiency. Walay tunog nga pag-start (system tray) mga **10MB** ra sa memory. Sa window mode, ang paggamit nagdepende sa pagkakomplikado sa font: mga **10MB** (standard languages sama sa English, German, ug uban pa) hangtod sa **38MB** (dakong character set sama sa CJK).
- **Advanced Network Management**: Walay-pugong nga pagdumala sa port forwarding (automatic firewall rule creation) ug global HTTP proxy configuration.
- **USB Device Management**: Lawom nga pag-integrate sa `usbipd-win`, nga nagtugot sa bind, attach, ug management operations sa lokal nga USB devices diretso sa dashboard UI alang sa tanan nga WSL instances.
- **Konfigurasyon ug mga log**: Pinaagi sa Settings, dumalahon ang default nga install folder, log folder ug lebel, pinulongan sa UI, dark mode, kanunay sa pagsusi sa update, autostart ug kinaiya sa pagsira, ug ang sidebar.
- **Pagpabiling aktibo sa distro**: Usa ka klik sa «Start» magpabiling nagdagan ang distro imbes palongon pagkahuman sa ~60 segundos nga walay kalihokan; mahimo usab nga i-on ang «Start ini nga distro kon magsugod ang Windows».
- **Global nga mga shortcut**: Pagtakda og global nga mga shortcut aron ipakita/tago ang panguna nga bintana o ablihan dayon ang terminal sa usa ka distro.


## 🖼️ Mga Screenshot sa Software
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

Gusto nimong makita ang dashboard nga naglihok? Bisitaha ang [opisyal nga website](https://www.wslui.com/) alang sa tibuok nga gallery sa mga screenshot.

## 📦 Giya sa Pag-install

### Paagi 1: Bisitaha ang website sa proyekto (Girekomenda)

Girekomenda namon ang pagbisita sa opisyal nga website aron mag-download, tungod kay naghatag kini daghang mga mirror link para sa mas hapsay nga kasinatian:

Adto sa [Download page](https://www.wslui.com/download/) ug pilia ang mirror nga angay sa imong rehiyon.

### Paagi 2: Pag-install pinaagi sa winget

Mahimo nimong i-install ang WSLDashboard direkta gikan sa Windows Package Manager (winget), gamit ang moniker o ang tibuok nga package identifier:

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

> Ang winget package identifier kay `Owu.WSLDashboard` ug ang moniker kay `wsl-dashboard` (case-insensitive). Bisan asa, molihok.

Para sa dugang impormasyon, bisitaha ang [WinGet community repository](https://github.com/microsoft/winget-pkgs/tree/master/manifests/o/Owu/WSLDashboard).

### Paagi 3: Pag-build gikan sa Source

Siguruha nga ang Rust toolchain gi-install (Rust 1.92+ o mas bag-ong bersyon).

1. I-clone ang repository:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. I-build ug i-run:

   - Development debug:

     ```powershell
     cargo run
     ```
   - I-build ang optimized release version gamit ang build script:

     > Ang build script nagkinahanglan sa `x86_64-pc-windows-msvc` toolchain.

     ```powershell
     .\build\portable\build.ps1 -Arch x64
     ```


## 📢 Opisyal na Komunidad Grupo

Malipayong pag-abot sa opisyal na mga grupo sa komunidad sa WSL UI! Sumali sa among mga channel para sa diskusyon sa mga tips sa paggamit, pag-report sa mga problema, pagkuha sa mga labing bag-ong update, ug pagkonekta sa mga developer ug uban pang mga user.

<div align="center">

| Global Group (Telegram) | Chinese Group (WeChat) |
| :---: | :---: |
| <img src="https://cdn1.wslui.com/static/images/group/group-telegram.png" width="240" alt="Telegram Group QR Code" /> | <img src="https://cdn1.wslui.com/static/images/group/group-wechat.png" width="240" alt="WeChat Group QR Code" /> |
| Scan para sumali | Scan para sumali |

</div>


## ❤️ Suportaha Kini nga Proyekto

- Kini nga proyekto naggamit sa GPL-3.0 open source license, libre alang sa tanan nga users.
- Gikan sa feature development, adlaw-adlaw nga testing, hangtod sa bug fixes, tanan nga trabaho gikan sa libre nga oras. Ang open source nga dalan lisud mag-inusara, ang imong pag-ila ug pagsuporta mao ang pwersa aron ang proyekto magpadayon.
- Kung gibati nimo nga kini nga himan nakatabang kanimo, ayaw pagpanuko sa pagtabang. Ang tanan nga donasyon gamiton alang sa server costs, version iteration, ug feature optimization.
- Gamay nga kaayo, mga bitoon. Salamat pag-usab sa imong pagsabot ug pagkamapinanggaron! [Bisitaha ang among donasyon nga pahina](https://www.wslui.com/donate/).


## ⭐️ Alang sa Gugma

Kung gibati nimo nga kini nga proyekto nakatabang kanimo, mapasalamaton ko kung mahimo nimong hatagan ug star sa GitHub. Ang imong pag-ila makatabang sa proyekto nga maabot ang mas daghang users. Kini nga pagdasig mao ang nagpadala kanako nga magpadayon.


## 📄 Lisensya sa Open Source

Kini nga proyekto lisensyado sa ilalim sa GPL-3.0 – tan-awa ang file sa [LICENSE](../LICENSE) alang sa mga detalye.
