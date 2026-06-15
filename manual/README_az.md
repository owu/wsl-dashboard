# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

Müasir, yüksək performanslı, yüngül və aşağı yaddaş istehlakı ilə WSL (Windows Subsystem for Linux) instans idarəetmə paneli. Rust və Slint əsasında qurulmuş, əla nativ təcrübə təqdim edir.

---

```diff
Qeyd:

- Bu proqram Microsoft Store vasitəsilə paylanmır.
- Burada eyni adlı tətbiqlər qeyri-qanuni pirat proqramlardır.
- Aldanmaqdan qorunmaq üçün lütfən yükləməyin.
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

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | Azərbaycan | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 Mündəricat
- [🌍 Dil dəstəyi](#-dil-dəstəyi)
- [🚀 Əsas xüsusiyyətlər və istifadə](#-əsas-xüsusiyyətlər-və-istifadə)
- [⚙️ Konfiqurasiya və loqlar](#️-konfiqurasiya-və-loqlar)
- [🖼️ Proqram skrinşotları](#️-proqram-skrinşotları)
- [🎬 Əməliyyat demosu](#-əməliyyat-demosu)
- [💻 Sistem tələbləri](#-sistem-tələbləri)
- [📦 Quraşdırma bələdçisi](#-quraşdırma-bələdçisi)
- [🛠️ Texnoloji stak və performans](#️-texnoloji-stak-və-performans)
- [🤝 İcmal dəstəyi](#-icmal-dəstəyi)
- [❤️ Bu layihəni dəstəkləyin](#️-bu-layihəni-dəstəkləyin)
- [⭐️ Sevgi ilə çalışan layihə](#️-sevgi-ile-çalışan-layihə)
- [📄 Açıq mənbə lisenziyası](#-açıq-mənbə-lisenziyası)

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
- **Tam instans nəzarəti**: Bir kliklə başlat, dayandır, sonlandır və qeydiyyatdan sil. Canlı vəziyyət monitorinqi, disk istifadəsi və fayl yerləri haqqında ətraflı məlumat.
- **Dağıtım idarəetməsi**: Defolt olaraq təyin et, fiziki miqrasiya (VHDX digər diskə köçürmə), `.tar` və ya `.tar.gz` arxivi kimi ixrac/klonlama.
- **Sürətli inteqrasiya**: Terminal, VS Code və ya fayl araşdırıcısına bir kliklə giriş, fərdi işçi kataloq və skript qarmaqları dəstəyi.
- **Distribusiya quraşdırması**: Microsoft Store, GitHub, lokal fayllar (RootFS/VHDX) və ya onlayn güzgülər vasitəsilə Linux distribusiyalarını quraşdırın (ən sürətli güzgünü seçmək üçün avtomatik sürət testi və daxili RootFS yükləmə köməkçisi ilə).
- **Qlobal təhlükəsizlik**: Eyni vaxtda miqrasiya/yedəkləmə əməliyyatlarının təhlükəsizliyi üçün mutex kilidləri, silinərkən Appx paketlərinin avtomatik təmizlənməsi.
- **Çox aşağı yaddaş istifadəsi**: Yüksək optimizasiya edilmiş resurs səmərəliliyi. Səssiz başlanğıc (sistem trey) təxminən **10MB** yaddaş. Pəncərə rejimində şrift mürəkkəbliyinə görə təxminən **18MB** (İngiliscə, Almanca kimi standart dillər) ilə **38MB** (Çin-Yapon-Koreya kimi böyük simvol dəstləri) arası.
- **Geniş şəbəkə idarəetməsi**: Port yönləndirmə (avtomatik firewall qaydaları yaratma) və qlobal HTTP proksi konfiqurasiyasını problemsiz idarə edin, birləşdirilmiş bağlantı təcrübəsi əldə edin.
- **USB cihaz idarəetməsi**: `usbipd-win` ilə dərin inteqrasiya, panel UI-də birbaşa bütün WSL instanslarında lokal USB cihazlarını asanlıqla birləşdirmə, əlavə etmə və idarə etmə.


## ⚙️ Konfiqurasiya və loqlar

Bütün konfiqurasiyalar "Ayarlar" görünüşü vasitəsilə idarə olunur:

- Yeni WSL instanslarının defolt quraşdırma kataloqunu seçin.
- Loq kataloqu və loq səviyyəsini konfiqurasiya edin (Error / Warn / Info / Debug / Trace).
- UI dilini seçin və ya sistem dilini izləyin.
- Qaranlıq rejimi dəyişdirin, əməliyyatdan sonra tətbiqin avtomatik bağlanmasını tənzimləyin.
- Yeniləmə yoxlama tezliyini konfiqurasiya edin (gündəlik, həftəlik, iki həftədən bir, aylıq).
- Başlanğıcda avtomatik açmanı aktivləşdirin (yol avtomatik düzəliş xüsusiyyəti ilə).
- Başlanğıcda treyə minimizasiyanı təyin edin.
- Bağla düyməsinin davranışını konfiqurasiya edin (proqramdan çıxmaq əvəzinə treyə minimizasiya edin).
- Müəyyən funksiya tablarının görünməsini dəyişdirərək yan paneli fərdiləşdirin.

Loq faylları konfiqurasiya edilmiş loq kataloqunda yazılır, problemlər bildirilərkən loqları əlavə etmək olar.


## 🖼️ Proqram skrinşotları

### Ana interfeys (qaranlıq & işıqlı rejim)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB & menyu yığılması
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### Şəbəkə idarəetməsi
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### Instans əlavə etmə & ayarlar
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>
<p align="center">
  <img src="../assets/screenshot/settings-advanced.png" width="48%" />
  <img src="../assets/screenshot/settings-interface.png" width="48%" />
</p>

### Haqqında & ianə
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/donate.png" width="48%" />
</p>

## 🎬 Əməliyyat demosu

[Bizi təkmilləşdirməyə kömək edin! Təqdimat videomuza baxın və fikirlərinizi paylaşın.](https://github.com/owu/wsl-dashboard/discussions/9)



## 💻 Sistem tələbləri

- WSL aktiv Windows 10 və ya Windows 11 (WSL 2 tövsiyə olunur).
- Ən azı bir WSL distributivi quraşdırılmış olmalıdır və ya yeni distributiv quraşdırmaq icazəsi olmalıdır.
- 64-bit CPU; çox-distributiv istifadəsi üçün 4 GB və ya daha çox RAM tövsiyə olunur.

## 📦 Quraşdırma bələdçisi

### Üsul 1: Əvvəlcədən hazırlanmış ikili faylları yükləyin

Ən asan yol hazır versiyanı istifadə etməkdir:

1. [GitHub Releases](https://github.com/owu/wsl-dashboard/releases) səhifəsinə keçin.
2. Windows üçün ən son `wsldashboard` icra olunan faylı yükləyin.
3. Sıxışmanı açın (sıxışdırılmış fayldırsa) və `wsldashboard.exe` faylını işə salın.

Quraşdırma tələb olunmur, bu tətbiq tək fayllı daşınan proqramdır.

### Üsul 2: Mənbədən qurun

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
      .\build\portable\build.ps1
      ```


