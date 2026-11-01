# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

وڌيڪ جي سڃاتڻ لاءِ جديدي، بلندڪارائي، ڳڻپيءَ واري ۽ گهٽ ميموري واري WSL (Windows Subsystem for Linux) انسٽنس مينيجمينٽ ڊيش بورڊ. Rust ۽ Slint سان ٺهيل، بهترين نيٽو تجربو فراهم ڪري ٿو.

```diff
اعلان:

- WSL Dashboard Microsoft Store ذريعي ورهاڪو نه ٿيندو.
- هتي "WSL Dashboard" نالي سان ڄاڻايل ڪو به اپليڪيشن غير مجاز آهي۽ نقل ٿي سگهي ٿي.
- مهرباني ڪري مٿي ڊائون لوڊ نه ڪريو ته مٿڀاري ڌوڪاڙي کان بچو.
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

README I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | سنڌي | [አማርኛ](./README_am.md)

---

## 📑 فهرست
[🌍 ٻولي جي سپورٽ](#-ٻولي-جي-سپورٽ) | [🚀 بنيادي خاصيتون ۽ استعمال](#-بنيادي-خاصيتون--استعمال) | [🖼️ سافٽ ويئر اسڪرين شاٽ](#️-سافٽ-ويئر-اسڪرين-شاٽ) | [📦 انسٽاليشن گائيڊ](#-انسٽاليشن-گائيڊ) | [📢 سرڪاري ڪميونيٽي گروپ](#-سرڪاري-ڪميونٽي-گروپ) | [❤️ هن پروجيڪٽ کي سپورٽ ڪريو](#️-هن-پروجيڪٽ-کي-سپورٽ-ڪريو) | [⭐️ پيار لاءِ](#️-پيار-لاءِ) | [📄 اوپن سورس لائسنس](#-اوپن-سورس-لائسنس)

---

## 🌍 ٻولي جي سپورٽ

انگريزي، سادي چيني، رواداري چيني، هندي، هسپانوي، فرانسيسي، عربي، بنگالي، پرتگالي، روسي، اردو، اندونيزي، جرمن، جياني، ترڪي، ڪوريائي، اٽليوي، ڊچ، سويڊيش، چيڪ، يوناني، هنگيرين، عبراني، نارويجي، ڊنش، فنش، سلوواڪ، سلوويني، آئيس لينڊي، ويتنامي، تيلگو، جاونيزي، ٿائي، تامل، فليپيني، پنجابي، ملائي، پولش، يوڪراني، فارسي، ڪنڙ، مراٺي، هوسا، ميانمار، ازبڪ، آذربائيجاني، سيبوانو، ملالم، سنڌي، امهريڪ

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="انگريزي" alt="انگريزي" />
  <img src="../assets/flags/cn.svg" width="32" title="سادي چيني" alt="سادي چيني" />
  <img src="../assets/flags/tw.svg" width="32" title="رواداري چيني" alt="رواداري چيني" />
  <img src="../assets/flags/in.svg" width="32" title="هندي" alt="هندي" />
  <img src="../assets/flags/es.svg" width="32" title="هسپانوي" alt="هسپانوي" />
  <img src="../assets/flags/fr.svg" width="32" title="فرانسيسي" alt="فرانسيسي" />
  <img src="../assets/flags/sa.svg" width="32" title="عربي" alt="عربي" />
  <img src="../assets/flags/bd.svg" width="32" title="بنگالي" alt="بنگالي" />
  <img src="../assets/flags/pt.svg" width="32" title="پرتگالي" alt="پرتگالي" />
  <img src="../assets/flags/ru.svg" width="32" title="روسي" alt="روسي" />
  <img src="../assets/flags/pk.svg" width="32" title="اردو" alt="اردو" />
  <img src="../assets/flags/id.svg" width="32" title="اندونيزي" alt="اندونيزي" />
  <img src="../assets/flags/de.svg" width="32" title="جرمن" alt="جرمن" />
  <img src="../assets/flags/jp.svg" width="32" title="جياني" alt="جياني" />
  <img src="../assets/flags/tr.svg" width="32" title="ترڪي" alt="ترڪي" />
  <img src="../assets/flags/kr.svg" width="32" title="ڪوريائي" alt="ڪوريائي" />
  <img src="../assets/flags/it.svg" width="32" title="اٽليوي" alt="اٽليوي" />
  <img src="../assets/flags/nl.svg" width="32" title="ڊچ" alt="ڊچ" />
  <img src="../assets/flags/se.svg" width="32" title="سويڊيش" alt="سويڊيش" />
  <img src="../assets/flags/cz.svg" width="32" title="چيڪ" alt="چيڪ" />
  <img src="../assets/flags/gr.svg" width="32" title="يوناني" alt="يوناني" />
  <img src="../assets/flags/hu.svg" width="32" title="هنگيرين" alt="هنگيرين" />
  <img src="../assets/flags/il.svg" width="32" title="عبراني" alt="عبراني" />
  <img src="../assets/flags/no.svg" width="32" title="نارويجي" alt="نارويجي" />
  <img src="../assets/flags/dk.svg" width="32" title="ڊنش" alt="ڊنش" />
  <img src="../assets/flags/fi.svg" width="32" title="فنش" alt="فنش" />
  <img src="../assets/flags/sk.svg" width="32" title="سلوواڪ" alt="سلوواڪ" />
  <img src="../assets/flags/si.svg" width="32" title="سلوويني" alt="سلوويني" />
  <img src="../assets/flags/is.svg" width="32" title="آئيس لينڊي" alt="آئيس لينڊي" />
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

## 🚀 بنيادي خاصيتون ۽ استعمال

- **جديدي نيٽو UI**: ڊارڪ/لائيٽ موڊ، ٻڌل انيميشن، ۽ **Skia** ڏانهان چلائي ويل بلندڪارائي رينڊرنگ سان براهه GUI۔
- **سسٽم ٽري انٽيگريشن**: ٻڌل ٽري سپورٽ (~10MB ميموري استعمال)، ٻڀي ڪلڪ تي ڏيکار/لڪايو ٽوگل ۽ ڪامليو ڪاريائي رائيٽ ڪلڪ ميني۔
- **سمارٽ اسٽارٽ اپ**: بوٽ تي آٽو اسٽارٽ، ٽري ۾ ميني مائيز (`/silent` پيراميٽر سان خاموش اسٽارٽ)، ۽ نڪرڻ تي آٽو بند Distro۔
- **ڪم شيڊولنگ**: لچڪدار ڪم شيڊولنگ مينيجمينٽ، WSL انسٽنسن ۾ مقرر وقت يا وقفي تي اسڪرپٽ ۽ ڪمانڊ جي آٽو ايڪسڪيوشن جي حمايت ڪري ٿو، خودڪار آپريشن لاءِ۔
- **مڪمل انسٽنس ڪنٽرول**: هڪ ڪلڪ تي اسٽارٽ، بند، ختم، ۽ غیر رجسٽر ڪريو۔ حقيقي وقت جي حالت جي مانيٽرنگ، ڊسڪ استعمال ۽ فائيل مقام ڏسو۔
- **ڊسٽرو مينيجمينٽ**: ڊيفولٽ طور مقرر ڪريو، فيزيڪل مائيگريشن (VHDX ٻي ڊسڪ تي منتقل ڪريو)، ۽ `.tar` يا `.tar.gz` آرڪائيو طور ايڪسپورٽ/ڪلون ڪريو۔
- **ڊسڪ مائونٽ**: WSL انسٽنس جي VHDX ورچوئل ڊسڪ کي سڌو سنئون مائونٽ ۽ مينيجمينٽ ڪريو، مائونٽ، انمائونٽ ۽ ڊسڪ مينيجمينٽ آپريشن جي حمايت ڪري ٿو، هوسٽ ۽ WSL جي وچ ۾ فائيل منتقلي آسان بڻائي ٿو۔
- **فوري انٽيگريشن**: هڪ ڪلڪ تي ٽرمينل، VS Code، يا فائيل ايڪسپلورر کوليو، ڪسٽم ڪم ڊائريڪٽري ۽ اسٽارٽ اپ اسڪرن هڪنگ سان سپورٽ۔
- **ڊسٽرو انسٽاليشن**: Microsoft Store، GitHub، مقامي فائيلون (RootFS/VHDX)، يا آن لائن ميررز ذريعي لينڪس ڊسٽري بيوشنز انسٽال ڪريو (آٽو اسپيڊ ٽيسٽ سان تيز ترين ميرر چونڊي ۽ بلٽ ان RootFS ڊائونلوڊ مددگار)۔
- **عالمي سلامتي**: هڪ وقت مائيگريشن/بيڪ اپ آپريشن جي سلامتي يقيني ڪرڻ لاءِ mutex استعمال، ۽ ڪڍڻ تي Appx پيڪيج آٽو صفي ڪريو۔
- **انتهائ گهٽ ميموري استعمال**: اعليٰ بهترين وسيلو ڪارڪردگي۔ خاموش اسٽارٽ (سسٽم ٽري) صرف **10MB** ميموري۔ ونڊو موڊ ۾ فانٽ ڪمپليڪسيٽي تي منحصر: **18MB** (معياري ٻوليون جهڙوڪ انگريزي، جرمن وغيره) کان **38MB** (وڏي حرف سٽ جهڙوڪ CJK) سُڌي۔
- **ايڊوانس نيٽورڪ مينيجمينٽ**: پورٽ فارورڊنگ (آٽو فائر وال رول ٺاهڻ) ۽ گلوبل HTTP پراڪسي ترتيب ڪامل طور مينيجر ڪريو۔
- **USB ڊوائس مينيجمينٽ**: `usbipd-win` سان ڊوڙھو انٽيگريشن، بڌل USB ڊوائسن تي bind، attach، ۽ مينيجمينٽ آپريشن ڊيش بورڊ UI ۾ سڌو ڪريو۔
- **ترتيب ۽ لاگ**: سيٽنگز مان ڊفالٽ انسٽال فولڊر، لاگ فولڊر ۽ ليول، UI ٻولي، ڊارڪ موڊ، اپڊيٽ چيڪ جي تعدد، آٽو اسٽارٽ ۽ بند ٿيڻ جو رويو، ۽ سائيڊبار کي منظم ڪريو.
- **ڊسٽرو کي هلندڙ رکڻ**: «اسٽارٽ» تي هڪ ڪلڪ ڊسٽرو کي ~60 سيڪنڊ بيڪاري کان پوءِ بند ٿيڻ بدران هليو رهڻ ڏئي ٿو؛ «ونڊوز شروع ٿيڻ تي هي ڊسٽرو شروع ڪريو» پڻ فعال ڪري سگهجي ٿو.
- **گلوبل شارٽ ڪٽس**: مکيه ونڊو ڏيکاري/لڪائڻ يا ڊسٽرو جو ٽرمينل فوراً کولڻ لاءِ گلوبل شارٽ ڪٽس مقرر ڪريو.


## 🖼️ سافٽ ويئر اسڪرين شاٽ
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

ڊيش بورڊ کي ڪم ڪندي ڏسڻ چاهيو ٿا؟ مڪمل اسڪرين شاٽ گيلري لاءِ [سرڪاري ويب سائيٽ](https://www.wslui.com/) جو دورو ڪريو.

## 📦 انسٽاليشن گائيڊ

### طريقو 1: پروجيڪٽ ويب سائيٽ جو دورو ڪريو (سفارش ڪيل)

اسان سرڪاري ويب سائيٽ تي وڃي ڊائونلوڊ ڪرڻ جي سفارش ڪريون ٿا، ڇو ته اهو هموار تجربي لاءِ ڪيترائي mirror لنڪس فراهم ڪري ٿو:

[ڊائونلوڊ صفحي](https://www.wslui.com/download/) تي وڃو ۽ پنهنجي علائقي لاءِ مناسب mirror چونڊيو.

### طريقو 2: winget ذريعي انسٽال ڪريو

توهان Windows Package Manager (winget) ذريعي سڌو سنئون WSLDashboard انسٽال ڪري سگهو ٿا، moniker يا مڪمل پيڪيج آئيڊينٽيفائر استعمال ڪندي:

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

> winget پيڪيج آئيڊينٽيفائر `Owu.WSLDashboard` آهي ۽ moniker `wsl-dashboard` آهي (case-insensitive). ٻنهي مان ڪو به ڪم ڪري ٿو.

وڌيڪ معلومات لاءِ، [WinGet ڪميونٽي رپوزٽري](https://github.com/microsoft/winget-pkgs/tree/master/manifests/o/Owu/WSLDashboard) جو دورو ڪريو.

### طريقو 3: سورس مان بلڊ ڪريو

Rust ٽول چين (Rust 1.92+ يا نئين ورزن) انسٽال آهي تي يقيني ڪريو۔

1. رپوزٽري ڪلون ڪريو:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. بلڊ ۽ رن ڪريو:

   - ڊولپمينٽ ڊيبگ:

     ```powershell
     cargo run
     ```
   - بلڊ اسڪرپٽ سان بهترين ريليز ورزن بلڊ ڪريو:

     > بلڊ اسڪرپٽ کي `x86_64-pc-windows-msvc` ٽول چين جي ضرورت آهي۔

     ```powershell
     .\build\portable\build.ps1 -Arch x64
     ```


## 📢 سرڪاري ڪميونٽي گروپ

WSL UI سرڪاري ڪميونٽي گروپس ۾ توهان جو خيڙو آهي! استعمال جي ٽپن تي بحث ڪرڻ، مسئلا رپورٽ ڪرڻ， نوان اپڊيٽ حاصل ڪرڻ ۽ विकसक ۽ ٻين صارفين سان رابطو ڪرڻ لاءِ اسان جي چینلز ۾ شامل ٿيو.

<div align="center">

| عالمي گروپ (Telegram) | چيني گروپ (WeChat) |
| :---: | :---: |
| <img src="https://cdn1.wslui.com/static/images/group/group-telegram.png" width="240" alt="Telegram Group QR Code" /> | <img src="https://cdn1.wslui.com/static/images/group/group-wechat.png" width="240" alt="WeChat Group QR Code" /> |
| شامل ٿيڻ لاءِ اسڪين ڪريو | شامل ٿيڻ لاءِ اسڪين ڪريو |

</div>


## ❤️ هن پروجيڪٽ کي سپورٽ ڪريو

- هن پروجيڪٽ GPL-3.0 اوپن سورس لائسنس استعمال ڪري ٿو، سڀ صارفين لاءِ مفت۔
- فيچر ڊولپمينٽ، روزانه جانچ، ۽ بگ فيڪس تائين، سڀ ڪم فارغ وقت مان آهي۔ اوپن سورس راهه اڪيلو چلڻ آسان ناهي، توهان جي تڃاتائي ۽ سپورٽ پروجيڪٽ جي دوامي لاءِ طاقت آهي۔
- جيڪڏهن توهان کي لڳي ٿو ته هن سڃاڻپ توهان کي مدد ڪري ٿي، مهرباني ڪري مدد ڪريو۔ سڀ ڊونيٽ سرور خرچ، ورزن آٽريشن، ۽ فيچر بهترين لاءِ استعمال ٿيڻ ڪي۔
- ننڍي مھرباني، تارا بڻجن. توهان جي سمجھ ۽ سخيءَ لاءِ ٻيهر شڪريو! [اسان جي ڊونيٽ صفحو ڏسو](https://www.wslui.com/donate/).


## ⭐️ پيار لاءِ

جيڪڏهن توهان کي لڳي ٿو ته هن پروجيڪٽ توهان کي مدد ڪري ٿي، ته GitHub تي ستاره ڏيڻ ۾ خوشي احساس ڪندس. توهان جي تڃاتائي پروجيڪٽ کي وڌيڪ صارفين تائين پهچڻ ۾ مدد ڪندي. هي حوصله افزائي منهنجي تي پيشي ڪري ٿي.


## 📄 اوپن سورس لائسنس

هن پروجيڪٽ GPL-3.0 تحت لائسنس آهي – تفصيل لاءِ [LICENSE](../LICENSE) فائيل ڏسو۔
