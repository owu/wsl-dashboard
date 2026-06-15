# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

ਆਧੁਨਿਕ, ਉੱਚ-ਪ੍ਰਦਰਸ਼ਨ, ਹਲਕਾ ਅਤੇ ਘੱਟ ਮੈਮਰੀ ਵਰਤੋਂ ਵਾਲਾ WSL (Windows Subsystem for Linux) ਇੰਸਟੈਂਸ ਪ੍ਰਬੰਧਨ ਡੈਸ਼ਬੋਰਡ। Rust ਅਤੇ Slint ਉੱਤੇ ਬਣਿਆ, ਸ਼ਾਨਦਾਰ ਨੇਟਿਵ ਅਨੁਭਵ ਪ੍ਰਦਾਨ ਕਰਦਾ ਹੈ।

---

```diff
ਨੋਟ:

- ਇਹ ਸਾਫਟਵੇਅਰ Microsoft Store ਰਾਹੀਂ ਵੰਡਿਆ ਨਹੀਂ ਜਾਂਦਾ।
- ਇੱਥੇ ਇਸੇ ਨਾਂ ਦੀਆਂ ਐਪਾਂ ਅਣਅਧਿਕ੍ਰਿਤ ਪਾਇਰੇਟਡ ਸਾਫਟਵੇਅਰ ਹਨ।
- ਧੋਖਾ ਤੋਂ ਬਚਣ ਲਈ ਕਿਰਪਾ ਕਰਕੇ ਡਾਊਨਲੋਡ ਨਾ ਕਰੋ।
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

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [తెలుగు](./README_te.md) | [தமிழ்](./README_ta.md) | [मराठी](./README_mr.md) | ਪੰਜਾਬੀ | [ಕನ್ನಡ](./README_kn.md) | [മലയാളം](./README_ml.md) | [Azərbaycan](./README_az.md) | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 ਵਿਸ਼ਾ-ਸੂਚੀ
- [🌍 ਭਾਸ਼ਾ ਸਹਾਇਤਾ](#-ਭਾਸ਼ਾ-ਸਹਾਇਤਾ)
- [🚀 ਮੁੱਖ ਵਿਸ਼ੇਸ਼ਤਾਵਾਂ & ਵਰਤੋਂ](#-ਮੁੱਖ-ਵਿਸ਼ੇਸ਼ਤਾਵਾਂ--ਵਰਤੋਂ)
- [⚙️ ਕੌਨਫਿਗਰੇਸ਼ਨ & ਲਾਗ](#️-ਕੌਨਫਿਗਰੇਸ਼ਨ--ਲਾਗ)
- [🖼️ ਸਾਫਟਵੇਅਰ ਸਕ੍ਰੀਨਸ਼ਾਟ](#️-ਸਾਫਟਵੇਅਰ-ਸਕ੍ਰੀਨਸ਼ਾਟ)
- [🎬 ਕਾਰਵਾਈ ਡੈਮੋ](#-ਕਾਰਵਾਈ-ਡੈਮੋ)
- [💻 ਸਿਸਟਮ ਲੋੜਾਂ](#-ਸਿਸਟਮ-ਲੋੜਾਂ)
- [📦 ਇੰਸਟਾਲੇਸ਼ਨ ਗਾਈਡ](#-ਇੰਸਟਾਲੇਸ਼ਨ-ਗਾਈਡ)
- [🛠️ ਤਕਨਾਲੋਜੀ ਸਟੈਕ & ਪ੍ਰਦਰਸ਼ਨ](#️-ਤਕਨਾਲੋਜੀ-ਸਟੈਕ--ਪ੍ਰਦਰਸ਼ਨ)
- [🤝 ਕਮਿਊਨਿਟੀ ਸਹਾਇਤਾ](#-ਕਮਿਊਨਿਟੀ-ਸਹਾਇਤਾ)
- [❤️ ਇਸ ਪ੍ਰੋਜੈਕਟ ਦਾ ਸਮਰਥਨ ਕਰੋ](#️-ਇਸ-ਪ੍ਰੋਜੈਕਟ-ਦਾ-ਸਮਰਥਨ-ਕਰੋ)
- [⭐️ ਪਿਆਰ ਨਾਲ ਚੱਲਣ ਵਾਲਾ ਪ੍ਰੋਜੈਕਟ](#️-ਪਿਆਰ-ਨਾਲ-ਚੱਲਣ-ਵਾਲਾ-ਪ੍ਰੋਜੈਕਟ)
- [📄 ਓਪਨ ਸੋਰਸ ਲਾਇਸੈਂਸ](#-ਓਪਨ-ਸੋਰਸ-ਲਾਇਸੈਂਸ)

---

## 🌍 ਭਾਸ਼ਾ ਸਹਾਇਤਾ

ਅੰਗਰੇਜ਼ੀ, ਸਰਲੀਕ੍ਰਿਤ ਚੀਨੀ, ਪਰੰਪਰਾਗਤ ਚੀਨੀ, ਹਿੰਦੀ, ਸਪੇਨੀ, ਫਰੈਂਚ, ਅਰਬੀ, ਬੰਗਾਲੀ, ਪੁਰਤਗਾਲੀ, ਰੂਸੀ, ਉਰਦੂ, ਇੰਡੋਨੇਸ਼ੀਆਈ, ਜਰਮਨ, ਜਾਪਾਨੀ, ਤੁਰਕੀ, ਕੋਰੀਆਈ, ਇਤਾਲਵੀ, ਡੱਚ, ਸਵੀਡਿਸ਼, ਚੈੱਕ, ਯੂਨਾਨੀ, ਹੰਗਰੀਆਈ, ਹਿਬਰੂ, ਨਾਰਵੇਜੀਆਈ, ਡੈਨਿਸ਼, ਫਿਨਿਸ਼, ਸਲੋਵਾਕ, ਸਲੋਵੇਨੀਆਈ, ਆਈਸਲੈਂਡਿਕ, ਵੀਅਤਨਾਮੀ, ਤੇਲਗੂ, ਜਾਵਾਨੀਜ਼, ਥਾਈ, ਤਾਮਿਲ, ਫਿਲੀਪੀਨੋ, ਪੰਜਾਬੀ, ਮਲਾਏ, ਪੋਲਿਸ਼, ਯੂਕ੍ਰੇਨੀਅਨ, ਫ਼ਾਰਸੀ, ਕੰਨੜ, ਮਰਾਠੀ, ਹੌਸਾ, ਬਰਮੀ, ਉਜ਼ਬੇਕ, ਅਜ਼ਰਬਾਈਜਾਨੀ, ਸੇਬੂਆਨੋ, ਮਲਿਆਲਮ, ਸਿੰਧੀ, ਅਮਹਾਰਿਕ

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="ਅੰਗਰੇਜ਼ੀ" alt="ਅੰਗਰੇਜ਼ੀ" />
  <img src="../assets/flags/cn.svg" width="32" title="ਸਰਲੀਕ੍ਰਿਤ ਚੀਨੀ" alt="ਸਰਲੀਕ੍ਰਿਤ ਚੀਨੀ" />
  <img src="../assets/flags/tw.svg" width="32" title="ਪਰੰਪਰਾਗਤ ਚੀਨੀ" alt="ਪਰੰਪਰਾਗਤ ਚੀਨੀ" />
  <img src="../assets/flags/in.svg" width="32" title="ਹਿੰਦੀ" alt="ਹਿੰਦੀ" />
  <img src="../assets/flags/es.svg" width="32" title="ਸਪੇਨੀ" alt="ਸਪੇਨੀ" />
  <img src="../assets/flags/fr.svg" width="32" title="ਫਰੈਂਚ" alt="ਫਰੈਂਚ" />
  <img src="../assets/flags/sa.svg" width="32" title="ਅਰਬੀ" alt="ਅਰਬੀ" />
  <img src="../assets/flags/bd.svg" width="32" title="ਬੰਗਾਲੀ" alt="ਬੰਗਾਲੀ" />
  <img src="../assets/flags/pt.svg" width="32" title="ਪੁਰਤਗਾਲੀ" alt="ਪੁਰਤਗਾਲੀ" />
  <img src="../assets/flags/ru.svg" width="32" title="ਰੂਸੀ" alt="ਰੂਸੀ" />
  <img src="../assets/flags/pk.svg" width="32" title="ਉਰਦੂ" alt="ਉਰਦੂ" />
  <img src="../assets/flags/id.svg" width="32" title="ਇੰਡੋਨੇਸ਼ੀਆਈ" alt="ਇੰਡੋਨੇਸ਼ੀਆਈ" />
  <img src="../assets/flags/de.svg" width="32" title="ਜਰਮਨ" alt="ਜਰਮਨ" />
  <img src="../assets/flags/jp.svg" width="32" title="ਜਾਪਾਨੀ" alt="ਜਾਪਾਨੀ" />
  <img src="../assets/flags/tr.svg" width="32" title="ਤੁਰਕੀ" alt="ਤੁਰਕੀ" />
  <img src="../assets/flags/kr.svg" width="32" title="ਕੋਰੀਆਈ" alt="ਕੋਰੀਆਈ" />
  <img src="../assets/flags/it.svg" width="32" title="ਇਤਾਲਵੀ" alt="ਇਤਾਲਵੀ" />
  <img src="../assets/flags/nl.svg" width="32" title="ਡੱਚ" alt="ਡੱਚ" />
  <img src="../assets/flags/se.svg" width="32" title="ਸਵੀਡਿਸ਼" alt="ਸਵੀਡਿਸ਼" />
  <img src="../assets/flags/cz.svg" width="32" title="ਚੈੱਕ" alt="ਚੈੱਕ" />
  <img src="../assets/flags/gr.svg" width="32" title="ਯੂਨਾਨੀ" alt="ਯੂਨਾਨੀ" />
  <img src="../assets/flags/hu.svg" width="32" title="ਹੰਗਰੀਆਈ" alt="ਹੰਗਰੀਆਈ" />
  <img src="../assets/flags/il.svg" width="32" title="ਹਿਬਰੂ" alt="ਹਿਬਰੂ" />
  <img src="../assets/flags/no.svg" width="32" title="ਨਾਰਵੇਜੀਆਈ" alt="ਨਾਰਵੇਜੀਆਈ" />
  <img src="../assets/flags/dk.svg" width="32" title="ਡੈਨਿਸ਼" alt="ਡੈਨਿਸ਼" />
  <img src="../assets/flags/fi.svg" width="32" title="ਫਿਨਿਸ਼" alt="ਫਿਨਿਸ਼" />
  <img src="../assets/flags/sk.svg" width="32" title="ਸਲੋਵਾਕ" alt="ਸਲੋਵਾਕ" />
  <img src="../assets/flags/si.svg" width="32" title="ਸਲੋਵੇਨੀਆਈ" alt="ਸਲੋਵੇਨੀਆਈ" />
  <img src="../assets/flags/is.svg" width="32" title="ਆਈਸਲੈਂਡਿਕ" alt="ਆਈਸਲੈਂਡਿਕ" />
  <img src="../assets/flags/vn.svg" width="32" title="ਵੀਅਤਨਾਮੀ" alt="ਵੀਅਤਨਾਮੀ" />
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


## 🚀 ਮੁੱਖ ਵਿਸ਼ੇਸ਼ਤਾਵਾਂ & ਵਰਤੋਂ

- **ਆਧੁਨਿਕ ਨੇਟਿਵ UI**: **Skia** ਦੁਆਰਾ ਸੰਚਾਲਿਤ ਉੱਚ-ਪ੍ਰਦਰਸ਼ਨ ਰੈਂਡਰਿੰਗ, ਡਾਰਕ/ਲਾਈਟ ਮੋਡ, ਸਹਿਜ ਐਨੀਮੇਸ਼ਨ ਨਾਲ ਸਹਿਜ GUI.
- **ਸਿਸਟਮ ਟਰੇ ਇੰਟੀਗ੍ਰੇਸ਼ਨ**: ਵਿਆਪਕ ਟਰੇ ਸਹਾਇਤਾ (ਲਗਭਗ 10MB ਮੈਮਰੀ ਵਰਤੋਂ), ਡਬਲ-ਕਲਿਕ ਨਾਲ ਦਿਖਾਓ/ਲੁਕਾਓ ਟੌਗਲ ਅਤੇ ਪੂਰਨ-ਵਿਸ਼ੇਸ਼ਤਾ ਰਾਈਟ-ਕਲਿਕ ਮੇਨੂ.
- **ਸਮਾਰਟ ਸਟਾਰਟਅਪ**: ਬੂਟ ਤੇ ਆਟੋ-ਸਟਾਰਟ, ਟਰੇ ਤੇ ਮਿਨੀਮਾਈਜ਼ (`/silent` ਆਰਗੂਮੈਂਟ ਨਾਲ ਸਾਇਲੈਂਟ ਲਾਂਚ), ਅਤੇ ਬਾਹਰ ਜਾਣ ਤੇ ਡਿਸਟ੍ਰੋ ਆਟੋ-ਕਲੋਜ਼.
- **ਵਿਆਪਕ ਇੰਸਟੈਂਸ ਕੰਟਰੋਲ**: ਇੱਕ ਕਲਿਕ ਨਾਲ ਸ਼ੁਰੂ ਕਰੋ, ਰੋਕੋ, ਸਮਾਪਤ ਕਰੋ ਅਤੇ ਰਜਿਸਟਰ ਹਟਾਓ। ਰੀਅਲ-ਟਾਈਮ ਸਥਿਤੀ ਨਿਗਰਾਨੀ, ਡਿਸਕ ਵਰਤੋਂ ਅਤੇ ਫਾਈਲ ਸਥਾਨਾਂ ਬਾਰੇ ਡੂੰਘੀ ਜਾਣਕਾਰੀ.
- **ਡਿਸਟ੍ਰੋ ਪ੍ਰਬੰਧਨ**: ਮੂਲ ਵਜੋਂ ਸੈੱਟ ਕਰਨਾ, ਭੌਤਿਕ ਮਾਈਗ੍ਰੇਸ਼ਨ (VHDX ਨੂੰ ਹੋਰ ਡਿਸਕ ਤੇ ਲਿਜਾਣਾ), ਅਤੇ `.tar` ਜਾਂ `.tar.gz` ਆਰਕਾਈਵ ਵਜੋਂ ਨਿਰਯਾਤ/ਕਲੋਨ.
- **ਤੇਜ਼ ਇੰਟੀਗ੍ਰੇਸ਼ਨ**: ਟਰਮੀਨਲ, VS Code ਜਾਂ ਫਾਈਲ ਐਕਸਪਲੋਰਰ ਤੇ ਇੱਕ ਕਲਿਕ ਨਾਲ ਪਹੁੰਚ, ਕਸਟਮ ਵਰਕਿੰਗ ਡਾਇਰੈਕਟਰੀ ਅਤੇ ਸਟਾਰਟਅਪ ਸਕ੍ਰਿਪਟ ਹੁੱਕ ਸਹਾਇਤਾ.
- **ਡਿਸਟਰੋ ਇੰਸਟਾਲੇਸ਼ਨ**: Microsoft Store, GitHub, ਲੋਕਲ ਫਾਈਲਾਂ (RootFS/VHDX), ਜਾਂ ਔਨਲਾਈਨ ਮਿਰਰਾਂ ਰਾਹੀਂ Linux ਡਿਸਟ੍ਰੀਬਿਊਸ਼ਨਾਂ ਇੰਸਟਾਲ ਕਰੋ (ਸਭ ਤੋਂ ਤੇਜ਼ ਮਿਰਰ ਚੁਣਨ ਲਈ ਆਟੋ ਸਪੀਡ ਟੈਸਟ ਅਤੇ ਅੰਦਰੂਨੀ RootFS ਡਾਊਨਲੋਡ ਸਹਾਇਕ)।
- **ਗਲੋਬਲ ਸੁਰੱਖਿਆ**: ਇੱਕੋ ਸਮੇਂ ਮਾਈਗ੍ਰੇਸ਼ਨ/ਬੈਕਅੱਪ ਕਾਰਵਾਈਆਂ ਦੀ ਸੁਰੱਖਿਆ ਲਈ ਮਿਊਟੈਕਸ ਲਾਕ ਵਰਤਣਾ, ਅਤੇ ਹਟਾਉਣ ਤੇ Appx ਪੈਕੇਜ ਆਟੋ-ਕਲੀਨਅੱਪ.
- **ਬਹੁਤ ਘੱਟ ਮੈਮਰੀ ਵਰਤੋਂ**: ਬਹੁਤ ਜ਼ਿਆਦਾ ਆਪਟੀਮਾਈਜ਼ ਕੀਤੀ ਸਰੋਤ ਕਾਰਗੁਜ਼ਾਰੀ। ਸਾਇਲੈਂਟ ਸਟਾਰਟਅਪ (ਸਿਸਟਮ ਟਰੇ) ਲਗਭਗ **10MB** ਮੈਮਰੀ। ਵਿੰਡੋ ਮੋਡ ਵਿੱਚ ਫੌਂਟ ਗੁੰਝਲਤਾ ਅਨੁਸਾਰ ਲਗਭਗ **18MB** (ਅੰਗਰੇਜ਼ੀ, ਜਰਮਨ ਵਰਗੀਆਂ ਮਿਆਰੀ ਭਾਸ਼ਾਵਾਂ) ਤੋਂ **38MB** (ਚੀਨੀ-ਜਾਪਾਨੀ-ਕੋਰੀਆਈ ਵਰਗੇ ਵੱਡੇ ਅੱਖਰ ਸੈੱਟ).
- **ਐਡਵਾਂਸਡ ਨੈੱਟਵਰਕ ਪ੍ਰਬੰਧਨ**: ਪੋਰਟ ਫਾਰਵਰਡਿੰਗ (ਆਟੋ-ਕ੍ਰੀਏਟ ਫਾਇਰਵਾਲ ਨਿਯਮ) ਅਤੇ ਗਲੋਬਲ HTTP ਪ੍ਰਾਕਸੀ ਕੌਨਫਿਗਰੇਸ਼ਨ ਨੂੰ ਸਹਿਜਤਾ ਨਾਲ ਪ੍ਰਬੰਧਿਤ ਕਰਨਾ, ਇੱਕਜੁੱਟ ਕਨੈਕਟਿਵਿਟੀ ਅਨੁਭਵ ਪ੍ਰਾਪਤ ਕਰਨਾ.
- **USB ਡਿਵਾਈਸ ਪ੍ਰਬੰਧਨ**: `usbipd-win` ਨਾਲ ਡੂੰਘੀ ਇੰਟੀਗ੍ਰੇਸ਼ਨ, ਡੈਸ਼ਬੋਰਡ UI ਵਿੱਚ ਸਿੱਧੇ ਸਾਰੇ WSL ਇੰਸਟੈਂਸਾਂ ਵਿੱਚ ਲੋਕਲ USB ਡਿਵਾਈਸਾਂ ਨੂੰ ਆਸਾਨੀ ਨਾਲ ਬਾਈਂਡ, ਅਟੈਚ ਅਤੇ ਪ੍ਰਬੰਧਿਤ ਕਰਨਾ.


## ⚙️ ਕੌਨਫਿਗਰੇਸ਼ਨ & ਲਾਗ

ਸਾਰੀਆਂ ਕੌਨਫਿਗਰੇਸ਼ਨ "ਸੈਟਿੰਗਜ਼" ਵਿਊ ਰਾਹੀਂ ਪ੍ਰਬੰਧਿਤ ਕੀਤੀਆਂ ਜਾਂਦੀਆਂ ਹਨ:

- ਨਵੇਂ WSL ਇੰਸਟੈਂਸਾਂ ਦੀ ਮੂਲ ਇੰਸਟਾਲ ਡਾਇਰੈਕਟਰੀ ਚੁਣੋ.
- ਲਾਗ ਡਾਇਰੈਕਟਰੀ ਅਤੇ ਲਾਗ ਪੱਧਰ ਕੌਨਫਿਗਰ ਕਰੋ (Error / Warn / Info / Debug / Trace).
- UI ਭਾਸ਼ਾ ਚੁਣੋ ਜਾਂ ਸਿਸਟਮ ਭਾਸ਼ਾ ਦਾ ਪਾਲਣ ਕਰੋ.
- ਡਾਰਕ ਮੋਡ ਟੌਗਲ ਕਰੋ, ਅਤੇ ਕਾਰਵਾਈ ਤੋਂ ਬਾਅਦ ਐਪ WSL ਆਟੋ-ਕਲੋਜ਼ ਕਰਦਾ ਹੈ ਜਾਂ ਨਹੀਂ.
- ਅੱਪਡੇਟ ਜਾਂਚ ਆਵ੍ਰਿੱਤੀ ਕੌਨਫਿਗਰ ਕਰੋ (ਰੋਜ਼ਾਨਾ, ਹਫ਼ਤਾਵਾਰੀ, ਦੋ-ਹਫ਼ਤਾਵਾਰੀ, ਮਹੀਨਾਵਾਰੀ).
- ਬੂਟ ਤੇ ਆਟੋ-ਸਟਾਰਟ ਸਮਰੱਥ ਕਰੋ (ਮਾਰਗ ਆਟੋ-ਫਿਕਸ ਵਿਸ਼ੇਸ਼ਤਾ ਨਾਲ).
- ਸਟਾਰਟਅਪ ਤੇ ਟਰੇ ਤੇ ਮਿਨੀਮਾਈਜ਼ ਸੈੱਟ ਕਰੋ.
- ਬੰਦ ਬਟਨ ਵਿਵਹਾਰ ਕੌਨਫਿਗਰ ਕਰੋ (ਪ੍ਰੋਗਰਾਮ ਤੋਂ ਬਾਹਰ ਜਾਣ ਦੀ ਬਜਾਏ ਟਰੇ ਤੇ ਮਿਨੀਮਾਈਜ਼).
- ਖਾਸ ਵਿਸ਼ੇਸ਼ਤਾ ਟੈਬਾਂ ਦੀ ਦਿੱਖ ਟੌਗਲ ਕਰਕੇ ਸਾਈਡਬਾਰ ਕਸਟਮਾਈਜ਼ ਕਰੋ.

ਲਾਗ ਫਾਈਲਾਂ ਕੌਨਫਿਗਰ ਕੀਤੀ ਲਾਗ ਡਾਇਰੈਕਟਰੀ ਵਿੱਚ ਲਿਖੀਆਂ ਜਾਂਦੀਆਂ ਹਨ, ਸਮੱਸਿਆਵਾਂ ਦੀ ਰਿਪੋਰਟ ਕਰਦੇ ਸਮੇਂ ਲਾਗ ਜੋੜੇ ਜਾ ਸਕਦੇ ਹਨ.


## 🖼️ ਸਾਫਟਵੇਅਰ ਸਕ੍ਰੀਨਸ਼ਾਟ

### ਮੁੱਖ ਇੰਟਰਫੇਸ (ਡਾਰਕ & ਲਾਈਟ ਮੋਡ)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB & ਮੇਨੂ ਕੋਲੈਪਸ
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### ਨੈੱਟਵਰਕ ਪ੍ਰਬੰਧਨ
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### ਇੰਸਟੈਂਸ ਜੋੜੋ & ਸੈਟਿੰਗਜ਼
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>
<p align="center">
  <img src="../assets/screenshot/settings-advanced.png" width="48%" />
  <img src="../assets/screenshot/settings-interface.png" width="48%" />
</p>

### ਬਾਰੇ & ਦਾਨ
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/donate.png" width="48%" />
</p>

## 🎬 ਕਾਰਵਾਈ ਡੈਮੋ

[ਸਾਨੂੰ ਸੁਧਾਰਨ ਵਿੱਚ ਮਦਦ ਕਰੋ! ਸਾਡਾ ਜਾਣ-ਪਛਾਣ ਵੀਡੀਓ ਦੇਖੋ ਅਤੇ ਆਪਣੇ ਵਿਚਾਰ ਸਾਂਝੇ ਕਰੋ.](https://github.com/owu/wsl-dashboard/discussions/9)



## 💻 ਸਿਸਟਮ ਲੋੜਾਂ

- WSL ਸਮਰੱਥ Windows 10 ਜਾਂ Windows 11 (WSL 2 ਸਿਫ਼ਾਰਸ਼ੀ).
- ਘੱਟੋ-ਘੱਟ ਇੱਕ WSL ਡਿਸਟ੍ਰਿਬਿਊਸ਼ਨ ਇੰਸਟਾਲ ਹੋਇਆ ਹੋਵੇ, ਜਾਂ ਨਵੀਂ ਡਿਸਟ੍ਰਿਬਿਊਸ਼ਨ ਇੰਸਟਾਲ ਕਰਨ ਦੀ ਇਜਾਜ਼ਤ ਹੋਵੇ.
- 64-ਬਿਟ CPU; ਬਹੁ-ਡਿਸਟ੍ਰੋ ਵਰਤੋਂ ਲਈ 4 GB ਜਾਂ ਵੱਧ RAM ਸਿਫ਼ਾਰਸ਼ੀ.

## 📦 ਇੰਸਟਾਲੇਸ਼ਨ ਗਾਈਡ

### ਤਰੀਕਾ 1: ਪ੍ਰੀ-ਬਿਲਟ ਬਾਈਨਰੀ ਡਾਊਨਲੋਡ ਕਰੋ

ਸਭ ਤੋਂ ਆਸਾਨ ਤਰੀਕਾ ਬਿਲਟ ਵਰਜ਼ਨ ਵਰਤਣਾ ਹੈ:

1. [GitHub Releases](https://github.com/owu/wsl-dashboard/releases) ਪੇਜ ਤੇ ਜਾਓ.
2. Windows ਲਈ ਨਵੀਨਤਮ `wsldashboard` ਐਗਜ਼ਿਕਿਊਟੇਬਲ ਡਾਊਨਲੋਡ ਕਰੋ.
3. ਡੀਕੰਪ੍ਰੈਸ ਕਰੋ (ਕੰਪ੍ਰੈਸਡ ਫਾਈਲ ਹੋਵੇ ਤਾਂ) ਅਤੇ `wsldashboard.exe` ਚਲਾਓ.

ਇੰਸਟਾਲੇਸ਼ਨ ਦੀ ਲੋੜ ਨਹੀਂ, ਇਹ ਐਪ ਸਿੰਗਲ-ਫਾਈਲ ਪੋਰਟੇਬਲ ਪ੍ਰੋਗਰਾਮ ਹੈ.

### ਤਰੀਕਾ 2: ਸੋਰਸ ਤੋਂ ਬਿਲਡ ਕਰੋ

Rust ਟੂਲਚੇਨ (Rust 1.92+ ਜਾਂ ਨਵੀਨਤਮ) ਇੰਸਟਾਲ ਹੋਈ ਹੋਣ ਦੀ ਯਕੀਨੀ ਬਣਾਓ.

1. ਰਿਪੋਜ਼ਟਰੀ ਕਲੋਨ ਕਰੋ:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. ਬਿਲਡ ਕਰੋ ਅਤੇ ਚਲਾਓ:

   - ਡਿਵੈਲਪਮੈਂਟ ਡੀਬੱਗ:

     ```powershell
     cargo run
     ```
   - ਆਪਟੀਮਾਈਜ਼ ਰਿਲੀਜ਼ ਬਿਲਡ ਲਈ ਬਿਲਡ ਸਕ੍ਰਿਪਟ ਵਰਤੋ:

     > ਬਿਲਡ ਸਕ੍ਰਿਪਟ ਨੂੰ `x86_64-pc-windows-msvc` ਟੂਲਚੇਨ ਦੀ ਲੋੜ ਹੈ.

     ```powershell
     .\build\portable\build.ps1
     ```


## 🛠️ ਤਕਨਾਲੋਜੀ ਸਟੈਕ & ਪ੍ਰਦਰਸ਼ਨ

- **ਕਰਨਲ**: ਮੈਮਰੀ ਸੁਰੱਖਿਆ ਅਤੇ ਜ਼ੀਰੋ-ਕਾਸਟ ਐਬਸਟ੍ਰੈਕਸ਼ਨ ਯਕੀਨੀ ਬਣਾਉਣ ਲਈ Rust ਵਿੱਚ ਲਾਗੂ.
- **UI ਫਰੇਮਵਰ्क**: ਉੱਚ-ਪ੍ਰਦਰਸ਼ਨ **Skia** ਰੈਂਡਰਿੰਗ ਬੈਕਐਂਡ ਨਾਲ Slint.
- **ਅਸਿੰਕ੍ਰੋਨਸ ਰਨਟਾਈਮ**: ਗੈਰ-ਬਲਾਕਿੰਗ ਸਿਸਟਮ ਕਮਾਂਡ ਅਤੇ I/O ਲਈ Tokio.
- **ਪ੍ਰਦਰਸ਼ਨ ਹਾਈਲਾਈਟਸ**:
  - **ਜਵਾਬ ਦੀ ਗਤੀ**: ਲਗਭਗ ਤੁਰੰਤ ਸਟਾਰਟਅਪ ਗਤੀ, WSL ਸਥਿਤੀ ਦੀ ਰੀਅਲ-ਟਾਈਮ ਨਿਗਰਾਨੀ.
  - **ਸਰੋਤ ਕਾਰਗੁਜ਼ਾਰੀ**: ਬਹੁਤ ਘੱਟ ਸਰੋਤ ਵਰਤੋਂ (ਵੇਰਵਿਆਂ ਲਈ [ਮੁੱਖ ਵਿਸ਼ੇਸ਼ਤਾਵਾਂ](#-ਮੁੱਖ-ਵਿਸ਼ੇਸ਼ਤਾਵਾਂ--ਵਰਤੋਂ) ਵੇਖੋ).
  - **ਪੋਰਟੇਬਿਲਟੀ**: ਆਪਟੀਮਾਈਜ਼ ਰਿਲੀਜ਼ ਬਿਲਡ ਇੱਕ ਸੰਕੁਚਿਤ ਐਗਜ਼ਿਕਿਊਟੇਬਲ ਬਣਾਉਂਦਾ ਹੈ.



## 🤝 ਕਮਿਊਨਿਟੀ ਸਹਾਇਤਾ

ਇਨ੍ਹਾਂ ਕਮਿਊਨਿਟੀਆਂ ਦੀ ਸਹਾਇਤਾ ਲਈ ਦਿਲੋਂ ਧੰਨਵਾਦ:

- [Rust Programming Language](https://www.rust-lang.org) - ਸ਼ਕਤੀਸ਼ਾਲੀ ਅਤੇ ਸੁਰੱਖਿਅਤ ਪ੍ਰੋਗਰਾਮਿੰਗ ਭਾਸ਼ਾ
- [Slint | Declarative GUI for Rust, C++, JavaScript & Python](https://slint.dev) - ਆਧੁਨਿਕ UI ਫਰੇਮਵਰਕ
- [WSL: Windows Subsystem for Linux](https://github.com/microsoft/WSL) - ਸ਼ਾਨਦਾਰ Windows Subsystem for Linux
- [Tokio - An asynchronous Rust runtime](https://tokio.rs) - ਕੁਸ਼ਲ ਅਸਿੰਕ੍ਰੋਨਸ ਰਨਟਾਈਮ
- [Windows Developer Community](https://developer.microsoft.com/en-us/windows/community) - ਲਗਾਤਾਰ ਪਲੇਟਫਾਰਮ ਸੁਧਾਰ
- [Reddit](https://www.reddit.com) - ਗਲੋਬਲ ਕਮਿਊਨਿਟੀ ਚਰਚਾ & ਸਹਾਇਤਾ
- [Hacker News](https://news.ycombinator.com) - ਗਲੋਬਲ ਕਮਿਊਨਿਟੀ ਚਰਚਾ & ਸਹਾਇਤਾ
- [Linux.do](https://linux.do) - IT ਪੇਸ਼ੇਵਰਾਂ ਲਈ ਮਸ਼ਹੂਰ ਕਮਿਊਨਿਟੀ
- [V2EX](https://www.v2ex.com) - ਚੀਨੀ ਤਕਨਾਲੋਜੀ ਕਮਿਊਨਿਟੀ ਚਰਚਾ

ਤੁਹਾਡਾ ਯੋਗਦਾਨ ਅਤੇ ਫੀਡਬੈਕ ਇਸ ਪ੍ਰੋਜੈਕਟ ਨੂੰ ਸੰਭਵ ਬਣਾਉਂਦਾ ਹੈ!


## ❤️ ਇਸ ਪ੍ਰੋਜੈਕਟ ਦਾ ਸਮਰਥਨ ਕਰੋ

- ਇਹ ਪ੍ਰੋਜੈਕਟ GPL-3.0 ਓਪਨ ਸੋਰਸ ਲਾਇਸੈਂਸ ਦਾ ਪਾਲਣ ਕਰਦਾ ਹੈ, ਸਾਰੇ ਉਪਭੋਗਤਾਵਾਂ ਲਈ ਮੁਫਤ.
- ਵਿਸ਼ੇਸ਼ਤਾ ਵਿਕਾਸ, ਰੋਜ਼ਾਨਾ ਟੈਸਟਿੰਗ ਤੋਂ ਲੈ ਕੇ ਬੱਗ ਫਿਕਸ ਤੱਕ, ਸਾਰਾ ਕੰਮ ਖਾਲੀ ਸਮੇਂ ਵਿੱਚ ਲਗਾਤਾਰ ਮਿਹਨਤ ਤੋਂ ਆਉਂਦਾ ਹੈ। ਓਪਨ ਸੋਰਸ ਰਸਤਾ ਅਕੇਲੇ ਆਸਾਨ ਨਹੀਂ, ਤੁਹਾਡੀ ਮਾਨਤਾ ਅਤੇ ਸਮਰਥਨ ਪ੍ਰੋਜੈਕਟ ਨੂੰ ਲੰਬੇ ਸਮੇਂ ਤੱਕ ਚਲਾਉਣ ਦੀ ਤਾਕਤ ਹੈ.
- ਜੇ ਤੁਹਾਨੂੰ ਲੱਗਦਾ ਹੈ ਕਿ ਇਹ ਟੂਲ ਤੁਹਾਡੀ ਸਹਾਇਤਾ ਕਰਦਾ ਹੈ, ਤਾਂ ਕਿਰਪਾ ਕਰਕੇ ਮਦਦ ਦਾ ਹੱਥ ਵਧਾਓ। ਸਾਰੇ ਦਾਨ ਸਰਵਰ ਖਰਚ, ਵਰਜ਼ਨ ਅੱਪਡੇਟ ਅਤੇ ਵਿਸ਼ੇਸ਼ਤਾ ਆਪਟੀਮਾਈਜ਼ੇਸ਼ਨ ਲਈ ਵਰਤੇ ਜਾਣਗੇ.
- ਛੋਟੀ ਦਿਆਲਤਾ ਵੀ ਤਾਰੇ ਵਾਂਗ ਚਮਕਦੀ ਹੈ। ਤੁਹਾਡੀ ਸਮਝ ਅਤੇ ਉਦਾਰਤਾ ਲਈ ਦੁਬਾਰਾ ਧੰਨਵਾਦ!

ਸਾਡਾ ਦਾਨ ਪੇਜ ਵੇਖੋ: [https://www.wslui.com/donate/](https://www.wslui.com/donate/)


## ⭐️ ਪਿਆਰ ਨਾਲ ਚੱਲਣ ਵਾਲਾ ਪ੍ਰੋਜੈਕਟ

ਜੇ ਤੁਹਾਨੂੰ ਲੱਗਦਾ ਹੈ ਕਿ ਇਹ ਪ੍ਰੋਜੈਕਟ ਤੁਹਾਡੇ ਲਈ ਮਦਦਗਾਰ ਸਾਬਤ ਹੋਇਆ ਹੈ, ਤਾਂ GitHub ਤੇ ਇੱਕ ਤਾਰਾ ਦੇ ਕੇ ਤੁਹਾਡੀ ਮਾਨਤਾ ਦੇਣ ਲਈ ਮੈਂ ਧੰਨਵਾਦੀ ਹੋਵਾਂਗਾ। ਤੁਹਾਡੀ ਮਾਨਤਾ ਪ੍ਰੋਜੈਕਟ ਨੂੰ ਵਿਆਪਕ ਉਪਭੋਗਤਾਵਾਂ ਤੱਕ ਪਹੁੰਚਣ ਵਿੱਚ ਮਦਦ ਕਰੇਗੀ। ਇਹ ਹੌਂਸਲਾ ਮੈਨੂੰ ਲਗਾਤਾਰ ਅੱਗੇ ਵਧਾਉਂਦਾ ਹੈ।


## 📄 ਓਪਨ ਸੋਰਸ ਲਾਇਸੈਂਸ

ਇਹ ਪ੍ਰੋਜੈਕਟ GPL-3.0 ਲਾਇਸੈਂਸ ਅਧੀਨ ਲਾਇਸੈਂਸਸ਼ੁਦਾ ਹੈ – ਵੇਰਵਿਆਂ ਲਈ [LICENSE](../LICENSE) ਫਾਈਲ ਵੇਖੋ.


---

Built with ❤️ for the WSL Community.