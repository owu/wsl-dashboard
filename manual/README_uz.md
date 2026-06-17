# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

Zamonaviy, yuqori samarali, yengil va past xotira iste'moli bilan WSL (Windows Subsystem for Linux) instansiyalarini boshqarish paneli. Rust va Slint asosida qurilgan, ajoyib nativ tajriba taqdim etadi.

---

```diff
E'lon:

- WSL Dashboard Microsoft Store orqali tarqatilmaydi.
- U yerda "WSL Dashboard" nomi bilan roʻyxatga olingan har qanday dastur ruxsatsiz va soxta boʻlishi mumkin.
- Iltimos, mumkin boʻlgan firibgarliklardan qochish uchun uni yuklab olmang.
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

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | Oʻzbek | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 Mündəricat
- [🌍 Til qo'llab-quvvatlash](#-til-qo'llab-quvvatlash)
- [🚀 Asosiy xususiyatlar va foydalanish](#-asosiy-xususiyatlar-va-foydalanish)
- [⚙️ Konfiguratsiya va loglar](#️-konfiguratsiya-va-loglar)
- [🖼️ Dastur skrinshotlari](#️-dastur-skrinshotlari)
- [🎬 Amaliyot demosu](#-amaliyot-demosu)
- [💻 Sistema talablari](#-sistema-talablari)
- [📦 O'rnatish bo'yicha qo'llanma](#-o'rnatish-bo'yicha-qo'llanma)
- [🛠️ Texnologik stak va samaradorlik](#️-texnologik-stak-va-samaradorlik)
- [🤝 Jamiyat qo'llab-quvvatlash](#-jamiyat-qo'llab-quvvatlash)
- [❤️ Loyihani qo'llab-quvvatlang](#️-loyihani-qo'llab-quvvatlang)
- [⭐️ Sevgi bilan ishlaydigan loyiha](#️-sevgi-bilan-ishlaydigan-loyiha)
- [📄 Ochiq manba litsenziyasi](#-ochiq-manba-litsenziyasi)

---

## 🌍 Til qo'llab-quvvatlash

Ingliz, Sadalashtirilgan Xitoy, An'anaviy Xitoy, Hindi, Ispan, Fransuz, Arab, Bengal, Portugiz, Rus, Urdu, Indoneziya, Nemis, Yapon, Turkiy, Koreya, Italiya, Golland, Shvetsiya, Chex, Gretsiya, Vengriya, Ibroniya, Norvegiya, Daniya, Finlyandiya, Slovak, Sloven, Island, Vetnam, Telugu, Yava, Tailand, Tamil, Filippin, Panjob, Malay, Polyak, Ukraina, Fors, Kannada, Marathi, Hausa, Myanma, O'zbek, Ozarbayjon, Sebuan, Malayalam, Sindhi, Amxar

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="Ingliz" alt="Ingliz" />
  <img src="../assets/flags/cn.svg" width="32" title="Sadalashtirilgan Xitoy" alt="Sadalashtirilgan Xitoy" />
  <img src="../assets/flags/tw.svg" width="32" title="An'anaviy Xitoy" alt="An'anaviy Xitoy" />
  <img src="../assets/flags/in.svg" width="32" title="Hindi" alt="Hindi" />
  <img src="../assets/flags/es.svg" width="32" title="Ispan" alt="Ispan" />
  <img src="../assets/flags/fr.svg" width="32" title="Fransuz" alt="Fransuz" />
  <img src="../assets/flags/sa.svg" width="32" title="Arab" alt="Arab" />
  <img src="../assets/flags/bd.svg" width="32" title="Bengal" alt="Bengal" />
  <img src="../assets/flags/pt.svg" width="32" title="Portugiz" alt="Portugiz" />
  <img src="../assets/flags/ru.svg" width="32" title="Rus" alt="Rus" />
  <img src="../assets/flags/pk.svg" width="32" title="Urdu" alt="Urdu" />
  <img src="../assets/flags/id.svg" width="32" title="Indoneziya" alt="Indoneziya" />
  <img src="../assets/flags/de.svg" width="32" title="Nemis" alt="Nemis" />
  <img src="../assets/flags/jp.svg" width="32" title="Yapon" alt="Yapon" />
  <img src="../assets/flags/tr.svg" width="32" title="Turkiy" alt="Turkiy" />
  <img src="../assets/flags/kr.svg" width="32" title="Koreya" alt="Koreya" />
  <img src="../assets/flags/it.svg" width="32" title="Italiya" alt="Italiya" />
  <img src="../assets/flags/nl.svg" width="32" title="Golland" alt="Golland" />
  <img src="../assets/flags/se.svg" width="32" title="Shvetsiya" alt="Shvetsiya" />
  <img src="../assets/flags/cz.svg" width="32" title="Chex" alt="Chex" />
  <img src="../assets/flags/gr.svg" width="32" title="Gretsiya" alt="Gretsiya" />
  <img src="../assets/flags/hu.svg" width="32" title="Vengriya" alt="Vengriya" />
  <img src="../assets/flags/il.svg" width="32" title="Ibroniya" alt="Ibroniya" />
  <img src="../assets/flags/no.svg" width="32" title="Norvegiya" alt="Norvegiya" />
  <img src="../assets/flags/dk.svg" width="32" title="Daniya" alt="Daniya" />
  <img src="../assets/flags/fi.svg" width="32" title="Finlyandiya" alt="Finlyandiya" />
  <img src="../assets/flags/sk.svg" width="32" title="Slovak" alt="Slovak" />
  <img src="../assets/flags/si.svg" width="32" title="Sloven" alt="Sloven" />
  <img src="../assets/flags/is.svg" width="32" title="Island" alt="Island" />
  <img src="../assets/flags/vn.svg" width="32" title="Vetnam" alt="Vetnam" />
  <img src="../assets/flags/in.svg" width="32" title="Telugu" alt="Telugu" />
  <img src="../assets/flags/id.svg" width="32" title="Yava" alt="Yava" />
  <img src="../assets/flags/th.svg" width="32" title="Tailand" alt="Tailand" />
  <img src="../assets/flags/in.svg" width="32" title="Tamil" alt="Tamil" />
  <img src="../assets/flags/ph.svg" width="32" title="Filippin" alt="Filippin" />
  <img src="../assets/flags/pk.svg" width="32" title="Panjob" alt="Panjob" />
  <img src="../assets/flags/my.svg" width="32" title="Malay" alt="Malay" />
  <img src="../assets/flags/pl.svg" width="32" title="Polyak" alt="Polyak" />
  <img src="../assets/flags/ua.svg" width="32" title="Ukraina" alt="Ukraina" />
  <img src="../assets/flags/ir.svg" width="32" title="Fors" alt="Fors" />
  <img src="../assets/flags/in.svg" width="32" title="Kannada" alt="Kannada" />
  <img src="../assets/flags/in.svg" width="32" title="Marathi" alt="Marathi" />
  <img src="../assets/flags/ng.svg" width="32" title="Hausa" alt="Hausa" />
  <img src="../assets/flags/mm.svg" width="32" title="Myanma" alt="Myanma" />
  <img src="../assets/flags/uz.svg" width="32" title="O'zbek" alt="O'zbek" />
  <img src="../assets/flags/az.svg" width="32" title="Ozarbayjon" alt="Ozarbayjon" />
  <img src="../assets/flags/ph.svg" width="32" title="Sebuan" alt="Sebuan" />
  <img src="../assets/flags/in.svg" width="32" title="Malayalam" alt="Malayalam" />
  <img src="../assets/flags/pk.svg" width="32" title="Sindhi" alt="Sindhi" />
  <img src="../assets/flags/et.svg" width="32" title="Amxar" alt="Amxar" />
</p>

---

## 🚀 Asosiy xususiyatlar va foydalanish

- **Zamonaviy UI dizayni** - Slint 6.x asosida qurilgan, silliq animatsiyalar va yuqori tezlik
- **Past xotira iste'moli** - Minimal xotira bilan ishlaydi
- **WMI integratsiyasi** - Windows Management Instrumentation orqali WSL instansiyalarini boshqarish
- **Ko'p tilli qo'llab-quvvatlash** - 51 til qo'llab-quvvatlanadi
- **Real vaqt monitoringi** - WSL instansiyalarining holatini real vaqtda kuzatish

### Asosiy funksiyalar

| Funktsiya | Tavsif |
|-----------|--------|
| Instansiyalarni boshqarish | Barcha WSL instansiyalarini ko'rish, ishga tushirish, to'xtatish |
| Import/Export | WSL instansiyalarini import va export qilish |
| Backup/Restore | Instansiyalarni zaxiralash va tiklash |
| Sozlamalar | Dastur sozlamalarini boshqarish |
| Loglar | Tizim loglarini ko'rish |

---

## ⚙️ Konfiguratsiya va loglar

Dastur sozlamalari va loglar fayllari quyidagi joylarda saqlanadi:

### Konfiguratsiya fayllari

| Fayl | Joylashuvi | Tavsif |
|------|------------|--------|
| `config.toml` | `%APPDATA%/wsl-dashboard/` | Asosiy konfiguratsiya |
| `i18n/` | Dastur katalogi | Til fayllari |

### Log fayllari

| Fayl | Joylashuvi | Tavsif |
|------|------------|--------|
| `app.log` | `%APPDATA%/wsl-dashboard/logs/` | Asosiy log fayli |
| `error.log` | `%APPDATA%/wsl-dashboard/logs/` | Xatolik loglari |

### Log darajalari

Log darajalari `config.toml` faylida sozlanishi mumkin:

```toml
[log]
level = "info"  # debug, info, warn, error
```

---

## 🖼️ Dastur skrinshotlari

### Asosiy interfeys

<p align="center">
  <img src="../docs/screenshots/main_window.png" width="800" alt="Asosiy oyna" />
</p>

### Sozlamalar

<p align="center">
  <img src="../docs/screenshots/settings.png" width="800" alt="Sozlamalar oynasi" />
</p>

### Til tanlash

<p align="center">
  <img src="../docs/screenshots/language_selector.png" width="800" alt="Lav tanlash" />
</p>

---

## 🎬 Amaliyot demosu

### WSL instansiyalarini boshqarish

1. **Instansiyalarni ko'rish** - Barcha WSL instansiyalari ro'yxatini ko'rish
2. **Ishga tushirish** - Instansiyani ishga tushirish
3. **To'xtatish** - Instansiyani to'xtatish
4. **Import/Export** - Instansiyalarni import va export qilish

### Backup/Restore

1. **Backup yaratish** - Instansiyani zaxiralash
2. **Restore qilish** - Zaxiralangan instansiyani tiklash
3. **Avtomatik backup** - Rejalashtirilgan zaxiralash

---

## 💻 Sistema talablari

### Minimal talablar

| Komponent | Talab |
|-----------|-------|
| OS | Windows 10 (1903+) yoki Windows 11 |
| WSL | WSL 2 |
| RAM | 512 MB bo'sh xotira |
| Disk | 100 MB bo'sh joy |
| CPU | 1 GHz yoki undan tez |

### Tavsiya etilgan talablar

| Komponent | Tavsiya |
|-----------|---------|
| OS | Windows 11 |
| WSL | WSL 2 (so'nggi versiya) |
| RAM | 2 GB yoki undan ko'p |
| Disk | 1 GB yoki undan ko'p |
| CPU | 2 GHz yoki undan tez |

### Kerakli komponentlar

- **WSL 2** - Windows Subsystem for Linux 2
- **.NET Runtime** - .NET 8.0 yoki undan yuqori (ixtiyoriy)
- **Visual C++ Redistributable** - 2015-2022

---

## 📦 O'rnatish bo'yicha qo'llanma

### 1-usul: Installer orqali

1. So'nggi versiyani yuklab oling: [Releases](https://github.com/your-repo/wsl-dashboard/releases)
2. `WSL-Dashboard-Setup.exe` faylini ishga tushiring
3. O'rnatish bo'yicha ko'rsatmalarga amal qiling

### 2-usul: Portable versiya

1. Portable versiyani yuklab oling
2. Arxivni istalgan joyga oching
3. `wsl-dashboard.exe` faylini ishga tushiring

### 3-usul: Manba koddan

```bash
# Repositoryni klonlash
git clone https://github.com/your-repo/wsl-dashboard.git
cd wsl-dashboard

