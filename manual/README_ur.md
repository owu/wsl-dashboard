# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL ڈیش بورڈ لوگو" />
</p>

ایک جدید، اعلیٰ کارکردگی، ہلکا پھلکا، اور کم میموری والا WSL (Windows Subsystem for Linux) انسٹینس مینجمنٹ ڈیش بورڈ۔ پریمیم مقامی تجربے کے لیے Rust اور Slint کے ساتھ بنایا گیا ہے۔

```diff
اطلاع:

- WSL Dashboard Microsoft Store کے ذریعے تقسیم نہیں کیا جاتا۔
- وہاں "WSL Dashboard" نام سان درج کوئی بھی ایپلیکیشن غیر مستند ہے اور جعلی ہو سکتی ہے۔
- ممکنہ دھوکے سے بچنے کے لیے براہ کرم اسے ڈاؤن لوڈ نہ کریں۔
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

README I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | اردو | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 فہرست مضامین
[🌍 معاون زبانیں](#-معاون-زبانیں) | [🚀 اہم خصوصیات اور استعمال](#-اہم-خصوصیات-اور-استعمال) | [🖼️ اسکرین شاٹس](#️-اسکرین-شاٹس) | [📦 انسٹالیشن گائیڈ](#-انسٹالیشن-گائیڈ) | [📢 سرکاری کمیونٹی گروپس](#-سرکاری-کمیونٹی-گروپس) | [❤️ اس پروجیکٹ کی حمایت کریں](#️-اس-پروجیکٹ-کی-حمایت-کریں) | [⭐️ محبت کی محنت](#️-محبت-کی-محنت) | [📄 لائسنس](#-لائسنس)

---

## 🌍 معاون زبانیں

انگریزی، سادہ چینی، روایتی چینی، ہندی، ہسپانوی، فرانسیسی، عربی، بنگالی، پرتگالی، روسی، اردو، انڈونیشیائی، جرمن، جاپانی، ترکی، کوریائی، اطالوی، ڈچ، سویڈش، چیک، یونانی، ہنگری، عبرانی، نارویجن، ڈینش، فننش، سلوواک، سلووینین، آئس لینڈک، ویتنامی، تلوگو، جاوانی، تھائی، تامل، فلپائنی، پنجابی، ملائی، پولش، یوکرینی، فارسی، کنڑا، مراٹھی، ہاؤسا، برمازی، ازبک، آذربائیجانی، سیبوانو، ملایالم، سندھی، امہاری۔

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
  <img src="../assets/flags/vn.svg" width="32" title="ویتنامی" alt="ویتنامی" />
  <img src="../assets/flags/in.svg" width="32" title="تلوگو" alt="تلوگو" />
  <img src="../assets/flags/id.svg" width="32" title="جاوانی" alt="جاوانی" />
  <img src="../assets/flags/th.svg" width="32" title="تھائی" alt="تھائی" />
  <img src="../assets/flags/in.svg" width="32" title="تامل" alt="تامل" />
  <img src="../assets/flags/ph.svg" width="32" title="فلپائنی" alt="فلپائنی" />
  <img src="../assets/flags/pk.svg" width="32" title="پنجابی" alt="پنجابی" />
  <img src="../assets/flags/my.svg" width="32" title="ملائی" alt="ملائی" />
  <img src="../assets/flags/pl.svg" width="32" title="پولش" alt="پولش" />
  <img src="../assets/flags/ua.svg" width="32" title="یوکرینی" alt="یوکرینی" />
  <img src="../assets/flags/ir.svg" width="32" title="فارسی" alt="فارسی" />
  <img src="../assets/flags/in.svg" width="32" title="کنڑا" alt="کنڑا" />
  <img src="../assets/flags/in.svg" width="32" title="مراٹھی" alt="مراٹھی" />
  <img src="../assets/flags/ng.svg" width="32" title="ہاؤسا" alt="ہاؤسا" />
  <img src="../assets/flags/mm.svg" width="32" title="برمازی" alt="برمازی" />
  <img src="../assets/flags/uz.svg" width="32" title="ازبک" alt="ازبک" />
  <img src="../assets/flags/az.svg" width="32" title="آذربائیجانی" alt="آذربائیجانی" />
  <img src="../assets/flags/ph.svg" width="32" title="سیبوانو" alt="سیبوانو" />
  <img src="../assets/flags/in.svg" width="32" title="ملایالم" alt="ملایالم" />
  <img src="../assets/flags/pk.svg" width="32" title="سندھی" alt="سندھی" />
  <img src="../assets/flags/et.svg" width="32" title="امہاری" alt="امہاری" />
</p>

## 🚀 اہم خصوصیات اور استعمال

- **جدید نیٹو UI**: ڈارک/لائٹ موڈ سپورٹ، ہموار اینیمیشنز، اور **Skia** کے ذریعے چلنے والی اعلی کارکردگی والی رینڈرنگ کے ساتھ بدیہی GUI۔
- **سسٹم ٹرے انٹیگریشن**: سسٹم ٹرے میں منیمائز کرنے کی مکمل سپورٹ (~10MB RAM کا استعمال)، ٹوگل کرنے کے لیے ڈبل کلک، اور ایک فعال رائٹ کلک مینو۔
- **ذہین اسٹارٹ اپ**: ڈیش بورڈ کو ونڈوز کے ساتھ شروع ہونے، ٹرے میں منیمائز ہونے (سائلنٹ موڈ `/silent` کے ساتھ)، اور باہر نکلنے پر ڈسٹری بیوشنز کو خودکار طور پر بند کرنے کے لیے ترتیب دیں۔
- **ٹاسک شیڈیولنگ**: لچکدار ٹاسک شیڈیولنگ کا انتظام، جو WSL انسٹنسز میں مخصوص اوقات یا وقفوں پر اسکرپٹس اور کمانڈز کے خودکار عمل کو سپورٹ کرتا ہے۔
- **جامع انسٹنس کنٹرول**: ایک کلک سے اسٹارٹ، اسٹاپ، ٹرمینیٹ اور ان رجسٹر کریں۔ ریئل ٹائم اسٹیٹس مانیٹرنگ اور ڈسک کے استعمال اور فائل کے مقامات کے بارے میں تفصیلی معلومات۔
- **ڈسٹرو مینجمنٹ**: بطور ڈیفالٹ سیٹ کریں، مائیگریشن (VHDX کو دوسری ڈرائیوز میں منتقل کریں)، اور `.tar` یا `.tar.gz` آرکائیوز میں ایکسپورٹ/کلون کریں۔
- **ڈسک ماؤنٹ**: WSL انسٹنسز کے VHDX ورچوئل ڈسکس کو براہ راست ماؤنٹ اور منظم کریں، ماؤنٹ، ان ماؤنٹ اور ڈسک مینجمنٹ آپریشنز کو سپورٹ کرتا ہے، جس سے میزبان اور WSL کے درمیان فائل کی منتقلی آسان ہوتی ہے۔
- **فوری انٹیگریشن**: حسب ضرورت ورکنگ ڈائرکٹریز اور اسٹارٹ اپ اسکرپٹ ہکس کے ساتھ ٹرمینل، VS Code، یا فائل ایکسپلورر میں فوری لانچ۔
- **ڈسٹرو انسٹالیشن**: مائیکروسافٹ اسٹور، GitHub، مقامی فائلیں (RootFS/VHDX)، یا آن لائن میررز کے ذریعے لینکس ڈسٹری بیوشنز انسٹال کریں (آٹو سپیڈ ٹیسٹ کے ساتھ تیز ترین میرر منتخب کریں اور بلٹ ان RootFS ڈاؤن لوڈ مددگار)۔
- **عالمی تحفظ**: محفوظ کنکرنٹ مائیگریشن/بیک اپ آپریشنز کے لیے Mutex لاکس اور ہٹانے پر خودکار Appx صفائی۔
- **انتہائی کم میموری کا استعمال**: کارکردگی کے لیے انتہائی بہتر بنایا گیا ہے۔ سائلنٹ اسٹارٹ اپ (سسٹم ٹرے) صرف **~10MB** RAM استعمال کرتا ہے۔ ونڈوز موڈ کا استعمال فونٹ کی پیچیدگی کے لحاظ سے مختلف ہوتا ہے: معیاری زبانوں کے لیے **~18MB** اور بڑے کریکٹر سیٹ والی زبانوں (چینی، جاپانی، کوریائی وغیرہ) کے لیے **~38MB**۔
- **اعلی درجے کی نیٹ ورکنگ**: بغیر کسی رکاوٹ کے پورٹ فارورڈنگ کا انتظام (خودکار فائر وال اصول کی تخلیق کے ساتھ) اور متحد رابطے کے لیے عالمی HTTP پراکسی کی تشکیل۔
- **USB ڈیوائس مینجمنٹ**: ڈیش بورڈ UI سے براہ راست آپ کے تمام WSL مثالوں میں مقامی USB آلات کو آسانی سے منسلک کرنے، جوڑنے اور ان کا انتظام کرنے کے لیے `usbipd-win` کے ساتھ مکمل انضمام۔
- **ترتیب اور لاگز**: ترتیبات میں ڈیفالٹ انسٹال فولڈر، لاگ فولڈر اور لیول، انٹرفیس زبان، ڈارک موڈ، اپڈیٹ چیک کی تعدد، آٹو اسٹارٹ اور بند ہونے کا رویہ، نیز سائیڈ بار کو حسبِ ضرورت ترتیب دیں۔
- **ڈسٹرو کو فعال رکھنا**: ایک کلک پر «اسٹارٹ» ڈسٹرو کو تقریباً 60 سیکنڈ بے کاری کے بعد بند ہونے کے بجائے چلتا رکھتا ہے؛ آپ «ونڈوز اسٹارٹ پر یہ ڈسٹرو شروع کریں» بھی فعال کر سکتے ہیں۔
- **گلوبل شارٹ کٹس**: مین ونڈو دکھانے/چھپانے یا کسی ڈسٹرو کا ٹرمینل فوراً کھولنے کے لیے گلوبل شارٹ کٹس مقرر کریں۔


## 🖼️ اسکرین شاٹس
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

ڈیش بورڈ کو عملی طور پر دیکھنا چاہتے ہیں؟ مکمل اسکرین شاٹ گیلری کے لیے [آفیشل ویب سائٹ](https://www.wslui.com/) ملاحظہ کریں۔

## 📦 انسٹالیشن گائیڈ

### آپشن 1: پروجیکٹ ویب سائٹ دیکھیں (تجویز کردہ)

ہم آفیشل ویب سائٹ سے ڈاؤن لوڈ کرنے کی تجویز کرتے ہیں، کیونکہ یہ ہموار تجربے کے لیے متعدد میرر لنکس فراہم کرتی ہے:

[ڈاؤن لوڈ صفحہ](https://www.wslui.com/download/) پر جائیں اور اپنے علاقے کے لیے موزوں میرر منتخب کریں۔

### آپشن 2: winget کے ذریعے انسٹال کریں

آپ Windows پیکیج مینیجر (winget) سے براہ راست WSLDashboard انسٹال کر سکتے ہیں، مانیکر یا مکمل پیکیج آئیڈینٹیفائر استعمال کرتے ہوئے:

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

> winget پیکیج آئیڈینٹیفائر `Owu.WSLDashboard` ہے اور مانیکر `wsl-dashboard` ہے (کیس-غیر حساس)۔ دونوں میں سے کوئی بھی کام کرے گا۔

مزید معلومات کے لیے، [WinGet کمیونٹی ریپوزٹری](https://github.com/microsoft/winget-pkgs/tree/master/manifests/o/Owu/WSLDashboard) دیکھیں۔

### آپشن 3: سورس سے بلڈ کریں

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
     .\build\portable\build.ps1 -Arch x64
     ```


