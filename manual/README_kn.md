# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

ಆಧುನಿಕ, ಹೆಚ್ಚಿನ-ಕಾರ್ಯಕ್ಷಮತೆ, ಹಗುರ ಮತ್ತು ಕಡಿಮೆ ಮೆಮೊರಿ ಬಳಕೆಯ WSL (Windows Subsystem for Linux) ಇನ್‌ಸ್ಟಾನ್ಸ್ ನಿರ್ವಹಣಾ ಡ್ಯಾಶ್‌ಬೋರ್ಡ್. Rust ಮತ್ತು Slint ಮೇಲೆ ನಿರ್ಮಿತ, ಅತ್ಯುತ್ತಮ ನೇಟಿವ್ ಅನುಭವವನ್ನು ಒದಗಿಸುತ್ತದೆ.

---

```diff
ಸೂಚನೆ:

- WSL Dashboard Microsoft Store ಮೂಲಕ ವಿತರಿಸಲಾಗುವುದಿಲ್ಲ.
- "WSL Dashboard" ಹೆಸರಿನಲ್ಲಿ ಅಲ್ಲಿ ಪಟ್ಟಿ ಮಾಡಲಾದ ಯಾವುದೇ ಅಪ್ಲಿಕೇಶನ್ ಅನಧಿಕೃತವಾಗಿದೆ ಮತ್ತು ನಕಲಿಯಾಗಿರಬಹುದು.
- ಸಂಭಾವ್ಯ ವಂಚನೆಗಳನ್ನು ತಪ್ಪಿಸಲು ದಯವಿಟ್ಟು ಅದನ್ನು ಡೌನ್‌ಲೋಡ್ ಮಾಡಬೇಡಿ.
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

I18N : [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | ಕನ್ನಡ | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 ವಿಷಯ ಸೂಚಿ
- [🌍 ಭಾಷಾ ಬೆಂಬಲ](#-ಭಾಷಾ-ಬೆಂಬಲ)
- [🚀 ಮುಖ್ಯ ವೈಶಿಷ್ಟ್ಯಗಳು & ಬಳಕೆ](#-ಮುಖ್ಯ-ವೈಶಿಷ್ಟ್ಯಗಳು--ಬಳಕೆ)
- [⚙️ ಸಂರಚನೆ & ಲಾಗ್‌ಗಳು](#️-ಸಂರಚನೆ--ಲಾಗ್ಗಳು)
- [🖼️ ಸಾಫ್ಟ್‌ವೇರ್ ಸ್ಕ್ರೀನ್‌ಶಾಟ್‌ಗಳು](#️-ಸಾಫ್ಟ್ವೇರ್-ಸ್ಕ್ರೀನ್ಶಾಟ್ಗಳು)
- [🎬 ಕಾರ್ಯಾಚರಣೆ ಡೆಮೋ](#-ಕಾರ್ಯಾಚರಣೆ-ಡೆಮೋ)
- [💻 ಸಿಸ್ಟಮ್ ಅಗತ್ಯತೆಗಳು](#-ಸಿಸ್ಟಮ್-ಅಗತ್ಯತೆಗಳು)
- [📦 ಸ್ಥಾಪನಾ ಮಾರ್ಗದರ್ಶಿ](#-ಸ್ಥಾಪನಾ-ಮಾರ್ಗದರ್ಶಿ)
- [🛠️ ತಂತ್ರಜ್ಞಾನ ಸ್ಟ್ಯಾಕ್ & ಕಾರ್ಯಕ್ಷಮತೆ](#️-ತಂತ್ರಜ್ಞಾನ-ಸ್ಟ್ಯಾಕ್--ಕಾರ್ಯಕ್ಷಮತೆ)
- [🤝 ಸಮುದಾಯ ಬೆಂಬಲ](#-ಸಮುದಾಯ-ಬೆಂಬಲ)
- [❤️ ಈ ಯೋಜನೆಗೆ ಬೆಂಬಲ ನೀಡಿ](#️-ಈ-ಯೋಜನೆಗೆ-ಬೆಂಬಲ-ನೀಡಿ)
- [⭐️ ಪ್ರೀತಿಯಿಂದ ನಡೆಯುವ ಯೋಜನೆ](#️-ಪ್ರೀತಿಯಿಂದ-ನಡೆಯುವ-ಯೋಜನೆ)
- [📄 ಮುಕ್ತ ಮೂಲ ಪರವಾನಗಿ](#-ಮುಕ್ತ-ಮೂಲ-ಪರವಾನಗಿ)

---

## 🌍 ಭಾಷಾ ಬೆಂಬಲ

ಇಂಗ್ಲಿಷ್, ಸರಳೀಕೃತ ಚೈನೀಸ್, ಸಾಂಪ್ರದಾಯಿಕ ಚೈನೀಸ್, ಹಿಂದಿ, ಸ್ಪ್ಯಾನಿಷ್, ಫ್ರೆಂಚ್, ಅರಬಿಕ್, ಬೆಂಗಾಲಿ, ಪೋರ್ಚುಗೀಸ್, ರಷ್ಯನ್, ಉರ್ದು, ಇಂಡೋನೇಷಿಯನ್, ಜರ್ಮನ್, ಜಪಾನೀಸ್, ಟರ್ಕಿಷ್, ಕೊರಿಯನ್, ಇಟಾಲಿಯನ್, ಡಚ್, ಸ್ವೀಡಿಷ್, ಚೆಕ್, ಗ್ರೀಕ್, ಹಂಗೇರಿಯನ್, ಹೀಬ್ರೂ, ನಾರ್ವೇಜಿಯನ್, ಡ್ಯಾನಿಷ್, ಫಿನ್ನಿಷ್, ಸ್ಲೊವಾಕ್, ಸ್ಲೊವೇನಿಯನ್, ಐಸ್ಲ್ಯಾಂಡಿಕ್, ವಿಯೆಟ್ನಾಮೀಸ್, ತೆಲುಗು, ಜಾವನೀಸ್, ಥೈ, ತಮಿಳು, ಫಿಲಿಪಿನೋ, ಪಂಜಾಬಿ, ಮಲಯ್, ಪೋಲಿಷ್, ಉಕ್ರೇನಿಯನ್, ಪರ್ಷಿಯನ್, ಕನ್ನಡ, ಮರಾಠಿ, ಹೌಸಾ, ಬರ್ಮೀಸ್, ಉಜ್ಬೇಕ್, ಅಜರ್‌ಬೈಜಾನಿ, ಸೆಬುವಾನೊ, ಮಲಯಾಳಂ, ಸಿಂಧಿ, ಅಂಹಾರಿಕ್

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="ಇಂಗ್ಲಿಷ್" alt="ಇಂಗ್ಲಿಷ್" />
  <img src="../assets/flags/cn.svg" width="32" title="ಸರಳೀಕೃತ ಚೈನೀಸ್" alt="ಸರಳೀಕೃತ ಚೈನೀಸ್" />
  <img src="../assets/flags/tw.svg" width="32" title="ಸಾಂಪ್ರದಾಯಿಕ ಚೈನೀಸ್" alt="ಸಾಂಪ್ರದಾಯಿಕ ಚೈನೀಸ್" />
  <img src="../assets/flags/in.svg" width="32" title="ಹಿಂದಿ" alt="ಹಿಂದಿ" />
  <img src="../assets/flags/es.svg" width="32" title="ಸ್ಪ್ಯಾನಿಷ್" alt="ಸ್ಪ್ಯಾನಿಷ್" />
  <img src="../assets/flags/fr.svg" width="32" title="ಫ್ರೆಂಚ್" alt="ಫ್ರೆಂಚ್" />
  <img src="../assets/flags/sa.svg" width="32" title="ಅರಬಿಕ್" alt="ಅರಬಿಕ್" />
  <img src="../assets/flags/bd.svg" width="32" title="ಬೆಂಗಾಲಿ" alt="ಬೆಂಗಾಲಿ" />
  <img src="../assets/flags/pt.svg" width="32" title="ಪೋರ್ಚುಗೀಸ್" alt="ಪೋರ್ಚುಗೀಸ್" />
  <img src="../assets/flags/ru.svg" width="32" title="ರಷ್ಯನ್" alt="ರಷ್ಯನ್" />
  <img src="../assets/flags/pk.svg" width="32" title="ಉರ್ದು" alt="ಉರ್ದು" />
  <img src="../assets/flags/id.svg" width="32" title="ಇಂಡೋನೇಷಿಯನ್" alt="ಇಂಡೋನೇಷಿಯನ್" />
  <img src="../assets/flags/de.svg" width="32" title="ಜರ್ಮನ್" alt="ಜರ್ಮನ್" />
  <img src="../assets/flags/jp.svg" width="32" title="ಜಪಾನೀಸ್" alt="ಜಪಾನೀಸ್" />
  <img src="../assets/flags/tr.svg" width="32" title="ಟರ್ಕಿಷ್" alt="ಟರ್ಕಿಷ್" />
  <img src="../assets/flags/kr.svg" width="32" title="ಕೊರಿಯನ್" alt="ಕೊರಿಯನ್" />
  <img src="../assets/flags/it.svg" width="32" title="ಇಟಾಲಿಯನ್" alt="ಇಟಾಲಿಯನ್" />
  <img src="../assets/flags/nl.svg" width="32" title="ಡಚ್" alt="ಡಚ್" />
  <img src="../assets/flags/se.svg" width="32" title="ಸ್ವೀಡಿಷ್" alt="ಸ್ವೀಡಿಷ್" />
  <img src="../assets/flags/cz.svg" width="32" title="ಚೆಕ್" alt="ಚೆಕ್" />
  <img src="../assets/flags/gr.svg" width="32" title="ಗ್ರೀಕ್" alt="ಗ್ರೀಕ್" />
  <img src="../assets/flags/hu.svg" width="32" title="ಹಂಗೇರಿಯನ್" alt="ಹಂಗೇರಿಯನ್" />
  <img src="../assets/flags/il.svg" width="32" title="ಹೀಬ್ರೂ" alt="ಹೀಬ್ರೂ" />
  <img src="../assets/flags/no.svg" width="32" title="ನಾರ್ವೇಜಿಯನ್" alt="ನಾರ್ವೇಜಿಯನ್" />
  <img src="../assets/flags/dk.svg" width="32" title="ಡ್ಯಾನಿಷ್" alt="ಡ್ಯಾನಿಷ್" />
  <img src="../assets/flags/fi.svg" width="32" title="ಫಿನ್ನಿಷ್" alt="ಫಿನ್ನಿಷ್" />
  <img src="../assets/flags/sk.svg" width="32" title="ಸ್ಲೊವಾಕ್" alt="ಸ್ಲೊವಾಕ್" />
  <img src="../assets/flags/si.svg" width="32" title="ಸ್ಲೊವೇನಿಯನ್" alt="ಸ್ಲೊವೇನಿಯನ್" />
  <img src="../assets/flags/is.svg" width="32" title="ಐಸ್ಲ್ಯಾಂಡಿಕ್" alt="ಐಸ್ಲ್ಯಾಂಡಿಕ್" />
  <img src="../assets/flags/vn.svg" width="32" title="Vietnamese" alt="Vietnamese" />
  <img src="../assets/flags/in.svg" width="32" title="ತೆಲುಗು" alt="ತೆಲುಗು" />
  <img src="../assets/flags/id.svg" width="32" title="Javanese" alt="Javanese" />
  <img src="../assets/flags/th.svg" width="32" title="Thai" alt="Thai" />
  <img src="../assets/flags/in.svg" width="32" title="ತಮಿಳು" alt="ತಮಿಳು" />
  <img src="../assets/flags/ph.svg" width="32" title="Filipino" alt="Filipino" />
  <img src="../assets/flags/pk.svg" width="32" title="Punjabi" alt="Punjabi" />
  <img src="../assets/flags/my.svg" width="32" title="Malay" alt="Malay" />
  <img src="../assets/flags/pl.svg" width="32" title="Polish" alt="Polish" />
  <img src="../assets/flags/ua.svg" width="32" title="Ukrainian" alt="Ukrainian" />
  <img src="../assets/flags/ir.svg" width="32" title="Persian" alt="Persian" />
  <img src="../assets/flags/in.svg" width="32" title="ಕನ್ನಡ" alt="ಕನ್ನಡ" />
  <img src="../assets/flags/in.svg" width="32" title="ಮರಾಠಿ" alt="ಮರಾಠಿ" />
  <img src="../assets/flags/ng.svg" width="32" title="Hausa" alt="Hausa" />
  <img src="../assets/flags/mm.svg" width="32" title="Burmese" alt="Burmese" />
  <img src="../assets/flags/uz.svg" width="32" title="Uzbek" alt="Uzbek" />
  <img src="../assets/flags/az.svg" width="32" title="ಅಜರ್‌ಬೈಜಾನಿ" alt="ಅಜರ್‌ಬೈಜಾನಿ" />
  <img src="../assets/flags/ph.svg" width="32" title="Cebuano" alt="Cebuano" />
  <img src="../assets/flags/in.svg" width="32" title="ಮಲಯಾಳಂ" alt="ಮಲಯಾಳಂ" />
  <img src="../assets/flags/pk.svg" width="32" title="Sindhi" alt="Sindhi" />
  <img src="../assets/flags/et.svg" width="32" title="Amharic" alt="Amharic" />
</p>


## 🚀 ಮುಖ್ಯ ವೈಶಿಷ್ಟ್ಯಗಳು & ಬಳಕೆ

- **ಆಧುನಿಕ ನೇಟಿವ್ UI**: **Skia** ನಿಂದ ಚಾಲಿತ ಹೆಚ್ಚಿನ-ಕಾರ್ಯಕ್ಷಮತೆ ರೆಂಡರಿಂಗ್, ಡಾರ್ಕ್/ಲೈಟ್ ಮೋಡ್, ನಯವಾದ ಅನಿಮೇಶನ್‌ಗಳೊಂದಿಗೆ ಸಹಜ GUI.
- **ಸಿಸ್ಟಮ್ ಟ್ರೇ ಸಂಯೋಜನೆ**: ಸಮಗ್ರ ಟ್ರೇ ಬೆಂಬಲ (ಸುಮಾರು 10MB ಮೆಮೊರಿ ಬಳಕೆ), ಡಬಲ್-ಕ್ಲಿಕ್ ಮೂಲಕ ತೋರಿಸು/ಮರೆಮಾಡು ಟಾಗಲ್ ಮತ್ತು ಪೂರ್ಣ-ವೈಶಿಷ್ಟ್ಯದ ರೈಟ್-ಕ್ಲಿಕ್ ಮೆನು.
- **ಸ್ಮಾರ್ಟ್ ಸ್ಟಾರ್ಟಪ್**: ಬೂಟ್‌ನಲ್ಲಿ ಸ್ವಯಂ-ಪ್ರಾರಂಭ, ಟ್ರೇಗೆ ಮಿನಿಮೈಜ್ (`/silent` ಆರ್ಗ್ಯುಮೆಂಟ್‌ನೊಂದಿಗೆ ಸೈಲೆಂಟ್ ಲಾಂಚ್), ಮತ್ತು ನಿರ್ಗಮನದಲ್ಲಿ ಡಿಸ್ಟ್ರೋ ಸ್ವಯಂ-ಮುಚ್ಚುವಿಕೆ.
- **ಸಮಗ್ರ ಇನ್‌ಸ್ಟಾನ್ಸ್ ನಿಯಂತ್ರಣ**: ಒಂದೇ ಕ್ಲಿಕ್‌ನಲ್ಲಿ ಪ್ರಾರಂಭಿಸಿ, ನಿಲ್ಲಿಸಿ, ಕೊನೆಗೊಳಿಸಿ ಮತ್ತು ನೋಂದಣಿ ತೆಗೆದುಹಾಕಿ. ನೈಜ-ಸಮಯ ಸ್ಥಿತಿ ಮೇಲ್ವಿಚಾರಣೆ, ಡಿಸ್ಕ್ ಬಳಕೆ ಮತ್ತು ಫೈಲ್ ಸ್ಥಳಗಳ ಬಗ್ಗೆ ಆಳವಾದ ಒಳನೋಟ.
- **ಡಿಸ್ಟ್ರೋ ನಿರ್ವಹಣೆ**: ಡೀಫಾಲ್ಟ್ ಆಗಿ ಹೊಂದಿಸುವಿಕೆ, ಭೌತಿಕ ವಲಸೆ (VHDX ಅನ್ನು ಇತರ ಡಿಸ್ಕ್‌ಗೆ ಸರಿಸುವಿಕೆ), ಮತ್ತು `.tar` ಅಥವಾ `.tar.gz` ಆರ್ಕೈವ್ ಆಗಿ ರಫ್ತು/ಕ್ಲೋನ್.
- **ತ್ವರಿತ ಸಂಯೋಜನೆ**: ಟರ್ಮಿನಲ್, VS Code ಅಥವಾ ಫೈಲ್ ಎಕ್ಸ್‌ಪ್ಲೋರರ್‌ಗೆ ಒಂದೇ ಕ್ಲಿಕ್‌ನಲ್ಲಿ ಪ್ರವೇಶ, ಕಸ್ಟಮ್ ವರ್ಕಿಂಗ್ ಡೈರೆಕ್ಟರಿ ಮತ್ತು ಸ್ಟಾರ್ಟಪ್ ಸ್ಕ್ರಿಪ್ಟ್ ಹುಕ್ಸ್ ಬೆಂಬಲ.
- **ಡಿಸ್ಟ್ರೋ ಸ್ಥಾಪನೆ**: Microsoft Store, GitHub, ಸ್ಥಳೀಯ ಫೈಲ್‌ಗಳು (RootFS/VHDX), ಅಥವಾ ಆನ್‌ಲೈನ್ ಮಿರರ್‌ಗಳ ಮೂಲಕ Linux ವಿತರಣೆಗಳನ್ನು ಸ್ಥಾಪಿಸಿ (ವೇಗದ ಮಿರರ್ ಆಯ್ಕೆಮಾಡಲು ಆಟೋ ಸ್ಪೀಡ್ ಟೆಸ್ಟ್ ಮತ್ತು ಅಂತರ್ನಿರ್ಮಿತ RootFS ಡೌನ್‌ಲೋಡ್ ಸಹಾಯಕ).
- **ಜಾಗತಿಕ ಸುರಕ್ಷತೆ**: ಏಕಕಾಲದ ವಲಸೆ/ಬ್ಯಾಕಪ್ ಕಾರ್ಯಾಚರಣೆಗಳ ಸುರಕ್ಷತೆಗಾಗಿ ಮ್ಯೂಟೆಕ್ಸ್ ಲಾಕ್‌ಗಳ ಬಳಕೆ, ಮತ್ತು ತೆಗೆದುಹಾಕುವಾಗ Appx ಪ್ಯಾಕೇಜ್‌ಗಳ ಸ್ವಯಂ-ಸ್ವಚ್ಛಗೊಳಿಸುವಿಕೆ.
- **ಅತ್ಯಂತ ಕಡಿಮೆ ಮೆಮೊರಿ ಬಳಕೆ**: ಅತ್ಯಧಿಕ ಆಪ್ಟಿಮೈಜ್ ಮಾಡಿದ ಸಂಪನ್ಮೂಲ ದಕ್ಷತೆ. ಸೈಲೆಂಟ್ ಸ್ಟಾರ್ಟಪ್ (ಸಿಸ್ಟಮ್ ಟ್ರೇ) ಸುಮಾರು **10MB** ಮೆಮೊರಿ. ವಿಂಡೋ ಮೋಡ್‌ನಲ್ಲಿ ಫಾಂಟ್ ಸಂಕೀರ್ಣತೆಯ ಆಧಾರದ ಮೇಲೆ ಸುಮಾರು **18MB** (ಇಂಗ್ಲಿಷ್, ಜರ್ಮನ್ ನಂತಹ ಪ್ರಮಾಣಿತ ಭಾಷೆಗಳು) ನಿಂದ **38MB** (ಚೈನೀಸ್-ಜಪಾನೀಸ್-ಕೊರಿಯನ್ ನಂತಹ ದೊಡ್ಡ ಅಕ್ಷರ ಸೆಟ್‌ಗಳು).
- **ಸುಧಾರಿತ ನೆಟ್‌ವರ್ಕ್ ನಿರ್ವಹಣೆ**: ಪೋರ್ಟ್ ಫಾರ್ವರ್ಡಿಂಗ್ (ಸ್ವಯಂ-ಕ್ರಿಯೇಟ್ ಫೈರ್‌ವಾಲ್ ನಿಯಮಗಳು) ಮತ್ತು ಜಾಗತಿಕ HTTP ಪ್ರಾಕ್ಸಿ ಸಂರಚನೆಯನ್ನು ನಯವಾಗಿ ನಿರ್ವಹಿಸುವಿಕೆ, ಏಕೀಕೃತ ಸಂಪರ್ಕ ಅನುಭವವನ್ನು ಸಾಧಿಸುವಿಕೆ.
- **USB ಸಾಧನ ನಿರ್ವಹಣೆ**: `usbipd-win` ಜೊತೆ ಆಳವಾದ ಸಂಯೋಜನೆ, ಡ್ಯಾಶ್‌ಬೋರ್ಡ್ UI ನಲ್ಲಿ ನೇರವಾಗಿ ಎಲ್ಲಾ WSL ಇನ್‌ಸ್ಟಾನ್ಸ್‌ಗಳಲ್ಲಿ ಸ್ಥಳೀಯ USB ಸಾಧನಗಳನ್ನು ಸುಲಭವಾಗಿ ಬೈಂಡ್, ಅಟ್ಯಾಚ್ ಮತ್ತು ನಿರ್ವಹಿಸುವಿಕೆ.


## ⚙️ ಸಂರಚನೆ & ಲಾಗ್‌ಗಳು

ಎಲ್ಲಾ ಸಂರಚನೆಗಳನ್ನು "ಸೆಟ್ಟಿಂಗ್‌ಗಳು" ವೀಕ್ಷಣೆಯಿಂದ ನಿರ್ವಹಿಸಲಾಗುತ್ತದೆ:

- ಹೊಸ WSL ಇನ್‌ಸ್ಟಾನ್ಸ್‌ಗಳಿಗೆ ಡೀಫಾಲ್ಟ್ ಸ್ಥಾಪನಾ ಡೈರೆಕ್ಟರಿ ಆಯ್ಕೆಮಾಡಿ.
- ಲಾಗ್ ಡೈರೆಕ್ಟರಿ ಮತ್ತು ಲಾಗ್ ಮಟ್ಟ ಸಂರಚಿಸಿ (Error / Warn / Info / Debug / Trace).
- UI ಭಾಷೆ ಆಯ್ಕೆಮಾಡಿ ಅಥವಾ ಸಿಸ್ಟಮ್ ಭಾಷೆಯನ್ನು ಅನುಸರಿಸಿ.
- ಡಾರ್ಕ್ ಮೋಡ್ ಟಾಗಲ್ ಮಾಡಿ, ಮತ್ತು ಕಾರ್ಯಾಚರಣೆಯ ನಂತರ ॲಪ್ WSL ಸ್ವಯಂ-ಮುಚ್ಚುವಿಕೆ.
- ನವೀಕರಣ ಪರಿಶೀಲನೆ ಆವರ್ತನೆ ಸಂರಚಿಸಿ (ದೈನಂದಿನ, ಸಾಪ್ತಾಹಿಕ, ಪಾಕ್ಷಿಕ, ಮಾಸಿಕ).
- ಬೂಟ್‌ನಲ್ಲಿ ಸ್ವಯಂ-ಪ್ರಾರಂಭ ಸಕ್ರಿಯಗೊಳಿಸಿ (ಮಾರ್ಗ ಸ್ವಯಂ-ಫಿಕ್ಸ್ ವೈಶಿಷ್ಟ್ಯದೊಂದಿಗೆ).
- ಸ್ಟಾರ್ಟಪ್‌ನಲ್ಲಿ ಟ್ರೇಗೆ ಮಿನಿಮೈಜ್ ಸೆಟ್ ಮಾಡಿ.
- ಮುಚ್ಚು ಬಟನ್ ವರ್ತನೆ ಸಂರಚಿಸಿ (ಪ್ರೋಗ್ರಾಂನಿಂದ ನಿರ್ಗಮಿಸುವ ಬದಲು ಟ್ರೇಗೆ ಮಿನಿಮೈಜ್).
- ನಿರ್ದಿಷ್ಟ ವೈಶಿಷ್ಟ್ಯ ಟ್ಯಾಬ್‌ಗಳ ದೃಶ್ಯತೆಯನ್ನು ಟಾಗಲ್ ಮಾಡುವ ಮೂಲಕ ಸೈಡ್‌ಬಾರ್ ಅನ್ನು ಕಸ್ಟಮೈಸ್ ಮಾಡಿ.

ಲಾಗ್ ಫೈಲ್‌ಗಳನ್ನು ಸಂರಚಿತ ಲಾಗ್ ಡೈರೆಕ್ಟರಿಯಲ್ಲಿ ಬರೆಯಲಾಗುತ್ತದೆ ಮತ್ತು ಸಮಸ್ಯೆಗಳನ್ನು ವರದಿ ಮಾಡುವಾಗ ಲಗತ್ತಿಸಬಹುದು.


## 🖼️ ಸಾಫ್ಟ್‌ವೇರ್ ಸ್ಕ್ರೀನ್‌ಶಾಟ್‌ಗಳು

### ಮುಖ್ಯ ಇಂಟರ್ಫೇಸ್ (ಡಾರ್ಕ್ & ಲೈಟ್ ಮೋಡ್)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB & ಮೆನು ಕೊಲಾಪ್ಸ್
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### ನೆಟ್‌ವರ್ಕ್ ನಿರ್ವಹಣೆ
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### ಇನ್‌ಸ್ಟಾನ್ಸ್ ಸೇರಿಸಿ & ಸೆಟ್ಟಿಂಗ್‌ಗಳು
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>
<p align="center">
  <img src="../assets/screenshot/settings-advanced.png" width="48%" />
  <img src="../assets/screenshot/settings-interface.png" width="48%" />
</p>

### ಬಗ್ಗೆ & ದಾನ
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/donate.png" width="48%" />
</p>

## 🎬 ಕಾರ್ಯಾಚರಣೆ ಡೆಮೋ

[ನಮ್ಮನ್ನು ಸುಧಾರಿಸಲು ಸಹಾಯ ಮಾಡಿ! ನಮ್ಮ ಪರಿಚಯ ವೀಡಿಯೊ ವೀಕ್ಷಿಸಿ ಮತ್ತು ನಿಮ್ಮ ಅಭಿಪ್ರಾಯಗಳನ್ನು ಹಂಚಿಕೊಳ್ಳಿ.](https://github.com/owu/wsl-dashboard/discussions/9)



## 💻 ಸಿಸ್ಟಮ್ ಅಗತ್ಯತೆಗಳು

- WSL ಸಕ್ರಿಯಗೊಳಿಸಿದ Windows 10 ಅಥವಾ Windows 11 (WSL 2 ಶಿಫಾರಸು).
- ಕನಿಷ್ಠ ಒಂದು WSL ವಿತರಣೆ ಸ್ಥಾಪಿಸಲಾಗಿದೆ, ಅಥವಾ ಹೊಸ ವಿತರಣೆ ಸ್ಥಾಪಿಸಲು ಅನುಮತಿ.
- 64-ಬಿಟ್ CPU; ಬಹು ಡಿಸ್ಟ್ರೋ ಬಳಕೆಗೆ 4 GB ಅಥವಾ ಹೆಚ್ಚಿನ RAM ಶಿಫಾರಸು.

## 📦 ಸ್ಥಾಪನಾ ಮಾರ್ಗದರ್ಶಿ

### ವಿಧಾನ 1: ಪೂರ್ವ-ನಿರ್ಮಿತ ಬೈನರಿ ಡೌನ್‌ಲೋಡ್ ಮಾಡಿ

ಆರಂಭಿಸಲು ಸುಲಭ ಮಾರ್ಗವೆಂದರೆ ನಿರ್ಮಿತ ಆವೃತ್ತಿಯನ್ನು ಬಳಸುವುದು:

1. [GitHub Releases](https://github.com/owu/wsl-dashboard/releases) ಪುಟಕ್ಕೆ ಹೋಗಿ.
2. Windows ಗಾಗಿ ಇತ್ತೀಚಿನ `wsldashboard` ಎಕ್ಸಿಕ್ಯೂಟೆಬಲ್ ಡೌನ್‌ಲೋಡ್ ಮಾಡಿ.
3. ಡಿಕಂಪ್ರೆಸ್ ಮಾಡಿ (ಕಂಪ್ರೆಸ್ಡ್ ಫೈಲ್ ಆಗಿದ್ದರೆ) ಮತ್ತು `wsldashboard.exe` ಚಲಾಯಿಸಿ.

ಸ್ಥಾಪನೆ ಅಗತ್ಯವಿಲ್ಲ, ಈ ॲಪ್ ಸಿಂಗಲ್-ಫೈಲ್ ಪೋರ್ಟಬಲ್ ಪ್ರೋಗ್ರಾಂ ಆಗಿದೆ.

### ವಿಧಾನ 2: ಮೂಲದಿಂದ ನಿರ್ಮಿಸಿ

Rust ಟೂಲ್‌ಚೈನ್ (Rust 1.92+ ಅಥವಾ ಹೊಸದು) ಸ್ಥಾಪಿಸಲಾಗಿದೆ ಎಂದು ಖಚಿತಪಡಿಸಿಕೊಳ್ಳಿ.

1. ರೆಪೊಸಿಟರಿ ಕ್ಲೋನ್ ಮಾಡಿ:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. ನಿರ್ಮಿಸಿ ಮತ್ತು ಚಲಾಯಿಸಿ:

   - ಡೆವಲಪ್‌ಮೆಂಟ್ ಡೀಬಗ್:

     ```powershell
     cargo run
     ```
   - ಆಪ್ಟಿಮೈಜ್ಡ್ ರಿಲೀಸ್ ಬಿಲ್ಡ್‌ಗಾಗಿ ಬಿಲ್ಡ್ ಸ್ಕ್ರಿಪ್ಟ್ ಬಳಸಿ:

     > ಬಿಲ್ಡ್ ಸ್ಕ್ರಿಪ್ಟ್‌ಗೆ `x86_64-pc-windows-msvc` ಟೂಲ್‌ಚೈನ್ ಅಗತ್ಯ.

     ```powershell
     .\build\portable\build.ps1
     ```


## 🛠️ ತಂತ್ರಜ್ಞಾನ ಸ್ಟ್ಯಾಕ್ & ಕಾರ್ಯಕ್ಷಮತೆ

- **ಕರ್ನಲ್**: ಮೆಮೊರಿ ಸುರಕ್ಷತೆ ಮತ್ತು ಝೀರೋ-ಕಾಸ್ಟ ಅಬ್ಸ್ಟ್ರಾಕ್ಷನ್‌ಗಳನ್ನು ಖಚಿತಪಡಿಸಲು Rust ನಲ್ಲಿ ಅನುಷ್ಠಾನ.
- **UI ಫ್ರೇಮ್‌ವರ್ಕ್**: ಹೆಚ್ಚಿನ-ಕಾರ್ಯಕ್ಷಮತೆ **Skia** ರೆಂಡರಿಂಗ್ ಬ್ಯಾಕೆಂಡ್‌ನೊಂದಿಗೆ Slint.
- **ಅಸಿಂಕ್ರೋನಸ್ ರನ್‌ಟೈಮ್**: ನಾನ್-ಬ್ಲಾಕಿಂಗ್ ಸಿಸ್ಟಮ್ ಕಮಾಂಡ್‌ಗಳು ಮತ್ತು I/O ಗಾಗಿ Tokio.
- **ಕಾರ್ಯಕ್ಷಮತೆ ಮುಖ್ಯಾಂಶಗಳು**:
  - **ಪ್ರತಿಕ್ರಿಯಾ ವೇಗ**: ತಕ್ಷಣದ ಸ್ಟಾರ್ಟಪ್ ವೇಗ, WSL ಸ್ಥಿತಿ ರಿಯಲ್-ಟೈಮ್ ನಿಗಾ.
  - **ಸಂಪನ್ಮೂಲ ದಕ್ಷತೆ**: ಅತ್ಯಂತ ಕಡಿಮೆ ಸಂಪನ್ಮೂಲ ಬಳಕೆ (ವಿವರಗಳಿಗೆ [ಮುಖ್ಯ ವೈಶಿಷ್ಟ್ಯಗಳು](#-ಮುಖ್ಯ-ವೈಶಿಷ್ಟ್ಯಗಳು--ಬಳಕೆ) ನೋಡಿ).
  - **ಪೋರ್ಟಬಿಲಿಟಿ**: ಆಪ್ಟಿಮೈಜ್ಡ್ ರಿಲೀಸ್ ಬಿಲ್ಡ್ ಒಂದು ಕಂಪ್ರೆಸ್ಡ್ ಎಕ್ಸಿಕ್ಯೂಟೆಬಲ್ ಅನ್ನು ಉತ್ಪಾದಿಸುತ್ತದೆ.



## 🤝 ಸಮುದಾಯ ಬೆಂಬಲ

ಈ ಸಮುದಾಯಗಳ ಬೆಂಬಲಕ್ಕಾಗಿ ಹೃತ್ಪೂರ್ವಕ ಕೃತಜ್ಞತೆ:

- [Rust Programming Language](https://www.rust-lang.org) - ಶಕ್ತಿಶಾಲಿ ಮತ್ತು ಸುರಕ್ಷಿತ ಪ್ರೋಗ್ರಾಮಿಂಗ್ ಭಾಷೆ
- [Slint | Declarative GUI for Rust, C++, JavaScript & Python](https://slint.dev) - ಆಧುನಿಕ UI ಫ್ರೇಮ್‌ವರ್ಕ್
- [WSL: Windows Subsystem for Linux](https://github.com/microsoft/WSL) - ಅತ್ಯುತ್ತಮ Windows Subsystem for Linux
- [Tokio - An asynchronous Rust runtime](https://tokio.rs) - ದಕ್ಷ ಅಸಿಂಕ್ರೋನಸ್ ರನ್‌ಟೈಮ್
- [Windows Developer Community](https://developer.microsoft.com/en-us/windows/community) - ನಿರಂತರ ಪ್ಲಾಟ್‌ಫಾರ್ಮ್ ಸುಧಾರಣೆ
- [Reddit](https://www.reddit.com) - ಜಾಗತಿಕ ಸಮುದಾಯ ಚರ್ಚೆ ಮತ್ತು ಬೆಂಬಲ
- [Hacker News](https://news.ycombinator.com) - ಜಾಗತಿಕ ಸಮುದಾಯ ಚರ್ಚೆ ಮತ್ತು ಬೆಂಬಲ
- [Linux.do](https://linux.do) - IT ವೃತ್ತಿಪರರಿಗೆ ಜನಪ್ರಿಯ ಸಮುದಾಯ
- [V2EX](https://www.v2ex.com) - ಚೈನೀಸ್ ಟೆಕ್ ಸಮುದಾಯ ಚರ್ಚೆ

ನಿಮ್ಮ ಕೊಡುಗೆಗಳು ಮತ್ತು ಪ್ರತಿಕ್ರಿಯೆಗಳು ಈ ಯೋಜನೆಯನ್ನು ಸಾಧ್ಯವಾಗಿಸುತ್ತವೆ!


## ❤️ ಈ ಯೋಜನೆಗೆ ಬೆಂಬಲ ನೀಡಿ

- ಈ ಯೋಜನೆ GPL-3.0 ಮುಕ್ತ ಮೂಲ ಪರವಾನಗಿಯನ್ನು ಅನುಸರಿಸುತ್ತದೆ, ಎಲ್ಲಾ ಬಳಕೆದಾರರಿಗೆ ಉಚಿತ.
- ವೈಶಿಷ್ಟ್ಯ ಅಭಿವೃದ್ಧಿ, ದೈನಂದಿನ ಪರೀಕ್ಷೆಯಿಂದ ಬಗ್ ಪರಿಹಾರದವರೆಗೆ, ಎಲ್ಲಾ ಕೆಲಸವು ಉಚಿತ ಸಮಯದ ನಿರಂತರತೆಯಿಂದ ಬರುತ್ತದೆ. ಮುಕ್ತ ಮೂಲ ಮಾರ್ಗವು ಒಬ್ಬರಿಗೆ ಸುಲಭವಲ್ಲ, ನಿಮ್ಮ ಗುರುತಿಸುವಿಕೆ ಮತ್ತು ಬೆಂಬಲವು ಯೋಜನೆಯನ್ನು ದೀರ್ಘಕಾಲ ಮುಂದುವರಿಸಲು ಶಕ್ತಿಯಾಗಿದೆ.
- ಈ ಸಾಧನವು ನಿಜವಾಗಿಯೂ ನಿಮಗೆ ಸಹಾಯಕಾರಿ ಎಂದು ನಿಮಗೆ ಅನಿಸಿದರೆ, ದಯವಿಟ್ಟು ಸಹಾಯ ಮಾಡಿ. ಎಲ್ಲಾ ದಾನಗಳನ್ನು ಸರ್ವರ್ ವೆಚ್ಚ, ಆವೃತ್ತಿ ನವೀಕರಣ ಮತ್ತು ವೈಶಿಷ್ಟ್ಯ ಆಪ್ಟಿಮೈಜೇಶನ್‌ಗಾಗಿ ಬಳಸಲಾಗುತ್ತದೆ.
- ಸಣ್ಣ ಕರುಣೆಯೂ ತಾರೆಯಂತೆ ಹೊಳೆಯುತ್ತದೆ. ನಿಮ್ಮ ತಿಳುವಳಿಕೆ ಮತ್ತು ಉದಾರತೆಗೆ ಮತ್ತೆ ಧನ್ಯವಾದಗಳು!

ನಮ್ಮ ದಾನ ಪುಟಕ್ಕೆ ಭೇಟಿ ನೀಡಿ: [https://www.wslui.com/donate/](https://www.wslui.com/donate/)


## ⭐️ ಪ್ರೀತಿಯಿಂದ ನಡೆಯುವ ಯೋಜನೆ

ಈ ಯೋಜನೆಯು ನಿಮಗೆ ಉಪಯುಕ್ತವೆಂದು ನಿಮಗೆ ಅನಿಸಿದರೆ, GitHub ನಲ್ಲಿ ತಾರೆ ನೀಡುವ ಮೂಲಕ ನಿಮ್ಮ ಗುರುತಿಸುವಿಕೆಯನ್ನು ತೋರಿಸಲು ನಾನು ಸಂತೋಷಿಸುತ್ತೇನೆ. ನಿಮ್ಮ ಗುರುತಿಸುವಿಕೆಯು ಯೋಜನೆಯನ್ನು ಹೆಚ್ಚಿನ ಬಳಕೆದಾರರಿಗೆ ತಲುಪಿಸಲು ಸಹಾಯ ಮಾಡುತ್ತದೆ. ಈ ಪ್ರೋತ್ಸಾಹವು ನನ್ನನ್ನು ಮುಂದುವರಿಸುತ್ತದೆ.


## 📄 ಮುಕ್ತ ಮೂಲ ಪರವಾನಗಿ

ಈ ಯೋಜನೆ GPL-3.0 ಪರವಾನಗಿಯ ಅಡಿಯಲ್ಲಿ ಪರವಾನಗಿ ಪಡೆದಿದೆ – ವಿವರಗಳಿಗೆ [LICENSE](../LICENSE) ಫೈಲ್ ನೋಡಿ.


---

Built with ❤️ for the WSL Community.