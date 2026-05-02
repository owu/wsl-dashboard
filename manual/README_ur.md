# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL ڈیش بورڈ لوگو" />
</p>

ایک جدید، اعلی کارکردگی، اور ہلکا پھلکا WSL (Windows Subsystem for Linux) انسٹنس مینجمنٹ ڈیش بورڈ۔ پریمیم نیٹو تجربے کے لیے Rust اور Slint کے ساتھ بنایا گیا ہے۔

---

```diff
نوٹس:

- یہ سافٹ ویئر Microsoft Store کے ذریعے تقسیم نہیں کیا جاتا۔
- وہاں ایک ہی نام کے تحت درج کوئی بھی ایپلی کیشن غیر مجاز ہے اور جعلی ہو سکتی ہے۔
- براہ کرم اسے ڈاؤنلوڈ نہ کریں تاکہ ممکنہ دھوکہ سے بچا جا سکے۔
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

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | اردو | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md)

---

## 📑 فہرست مضامین
- [🌍 معاون زبانیں](#-معاون-زبانیں)
- [🚀 اہم خصوصیات اور استعمال](#-اہم-خصوصیات-اور-استعمال)
- [⚙️ کنفیگریشن اور لاگز](#️-کنفیگریشن-اور-لاگز)
- [🖼️ اسکرین شاٹس](#️-اسکرین-شاٹس)
- [🎬 آپریشن ڈیمو](#-آپریشن-ڈیمو)
- [💻 سسٹم کی ضروریات](#-سسٹم-کی-ضروریات)
- [📦 انسٹالیشن گائیڈ](#-انسٹالیشن-گائیڈ)
- [🛠️ ٹیکنالوجی اور کارکردگی](#️-ٹیکنالوجی-اور-کارکردگی)
- [⭐️ محبت کی محنت](#️-محبت-کی-محنت)
- [📄 لائسنس](#-لائسنس)

---

## 🌍 معاون زبانیں

انگریزی، سادہ چینی، روایتی چینی، ہندی، ہسپانوی، فرانسیسی، عربی، بنگالی، پرتگالی، روسی، اردو، انڈونیشیائی، جرمن، جاپانی، ترکی، کوریائی، اطالوی، ڈچ، سویڈش، چیک، یونانی، ہنگری، عبرانی، نارویجن، ڈینش، فننش، سلوواک، سلووینین، آئس لینڈک۔

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="انگریزی" alt="انگریزی" />
  <img src="../assets/flags/cn.svg" width="32" title="آسان چینی" alt="آسان چینی" />
  <img src="../assets/flags/tw.svg" width="32" title="روایتی چینی" alt="روایتی چینی" />
  <img src="../assets/flags/in.svg" width="32" title="ہندی" alt="ہندی" />
  <img src="../assets/flags/es.svg" width="32" title="ہسپانوی" alt="ہسپانوی" />
  <img src="../assets/flags/fr.svg" width="32" title="فرانسیسی" alt="فرانسیسی" />
  <img src="../assets/flags/sa.svg" width="32" title="عربی" alt="عربی" />
  <img src="../assets/flags/bd.svg" width="32" title="بنگالی" alt="بنگالی" />
  <img src="../assets/flags/pt.svg" width="32" title="پرتگالی" alt="پرتگالی" />
  <img src="../assets/flags/ru.svg" width="32" title="روسی" alt="روسی" />
  <img src="../assets/flags/pk.svg" width="32" title="اردو" alt="اردو" />
  <img src="../assets/flags/id.svg" width="32" title="انڈونیشیائی" alt="انڈونیشیائی" />
  <img src="../assets/flags/de.svg" width="32" title="جرمن" alt="جرمن" />
  <img src="../assets/flags/jp.svg" width="32" title="جاپانی" alt="جاپانی" />
  <img src="../assets/flags/tr.svg" width="32" title="ترکی" alt="ترکی" />
  <img src="../assets/flags/kr.svg" width="32" title="کوریائی" alt="کوریائی" />
  <img src="../assets/flags/it.svg" width="32" title="اطالوی" alt="اطالوی" />
  <img src="../assets/flags/nl.svg" width="32" title="ڈچ" alt="ڈچ" />
  <img src="../assets/flags/se.svg" width="32" title="سویڈش" alt="سویڈش" />
  <img src="../assets/flags/cz.svg" width="32" title="چیک" alt="چیک" />
  <img src="../assets/flags/gr.svg" width="32" title="یونانی" alt="یونانی" />
  <img src="../assets/flags/hu.svg" width="32" title="ہنگری" alt="ہنگری" />
  <img src="../assets/flags/il.svg" width="32" title="عبرانی" alt="عبرانی" />
  <img src="../assets/flags/no.svg" width="32" title="نارویجن" alt="نارویجن" />
  <img src="../assets/flags/dk.svg" width="32" title="ڈنمارکی" alt="ڈنمارکی" />
  <img src="../assets/flags/fi.svg" width="32" title="فینش" alt="فینش" />
  <img src="../assets/flags/sk.svg" width="32" title="سلوواک" alt="سلوواک" />
  <img src="../assets/flags/si.svg" width="32" title="سلووینین" alt="سلووینین" />
  <img src="../assets/flags/is.svg" width="32" title="آئس لینڈی" alt="آئس لینڈی" />
</p>


## 🚀 اہم خصوصیات اور استعمال

- **جدید نیٹو UI**: ڈارک/لائٹ موڈ سپورٹ، ہموار اینیمیشنز، اور **Skia** کے ذریعے چلنے والی اعلی کارکردگی والی رینڈرنگ کے ساتھ بدیہی GUI۔
- **سسٹم ٹرے انٹیگریشن**: سسٹم ٹرے میں منیمائز کرنے کی مکمل سپورٹ (~10MB RAM کا استعمال)، ٹوگل کرنے کے لیے ڈبل کلک، اور ایک فعال رائٹ کلک مینو۔
- **ذہین اسٹارٹ اپ**: ڈیش بورڈ کو ونڈوز کے ساتھ شروع ہونے، ٹرے میں منیمائز ہونے (سائلنٹ موڈ `/silent` کے ساتھ)، اور باہر نکلنے پر ڈسٹری بیوشنز کو خودکار طور پر بند کرنے کے لیے ترتیب دیں۔
- **جامع انسٹنس کنٹرول**: ایک کلک سے اسٹارٹ، اسٹاپ، ٹرمینیٹ اور ان رجسٹر کریں۔ ریئل ٹائم اسٹیٹس مانیٹرنگ اور ڈسک کے استعمال اور فائل کے مقامات کے بارے میں تفصیلی معلومات۔
- **ڈسٹرو مینجمنٹ**: بطور ڈیفالٹ سیٹ کریں، مائیگریشن (VHDX کو دوسری ڈرائیوز میں منتقل کریں)، اور `.tar` یا `.tar.gz` آرکائیوز میں ایکسپورٹ/کلون کریں۔
- **فوری انٹیگریشن**: حسب ضرورت ورکنگ ڈائرکٹریز اور اسٹارٹ اپ اسکرپٹ ہکس کے ساتھ ٹرمینل، VS Code، یا فائل ایکسپلورر میں فوری لانچ۔
- **اسمارٹ انسٹالیشن**: مائیکروسافٹ اسٹور، GitHub، یا مقامی فائلوں (RootFS/VHDX) سے انسٹال کریں۔ اس میں بلٹ ان RootFS ڈاؤن لوڈ مددگار شامل ہے۔
- **عالمی تحفظ**: محفوظ کنکرنٹ مائیگریشن/بیک اپ آپریشنز کے لیے Mutex لاکس اور ہٹانے پر خودکار Appx صفائی۔
- **انتہائی کم میموری کا استعمال**: کارکردگی کے لیے انتہائی بہتر بنایا گیا ہے۔ سائلنٹ اسٹارٹ اپ (سسٹم ٹرے) صرف **~10MB** RAM استعمال کرتا ہے۔ ونڈوز موڈ کا استعمال فونٹ کی پیچیدگی کے لحاظ سے مختلف ہوتا ہے: معیاری زبانوں کے لیے **~18MB** اور بڑے کریکٹر سیٹ والی زبانوں (چینی، جاپانی، کوریائی وغیرہ) کے لیے **~38MB**۔
- **اعلی درجے کی نیٹ ورکنگ**: بغیر کسی رکاوٹ کے پورٹ فارورڈنگ کا انتظام (خودکار فائر وال اصول کی تخلیق کے ساتھ) اور متحد رابطے کے لیے عالمی HTTP پراکسی کی تشکیل۔
- **USB ڈیوائس مینجمنٹ**: ڈیش بورڈ UI سے براہ راست آپ کے تمام WSL مثالوں میں مقامی USB آلات کو آسانی سے منسلک کرنے، جوڑنے اور ان کا انتظام کرنے کے لیے `usbipd-win` کے ساتھ مکمل انضمام۔


## ⚙️ کنفیگریشن اور لاگز

تمام کنفیگریشن 'سیٹنگز' ویو کے ذریعے منظم کی جاتی ہیں:

- نئے WSL انسٹنس کے لیے ڈیفالٹ انسٹالیشن ڈائرکٹری منتخب کریں۔
- لاگ ڈائرکٹری اور لاگ لیول (Error / Warn / Info / Debug / Trace) سیٹ کریں۔
- UI کی زبان منتخب کریں یا اسے سسٹم کی زبان کے مطابق رہنے دیں۔
- ڈارک موڈ کو ٹوگل کریں اور یہ منتخب کریں کہ آیا ایپ آپریشنز کے بعد WSL کو خودکار طور پر بند کر سکتی ہے۔
- ترتیب دیں کہ ایپ کتنی بار اپ ڈیٹس چیک کرے (روزانہ، ہفتہ وار، پندرہ روزہ، ماہانہ)۔
- سسٹم بوٹ پر خودکار اسٹارٹ اپ فعال کریں (خودکار پاتھ مرمت کے ساتھ)۔
- اسٹارٹ اپ پر ایپ کو سسٹم ٹرے میں منیمائز کرنے کے لیے سیٹ کریں۔
- کلوز بٹن کو باہر نکلنے کے بجائے سسٹم ٹرے میں منیمائز کرنے کے لیے کنفیگر کریں۔
- مخصوص فیچر ٹیبز کی مرئیت کو ٹوگل کر کے سائیڈ بار کو اپنی مرضی کے مطابق بنائیں۔

لاگ فائلیں کنفیگر کردہ لاگ ڈائرکٹری میں لکھی جاتی ہیں اور مسائل کی اطلاع دیتے وقت منسلک کی جا سکتی ہیں۔


## 🖼️ اسکرین شاٹس

### ہوم (لائٹ اور ڈارک موڈ)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### یو ایس بی اور فولڈ ایبل مینو
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### نیٹ ورک
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### انسٹنس شامل کریں اور سیٹنگز
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>

### درخواست کے بارے میں
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
</p>

## 🎬 آپریشن ڈیمو

WSL ڈیش بورڈ کے عمل کا ڈیمو نیچے دیا گیا ہے:

![WSL ڈیش بورڈ ڈیمو](../assets/screenshot/demo.gif)



## 💻 سسٹم کی ضروریات

- ونڈوز 10 یا ونڈوز 11 جس میں WSL فعال ہو (WSL 2 تجویز کردہ)۔
- کم از کم ایک WSL ڈسٹری بیوشن انسٹال ہو، یا نئی ڈسٹری بیوشنز انسٹال کرنے کی اجازت ہو۔
- 64 بٹ CPU؛ متعدد ڈسٹروز کے ہموار استعمال کے لیے 4 GB RAM یا اس سے زیادہ تجویز کردہ۔

## 📦 انسٹالیشن گائیڈ

### آپشن 1: تیار شدہ بائنری ڈاؤن لوڈ کریں

شروع کرنے کا سب سے آسان طریقہ پہلے سے کمپائل شدہ ریلیز کا استعمال کرنا ہے:

1. [GitHub Releases](https://github.com/owu/wsl-dashboard/releases) صفحہ پر جائیں۔
2. ونڈوز کے لیے تازہ ترین `wsldashboard` ایگزیکیوٹیبل فائل ڈاؤن لوڈ کریں۔
3. فائل کو ایکسٹریکٹ کریں (اگر زپ ہے) اور `wsldashboard.exe` چلائیں۔

کسی انسٹالر کی ضرورت نہیں ہے؛ ایپ ایک واحد پورٹیبل بائنری ہے۔

### آپشن 2: سورس سے بلڈ کریں

یقینی بنائیں کہ آپ کے پاس Rust ٹول چین (Rust 1.92+ یا نیا) انسٹال ہے۔

1. ریپوزٹری کلون کریں:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. بلڈ اور رن کریں:

   - ڈویلپمنٹ کے لیے:

     ```powershell
     cargo run
     ```
   - بلڈ اسکرپٹ کا استعمال کرتے ہوئے بہترین ریلیز بلڈ بنائیں:

     > بلڈ اسکرپٹ کے لیے `x86_64-pc-windows-msvc` ٹول چین درکار ہے۔

     ```powershell
     .\build\scripts\build.ps1
     ```


## 🛠️ ٹیکنالوجی اور کارکردگی

- **کور**: میموری کی حفاظت اور زیرو-کاسٹ ایبسٹریکشنز کے لیے Rust میں لاگو کیا گیا۔
- **UI فریم ورک**: اعلی کارکردگی والے **Skia** رینڈرنگ بیک اینڈ کے ساتھ Slint۔
- **Async رن ٹائم**: نان بلاکنگ سسٹم کمانڈز اور I/O کے لیے Tokio۔
- **کارکردگی کی جھلکیاں**:
  - **ردعمل**: تقریباً فوری اسٹارٹ اپ اور ریئل ٹائم WSL اسٹیٹس مانیٹرنگ۔
  - **افادیت**: انتہائی کم وسائل کا استعمال (مزید تفصیلات کے لیے [اہم خصوصیات](#-اہم-خصوصیات-اور-استعمال) دیکھیں)۔
  - **پورٹیبلٹی**: بہتر ریلیز بلڈ ایک واحد کومپیکٹ ایگزیکیوٹیبل فائل تیار کرتا ہے۔



## ⭐️ محبت کی محنت

اگر آپ کو یہ پروجیکٹ کارآمد معلوم ہوا ہے، تو میں مشکور ہوں گا اگر آپ GitHub پر اسٹار چھوڑ سکیں۔ آپ کی تائید اسے وسیع تر سامعین تک پہنچانے میں مدد دیتی ہے اور اس کی تہہ دل سے تعریف کی جاتی ہے۔ یہی حوصلہ افزائی مجھے تعمیر جاری رکھنے کی ترغیب دیتی ہے۔

## 📄 لائسنس

یہ پروجیکٹ GPL-3.0 کے تحت لائسنس یافتہ ہے – تفصیلات کے لیے [LICENSE](../LICENSE) فائل دیکھیں۔

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

