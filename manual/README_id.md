# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" />
</p>

Dashboard manajemen instance WSL (Windows Subsystem for Linux) yang modern, berkinerja tinggi, dan ringan. Dibangun dengan Rust dan Slint untuk pengalaman native premium.

---

[![Rust](https://img.shields.io/badge/rust-v1.92+-orange.svg)](https://www.rust-lang.org)
[![Slint](https://img.shields.io/badge/UI-Slint-blue.svg)](https://slint.dev)
[![License](https://img.shields.io/badge/license-GPL--3.0-blue.svg)](LICENSE)

I18N: [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [English](../README.md) | [日本語](./README_ja.md) | [Français](./README_fr.md) | [Español](./README_es.md) | [Русский](./README_ru.md) | [Português](./README_pt.md) | [Deutsch](./README_de.md) | [Italiano](./README_it.md) | [Türkçe](./README_tr.md) | Bahasa Indonesia | [हिन्दी](./README_hi.md) | [বাংলা](./README_bn.md)

---

## 🖼️ Tangkapan Layar

### Beranda (Mode Terang & Gelap)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

### Tambah Instance & Pengaturan
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>

### Tentang & Menu ciutkan
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

## 🎬 Demo Operasi

Di bawah ini adalah demonstrasi WSL Dashboard dalam beraksi:

![Demo WSL Dashboard](../assets/screenshot/demo.gif)

## 🚀 Fitur Utama

- GUI intuitif dengan dukungan mode gelap dan animasi yang mulus.
- Manajemen satu klik untuk semua distribusi WSL Anda (Mulai, Berhenti, Hentikan Paksa, Batalkan Registrasi).
- Akses cepat ke terminal distribusi, VS Code, dan File Explorer.
- Pemantauan dan tampilan status instance WSL secara real-time.
- Ekspor dan cadangkan ke arsip `.tar` atau `.tar.gz` yang dikompresi.
- Impor dan kloning instance dari cadangan atau distribusi yang ada.
- Relokasi instance WSL berukuran besar (migrasi VHDX) ke disk lain untuk menghemat ruang di drive C:.
- Instalasi distribusi cerdas dari Microsoft Store atau GitHub.
- Bantuan unduhan RootFS bawaan untuk instalasi manual.
- Wawasan mendetail tentang lokasi file VHDX, ukuran disk virtual, dan penggunaan disk aktual.

## Persyaratan Sistem

- Windows 10 atau Windows 11 dengan WSL diaktifkan (disarankan WSL 2).
- Setidaknya satu distribusi WSL terpasang, atau izin untuk memasang yang baru.
- CPU 64-bit; RAM 4 GB atau lebih disarankan untuk penggunaan multi-distro yang lancar.

## 📦 Instalasi

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

   - Rakitan rilis yang dioptimalkan:

     ```powershell
     cargo run --release
     ```

   - Menggunakan skrip build (disarankan untuk menghasilkan binary rilis):

     > Skrip build memerlukan alat bantu `x86_64-pc-windows-gnu`.

     ```powershell
     .\build\scripts\build.ps1
     ```

## 🧭 Ringkasan Penggunaan

- **Kelola distribusi yang ada**: mulai, berhenti, hentikan paksa, atau batalkan registrasi distribusi WSL apa pun dari tampilan utama.
- **Buka alat dengan cepat**: luncurkan distribusi di terminal, VS Code, atau File Explorer Anda dengan satu klik.
- **Buat instance baru**: gunakan tampilan Tambah Instance untuk memasang dari Microsoft Store, mengunduh gambar RootFS, atau mengkloning distribusi yang ada.
- **Cadangkan dan pulihkan**: ekspor distribusi ke arsip `.tar` / `.tar.gz` dan impor nanti atau di mesin lain.
- **Pindahkan lokasi penyimpanan**: relokasi file VHDX besar ke drive lain untuk menghemat ruang drive C:.
- **Pantau status**: awasi status distribusi real-time dan penggunaan penyimpanan saat WSL Dashboard berjalan.

## ⚙️ Konfigurasi & Log

Semua konfigurasi dikelola melalui tampilan Pengaturan:

- Pilih direktori instalasi default untuk instance WSL baru.
- Konfigurasikan direktori log dan level log (Error / Warn / Info / Debug / Trace).
- Pilih bahasa UI atau biarkan mengikuti bahasa sistem.
- Alihkan mode gelap dan apakah aplikasi dapat menghentikan WSL secara otomatis setelah operasi.
- Konfigurasikan seberapa sering aplikasi memeriksa pembaruan (harian, mingguan, dua mingguan, bulanan).

File log ditulis ke direktori log yang dikonfigurasi dan dapat dilampirkan saat melaporkan masalah.

## 🛠️ Stack Teknologi & Performa

- **Inti**: diimplementasikan dalam Rust untuk keamanan memori dan abstraksi tanpa biaya.
- **Framework UI**: Slint, toolkit UI modern yang dipercepat GPU (backend: `winit`).
- **Async runtime**: Tokio untuk perintah sistem dan I/O yang sangat konkuren dan non-blocking.
- **Performa**:
  - **Penggunaan memori**: biasanya sekitar 60–80 MB RAM.
  - **Responsivitas**: startup hampir instan dan pembaruan status WSL real-time menggunakan streaming.
  - **Ukuran binary**: rakitan rilis yang dioptimalkan menghasilkan executable tunggal yang ringkas.

## 🌍 Dukungan Bahasa

Dukungan internasionalisasi penuh disediakan untuk bahasa berikut:

| Bahasa | Kode | Emoji |
| :--- | :---: | :---: |
| Tionghoa Sederhana | `zh-CN` | 🇨🇳 |
| Tionghoa Tradisional | `zh-TW` | 🇭🇰 / 🇹🇼 |
| Inggris | `en` | 🇺🇸 |
| Jepang | `ja` | 🇯🇵 |
| Prancis | `fr` | 🇫🇷 |
| Spanyol | `es` | 🇪🇸 |
| Rusia | `ru` | 🇷🇺 |
| Portugis | `pt` | 🇵🇹 |
| Jerman | `de` | 🇩🇪 |
| Italia | `it` | 🇮🇹 |
| Turki | `tr` | 🇹🇷 |
| Bahasa Indonesia | `id` | 🇮🇩 |
| Hindi | `hi` | 🇮🇳 |
| Bengali | `bn` | 🇧🇩 |

## 📄 Lisensi

Proyek ini dilisensikan di bawah GPL-3.0 – lihat file [LICENSE](LICENSE) untuk detailnya.

---

Built with ❤️ for the WSL Community.
