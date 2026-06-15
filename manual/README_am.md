# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

ዘመናዊ፣ ከፍተኛ አፈጻጸም፣ ቀላል እና ዝቅተኛ ማዕረግ ማስታወሻ ያለው WSL (Windows Subsystem for Linux) ተጠቃሚ ማናጀር ዳሽቦርድ። በ Rust እና Slint የተገነባ፣ ምርጥ ነባሪ ተሞክሮ ይሰጣል።

---

```diff`
ማሳሰቢያ:

- ይህ ሶፍትዌር በ Microsoft Store አይሰጠም።
- በዚያ ተመሳሳይ ስም ያላቸው መተግበሪያዎች ያልተፈቀዱ ሶፍትዌሮች ናቸው።
- አትውልዱ፣ እንዳይታለሉ።
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

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | አማርኛ

---

## 📑 ማውጫ
- [🌍 ቋንቋ ድጋፍ](#-ቋንቋ-ድጋፍ)
- [🚀 ዋና ባህሪያት እና አጠቃቀም](#-ዋና-ባህሪያት-እና-አጠቃቀም)
- [⚙️ ማዋቀር እና ምዝግብ](#️-ማዋቀር-እና-ምዝግብ)
- [🖼️ ሶፍትዌር ስክሪንሾቶች](#️-ሶፍትዌር-ስክሪንሾቶች)
- [🎬 ስራ ማሳያ](#-ስራ-ማሳያ)
- [💻 የስርዓት መስፈርቶች](#-የስርዓት-መስፈርቶች)
- [📦 መ拜师学 መመሪያ](#-መ拜师学-መመሪያ)
- [🛠️ ቴክ ስታክ እና አፈጻጸም](#️-ቴክ-ስታክ-እና-አፈጻጸም)
- [🤝 ማህበረሰብ ድጋፍ](#-ማህበረሰብ-ድጋፍ)
- [❤️ ይህን ፕሮጀክት ይደግፉ](#️-ይህን-ፕሮጀክት-ይደግፉ)
- [⭐️ ለፍቅር](#️-ለፍቅር)
- [📄 ክፍት ምንጭ ፈቃድ](#-ክፍት-ምንጭ-ፈቃድ)

---

## 🌍 ቋንቋ ድጋፍ

እንግሊዝኛ፣ ቀላል ቻይንኛ፣ ባህላዊ ቻይንኛ፣ ሂንዲ፣ ስፔኛ፣ ፈረንሳይኛ፣ ዓረብኛ፣ ቤንጋሊ፣ ፖርቱጋሊኛ፣ ሩሲያኛ፣ ኡርዱ፣ ኢንዶናዥኛ፣ ጀርመንኛ፣ ጃፓንኛ፣ ቱርክኛ፣ ኮሪያኛ፣ ጣሊያንኛ፣ ደች፣ ስዊድኛ፣ ቼክኛ፣ ግሪክኛ፣ ሃንጋሪኛ፣ እብራይስጥ፣ ኖርዌጂያንኛ፣ ዴኒሽ፣ ፊኒሽ፣ ስሎቫክኛ፣ ስሎቬንኛ፣ የአይስላንድኛ፣ ቬትናምኛ፣ ቴሉጉ፣ ጃቫኒስ፣ ታይኛ፣ ታሚል፣ ፊሊፒንኛ፣ ፓንጃቢኛ፣ ማላይኛ፣ ፖሊሽ፣ ዩክሬንኛ፣ ፋርሲ፣ ካናዳኛ፣ ማራሄኛ፣ ሃውሳ፣ ሜይንማር፣ ኡዝቤክኛ፣ አዘርባጃንኛ፣ ሴቡዋኖ፣ ማላያላምኛ፣ ሲንዲ፣ አምሃሪክ

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="እንግሊዝኛ" alt="እንግሊዝኛ" />
  <img src="../assets/flags/cn.svg" width="32" title="ቀላል ቻይንኛ" alt="ቀላል ቻይንኛ" />
  <img src="../assets/flags/tw.svg" width="32" title="ባህላዊ ቻይንኛ" alt="ባህላዊ ቻይንኛ" />
  <img src="../assets/flags/in.svg" width="32" title="ሂንዲ" alt="ሂንዲ" />
  <img src="../assets/flags/es.svg" width="32" title="ስፔኛ" alt="ስፔኛ" />
  <img src="../assets/flags/fr.svg" width="32" title="ፈረንሳይኛ" alt="ፈረንሳይኛ" />
  <img src="../assets/flags/sa.svg" width="32" title="ዓረብኛ" alt="ዓረብኛ" />
  <img src="../assets/flags/bd.svg" width="32" title="ቤንጋሊ" alt="ቤንጋሊ" />
  <img src="../assets/flags/pt.svg" width="32" title="ፖርቱጋሊኛ" alt="ፖርቱጋሊኛ" />
  <img src="../assets/flags/ru.svg" width="32" title="ሩሲያኛ" alt="ሩሲያኛ" />
  <img src="../assets/flags/pk.svg" width="32" title="ኡርዱ" alt="ኡርዱ" />
  <img src="../assets/flags/id.svg" width="32" title="ኢንዶናዥኛ" alt="ኢንዶናዥኛ" />
  <img src="../assets/flags/de.svg" width="32" title="ጀርመንኛ" alt="ጀርመንኛ" />
  <img src="../assets/flags/jp.svg" width="32" title="ጃፓንኛ" alt="ጃፓንኛ" />
  <img src="../assets/flags/tr.svg" width="32" title="ቱርክኛ" alt="ቱርክኛ" />
  <img src="../assets/flags/kr.svg" width="32" title="ኮሪያኛ" alt="ኮሪያኛ" />
  <img src="../assets/flags/it.svg" width="32" title="ጣሊያንኛ" alt="ጣሊያንኛ" />
  <img src="../assets/flags/nl.svg" width="32" title="ደች" alt="ደች" />
  <img src="../assets/flags/se.svg" width="32" title="ስዊድኛ" alt="ስዊድኛ" />
  <img src="../assets/flags/cz.svg" width="32" title="ቼክኛ" alt="ቼክኛ" />
  <img src="../assets/flags/gr.svg" width="32" title="ግሪክኛ" alt="ግሪክኛ" />
  <img src="../assets/flags/hu.svg" width="32" title="ሃንጋሪኛ" alt="ሃንጋሪኛ" />
  <img src="../assets/flags/il.svg" width="32" title="እብራይስጥ" alt="እብራይስጥ" />
  <img src="../assets/flags/no.svg" width="32" title="ኖርዌጂያንኛ" alt="ኖርዌጂያንኛ" />
  <img src="../assets/flags/dk.svg" width="32" title="ዴኒሽ" alt="ዴኒሽ" />
  <img src="../assets/flags/fi.svg" width="32" title="ፊኒሽ" alt="ፊኒሽ" />
  <img src="../assets/flags/sk.svg" width="32" title="ስሎቫክኛ" alt="ስሎቫክኛ" />
  <img src="../assets/flags/si.svg" width="32" title="ስሎቬንኛ" alt="ስሎቬንኛ" />
  <img src="../assets/flags/is.svg" width="32" title="የአይስላንድኛ" alt="የአይስላንድኛ" />
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


## 🚀 ዋና ባህሪያት እና አጠቃቀም

- **ዘመናዊ ነባሪ UI**: ጥቁር/ብሩህ ሁኔታ፣ ለስላሳ አኒሜሽን ያለው ተራ መስክ GUI፣ እና በ **Skia** የሚሰራ ከፍተኛ አፈጻጸም ማሳየት።
- **ስርዓት ትራይ ውህደት**: ሙሉ ድጋፍ ትራይ (~10MB ማዕረግ ጥቅም)፣ በድርብ ጠቅታ ማሳየት/መደበቅ ተቀያያሪ እና ሙሉ ተግባራዊ ቀኝ ጠቅታ ዝርዝር።
- **ብልህ ማስጀመሪያ**: ቡት ላይ ራስ-ጀማሪ ድጋፍ፣ ወደ ትራይ ማሳነሻ (ዝምታ ማስጀመሪያ `/silent` ተለዋዋጭ)፣ እና ሲወጣ ዲስትሮ በራስ-ሰር መዝጋት።
- **ሙሉ ተጠቃሚ ቁጥጥር**: በአንድ ጠቅታ ማስጀመሪያ፣ ማቆሚያ፣ ማብቂያ እና ምዝገባ ማስወገጃ። በእውነተኛ ጊዜ ሁኔታ መከታተያ፣ ዲስክ ጥቅም እና ፋይል ቦታ መመልከቻ።
- **ዲስትሮ አስተዳደር**: ነባር ማድረግ፣ አካላዊ ማዘዋወር (VHDX ወደ ሌላ ዲስክ ማንቀሳቀስ)፣ እና ኤክስፖርት/ቅጂ `.tar` ወይም `.tar.gz` በመሆን።
- **ፈጣን ውህደት**: በአንድ ጠቅታ ተርሚናል፣ VS Code፣ ወይም ፋይል መዳሰሻ መክፈቻ፣ በተለይ የሥራ ማውጫ እና ማስጀመሪያ ስክሪፕት ማሰሪያ ድጋፍ።
- **ዲስትሪቢዩሽን መ拜师学**: Linux ዲስትሪቢዩሽኖችን በMicrosoft Store፣ GitHub፣ አካባቢ ፋይሎች (RootFS/VHDX) ወይም የመስመር ላይ ምrors ያጫኑ (በአውቶማቲክ ፍጥነት ምርመራ ፈጣን ምror ለመምረጥ እና የተገነባ RootFS ማውረጃ ረዳት ያለው)።
- **ዓለም አቀፍ ደህንነት**: ተመሳሳይ ማዘዋወር/ማስቀመጥ ድርጅቶችን ለማረጋገጥ mutex ጥቅም፣ እና ሲያስወግዱ Appx ጥቅሎችን በራስ-ሰር ማጽዳ።
- **እጅጉን ዝቅተኛ ማዕረግ ጥቅም**: ከፍተኛ ጥቅም ላይ ያሉ ሀብቶች። ዝምታ ማስጀመሪያ (ስርዓት ትራይ) **10MB** ማዕረግ ብቻ። በመስኮት ሁኔታ ፊደል ውስብስብነት ላይ ተመስርቶ: **18MB** (መደበኛ ቋንቋዎች እንደ እንግሊዝኛ፣ ጀርመንኛ ወዘተ) እስከ **38MB** (ትልልቅ ቁምፊ ስብስብ እንደ CJK)።
- **የላቀ አውታር አስተዳደር**: ፖርት ማስተላለፊያ (ራስ-ሰር firewall ህግ ፈጠራ) እና ዓለም አቀፍ HTTP ፕሮክሲ ማዋቀርን ሳይቋሽሽ ማስተዳድር።
- **USB መሣሪያ አስተዳደር**: ከ `usbipd-win` ጋር ጥልቅ ውህደት፣ ሁሉንም WSL ተጠቃሚዎች በዳሽቦርድ UI ውስጥ የአካባቢ USB መሣሪያዎችን bind፣ attach እና አስተዳደር ድርጅቶችን በቀጥታ መፈጸም።


## ⚙️ ማዋቀር እና ምዝግብ

ሁሉም ማዋቀር በ "ቅንብሮች" ትዕይንት ይተዳደራል:

- ለአዲስ WSL ተጠቃሚዎች ነባር መ拜师学 ማውጫ ይምረጡ።
- የምዝግብ ማውጫ እና የምዝግብ ደረጃ (Error / Warn / Info / Debug / Trace) ያዋቅሩ።
- UI ቋንቋ ይምረጡ ወይም የስርዓት ቋንቋን ይከተሉ።
- ጥቁር ሁኔታን ይቀይሩ፣ እና አፕ ከስራ በኋላ WSL በራስ-ሰር ይዘጋ እንደሆን።
- የማዘመኛ ማረጋገጫ ድግግሞሽን ያዋቅሩ (በየቀኑ፣ በየሳምንቱ፣ በየሁለት ሳምንት፣ በየወሩ)።
- ቡት ላይ ራስ-ጀማሪን ያብሩ (ራስ-ሰር መንገድ ማረም ባህሪ ያለው)።
- ሲጀምር ወደ ትራይ ማሳነሻን ያዘጋጁ።
- የመዝጊያ ቁልፍ ባህሪን ያዋቅሩ (ከፕሮግራሙ መውጣት ይልቅ ወደ ትራይ ማሳነሻ)።
- ስለ ስለተለይ የባህሪ ትሮች ታይነትን በመቀያየር ጎን አሞሌን ያብጁ።

የምዝግብ ፋይሎች ወደ ተዋቀረ የምዝግብ ማውጫ ይጻፋሉ፣ እና ችግር ሲያሳውቁ ምዝግቦችን ማያያዥ ይችላሉ።


## 🖼️ ሶፍትዌር ስክሪንሾቶች

### ዋና ተግባር (ጥቁር & ብሩህ ሁኔታ)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB & ዝርዝር ማሳጠፊያ
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### አውታር አስተዳደር
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### ተጠቃሚ መጨመር & ቅንብሮች
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>
<p align="center">
  <img src="../assets/screenshot/settings-advanced.png" width="48%" />
  <img src="../assets/screenshot/settings-interface.png" width="48%" />
</p>

### ስለ & ስጦታ
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/donate.png" width="48%" />
</p>

## 🎬 ስራ ማሳያ

[ለማሻሻል ይረዱን! የእኛን መግቢያ ቪዲዮ ይመልከቱ እና ሀሳብዎን ያካፍሉ።](https://github.com/owu/wsl-dashboard/discussions/9)



## 💻 የስርዓት መስፈርቶች

- WSL ያስችለው Windows 10 ወይም Windows 11 (WSL 2 መጠቀም ይመከራል)።
- ቢያንስ አንድ WSL ዲስትሮ የተመሰበረ ወይም አዲስ ዲስትሮ ለመ拜师学 ፈቃድ ያለው።
- 64-bit CPU; ለምቹ multi-distro ጥቅም 4 GB ወይም ከዚያ በላይ RAM ይመከራል።

## 📦 መ拜师学 መመሪያ

### ዘዴ 1: ቅድመ-ተሰብሮ ብሄሪ ማውረጃ

ቀላሉ ዘዴ ተዘጋጅቶ ያለውን ስሪት መጠቀም ነው:

1. ወደ [GitHub Releases](https://github.com/owu/wsl-dashboard/releases) ገጽ ይሂዱ።
2. ለ Windows የቅርብ ጊዜውን `wsldashboard` ተጫዋች ያውርዱ።
3. (ግፊት ፋይል ከሆነ) ያውልዱ እና `wsldashboard.exe` ያሂዱ።

መ拜师学 አያስፈልግም፣ ይህ አፕ ነጠላ-ፋይል ተንቀሳቃሽ ፕሮግራም ነው።

### ዘዴ 2: ከምንጭ ግንባታ

Rust ቱልቻይን (Rust 1.92+ ወይም አዲስ ስሪት) ተመስብሮ መሆኑን ያረጋግጡ።

1. ማከማቻ ቅጂ ያድርጉ:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. ግንባታ እና ስራ:

   - ልማት ስህተት ማረሚያ:

     ```powershell
     cargo run
     ```
   - በግንባታ ስክሪፕት ጥቅም ላይ የዋለ ስሪት ግንባታ:

     > ግንባታ ስክሪፕቱ `x86_64-pc-windows-msvc` ቱልቻይን ይፈልጋል።

     ```powershell
     .\build\portable\build.ps1
     ```


## 🛠️ ቴክ ስታክ እና አፈጻጸም

- **ኬርነል**: በ Rust የተተገበረ፣ የማዕረግ ደህንነት እና ዜሮ-ወጪ ማጻፊያን ያረጋግጣል።
- **UI ግንባታ**: ከፍተኛ አፈጻጸም **Skia** ማሳያ ተመለስተኛ ያለው Slint።
- **ያልተገለጸ ሪን ተመን**: ለማይገለበጥ የስርዓት ትዕዛዞች እና I/O Tokio።
- **የአፈጻጸም ጉልህ ነጥቦች**:
  - **ምላሽ ፍጥነት**: ወዲያውኑ ማስጀመሪያ፣ እና በእውነተኛ ጊዜ WSL ሁኔታ መከታተያ።
  - **ሀብት ቁጠባ**: እጅጉን ዝቅተኛ ሀብት ጥቅም ([ዋና ባህሪያት](#-ዋና-ባህሪያት-እና-አጠቃቀም) ውስጥ ዝርዝር ይመልከቱ)።
  - **ተንቀሳቃሽነት**: ጥቅም ላይ የዋለ ስሪት ነጠላ ትንሽ ተጫዋች ፋይል ይፈጥራል።



## 🤝 ማህበረሰብ ድጋፍ

ከሚከተሉት ማህበረሰቦች ድጋፍ ለ በጣም አመሰግናለሁ:

- [Rust Programming Language](https://www.rust-lang.org) - ጠንካራ እና ደህንነቱ የተጠበቀ የፕሮግራም ቋንቋ
- [Slint | Declarative GUI for Rust, C++, JavaScript & Python](https://slint.dev) - ዘመናዊ UI ግንባታ
- [WSL: Windows Subsystem for Linux](https://github.com/microsoft/WSL) - ምርጥ Windows Subsystem for Linux
- [Tokio - An asynchronous Rust runtime](https://tokio.rs) - ቆንጆ ያልተገለጸ ሪን ተመን
- [Windows Developer Community](https://developer.microsoft.com/en-us/windows/community) - ቀጥታ የመድረክ ማሻሻያ
- [Reddit](https://www.reddit.com) - ዓለም አቀፍ የማህበረሰብ ውይይት እና ድጋፍ
- [Hacker News](https://news.ycombinator.com) - ዓለም አቀፍ የማህበረሰብ ውይይት እና ድጋፍ
- [Linux.do](https://linux.do) - ለ IT ባለሙያዎች ታዋቂ ማህበረሰብ
- [V2EX](https://www.v2ex.com) - የቻይና ቴክ ማህበረሰብ ውይይት

እርስዎ አስተዋጽኦ እና ግብረመልስ ይህንን ፕሮጀክት ያስችለዋል!


## ❤️ ይህን ፕሮጀክት ይደግፉ

- ይህ ፕሮጀክት GPL-3.0 ክፍት ምንጭ ፈቃድ ይጠቀማል፣ ለሁሉም ተጠቃሚዎች ነፃ።
- ከባህሪ ልማት፣ በየቀኑ ማሞከር እስከ ስህተት ማረም ድረስ ሁሉም ስራ ከነጻ ጊዜ ይመጣል። የክፍት ምንጭ መንገድ ብቻውን መሄድ አይቀላም፣ የእርስዎ አቀባበል እና ድጋፍ ፕሮጀክቱ ለረጅም ጊዜ እንዲቀጥል ጉልበት ነው።
- ይህ መሣሪያ እርስዎን በእርግጥ እንደረዳ ቢሰማዎ እባክዎን ይርዱ። ሁሉም ስጦታ ለሰርቨር ወጪ፣ የስሪት ማዘዋወር እና የባህሪ ማሻሻያ ይጠቀማል።
- ትንሽ ጸጋ፣ ከዋክብት ይሆናሉ። ለተረድኦትዎ እና ለልበ ስልጣኔዎ እንደገና አመሰግናለሁ!

የእኛን የስጦታ ገጽ ይጎብኙ: [https://www.wslui.com/donate/](https://www.wslui.com/donate/)


## ⭐️ ለፍቅር

ይህ ፕሮጀክት እርስዎን እንደረዳ ቢሰማዎ፣ በ GitHub ላይ ኮከብ መስጠት ደስ ይለኛል። የእርስዎ አቀባበል ፕሮጀክቱን ወደ ብዙ ተጠቃሚዎች እንዲደርስ ይረዳል። ይህ ማበረታቻ እኔን ቀጥል ያደርገኛል።


## 📄 ክፍት ምንጭ ፈቃድ

ይህ ፕሮጀክት በ GPL-3.0 ፈቃድ ስር ይፈቀዳል – ዝርዝር ለመመልከት [LICENSE](../LICENSE) ፋይል ይመልከቱ።


---

Built with ❤️ for the WSL Community.