# Rust toolchain o'rnatilganligini tekshirish
rustc --version
cargo --version

# Loyihani qurish
cargo build --release

# Ishga tushirish
cargo run --release
```

---

## 🛠️ Texnologik stak va samaradorlik

### Asosiy texnologiyalar

| Texnologiya | Versiya | Maqsad |
|-------------|---------|--------|
| Rust | 1.92+ | Asosiy til |
| Slint | 6.x | UI framework |
| Tokio | 1.x | Asinxron runtime |
| Serde | 1.x | Serializatsiya |
| Windows-rs | 0.x | Windows API |

### Samaradorlik ko'rsatkichlari

| Ko'rsatkich | Qiymat |
|-------------|--------|
| Xotira iste'moli | ~50 MB |
| Ishga tushish vaqti | ~2 soniya |
| CPU iste'moli | Minimal (kutish holatida ~0%) |
| Disk joylashuvi | ~30 MB |

### Arxitektura

```
┌─────────────────────────────────────┐
│           Slint UI Layer            │
├─────────────────────────────────────┤
│         Application Logic           │
├─────────────────────────────────────┤
│      WSL Management Layer           │
├─────────────────────────────────────┤
│       Windows API Layer             │
└─────────────────────────────────────┘
```

---

## 🤝 Jamiyat qo'llab-quvvatlash

### Xatoliklar haqida xabar berish

Agar siz xatolik topsangiz, iltimos [Issues](https://github.com/your-repo/wsl-dashboard/issues) sahifasida xabar bering.

### Takliflar

Yangi funksiyalar yoki yaxshilashlar uchun [Discussions](https://github.com/your-repo/wsl-dashboard/discussions) sahifasida muhokama qiling.

### Kod hissasi

1. Repositoryni fork qiling
2. Yangi branch yarating: `git checkout -b feature/your-feature`
3. O'zgarishlarni kiriting: `git commit -m 'Add your feature'`
4. Branchga push qiling: `git push origin feature/your-feature`
5. Pull Request yarating

---

## ❤️ Loyihani qo'llab-quvvatlang

Agar sizga bu loyiha yoqqan bo'lsa, iltimos ⭐ bosing va do'stlaringizga ayting!

### Qo'llab-quvvatlash usullari

- ⭐ **Star bosing** - Loyihani GitHubda yulduzlang
- 🐛 **Xatoliklar haqida xabar bering** - Topilgan muammolarni bildiring
- 💡 **Takliflar bering** - Yangi g'oyalar va takliflar
- 📖 **Hujjatlarni yaxshilang** - Tarjima va hujjatlar
- 💻 **Kod yozing** - Yangi funksiyalar qo'shing

---

## ⭐️ Sevgi bilan ishlaydigan loyiha

Bu loyiha sevgi va ehtiros bilan yaratilgan. Maqsad - WSL foydalanuvchilari uchun eng qulay va samarali boshqarish panelini yaratish.

### Loyiha maqsadlari

- ✅ Zamonaviy va chiroyli interfeys
- ✅ Yuqori samaradorlik
- ✅ Past xotira iste'moli
- ✅ Ko'p tilli qo'llab-quvvatlash
- ✅ Oson foydalanish
- ✅ Barqaror ishlash

---

## 📄 Ochiq manba litsenziyasi

Bu loyiha [GPL-3.0](../LICENSE) litsenziyasi ostida tarqatiladi.

```
GNU GENERAL PUBLIC LICENSE
Version 3, 29 June 2007

Copyright (C) 2007 Free Software Foundation, Inc. <https://fsf.org/>
Everyone is permitted to copy and distribute verbatim copies
of this license document, but changing it is not allowed.
```

### Asosiy shartlar

- ✅ Erkin foydalanish
- ✅ O'zgartirish
- ✅ Tarqatish
- ⚠️ Bir xil litsenziya ostida tarqatish shart
- ⚠️ Manba kodni ochiq saqlash shart

---

<p align="center">
  <sub>Bu hujjat <a href="https://github.com/your-repo/wsl-dashboard">WSL Dashboard</a> loyihasi uchun yaratilgan.</sub>
</p>
