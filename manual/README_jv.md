# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

Dashboard manajemen instans WSL (Windows Subsystem for Linux) modern, berkinerja tinggi, ringan, lan entheng ing memori. Dibangun nganggo Rust lan Slint, nyedhiyakake pengalaman native sing paling apik.

---

```diff
Wigati:

- Piranti lunak iki ora disuwurake liwat Microsoft Store.
- Aplikasi kanthi jeneng sing padha ing kana yaiku piranti lunak bajakan sing ora sah.
- Aja ndownload, supaya ora ketipu.
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

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | Basa Jawa | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 Daftar Isi
- [🌍 Dhukungan Basa](#-dhukungan-basa)
- [🚀 Fitur Inti lan Panggunaan](#-fitur-inti-lan-panggunaan)
- [⚙️ Konfigurasi lan Log](#️-konfigurasi-lan-log)
- [🖼️ Screenshot Piranti Lunak](#️-screenshot-piranti-lunak)
- [🎬 Demo Operasi](#-demo-operasi)
- [💻 Syarat Sistem](#-syarat-sistem)
- [📦 Panduan Instalasi](#-panduan-instalasi)
- [🛠️ Tumpukan Teknologi lan Kinerja](#️-tumpukan-teknologi-lan-kinerja)
- [🤝 Dhukungan Komunitas](#-dhukungan-komunitas)
- [❤️ Dhukung Proyek Iki](#️-dhukung-proyek-iki)
- [⭐️ Kanggo Katresnan](#️-kanggo-katresnan)
- [📄 Lisensi Sumber Terbuka](#-lisensi-sumber-terbuka)

---

## 🌍 Dhukungan Bhasa

Inggris, Tionghoa Ringkes, Tionghoa Tradisional, Hindi, Spanyol, Prancis, Arab, Bengali, Portugis, Rusia, Urdu, Indonesia, Jerman, Jepang, Turki, Korea, Italia, Walanda, Swedia, Ceko, Yunani, Hongaria, Ibrani, Norwegia, Denmark, Finlandia, Slowakia, Slovenia, Islandia, Vietnam, Telugu, Jawa, Thai, Tamil, Filipino, Punjabi, Melayu, Poland, Ukraina, Persia, Kannada, Marathi, Hausa, Myanmar, Uzbek, Azerbaijan, Cebuano, Malayalam, Sindhi, Amharic

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="Inggris" alt="Inggris" />
  <img src="../assets/flags/cn.svg" width="32" title="Tionghoa Ringkes" alt="Tionghoa Ringkes" />
  <img src="../assets/flags/tw.svg" width="32" title="Tionghoa Tradisional" alt="Tionghoa Tradisional" />
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
  <img src="../assets/flags/nl.svg" width="32" title="Walanda" alt="Walanda" />
  <img src="../assets/flags/se.svg" width="32" title="Swedia" alt="Swedia" />
  <img src="../assets/flags/cz.svg" width="32" title="Ceko" alt="Ceko" />
  <img src="../assets/flags/gr.svg" width="32" title="Yunani" alt="Yunani" />
  <img src="../assets/flags/hu.svg" width="32" title="Hongaria" alt="Hongaria" />
  <img src="../assets/flags/il.svg" width="32" title="Ibrani" alt="Ibrani" />
  <img src="../assets/flags/no.svg" width="32" title="Norwegia" alt="Norwegia" />
  <img src="../assets/flags/dk.svg" width="32" title="Denmark" alt="Denmark" />
  <img src="../assets/flags/fi.svg" width="32" title="Finlandia" alt="Finlandia" />
  <img src="../assets/flags/sk.svg" width="32" title="Slowakia" alt="Slowakia" />
  <img src="../assets/flags/si.svg" width="32" title="Slovenia" alt="Slovenia" />
  <img src="../assets/flags/is.svg" width="32" title="Islandia" alt="Islandia" />
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


## 🚀 Fitur Inti lan Panggunaan

- **UI Native Modern**: GUI intuitif kanthi mode peteng/terang, animasi lancar, lan rendering berkinerja tinggi sing diprodhuksi dening **Skia**.
- **Integrasi System Tray**: Dhukungan tray lengkap (~10MB panggunaan memori), kanthi toggle tampilan/ndhelikake klik pindho lan menu klik-tengen sing fungsional.
- **Startup Pinter**: Dhukungan miwiti otomatis nalika boot, minimize menyang tray (nganggo parameter `/silent` kanggo miwiti tanpa swara), lan nutup distro kanthi otomatis nalika metu.
- **Kontrol Instans Lengkap**: Miwiti, mandeg, mateni, lan copot registrasi siji klik. Pemantauan status wektu nyata, ndeleng rinci panggunaan disk lan lokasi file.
- **Manajemen Distro**: Setel minangka default, migrasi fisik (pindhah VHDX menyang disk liyane), lan ekspor/kanggo arsip `.tar` utawa `.tar.gz`.
- **Integrasi Cepet**: Siji klik mlebu terminal, VS Code, utawa File Explorer, kanthi dhukungan direktori kerja khusus lan hook skrip startup.
- **Instalasi Distro**: Instal distribusi Linux liwat Microsoft Store, GitHub, file lokal (RootFS/VHDX), utawa mirror online (karo tes kecepatan otomatis kanggo milih mirror paling cepet lan asisten download RootFS ing-build).
- **Keamanan Global**: Nggunakake mutex kanggo njamin keamanan operasi migrasi/backup bebarengan, lan resik otomatis paket Appx nalika mbusak.
- **Panggunaan Memori Amat Cilik**: Efisiensi sumber daya sing dioptimalake banget. Miwiti tanpa swara (system tray) mung babagan **10MB** memori. Ing mode jendela, panggunaan gumantung kerumitan font: babagan **18MB** (basas standar kaya Inggris, Jerman lsp.) nganti **38MB** (set karakter gedhe kaya CJK).
- **Manajemen Jaringan Lanjut**: Ngatur forward port (nggawe aturan firewall kanthi otomatis) lan konfigurasi proxy HTTP global kanthi lancar, nyedhiyakake pengalaman koneksi sing terpadu.
- **Manajemen Piranti USB**: Integrasi jero karo `usbipd-win`, ngidini operasi bind, attach, lan manajemen piranti USB lokal kanthi gampang langsung ing UI dashboard kanggo kabeh instans WSL.


## ⚙️ Konfigurasi lan Log

Kabeh konfigurasi diatur liwat tampilan "Setelan":

- Pilih direktori instalasi default kanggo instans WSL anyar.
- Konfigurasi direktori log lan level log (Error / Warn / Info / Debug / Trace).
- Pilih basa UI utawa tindakake basa sistem.
- Ganti mode peteng, lan apa aplikasi kudu nutup WSL kanthi otomatis sawise operasi.
- Konfigurasi frekuensi priksa pembaruan (saben dina, saben minggu, saben rong minggu, saben wulan).
- Aktifake miwiti nalika boot (kanthi fitur perbaiki path otomatis).
- Setel minimize menyang tray nalika miwiti.
- Konfigurasi tombol tutup (minimize menyang tray tinimbang metu saka program).
- Kustomisasi sidebar kanthi ndhelikake/nduduhake tab fitur tartamtu.

File log bakal ditulis menyang direktori log sing wis dikonfigurasi, lan bisa dilampirake nalika nglaporake masalah.


## 🖼️ Screenshot Piranti Lunak

### Antarmuka Utama (Mode Peteng & Terang)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB & Lipet Menu
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### Manajemen Jaringan
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### Tambah Instans & Setelan
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>
<p align="center">
  <img src="../assets/screenshot/settings-advanced.png" width="48%" />
  <img src="../assets/screenshot/settings-interface.png" width="48%" />
</p>

### Babagan & Sumbangan
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/donate.png" width="48%" />
</p>

## 🎬 Demo Operasi

[Bantu kita nambah! Tonton video perkenalan kita lan wenehana panemu.](https://github.com/owu/wsl-dashboard/discussions/9)



## 💻 Syarat Sistem

- Windows 10 utawa Windows 11 kanthi WSL diaktifake (disaranake nggunakake WSL 2).
- Setidaknya siji distro WSL wis dipasang, utawa duwe hak kanggo nginstal distro anyar.
- CPU 64-bit; disaranake 4 GB utawa luwih RAM kanggo panggunaan multi-distro sing lancar.

## 📦 Panduan Instalasi

### Cara 1: Download Binary Pre-kompilasi

Cara paling gampang yaiku nggunakake versi sing wis dikompilasi:

1. Bukak kaca [GitHub Releases](https://github.com/owu/wsl-dashboard/releases).
2. Download file eksekusi `wsldashboard` paling anyar kanggo Windows.
3. Ekstrak (yen file kompres) lan jalur `wsldashboard.exe`.

Ora perlu instalasi, aplikasi iki minangka program portabel file tunggal.

### Cara 2: Bangun saka Sumber

Priksa manawa toolchain Rust wis dipasang (Rust 1.92+ utawa versi luwih anyar).

1. Klon repositori:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. Bangun lan jalur:

   - Debug pangembangan:

     ```powershell
     cargo run
     ```
   - Bangun versi rilis sing dioptimalake nganggo skrip bangunan:

     > Skrip bangunan mbutuhake toolchain `x86_64-pc-windows-msvc`.

     ```powershell
     .\build\portable\build.ps1
     ```


## 🛠️ Tumpukan Teknologi lan Kinerja

- **Inti**: Diimplementasikake nganggo Rust, njamin keamanan memori lan abstraksi biaya nol.
- **Framework UI**: Slint kanthi backend rendering **Skia** berkinerja tinggi.
- **Runtime Asinkron**: Tokio, kanggo perintah sistem lan I/O non-blocking.
- **Sorotan Kinerja**:
  - **Kacepetan Respons**: Miwiti sing cetha nyaris instan, lan pemantauan status WSL wektu nyata.
  - **Efisiensi Sumber Daya**: Panggunaan sumber daya sing amat sithik (deleng rinci ing [Fitur Inti](#-fitur-inti-lan-panggunaan)).
  - **Portabilitas**: Versi rilis sing dioptimalake ngasilake file eksekusi tunggal sing ringkas.



## 🤝 Dhukungan Komunitas

Matur nuwun sanget kanggo dhukungan komunitas ing ngisor iki:

- [Rust Programming Language](https://www.rust-lang.org) - Basa pamrograman sing kuat lan aman
- [Slint | Declarative GUI for Rust, C++, JavaScript & Python](https://slint.dev) - Framework UI modern
- [WSL: Windows Subsystem for Linux](https://github.com/microsoft/WSL) - Windows Subsystem for Linux sing apik banget
- [Tokio - An asynchronous Rust runtime](https://tokio.rs) - Runtime asinkron sing efisien
- [Windows Developer Community](https://developer.microsoft.com/en-us/windows/community) - Peningkatan platform terus-terusan
- [Reddit](https://www.reddit.com) - Diskusi lan dhukungan komunitas global
- [Hacker News](https://news.ycombinator.com) - Diskusi lan dhukungan komunitas global
- [Linux.do](https://linux.do) - Komunitas populer kanggo profesional IT
- [V2EX](https://www.v2ex.com) - Diskusi komunitas teknis Tionghoa

Kontribusi lan umpan balik sampeyan ndadekake proyek iki bisa!


## ❤️ Dhukung Proyek Iki

- Proyek iki nggunakake lisensi sumber terbuka GPL-3.0, gratis kanggo kabeh pangguna.
- Saka pangembangan fitur, tes saben dina, nganti ndandani masalah, kabeh digawe wektu luang. Dalan sumber terbuka ora gampang yen mlaku dewe, pengakuan lan dhukungan sampeyan minangka kekuwatan kanggo proyek iki terus maju.
- Yen sampeyan rumangsa alat iki mbantu, aja sungkan melu mbantu. Kabeh sumbangan bakal digunakake kanggo biaya server, iterasi versi, lan optimalisasi fitur, supaya proyek terus dianyari lan maju kanthi mantep.
- Kecil apik, dadi lintang. Matur nuwun maneh kanggo pangerten lan kermurahhatan sampeyan!

Kunjungi kaca sumbangan kita: [https://www.wslui.com/donate/](https://www.wslui.com/donate/)


## ⭐️ Kanggo Katresnan

Yen sampeyan rumangsa proyek iki mbantu, aku bakal rumangsa bungah yen sampeyan bisa menehi lintang ing GitHub. Pengakuan sampeyan bakal mbantu proyek nggayuh pangguna sing luwih akeh, lan aku ucapake matur nuwun. Motivasi iki sing nggawe aku terus maju.


## 📄 Lisensi Sumber Terbuka

Proyek iki dilisensi nganggo GPL-3.0 – deleng file [LICENSE](../LICENSE) kanggo rincian.


---

Built with ❤️ for the WSL Community.