## 🛠️ Texnoloji stak və performans

- **Nüvə**: Yaddaş təhlükəsizliyi və sıfır xərcli abstraksiyaları təmin etmək üçün Rust ilə həyata keçirilib.
- **UI Framework**: Yüksək performanslı **Skia** renderləmə backend-i olan Slint.
- **Asinxron Runtime**: Qeyri-bloklayan sistem əmrləri və I/O üçün Tokio.
- **Performans üstünlükləri**:
  - **Cavab sürəti**: Təxminən ani başlanğıc sürəti, WSL vəziyyətinin canlı monitorinqi.
  - **Resurs səmərəliliyi**: Çox aşağı resurs istifadəsi (ətraflı məlumat üçün [Əsas xüsusiyyətlər](#-əsas-xüsusiyyətlər-və-istifadə) bölməsinə baxın).
  - **Daşınabilirlik**: Optimizasiya edilmiş buraxılış versiyası kompakt icra olunan fayl yaradır.



## 🤝 İcmal dəstəyi

Bu icmaların dəstəyinə ürəkdən təşəkkür edirik:

- [Rust Programming Language](https://www.rust-lang.org) - Güclü və təhlükəsiz proqramlaşdırma dili
- [Slint | Declarative GUI for Rust, C++, JavaScript & Python](https://slint.dev) - Müasir UI framework
- [WSL: Windows Subsystem for Linux](https://github.com/microsoft/WSL) - Əla Windows Subsystem for Linux
- [Tokio - An asynchronous Rust runtime](https://tokio.rs) - Səmərəli asinxron runtime
- [Windows Developer Community](https://developer.microsoft.com/en-us/windows/community) - Davamlı platforma təkmilləşdirmələri
- [Reddit](https://www.reddit.com) - Qlobal icma müzakirəsi & dəstəyi
- [Hacker News](https://news.ycombinator.com) - Qlobal icma müzakirəsi & dəstəyi
- [Linux.do](https://linux.do) - IT mütəxəssisləri üçün populyar icma
- [V2EX](https://www.v2ex.com) - Çin texnoloji icma müzakirəsi

Sizin töhfəniz və rəyləriniz bu layihəni mümkün etdi!


## ❤️ Bu layihəni dəstəkləyin

- Bu layihə GPL-3.0 açıq mənbə lisenziyası ilə yayımlanır, bütün istifadəçilər üçün pulsuzdur.
- Xüsusiyyət inkişafı, gündəlik testlərdən xəta düzəlişlərinə qədər bütün işlər boş vaxtdakı davamlı səydir. Açıq mənbə yolu tək getmək asan deyil, sizin tanınmanız və dəstəyiniz layihənin uzunmüddətli davam etməsi üçün gücdür.
- Bu alətin sizə həqiqətən faydalı olduğunu düşünürsünüzsə, lütfən əl uzadın. Bütün ianələr server xərcləri, versiya yeniləmələri və xüsusiyyət optimizasiyası üçün istifadə olunacaq.
- Kiçik bir mərhəmət ulduz kimi parlayır. Anlayışınız və səxavətiniz üçün bir daha təşəkkürlər!

İanə səhifəmizə daxil olun: [https://www.wslui.com/donate/](https://www.wslui.com/donate/)


## ⭐️ Sevgi ilə çalışan layihə

Bu layihə sizə faydalı olduqda, GitHub-da bir ulduz verərək tanınmanızı göstərməyinizə minnətdar olaram. Sizin tanınmanız layihənin daha geniş istifadəçi kütləsinə çatmasına kömək edəcək. Bu təşviq məni davamlı olaraq irəli aparır.


## 📄 Açıq mənbə lisenziyası

Bu layihə GPL-3.0 lisenziyası altında lisenziyalaşdırılmışdır – ətraflı məlumat üçün [LICENSE](../LICENSE) faylına baxın.


---

Built with ❤️ for the WSL Community.