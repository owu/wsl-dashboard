# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

Dasbor manajemen instans WSL (Windows Subsystem for Linux) yang modern, berkinerja tinggi, ringan, dan hemat memori. Dibangun dengan Rust dan Slint untuk pengalaman asli premium.

---

```diff
Pemberitahuan:

- Perangkat lunak ini tidak didistribusikan melalui Microsoft Store.
- Setiap aplikasi yang terdaftar di sana dengan nama yang sama tidak sah dan mungkin palsu.
- Harap jangan unduh untuk menghindari penipuan.
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

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | Bahasa Indonesia | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 Daftar Isi
- [🌍 Dukungan Bahasa](#-dukungan-bahasa)
- [🚀 Fitur Utama & Penggunaan](#-fitur-utama--penggunaan)
- [⚙️ Konfigurasi & Log](#️-konfigurasi--log)
- [🖼️ Tangkapan Layar](#️-tangkapan-layar)
- [🎬 Demo Operasi](#-demo-operasi)
- [💻 Persyaratan Sistem](#-persyaratan-sistem)
- [📦 Panduan Instalasi](#-panduan-instalasi)
- [🛠️ Stack Teknologi & Performa](#️-stack-teknologi--performa)
- [🤝 Dukungan Komunitas](#-dukungan-komunitas)
- [❤️ Dukung proyek ini](#️-dukung-proyek-ini)
- [⭐️ Karya penuh cinta](#️-karya-penuh-cinta)
- [📄 Lisensi](#-lisensi)

---

## 🌍 Dukungan Bahasa

Inggris, Mandarin, Mandarin, Hindi, Spanyol, Prancis, Arabic, Bengali, Portugis, Rusia, Urdu, Indonesia, Jerman, Jepang, Turki, Korean, Italia, Dutch, Swedish, Czech, Greek, Hungarian, Hebrew, Norwegian, Danish, Finnish, Slovak, Slovenian, Icelandic, Vietnam, Telugu, Jawa, Thai, Tamil, Filipino, Punjabi, Melayu, Polandia, Ukraina, Persia, Kannada, Marathi, Hausa, Myanmar, Uzbek, Azerbaijan, Cebuano, Malayalam, Sindhi, Amharic

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="Inggris" alt="Inggris" />
  <img src="../assets/flags/cn.svg" width="32" title="Tionghoa (Sederhana)" alt="Tionghoa (Sederhana)" />
  <img src="../assets/flags/tw.svg" width="32" title="Tionghoa (Tradisional)" alt="Tionghoa (Tradisional)" />
  <img src="../assets/flags/in.svg" width="32" title="Hindi" alt="Hindi" />
  <img src="../assets/flags/es.svg" width="32" title="Spanyol" alt="Spanyol" />
  <img src="../assets/flags/fr.svg" width="32" title="Prancis" alt="Prancis" />
  <img src="../assets/flags/sa.svg" width="32" title="Arab" alt="Arab" />
  <img src="../assets/flags/bd.svg" width="32" title="Bengali" alt="Bengali" />
  <img src="../assets/flags/pt.svg" width="32" title="Portugis" alt="Portugis" />
  <img src="../assets/flags/ru.svg" width="32" title="Rusia" alt="Rusia" />
  <img src="../assets/flags/pk.svg" width="32" title="Urdu" alt="Urdu" />
  <img src="../assets/flags/id.svg" width="32" title="Indonesia" alt="Indonesia" />
  <img src="../assets/flags/de.svg" width="32" title="Jerman" alt="Jerman" />
  <img src="../assets/flags/jp.svg" width="32" title="Jepang" alt="Jepang" />
  <img src="../assets/flags/tr.svg" width="32" title="Turki" alt="Turki" />
  <img src="../assets/flags/kr.svg" width="32" title="Korea" alt="Korea" />
  <img src="../assets/flags/it.svg" width="32" title="Italia" alt="Italia" />
  <img src="../assets/flags/nl.svg" width="32" title="Belanda" alt="Belanda" />
  <img src="../assets/flags/se.svg" width="32" title="Swedia" alt="Swedia" />
  <img src="../assets/flags/cz.svg" width="32" title="Ceko" alt="Ceko" />
  <img src="../assets/flags/gr.svg" width="32" title="Yunani" alt="Yunani" />
  <img src="../assets/flags/hu.svg" width="32" title="Hungaria" alt="Hungaria" />
  <img src="../assets/flags/il.svg" width="32" title="Ibrani" alt="Ibrani" />
  <img src="../assets/flags/no.svg" width="32" title="Norwegia" alt="Norwegia" />
  <img src="../assets/flags/dk.svg" width="32" title="Denmark" alt="Denmark" />
  <img src="../assets/flags/fi.svg" width="32" title="Finlandia" alt="Finlandia" />
  <img src="../assets/flags/sk.svg" width="32" title="Slowak" alt="Slowak" />
  <img src="../assets/flags/si.svg" width="32" title="Slovenia" alt="Slovenia" />
  <img src="../assets/flags/is.svg" width="32" title="Islandia" alt="Islandia" />
  <img src="../assets/flags/vn.svg" width="32" title="Vietnam" alt="Vietnam" />
  <img src="../assets/flags/in.svg" width="32" title="Telugu" alt="Telugu" />
  <img src="../assets/flags/id.svg" width="32" title="Jawa" alt="Jawa" />
  <img src="../assets/flags/th.svg" width="32" title="Thai" alt="Thai" />
  <img src="../assets/flags/in.svg" width="32" title="Tamil" alt="Tamil" />
  <img src="../assets/flags/ph.svg" width="32" title="Filipino" alt="Filipino" />
  <img src="../assets/flags/pk.svg" width="32" title="Punjabi" alt="Punjabi" />
  <img src="../assets/flags/my.svg" width="32" title="Melayu" alt="Melayu" />
  <img src="../assets/flags/pl.svg" width="32" title="Polandia" alt="Polandia" />
  <img src="../assets/flags/ua.svg" width="32" title="Ukraina" alt="Ukraina" />
  <img src="../assets/flags/ir.svg" width="32" title="Persia" alt="Persia" />
  <img src="../assets/flags/in.svg" width="32" title="Kannada" alt="Kannada" />
  <img src="../assets/flags/in.svg" width="32" title="Marathi" alt="Marathi" />
  <img src="../assets/flags/ng.svg" width="32" title="Hausa" alt="Hausa" />
  <img src="../assets/flags/mm.svg" width="32" title="Myanmar" alt="Myanmar" />
  <img src="../assets/flags/uz.svg" width="32" title="Uzbek" alt="Uzbek" />
  <img src="../assets/flags/az.svg" width="32" title="Azerbaijan" alt="Azerbaijan" />
  <img src="../assets/flags/ph.svg" width="32" title="Cebuano" alt="Cebuano" />
  <img src="../assets/flags/in.svg" width="32" title="Malayalam" alt="Malayalam" />
  <img src="../assets/flags/pk.svg" width="32" title="Sindhi" alt="Sindhi" />
  <img src="../assets/flags/et.svg" width="32" title="Amharic" alt="Amharic" />
</p>


## 🚀 Fitur Utama & Penggunaan

- **UI Native Modern**: GUI intuitif dengan dukungan mode terang/gelap, animasi halus, dan rendering performa tinggi bertenaga **Skia**.
- **Integrasi System Tray**: Dukungan penuh untuk meminimalkan ke area notifikasi (~10MB penggunaan RAM), klik ganda untuk beralih, dan menu klik kanan yang fungsional.
- **Startup Cerdas**: Konfigurasikan dashboard untuk mulai saat Windows menyala, meminimalkan ke tray (mode senyap dengan `/silent`), dan mematikan distribusi secara otomatis saat keluar.
- **Kontrol Instance Komprehensif**: Mulai, Berhenti, Hentikan Paksa, dan Batalkan Registrasi dalam satu klik. Pemantauan status real-time serta wawasan mendalam tentang penggunaan disk dan lokasi file.
- **Manajemen Distro**: Tetapkan sebagai default, migrasi (pindahkan VHDX ke drive lain), serta ekspor/kloning ke format `.tar` atau `.tar.gz`.
- **Integrasi Cepat**: Luncurkan Terminal, VS Code, atau File Explorer secara instan dengan direktori kerja yang dapat disesuaikan dan hook skrip startup.
- **Instalasi Distro**: Instal distribusi Linux melalui Microsoft Store, GitHub, file lokal (RootFS/VHDX), atau mirror online (dengan tes kecepatan otomatis untuk memilih mirror tercepat dan asisten unduhan RootFS bawaan).
- **Keamanan Global**: Kunci mutex untuk operasi migrasi/cadangan bersamaan yang aman, dan pembersihan Appx otomatis saat penghapusan.
- **Jejak Memori Ultra Rendah**: Sangat dioptimalkan untuk efisiensi. Startup senyap (system tray) hanya menggunakan **~10MB** RAM. Penggunaan mode jendela bervariasi menurut kompleksitas font: **~18MB** untuk bahasa standar dan **~38MB** untuk bahasa dengan set karakter besar (Mandarin, Jepang, Korea).
- **Jaringan Lanjutan**: Manajemen port forwarding yang mulus (dengan pembuatan aturan firewall otomatis) dan konfigurasi proxy HTTP global untuk konektivitas terpadu.
- **Manajemen Perangkat USB**: Integrasi penuh dengan `usbipd-win` untuk penyatuan, pelekatan, dan manajemen perangkat USB lokal tanpa kesulitan di seluruh instance WSL Anda melalui antarmuka dasbor.


## ⚙️ Konfigurasi & Log

Semua konfigurasi dikelola melalui tampilan Pengaturan:

- Pilih direktori instalasi default untuk instance WSL baru.
- Konfigurasikan direktori log dan level log (Error / Warn / Info / Debug / Trace).
- Pilih bahasa UI atau biarkan mengikuti bahasa sistem.
- Alihkan mode gelap, dan matikan WSL otomatis setelah operasi.
- Konfigurasikan frekuensi pemeriksaan pembaruan (harian, mingguan, dua mingguan, bulanan).
- Aktifkan startup otomatis saat boot sistem (dengan perbaikan jalur otomatis).
- Atur aplikasi untuk meminimalkan ke tray saat startup.
- Konfigurasikan tombol tutup untuk meminimalkan ke tray alih-alih keluar dari program.
- Sesuaikan bilah samping dengan mengatur visibilitas tab fitur tertentu.

File log ditulis ke direktori log yang dikonfigurasi dan dapat dilampirkan saat melaporkan masalah.


## 🖼️ Tangkapan Layar

### Beranda (Mode Gelap & Terang)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB & Menu ciutkan
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### jaringan
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### Tambah Instance & Pengaturan
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>
<p align="center">
  <img src="../assets/screenshot/settings-advanced.png" width="48%" />
  <img src="../assets/screenshot/settings-interface.png" width="48%" />
</p>

### Tentang & Donasi
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/donate.png" width="48%" />
</p>

## 🎬 Demo Operasi

[Bantu kami meningkatkan! Tonton video pengantar kami dan bagikan pendapat Anda.](https://github.com/owu/wsl-dashboard/discussions/9)



## 💻 Persyaratan Sistem

- Windows 10 atau Windows 11 dengan WSL diaktifkan (disarankan WSL 2).
- Setidaknya satu distribusi WSL terpasang, atau izin untuk memasang yang baru.
- CPU 64-bit; RAM 4 GB atau lebih disarankan untuk penggunaan yang lancar.

## 📦 Panduan Instalasi

### Opsi 1: Unduh binary yang sudah dikompilasi

Cara termudah untuk memulai adalah menggunakan rilis yang sudah dikompilasi:

1. Buka halaman [GitHub Releases](https://github.com/owu/wsl-dashboard/releases).
2. Unduh executable `wsldashboard` terbaru untuk Windows.
3. Ekstrak (jika dikemas) dan jalankan `wsldashboard.exe`.

Tidak diperlukan penginstal; aplikasi ini adalah binary portabel tunggal.

### Opsi 2: Rakit dari sumber kode

Pastikan Anda telah memasang alat bantu Rust (Rust 1.92+ atau yang lebih baru).

1. Kloning repositori:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. Rakit dan jalankan:

   - Untuk pengembangan:

     ```powershell
     cargo run
     ```
   - Buat rakitan rilis yang dioptimalkan melalui skrip:

     > Skrip build memerlukan alat bantu `x86_64-pc-windows-msvc`.

     ```powershell
     .\build\portable\build.ps1
     ```


## 🛠️ Stack Teknologi & Performa

- **Inti**: Diimplementasikan dalam Rust untuk keamanan memori dan abstraksi tanpa biaya.
- **Framework UI**: Slint dengan backend rendering **Skia** performa tinggi.
- **Async Runtime**: Tokio untuk perintah sistem dan I/O non-blocking.
- **Sorotan Performa**:
  - **Responsivitas**: Startup hampir instan dan pemantauan status WSL real-time.
  - **Efisiensi**: Penggunaan sumber daya ultra rendah (lihat [Fitur Utama](#-fitur-utama--penggunaan) untuk detail).
  - **Portabilitas**: Rakitan rilis yang dioptimalkan menghasilkan executable tunggal yang ringkas.



## 🤝 Dukungan Komunitas

Terima kasih banyak kepada komunitas berikut atas dukungan mereka:

- [Rust Programming Language](https://www.rust-lang.org) - Untuk bahasa pemrograman yang kuat dan aman
- [Slint | Declarative GUI for Rust, C++, JavaScript & Python](https://slint.dev) - Untuk framework UI modern
- [WSL: Windows Subsystem for Linux](https://github.com/microsoft/WSL) - Untuk Windows Subsystem for Linux yang luar biasa
- [Tokio - An asynchronous Rust runtime](https://tokio.rs) - Untuk runtime asinkron yang efisien
- [Windows Developer Community](https://developer.microsoft.com/en-us/windows/community) - Untuk peningkatan platform yang berkelanjutan
- [Reddit](https://www.reddit.com) - Untuk diskusi dan dukungan komunitas global
- [Hacker News](https://news.ycombinator.com) - Untuk diskusi dan dukungan komunitas global
- [Linux.do](https://linux.do) - Untuk komunitas populer bagi profesional TI
- [V2EX](https://www.v2ex.com) - Untuk diskusi komunitas teknologi Tionghoa

Kontribusi dan umpan balik Anda membuat proyek ini menjadi mungkin！


## ❤️ Dukung proyek ini

- Proyek ini dilisensikan di bawah GPL-3.0 dan gratis untuk semua pengguna.
- Dari pengembangan fitur dan pengujian harian hingga perbaikan bug, semua pekerjaan dilakukan di waktu luang. Jalan open source tidak mudah ditempuh sendiri — pengakuan dan dukungan Anda memberikan kepercayaan diri proyek untuk terus berjalan.
- Jika alat ini benar-benar membantu Anda, pertimbangkan untuk memberikan dukungan. Semua donasi digunakan untuk biaya server, pembaruan versi, dan peningkatan fitur, menjaga proyek tetap diperbarui dan berkembang secara stabil.
- Setiap kebaikan kecil adalah seberkas cahaya bintang. Terima kasih lagi atas pengertian dan kemurahan hati Anda！

Kunjungi halaman donasi kami：[https://www.wslui.com/donate/](https://www.wslui.com/donate/)


## ⭐️ Karya penuh cinta

Jika Anda merasa proyek ini bermanfaat, saya akan sangat berterima kasih jika Anda dapat memberikan bintang di GitHub. Dukungan Anda membantu proyek ini menjangkau khalayak yang lebih luas dan sangat dihargai. Dorongan inilah yang memotivasi saya untuk terus berkarya.


## 📄 Lisensi

Proyek ini dilisensikan di bawah GPL-3.0 – lihat file [LICENSE](../LICENSE) untuk detailnya.


---

Built with ❤️ for the WSL Community.
