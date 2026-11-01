# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

Müasir, yüksək performanslı, yüngül və aşağı yaddaş istehlakı ilə WSL (Windows Subsystem for Linux) instans idarəetmə paneli. Rust və Slint əsasında qurulmuş, əla nativ təcrübə təqdim edir.

```diff
Elan:

- WSL Dashboard Microsoft Store vasitəsilə paylanmır.
- Orada "WSL Dashboard" adı altında sadalanan hər hansı proqram icazəsizdir və saxta ola bilər.
- Zəhmət olmasa, mümkün fırıldaqçılıqlardan qaçmaq üçün onu yükləməyin.
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

README I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | Azərbaycan | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 Mündəricat
[🌍 Dil dəstəyi](#-dil-dəstəyi) | [🚀 Əsas xüsusiyyətlər və istifadə](#-əsas-xüsusiyyətlər-və-istifadə) | [🖼️ Proqram skrinşotları](#️-proqram-skrinşotları) | [📦 Quraşdırma bələdçisi](#-quraşdırma-bələdçisi) | [📢 Rəsmi icma qrupları](#-rəsmi-icma-qrupları) | [❤️ Bu layihəni dəstəkləyin](#️-bu-layihəni-dəstəkləyin) | [⭐️ Sevgi ilə çalışan layihə](#️-sevgi-ilə-çalışan-layihə) | [📄 Açıq mənbə lisenziyası](#-açıq-mənbə-lisenziyası)

---

## 🌍 Dil dəstəyi

İngiliscə, Sadələşdirilmiş Çincə, Ənənəvi Çincə, Hindi, İspan, Fransız, Ərəb, Benqal, Portuqal, Rus, Urdu, İndoneziya, Alman, Yapon, Türkcə, Koreya, İtalyan, Holland, İsveç, Çex, Yunan, Macar, İvrit, Norveç, Danimarka, Finlandiya, Slovak, Sloven, İsland, Vyetnam, Telugu, Cava, Tay, Tamil, Filipin, Pencap, Malay, Polyak, Ukrayna, Fars, Kannada, Marathi, Hausa, Myanmar, Özbək, Azərbaycanca, Sebuan, Malayalam, Sindhi, Amhar

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="İngiliscə" alt="İngiliscə" />
  <img src="../assets/flags/cn.svg" width="32" title="Sadələşdirilmiş Çincə" alt="Sadələşdirilmiş Çincə" />
  <img src="../assets/flags/tw.svg" width="32" title="Ənənəvi Çincə" alt="Ənənəvi Çincə" />
  <img src="../assets/flags/in.svg" width="32" title="Hindi" alt="Hindi" />
  <img src="../assets/flags/es.svg" width="32" title="İspan" alt="İspan" />
  <img src="../assets/flags/fr.svg" width="32" title="Fransız" alt="Fransız" />
  <img src="../assets/flags/sa.svg" width="32" title="Ərəb" alt="Ərəb" />
  <img src="../assets/flags/bd.svg" width="32" title="Benqal" alt="Benqal" />
  <img src="../assets/flags/pt.svg" width="32" title="Portuqal" alt="Portuqal" />
  <img src="../assets/flags/ru.svg" width="32" title="Rus" alt="Rus" />
  <img src="../assets/flags/pk.svg" width="32" title="Urdu" alt="Urdu" />
  <img src="../assets/flags/id.svg" width="32" title="İndoneziya" alt="İndoneziya" />
  <img src="../assets/flags/de.svg" width="32" title="Alman" alt="Alman" />
  <img src="../assets/flags/jp.svg" width="32" title="Yapon" alt="Yapon" />
  <img src="../assets/flags/tr.svg" width="32" title="Türkcə" alt="Türkcə" />
  <img src="../assets/flags/kr.svg" width="32" title="Koreya" alt="Koreya" />
  <img src="../assets/flags/it.svg" width="32" title="İtalyan" alt="İtalyan" />
  <img src="../assets/flags/nl.svg" width="32" title="Holland" alt="Holland" />
  <img src="../assets/flags/se.svg" width="32" title="İsveç" alt="İsveç" />
  <img src="../assets/flags/cz.svg" width="32" title="Çex" alt="Çex" />
  <img src="../assets/flags/gr.svg" width="32" title="Yunan" alt="Yunan" />
  <img src="../assets/flags/hu.svg" width="32" title="Macar" alt="Macar" />
  <img src="../assets/flags/il.svg" width="32" title="İvrit" alt="İvrit" />
  <img src="../assets/flags/no.svg" width="32" title="Norveç" alt="Norveç" />
  <img src="../assets/flags/dk.svg" width="32" title="Danimarka" alt="Danimarka" />
  <img src="../assets/flags/fi.svg" width="32" title="Finlandiya" alt="Finlandiya" />
  <img src="../assets/flags/sk.svg" width="32" title="Slovak" alt="Slovak" />
  <img src="../assets/flags/si.svg" width="32" title="Sloven" alt="Sloven" />
  <img src="../assets/flags/is.svg" width="32" title="İsland" alt="İsland" />
  <img src="../assets/flags/vn.svg" width="32" title="Vyetnam" alt="Vyetnam" />
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

## 🚀 Əsas xüsusiyyətlər və istifadə

- **Müasir nativ UI**: **Skia** ilə yüksək performanslı renderləmə, qaranlıq/işıqlı rejim, hamar animasiyalar ilə intuitiv GUI.
- **Sistem trey inteqrasiyası**: Tam trey dəstəyi (~10MB yaddaş istifadəsi), cüt klik ilə göstər/gizlə dəyişdirmə və tam funksional sağ klik menyusu.
- **Ağıllı başlanğıc**: Avtomatik başlanğıc, treyə minimizasiya (`/silent` arqumenti ilə səssiz işə salma), çıxışda avtomatik bağlama.
- **Tapşırıqların planlaşdırılması**: Çevik tapşırıqların planlaşdırılması idarəetməsi, avtomatlaşdırılmış əməliyyatlar üçün WSL instanslarında müəyyən edilmiş vaxtda və ya intervalda skript və əmrlərin avtomatik icrasını dəstəkləyir.
- **Tam instans nəzarəti**: Bir kliklə başlat, dayandır, sonlandır və qeydiyyatdan sil. Canlı vəziyyət monitorinqi, disk istifadəsi və fayl yerləri haqqında ətraflı məlumat.
- **Dağıtım idarəetməsi**: Defolt olaraq təyin et, fiziki miqrasiya (VHDX digər diskə köçürmə), `.tar` və ya `.tar.gz` arxivi kimi ixrac/klonlama.
- **Disk bağlama**: WSL instansı VHDX virtual disklərini birbaşa bağlamaq və idarə etmək, mount, unmount və disk idarəetmə əməliyyatlarını dəstəkləyir, host və WSL arasında fayl ötürülməsini asanlaşdırır.
- **Sürətli inteqrasiya**: Terminal, VS Code və ya fayl araşdırıcısına bir kliklə giriş, fərdi işçi kataloq və skript qarmaqları dəstəyi.
- **Distribusiya quraşdırması**: Microsoft Store, GitHub, lokal fayllar (RootFS/VHDX) və ya onlayn güzgülər vasitəsilə Linux distribusiyalarını quraşdırın (ən sürətli güzgünü seçmək üçün avtomatik sürət testi və daxili RootFS yükləmə köməkçisi ilə).
- **Qlobal təhlükəsizlik**: Eyni vaxtda miqrasiya/yedəkləmə əməliyyatlarının təhlükəsizliyi üçün mutex kilidləri, silinərkən Appx paketlərinin avtomatik təmizlənməsi.
- **Çox aşağı yaddaş istifadəsi**: Yüksək optimizasiya edilmiş resurs səmərəliliyi. Səssiz başlanğıc (sistem trey) təxminən **10MB** yaddaş. Pəncərə rejimində şrift mürəkkəbliyinə görə təxminən **18MB** (İngiliscə, Almanca kimi standart dillər) ilə **38MB** (Çin-Yapon-Koreya kimi böyük simvol dəstləri) arası.
- **Geniş şəbəkə idarəetməsi**: Port yönləndirmə (avtomatik firewall qaydaları yaratma) və qlobal HTTP proksi konfiqurasiyasını problemsiz idarə edin, birləşdirilmiş bağlantı təcrübəsi əldə edin.
- **USB cihaz idarəetməsi**: `usbipd-win` ilə dərin inteqrasiya, panel UI-də birbaşa bütün WSL instanslarında lokal USB cihazlarını asanlıqla birləşdirmə, əlavə etmə və idarə etmə.
- **Konfiqurasiya və jurnallar**: Parametrlərdə standart quraşdırma qovluğunu, jurnal qovluğu və səviyyəsini, interfeys dilini, qaranlıq rejimi, yeniləmə yoxlama tezliyini, avtomatik başlatmanı və bağlanma davranışını, həmçinin yan paneli idarə edin.
- **Distronun aktiv saxlanması**: «Başlat» düyməsinə bir klik distronun ~60 saniyə boş qaldıqdan sonra bağlanması əvəzinə işləməyə davam etməsini təmin edir; «Windows başlayarkən bu distronu başlat» seçimini də aktivləşdirə bilərsiniz.
- **Qlobal qısayollar**: Əsas pəncərəni göstərmək/gizlətmək və ya seçilmiş distronun terminalını dərhal açmaq üçün qlobal qısayollar təyin edin.


## 🖼️ Proqram skrinşotları
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

Paneli iş üzərində görmək istəyirsiniz? Tam ekran şəkilləri qalereyası üçün [rəsmi veb-sayta](https://www.wslui.com/) baş çəkin.

## 📦 Quraşdırma bələdçisi

### Üsul 1: Layihənin vebsaytına daxil olun (Tövsiyə olunur)

Rəsmi vebsaytı ziyarət edərək yükləməyi tövsiyə edirik, çünki o, daha rahat təcrübə üçün çoxsaylı mirror linkləri təqdim edir:

[Yükləmə səhifəsinə](https://www.wslui.com/download/) keçin və regionunuz üçün uyğun miroru seçin.

### Üsul 2: winget vasitəsilə quraşdırın

Siz WSLDashboard'ı Windows Package Manager (winget) vasitəsilə birbaşa quraşdıra bilərsiniz, moniker və ya tam paket identifikatorundan istifadə edərək:

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

> winget paket identifikatoru `Owu.WSLDashboard` və moniker `wsl-dashboard` (case-insensitive)-dir. İkisi də işləyir.

Ətraflı məlumat üçün [WinGet icma repozitoriyasına](https://github.com/microsoft/winget-pkgs/tree/master/manifests/o/Owu/WSLDashboard) baş çəkin.

### Üsul 3: Mənbədən qurun

Rust alət zəncirinin (Rust 1.92+ və ya ən son versiya) quraşdırıldığından əmin olun.

1. Repozitoriyanı klonlayın:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. Qurun və işə salın:

   - Tərtibat debug:

     ```powershell
     cargo run
     ```
   - Optimizasiya edilmiş buraxılış versiyası üçün qurğu skriptini istifadə edin:

     > Qurğu skripti `x86_64-pc-windows-msvc` alət zənciri tələb edir.

     ```powershell
     .\build\portable\build.ps1 -Arch x64
     ```


## 📢 Rəsmi icma qrupları

WSL UI rəsmi icma qruplarına xoş gəldiniz! İstifadə məsləhətlərini müzakirə etmək, problemləri bildirilmək, son yeniləmələri almaq və tərtibatçılar ilə digər istifadəçilərlə əlaqə qurmaq üçün kanallarımıza qoşulun.

<div align="center">

| Qlobal Qrup (Telegram) | Çin Qrupu (WeChat) |
| :---: | :---: |
| <img src="https://cdn1.wslui.com/static/images/group/group-telegram.png" width="240" alt="Telegram Group QR Code" /> | <img src="https://cdn1.wslui.com/static/images/group/group-wechat.png" width="240" alt="WeChat Group QR Code" /> |
| Qoşulmaq üçün skaner edin | Qoşulmaq üçün skaner edin |

</div>


## ❤️ Bu layihəni dəstəkləyin

- Bu layihə GPL-3.0 açıq mənbə lisenziyası ilə yayımlanır, bütün istifadəçilər üçün pulsuzdur.
- Xüsusiyyət inkişafı, gündəlik testlərdən xəta düzəlişlərinə qədər bütün işlər boş vaxtdakı davamlı səydir. Açıq mənbə yolu tək getmək asan deyil, sizin tanınmanız və dəstəyiniz layihənin uzunmüddətli davam etməsi üçün gücdür.
- Bu alətin sizə həqiqətən faydalı olduğunu düşünürsünüzsə, lütfən əl uzadın. Bütün ianələr server xərcləri, versiya yeniləmələri və xüsusiyyət optimizasiyası üçün istifadə olunacaq.
- Kiçik bir mərhəmət ulduz kimi parlayır. Anlayışınız və səxavətiniz üçün bir daha təşəkkürlər! [İanə səhifəmizə daxil olun](https://www.wslui.com/donate/).


## ⭐️ Sevgi ilə çalışan layihə

Bu layihə sizə faydalı olduqda, GitHub-da bir ulduz verərək tanınmanızı göstərməyinizə minnətdar olaram. Sizin tanınmanız layihənin daha geniş istifadəçi kütləsinə çatmasına kömək edəcək. Bu təşviq məni davamlı olaraq irəli aparır.


## 📄 Açıq mənbə lisenziyası

Bu layihə GPL-3.0 lisenziyası altında lisenziyalaşdırılmışdır – ətraflı məlumat üçün [LICENSE](../LICENSE) faylına baxın.
