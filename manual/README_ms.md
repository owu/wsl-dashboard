# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

Papan pemuka pengurusan instans WSL (Windows Subsystem for Linux) yang moden, berprestasi tinggi, ringan dan menggunakan sedikit memori. Dibina dengan Rust dan Slint untuk pengalaman natif yang cemerlang.

```diff
Pemberitahuan:

- WSL Dashboard tidak diedarkan melalui Microsoft Store.
- Sebarang aplikasi yang disenaraikan di sana dengan nama "WSL Dashboard" adalah tidak dibenarkan dan mungkin palsu.
- Sila jangan memuat turunnya untuk mengelakkan penipuan yang mungkin berlaku.
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

README I18N : [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | Bahasa Melayu | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 Kandungan
[🌍 Sokongan Bahasa](#-sokongan-bahasa) | [🚀 Ciri Utama & Penggunaan](#-ciri-utama--penggunaan) | [🖼️ Tangkapan Skrin](#️-tangkapan-skrin) | [📦 Panduan Pemasangan](#-panduan-pemasangan) | [📢 Kumpulan Komuniti Rasmi](#-kumpulan-komuniti-rasmi) | [❤️ Sokong Projek Ini](#️-sokong-projek-ini) | [⭐️ Sokongan Ikhlas](#️-sokongan-ikhlas) | [📄 Lesen Sumber Terbuka](#-lesen-sumber-terbuka)

---

## 🌍 Sokongan Bahasa

Bahasa Inggeris, Bahasa Cina Ringkas, Bahasa Cina Tradisional, Bahasa Hindi, Bahasa Sepanyol, Bahasa Perancis, Bahasa Arab, Bahasa Benggali, Bahasa Portugis, Bahasa Rusia, Bahasa Urdu, Bahasa Indonesia, Bahasa Jerman, Bahasa Jepun, Bahasa Turki, Bahasa Korea, Bahasa Itali, Bahasa Belanda, Bahasa Sweden, Bahasa Czech, Bahasa Greek, Bahasa Hungary, Bahasa Ibrani, Bahasa Norway, Bahasa Denmark, Bahasa Finland, Bahasa Slovak, Bahasa Slovenia, Bahasa Iceland, Bahasa Vietnam, Bahasa Telugu, Bahasa Jawa, Bahasa Thai, Bahasa Tamil, Bahasa Filipino, Bahasa Punjabi, Bahasa Melayu, Bahasa Poland, Bahasa Ukraine, Bahasa Parsi, Bahasa Kannada, Bahasa Marathi, Bahasa Hausa, Bahasa Myanmar, Bahasa Uzbek, Bahasa Azerbaijan, Bahasa Cebuano, Bahasa Malayalam, Bahasa Sindhi, Bahasa Amharic

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="Bahasa Inggeris" alt="Bahasa Inggeris" />
  <img src="../assets/flags/cn.svg" width="32" title="Bahasa Cina Ringkas" alt="Bahasa Cina Ringkas" />
  <img src="../assets/flags/tw.svg" width="32" title="Bahasa Cina Tradisional" alt="Bahasa Cina Tradisional" />
  <img src="../assets/flags/in.svg" width="32" title="Bahasa Hindi" alt="Bahasa Hindi" />
  <img src="../assets/flags/es.svg" width="32" title="Bahasa Sepanyol" alt="Bahasa Sepanyol" />
  <img src="../assets/flags/fr.svg" width="32" title="Bahasa Perancis" alt="Bahasa Perancis" />
  <img src="../assets/flags/sa.svg" width="32" title="Bahasa Arab" alt="Bahasa Arab" />
  <img src="../assets/flags/bd.svg" width="32" title="Bahasa Benggali" alt="Bahasa Benggali" />
  <img src="../assets/flags/pt.svg" width="32" title="Bahasa Portugis" alt="Bahasa Portugis" />
  <img src="../assets/flags/ru.svg" width="32" title="Bahasa Rusia" alt="Bahasa Rusia" />
  <img src="../assets/flags/pk.svg" width="32" title="Bahasa Urdu" alt="Bahasa Urdu" />
  <img src="../assets/flags/id.svg" width="32" title="Bahasa Indonesia" alt="Bahasa Indonesia" />
  <img src="../assets/flags/de.svg" width="32" title="Bahasa Jerman" alt="Bahasa Jerman" />
  <img src="../assets/flags/jp.svg" width="32" title="Bahasa Jepun" alt="Bahasa Jepun" />
  <img src="../assets/flags/tr.svg" width="32" title="Bahasa Turki" alt="Bahasa Turki" />
  <img src="../assets/flags/kr.svg" width="32" title="Bahasa Korea" alt="Bahasa Korea" />
  <img src="../assets/flags/it.svg" width="32" title="Bahasa Itali" alt="Bahasa Itali" />
  <img src="../assets/flags/nl.svg" width="32" title="Bahasa Belanda" alt="Bahasa Belanda" />
  <img src="../assets/flags/se.svg" width="32" title="Bahasa Sweden" alt="Bahasa Sweden" />
  <img src="../assets/flags/cz.svg" width="32" title="Bahasa Czech" alt="Bahasa Czech" />
  <img src="../assets/flags/gr.svg" width="32" title="Bahasa Greek" alt="Bahasa Greek" />
  <img src="../assets/flags/hu.svg" width="32" title="Bahasa Hungary" alt="Bahasa Hungary" />
  <img src="../assets/flags/il.svg" width="32" title="Bahasa Ibrani" alt="Bahasa Ibrani" />
  <img src="../assets/flags/no.svg" width="32" title="Bahasa Norway" alt="Bahasa Norway" />
  <img src="../assets/flags/dk.svg" width="32" title="Bahasa Denmark" alt="Bahasa Denmark" />
  <img src="../assets/flags/fi.svg" width="32" title="Bahasa Finland" alt="Bahasa Finland" />
  <img src="../assets/flags/sk.svg" width="32" title="Bahasa Slovak" alt="Bahasa Slovak" />
  <img src="../assets/flags/si.svg" width="32" title="Bahasa Slovenia" alt="Bahasa Slovenia" />
  <img src="../assets/flags/is.svg" width="32" title="Bahasa Iceland" alt="Bahasa Iceland" />
  <img src="../assets/flags/vn.svg" width="32" title="Bahasa Vietnam" alt="Bahasa Vietnam" />
  <img src="../assets/flags/in.svg" width="32" title="Telugu" alt="Telugu" />
  <img src="../assets/flags/id.svg" width="32" title="Javanese" alt="Javanese" />
  <img src="../assets/flags/th.svg" width="32" title="Bahasa Thai" alt="Bahasa Thai" />
  <img src="../assets/flags/in.svg" width="32" title="Tamil" alt="Tamil" />
  <img src="../assets/flags/ph.svg" width="32" title="Bahasa Filipino" alt="Bahasa Filipino" />
  <img src="../assets/flags/pk.svg" width="32" title="Punjabi" alt="Punjabi" />
  <img src="../assets/flags/my.svg" width="32" title="Bahasa Melayu" alt="Bahasa Melayu" />
  <img src="../assets/flags/pl.svg" width="32" title="Bahasa Poland" alt="Bahasa Poland" />
  <img src="../assets/flags/ua.svg" width="32" title="Bahasa Ukraine" alt="Bahasa Ukraine" />
  <img src="../assets/flags/ir.svg" width="32" title="Bahasa Parsi" alt="Bahasa Parsi" />
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

## 🚀 Ciri Utama & Penggunaan

- **Antaramuka Natif Moden**: GUI intuitif dengan sokongan mod gelap/terang, animasi lancar dan rendering berprestasi tinggi yang dikuasakan oleh **Skia**.
- **Integrasi Dulang Sistem**: Sokongan dulang penuh (~10MB penggunaan memori), dwi-klik untuk tunjuk/sembunyi dan menu klik kanan yang berfungsi sepenuhnya.
- **Permulaan Pintar**: Sokongan auto-mula, minimasi ke dulang (permulaan senyap dengan parameter `/silent`) dan penutupan automatik distro semasa keluar.
- **Penjadualan Tugas**: Pengurusan penjadualan tugas yang fleksibel, menyokong pelaksanaan automatik skrip dan arahan dalam instans WSL pada masa atau selang tertentu untuk operasi automatik.
- **Kawalan Instans Menyeluruh**: Mula, hentikan, paksa henti dan nyahdaftar dengan satu klik. Pemantauan status masa nyata, butiran penggunaan cakera dan lokasi fail.
- **Pengurusan Distro**: Tetapkan sebagai lalai, migrasi fizikal (pindah VHDX ke cakera lain) dan eksport/klon sebagai `.tar` atau `.tar.gz`.
- **Pemasangan Cakera**: Lekap dan uruskan cakera maya VHDX instans WSL secara langsung, menyokong operasi lekap, nyahlekap dan pengurusan cakera, memudahkan pemindahan fail antara hos dan WSL.
- **Integrasi Pantas**: Buka terminal, VS Code atau pengurus fail dengan satu klik, sokong direktori kerja tersuai dan skrip cangkuk pelancaran.
- **Pemasangan Distro**: Pasang distribusi Linux melalui Microsoft Store, GitHub, fail tempatan (RootFS/VHDX) atau cermin dalam talian (dengan ujian kelajuan automatik untuk memilih cermin terpantas dan alat bantu muat turun RootFS terbina dalam).
- **Keselamatan Global**: Mutex untuk keselamatan operasi migrasi/backup serentak dan pembersihan pakej Appx automatik semasa pembuangan.
- **Penggunaan Memori Sangat Rendah**: Kecekapan sumber yang dioptimumkan tinggi. Permulaan senyap (dulang sistem) hanya menggunakan kira-kira **10MB** memori. Dalam mod tetingkap, penggunaan adalah kira-kira **18MB** (bahasa standard seperti Inggeris, Jerman, dll.) hingga **38MB** (set aksara besar seperti CJK) bergantung pada kerumitan fon.
- **Pengurusan Rangkaian Lanjutan**: Pengurusan pengalihan port yang lancar (penciptaan peraturan dinding api automatik) dan konfigurasi proksi HTTP global untuk pengalaman penyambungan yang bersatu.
- **Pengurusan Peranti USB**: Integrasi mendalam dengan `usbipd-win`, membolehkan pengikatan, penyambungan dan pengurusan peranti USB tempatan dengan mudah untuk semua instans WSL terus dari UI papan pemuka.
- **Konfigurasi dan log**: Urus folder pemasangan lalai, folder dan tahap log, bahasa UI, mod gelap, kekerapan semakan kemas kini, autostart dan tingkah laku penutupan, serta bar sisi melalui Tetapan.
- **Menjaga distro kekal aktif**: Satu klik «Mula» mengekalkan distro berjalan dan bukannya dimatikan selepas ~60 saat melahu; anda juga boleh mendayakan «Mulakan distro ini apabila Windows bermula».
- **Pintasan global**: Tetapkan pintasan global untuk menunjukkan/menyembunyikan tetingkap utama atau membuka terminal distro dengan segera.


## 🖼️ Tangkapan Skrin
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

Mahu melihat papan pemuka beraksi? Lawati [laman web rasmi](https://www.wslui.com/) untuk galeri tangkapan skrin penuh.

## 📦 Panduan Pemasangan

### Kaedah 1: Lawati laman web projek (Disyorkan)

Kami mengesyorkan melawati laman web rasmi untuk memuat turun, kerana ia menawarkan berbilang pautan cermin untuk pengalaman yang lebih lancar:

Pergi ke [halaman Muat Turun](https://www.wslui.com/download/) dan pilih cermin yang sesuai untuk rantau anda.

### Kaedah 2: Pasang melalui winget

Anda boleh memasang WSLDashboard terus dari Windows Package Manager (winget), menggunakan moniker atau pengecam pakej penuh:

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

### Kaedah 3: Bina dari Kod Sumber

Pastikan anda telah memasang Rust toolchain (Rust 1.92+ atau lebih baharu).

1. Klon repositori:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. Bina dan jalankan:

   - Untuk pembangunan dan nyahpijat:

     ```powershell
     cargo run
     ```
   - Bina versi keluaran yang dioptimumkan menggunakan skrip binaan:

     > Skrip binaan memerlukan toolchain `x86_64-pc-windows-msvc`.

     ```powershell
     .\build\portable\build.ps1 -Arch x64
     ```


## 📢 Kumpulan Komuniti Rasmi

Selamat datang ke kumpulan komuniti WSL UI rasmi! Sertai saluran kami untuk membincangkan tip penggunaan, melaporkan isu, mendapatkan terkini update dan berhubung dengan pembangun serta pengguna lain.

<div align="center">

| Kumpulan Global (Telegram) | Kumpulan China (WeChat) |
| :---: | :---: |
| <img src="https://cdn1.wslui.com/static/images/group/group-telegram.png" width="240" alt="Telegram Group QR Code" /> | <img src="https://cdn1.wslui.com/static/images/group/group-wechat.png" width="240" alt="WeChat Group QR Code" /> |
| Imbas untuk menyertai | Imbas untuk menyertai |

</div>


## ❤️ Sokong Projek Ini

- Projek ini dilesenkan di bawah GPL-3.0 dan tersedia secara percuma untuk semua pengguna.
- Dari pembangunan ciri, pengujian harian hingga pembaikan pepijat — semua kerja dilakukan dalam masa lapang. Jalan sumber terbuka tidak mudah untuk ditempuh sendiri. Pengiktirafan dan sokongan anda memberi projek ini keyakinan untuk terus maju.
- Jika alat ini benar-benar membantu anda, sila pertimbangkan untuk memberi sokongan. Semua derma digunakan untuk kos pelayan, pembangunan versi dan penambahbaikan ciri, supaya projek sentiasa dikemas kini dan maju dengan stabil.
- Setiap kebaikan kecil adalah sinar bintang. Sekali lagi terima kasih atas kefahaman dan kemurahan hati anda! [Lawati halaman derma kami](https://www.wslui.com/donate/).


## ⭐️ Sokongan Ikhlas

Jika anda merasakan projek ini bermanfaat, saya amat berterima kasih jika anda dapat memberikan bintang di GitHub. Sokongan anda akan membantu projek menjangkau lebih ramai pengguna dan saya sangat menghargainya. Sokongan inilah yang mendorong saya untuk terus maju.


## 📄 Lesen Sumber Terbuka

Projek ini dilesenkan di bawah GPL-3.0 – lihat fail [LICENSE](../LICENSE) untuk butiran lanjut.
