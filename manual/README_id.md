# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

Dashboard manajemen instance WSL (Windows Subsystem for Linux) yang modern, berkinerja tinggi, dan ringan. Dibangun dengan Rust dan Slint untuk pengalaman native premium.

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

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | Bahasa Indonesia | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md)

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
- [⭐️ Karya penuh cinta](#️-karya-penuh-cinta)
- [📄 Lisensi](#-lisensi)

---

## 🌍 Dukungan Bahasa

Inggris, Mandarin, Mandarin, Hindi, Spanyol, Prancis, Arabic, Bengali, Portugis, Rusia, Urdu, Indonesia, Jerman, Jepang, Turki, Korean, Italia, Dutch, Swedish, Czech, Greek, Hungarian, Hebrew, Norwegian, Danish, Finnish, Slovak, Slovenian, Icelandic

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
</p>


## 🚀 Fitur Utama & Penggunaan

- **UI Native Modern**: GUI intuitif dengan dukungan mode terang/gelap, animasi halus, dan rendering performa tinggi bertenaga **Skia**.
- **Integrasi System Tray**: Dukungan penuh untuk meminimalkan ke area notifikasi (~10MB penggunaan RAM), klik ganda untuk beralih, dan menu klik kanan yang fungsional.
- **Startup Cerdas**: Konfigurasikan dashboard untuk mulai saat Windows menyala, meminimalkan ke tray (mode senyap dengan `/silent`), dan mematikan distribusi secara otomatis saat keluar.
- **Kontrol Instance Komprehensif**: Mulai, Berhenti, Hentikan Paksa, dan Batalkan Registrasi dalam satu klik. Pemantauan status real-time serta wawasan mendalam tentang penggunaan disk dan lokasi file.
- **Manajemen Distro**: Tetapkan sebagai default, migrasi (pindahkan VHDX ke drive lain), serta ekspor/kloning ke format `.tar` atau `.tar.gz`.
- **Integrasi Cepat**: Luncurkan Terminal, VS Code, atau File Explorer secara instan dengan direktori kerja yang dapat disesuaikan dan hook skrip startup.
- **Instalasi Cerdas**: Instal dari Microsoft Store, GitHub, atau file lokal (RootFS/VHDX). Termasuk asisten unduhan RootFS bawaan.
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

### Tentang
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
</p>

## 🎬 Demo Operasi

Di bawah ini adalah demonstrasi WSL Dashboard dalam beraksi:

![Demo WSL Dashboard](../assets/screenshot/demo.gif)



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
     .\build\scripts\build.ps1
     ```


## 🛠️ Stack Teknologi & Performa

- **Inti**: Diimplementasikan dalam Rust untuk keamanan memori dan abstraksi tanpa biaya.
- **Framework UI**: Slint dengan backend rendering **Skia** performa tinggi.
- **Async Runtime**: Tokio untuk perintah sistem dan I/O non-blocking.
- **Sorotan Performa**:
  - **Responsivitas**: Startup hampir instan dan pemantauan status WSL real-time.
  - **Efisiensi**: Penggunaan sumber daya ultra rendah (lihat [Fitur Utama](#-fitur-utama--penggunaan) untuk detail).
  - **Portabilitas**: Rakitan rilis yang dioptimalkan menghasilkan executable tunggal yang ringkas.



## ⭐️ Karya penuh cinta

Jika Anda merasa proyek ini bermanfaat, saya akan sangat berterima kasih jika Anda dapat memberikan bintang di GitHub. Dukungan Anda membantu proyek ini menjangkau khalayak yang lebih luas dan sangat dihargai. Dorongan inilah yang memotivasi saya untuk terus berkarya.

## 📄 Lisensi

Proyek ini dilisensikan di bawah GPL-3.0 – lihat file [LICENSE](../LICENSE) untuk detailnya.

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