## 📢 سرکاری کمیونٹی گروپس

WSL UI کے سرکاری کمیونٹی گروپس میں خوش آمدید! استعمال کے تجربات پر بات چیت کرنے، مسائل کی اطلاع دینے، تازہ ترین اپڈیٹس حاصل کرنے اور ڈویلپرز اور دیگر صارفین سے جڑنے کے لیے ہمارے چینلز میں شامل ہوں۔

<div align="center">

| عالمی گروپ (Telegram) | چینی گروپ (WeChat) |
| :---: | :---: |
| <img src="https://cdn1.wslui.com/static/images/group/group-telegram.png" width="240" alt="Telegram Group QR Code" /> | <img src="https://cdn1.wslui.com/static/images/group/group-wechat.png" width="240" alt="WeChat Group QR Code" /> |
| شامل ہونے کے لیے اسکین کریں | شامل ہونے کے لیے اسکین کریں |

</div>


## ❤️ اس پروجیکٹ کی حمایت کریں

- یہ پروجیکٹ GPL-3.0 اوپن سورس لائسنس کے تحت ہے اور تمام صارفین کے لیے مفت ہے۔
- فنکشنل ڈویلپمنٹ، روزانہ ٹیسٹنگ سے لے کر بگ فکس تک، تمام کام فارغ وقت میں کیا جاتا ہے۔ اوپن سورس کا راستہ اکیلے چلنا آسان نہیں — آپ کی تسلیم اور سپورٹ پروجیکٹ کو آگے بڑھنے کا حوصلہ دیتی ہے۔
- اگر اس ٹول نے واقعی آپ کی مدد کی ہے، تو براہ کرم ساتھ دیں۔ تمام عطیات سرور اخراجات، ورژن اپڈیٹس اور فیچر بہتری پر خرچ کیے جاتے ہیں، جو پروجیکٹ کو مسلسل اپڈیٹ اور مستحکم ترقی بنائے رکھتے ہیں۔
- ہر چھوٹی نیکی ستاروں کی روشنی کی ایک کرن ہے۔ آپ کی سمجھ اور سخاوت کا دوبارہ شکریہ！ [ہمارے عطیہ صفحے پر جائیں](https://www.wslui.com/donate/).


## ⭐️ محبت کی محنت

اگر آپ کو یہ پروجیکٹ کارآمد معلوم ہوا ہے، تو میں مشکور ہوں گا اگر آپ GitHub پر اسٹار چھوڑ سکیں۔ آپ کی تائید اسے وسیع تر سامعین تک پہنچانے میں مدد دیتی ہے اور اس کی تہہ دل سے تعریف کی جاتی ہے۔ یہی حوصلہ افزائی مجھے تعمیر جاری رکھنے کی ترغیب دیتی ہے۔


## 📄 لائسنس

یہ پروجیکٹ GPL-3.0 کے تحت لائسنس یافتہ ہے – تفصیلات کے لیے [LICENSE](../LICENSE) فائل دیکھیں۔
