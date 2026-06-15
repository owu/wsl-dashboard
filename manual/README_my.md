# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

WSL (Windows Subsystem for Linux) ထိန်းချုပ်မှု Dashboard သစ်၊ စွမ်းဆောင်ရည်မြင့်၊ ပေါ့ပါးပြီး မမ်မိုရီသုံးစွဲမှုနည်းပါးသော။ Rust နှင့် Slint ဖြင့်တည်ဆောက်ထားပြီး အကောင်းဆုံး native အတွေ့အကြုံကို ပေးအပ်ပါသည်။

---

```diff
သတိပြုရန်:

- ဤဆော့ဖ်ဝဲကို Microsoft Store မှ ဖြန့်ဝေခြင်းမရှိပါ။
- ထိုနေရာရှိ တူညီသောအမည်ဖြင့် အက်ပ်များသည် ခွင့်ပြုချက်မရှိသော တရားမဝင် ဆော့ဖ်ဝဲများဖြစ်သည်။
- လိမ်လည်မခံရစေရန် ဒေါင်းလုဒ်မလုပ်ပါနှင့်။
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

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | မြန်မာ | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 မာတိကာ
- [🌍 ဘာသာစကား ပံ့ပိုးမှု](#-ဘာသာစကား-ပံ့ပိုးမှု)
- [🚀 အဓိကလုပ်ဆောင်ချက်များနှင့် အသုံးပြုပုံ](#-အဓိကလုပ်ဆောင်ချက်များနှင့်-အသုံးပြုပုံ)
- [⚙️ ဖွဲ့စည်းမှုနှင့် Log](#️-ဖွဲ့စည်းမှုနှင့်-log)
- [🖼️ ဆော့ဖ်ဝဲ မျက်နှာပြင်ရိုက်ချက်များ](#️-ဆော့ဖ်ဝဲ-မျက်နှာပြင်ရိုက်ချက်များ)
- [🎬 လုပ်ဆောင်ချက် သရုပ်ပြ](#-လုပ်ဆောင်ချက်-သရုပ်ပြ)
- [💻 စနစ် လိုအပ်ချက်များ](#-စနစ်-လိုအပ်ချက်များ)
- [📦 ထည့်သွင်းမှု လမ်းညွှန်](#-ထည့်သွင်းမှု-လမ်းညွှန်)
- [🛠️ နည်းပညာ အစုနှင့် စွမ်းဆောင်ရည်](#️-နည်းပညာ-အစုနှင့်-စွမ်းဆောင်ရည်)
- [🤝 အသိုင်းအဝိုင်း ပံ့ပိုးမှု](#-အသိုင်းအဝိုင်း-ပံ့ပိုးမှု)
- [❤️ ဤစီမံကိန်းကို ပံ့ပိုးပါ](#️-ဤစီမံကိန်းကို-ပံ့ပိုးပါ)
- [⭐️ ချစ်ခြင်းဖြင့်](#️-ချစ်ခြင်းဖြင့်)
- [📄 အိုင်ပင်းဆို့စ် လိုင်စင်](#-အိုင်ပင်းဆို့စ်-လိုင်စင်)

---

## 🌍 ဘာသာစကား ပံ့ပိုးမှု

အင်္ဂလိပ်၊ ရိုးရှင်းတရုတ်၊ ရိုးရာတရုတ်၊ ဟိန္ဒီ၊ စပိန်၊ ပြင်သစ်၊ အာရဗီ၊ ဘင်္ဂါလီ၊ ပေါ်တူဂီ၊ ရုရှ၊ အူရဒူ၊ အင်ဒိုနီးရှား၊ ဂျာမန်၊ ဂျပန်၊ တူရကီ၊ ကိုရီးယား၊ အီတလီ၊ ဒတ်ခ်ျ၊ ဆွီဒင်၊ ချက်၊ ဂရိ၊ ဟန်ဂေရီ၊ ဟီဘရူး၊ နော်ဝေ၊ ဒိန်းမတ်၊ ဖင်လန်၊ ဆလိုဗက်၊ ဆလိုဗေးနီးယား၊ အိုက်စလန်၊ ဗီယက်နမ်၊ တယ်လီဂူ၊ ဂျာဗား၊ ထိုင်း၊ တမီးလ်၊ ဖိလစ်ပိုင်၊ ပန်ချာပီ၊ မလေး၊ ပိုလန်၊ ယူကရိန်း၊ ပါရှန်၊ ကန္နာဒါ၊ မာရသီ၊ ဟော်စာ၊ မြန်မာ၊ ဥဇဘက်၊ အဇာဘိုင်ဂျန်၊ ဆီဘူအာနို၊ မလာယာလမ်၊ ဆင်ဒီ၊ အမ်ဟာရစ်

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="အင်္ဂလိပ်" alt="အင်္ဂလိပ်" />
  <img src="../assets/flags/cn.svg" width="32" title="ရိုးရှင်းတရုတ်" alt="ရိုးရှင်းတရုတ်" />
  <img src="../assets/flags/tw.svg" width="32" title="ရိုးရာတရုတ်" alt="ရိုးရာတရုတ်" />
  <img src="../assets/flags/in.svg" width="32" title="ဟိန္ဒီ" alt="ဟိန္ဒီ" />
  <img src="../assets/flags/es.svg" width="32" title="စပိန်" alt="စပိန်" />
  <img src="../assets/flags/fr.svg" width="32" title="ပြင်သစ်" alt="ပြင်သစ်" />
  <img src="../assets/flags/sa.svg" width="32" title="အာရဗီ" alt="အာရဗီ" />
  <img src="../assets/flags/bd.svg" width="32" title="ဘင်္ဂါလီ" alt="ဘင်္ဂါလီ" />
  <img src="../assets/flags/pt.svg" width="32" title="ပေါ်တူဂီ" alt="ပေါ်တူဂီ" />
  <img src="../assets/flags/ru.svg" width="32" title="ရုရှ" alt="ရုရှ" />
  <img src="../assets/flags/pk.svg" width="32" title="အူရဒူ" alt="အူရဒူ" />
  <img src="../assets/flags/id.svg" width="32" title="အင်ဒိုနီးရှား" alt="အင်ဒိုနီးရှား" />
  <img src="../assets/flags/de.svg" width="32" title="ဂျာမန်" alt="ဂျာမန်" />
  <img src="../assets/flags/jp.svg" width="32" title="ဂျပန်" alt="ဂျပန်" />
  <img src="../assets/flags/tr.svg" width="32" title="တူရကီ" alt="တူရကီ" />
  <img src="../assets/flags/kr.svg" width="32" title="ကိုရီးယား" alt="ကိုရီးယား" />
  <img src="../assets/flags/it.svg" width="32" title="အီတလီ" alt="အီတလီ" />
  <img src="../assets/flags/nl.svg" width="32" title="ဒတ်ခ်ျ" alt="ဒတ်ခ်ျ" />
  <img src="../assets/flags/se.svg" width="32" title="ဆွီဒင်" alt="ဆွီဒင်" />
  <img src="../assets/flags/cz.svg" width="32" title="ချက်" alt="ချက်" />
  <img src="../assets/flags/gr.svg" width="32" title="ဂရိ" alt="ဂရိ" />
  <img src="../assets/flags/hu.svg" width="32" title="ဟန်ဂေရီ" alt="ဟန်ဂေရီ" />
  <img src="../assets/flags/il.svg" width="32" title="ဟီဘရူး" alt="ဟီဘရူး" />
  <img src="../assets/flags/no.svg" width="32" title="နော်ဝေ" alt="နော်ဝေ" />
  <img src="../assets/flags/dk.svg" width="32" title="ဒိန်းမတ်" alt="ဒိန်းမတ်" />
  <img src="../assets/flags/fi.svg" width="32" title="ဖင်လန်" alt="ဖင်လန်" />
  <img src="../assets/flags/sk.svg" width="32" title="ဆလိုဗက်" alt="ဆလိုဗက်" />
  <img src="../assets/flags/si.svg" width="32" title="ဆလိုဗေးနီးယား" alt="ဆလိုဗေးနီးယား" />
  <img src="../assets/flags/is.svg" width="32" title="အိုက်စလန်" alt="အိုက်စလန်" />
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


## 🚀 အဓိကလုပ်ဆောင်ချက်များနှင့် အသုံးပြုပုံ

- **ခေတ်မီ Native UI**: မှောင်/လင်း မုဒ်၊ ချောမွေ့သော animation များဖြင့် ရိုးရှင်းသော GUI၊ **Skia** ဖြင့် မြင့်မားသော စွမ်းဆောင်ရည် rendering။
- **System Tray ပေါင်းစည်းမှု**: tray အပြည့်အစုံ ပံ့ပိုးမှု (~10MB မမ်မိုရီသုံးစွဲမှု)၊ နှစ်ချက်နှိပ်၍ ပြ/ဝှက် ပြောင်းလဲခြင်းနှင့် လုပ်ဆောင်ချက်ပြည့်စုံသော ညာဖက်ကလစ် မီနူး။
- **ဉာဏ်ရည်ထက်မြက်သော စတင်ခြင်း**: ဘုတ်တက်စတင်ခြင်း၊ tray သို့ အသေးငယ်ဆုံးချခြင်း (`/silent` parameter ဖြင့် အသံမထွက်ဘဲ စတင်ခြင်း)၊ ထွက်သည့်အခါ distro ကို အလိုအလျောက် ပိတ်ခြင်း။
- **အပြည့်အစုံ ထိန်းချုပ်မှု**: တစ်ချက်နှိပ်၍ စတင်ခြင်း၊ ရပ်တန့်ခြင်း၊ အဆုံးသတ်ခြင်းနှင့် မှတ်ပုံတင်ဖျက်သိမ်းခြင်း။ အချိန်နှင့်တပြေးညီ အခြေအနေ စောင့်ကြည့်ခြင်း၊ disk သုံးစွဲမှုနှင့် ဖိုင်တည်နေရာ ကြည့်ရှုခြင်း။
- **Distro စီမံခန့်ခွဲမှု**: မူလအဖြစ်သတ်မှတ်ခြင်း၊ ရုပ်ပိုင်းဆိုင်ရာ ပြောင်းရွှေ့ခြင်း (VHDX ကို အခြား disk သို့ ရွှေ့ခြင်း)၊ ထုတ်ယူခြင်း/ကူးယူခြင်း `.tar` သို့မဟုတ် `.tar.gz` အဖြစ်။
- **လျင်မြန်စွာ ပေါင်းစည်းခြင်း**: terminal၊ VS Code သို့မဟုတ် File Explorer သို့ တစ်ချက်နှိပ်၍ ဝင်ရောက်ခြင်း၊ စိတ်ကြိုက် လုပ်ငန်းလမ်းညွှန်နှင့် စတင်ခြင်း script hook များ ပံ့ပိုးခြင်း။
- **ဒစ်စထရို ထည့်သွင်းခြင်း**: Microsoft Store၊ GitHub၊ ဒေသခံဖိုင်များ (RootFS/VHDX) သို့မဟုတ် အွန်လိုင်း မှန်များမှတစ်ဆင့် Linux ဖြန့်ဝေမှုများကို ထည့်သွင်းပါ (အမြန်ဆုံး မှန်ကို ရွေးချယ်ရန် အလိုအလျောက် အမြန်နှုန်း စမ်းသပ်မှုနှင့် တည်ဆောက်ထားသော RootFS ဒေါင်းလုဒ် ကူညီသူ)။
- **ကမ္ဘာလုံးဆိုင်ရာ လုံခြုံရေး**: mutex ကို အသုံးပြု၍ တစ်ပြိုင်နက် ပြောင်းရွှေ့/အရန်သိမ်းဆည်းမှုများ၏ လုံခြုံရေးကို သေချာစေခြင်း၊ ဖယ်ရှားသည့်အခါ Appx package များကို အလိုအလျောက် ရှင်းလင်းခြင်း။
- **မမ်မိုရီ သုံးစွဲမှု အလွန်နည်းပါးခြင်း**: အလွန်ကောင်းမွန်စွာ optimize လုပ်ထားသော အရင်းအမြစ် ထိရောက်မှု။ အသံမထွက်ဘဲ စတင်ခြင်း (system tray) သည် **10MB** မမ်မိုရီသာ သုံးစွဲသည်။ window မုဒ်တွင် font ရှုပ်ထွေးမှုပေါ် မူတည်၍ **18MB** (အင်္ဂလိပ်၊ ဂျာမန် စသော စံဘာသာစကားများ) မှ **38MB** (CJK ကဲ့သို့ အကြီးစား character set များ) အထိ သုံးစွဲသည်။
- **အဆင့်မြင့် ကွန်ရက် စီမံခန့်ခွဲမှု**: port forwarding (firewall စည်းမျဉ်းများကို အလိုအလျောက် ဖန်တီးခြင်း) နှင့် global HTTP proxy configuration ကို ချောမွေ့စွာ စီမံခန့်ခွဲခြင်း။
- **USB စက်ပစ္စည်း စီမံခန့်ခွဲမှု**: `usbipd-win` နှင့် နက်ရှိုင်းစွာ ပေါင်းစည်းခြင်း၊ WSL instance အားလုံးအတွက် dashboard UI တွင် local USB စက်ပစ္စည်းများကို bind၊ attach နှင့် စီမံခန့်ခွဲမှု လုပ်ဆောင်ချက်များ ပြုလုပ်နိုင်သည်။


## ⚙️ ဖွဲ့စည်းမှုနှင့် Log

ဖွဲ့စည်းမှုအားလုံးကို "ဆက်တင်များ" မြင်ကွင်းမှတစ်ဆင့် စီမံခန့်ခွဲသည်:

- WSL instance အသစ်များအတွက် မူလ ထည့်သွင်းမှု လမ်းညွှန်ကို ရွေးချယ်ပါ။
- Log လမ်းညွှန်နှင့် log level (Error / Warn / Info / Debug / Trace) ကို ဖွဲ့စည်းပါ။
- UI ဘာသာစကားကို ရွေးချယ်ပါ သို့မဟုတ် စနစ် ဘာသာစကားကို လိုက်နာပါ။
- မှောင်မုဒ် ပြောင်းလဲခြင်း၊ နှင့် လုပ်ဆောင်ချက်ပြီးနောက် app သည် WSL ကို အလိုအလျောက် ပိတ်သင့်မသင့်။
- Update စစ်ဆေးမှု ကြိမ်နှုန်းကို ဖွဲ့စည်းပါ (နေ့စဉ်၊ အပတ်စဉ်၊ နှစ်ပတ်လျှင်တစ်ကြိမ်၊ လစဉ်)။
- Boot စတင်ခြင်းကို ဖွင့်ပါ (လမ်းညွှန် အလိုအလျောက် ပြင်ဆင်ခြင်း လုပ်ဆောင်ချက်ဖြင့်)။
- စတင်သည့်အခါ tray သို့ အသေးငယ်ဆုံးချရန် သတ်မှတ်ပါ။
- ပိတ်ရန် ခလုတ်၏ လုပ်ဆောင်ချက်ကို ဖွဲ့စည်းပါ (ပရိုဂရမ်မှ ထွက်ခြင်းထက် tray သို့ အသေးငယ်ဆုံးချခြင်း)။
- သီးခြား လုပ်ဆောင်ချက် tab များ၏ မြင်ကွင်းကို ပြောင်းလဲခြင်းဖြင့် sidebar ကို စိတ်ကြိုက်ပြင်ဆင်ပါ။

Log ဖိုင်များကို ဖွဲ့စည်းထားသော log လမ်းညွှန်သို့ ရေးသားပြီး၊ ပြဿနာ အစီရင်ခံသည့်အခါ log များကို ပူးတွဲနိုင်သည်။


## 🖼️ ဆော့ဖ်ဝဲ မျက်နှာပြင်ရိုက်ချက်များ

### ပင်မ မျက်နှာပြင် (မှောင် & လင်း မုဒ်)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB & Menu ခေါက်ခြင်း
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### ကွန်ရက် စီမံခန့်ခွဲမှု
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### Instance ထည့်ခြင်း & ဆက်တင်များ
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>
<p align="center">
  <img src="../assets/screenshot/settings-advanced.png" width="48%" />
  <img src="../assets/screenshot/settings-interface.png" width="48%" />
</p>

### အကြောင်း & လှူဒါန်းခြင်း
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/donate.png" width="48%" />
</p>

## 🎬 လုပ်ဆောင်ချက် သရုပ်ပြ

[ကျွန်ုပ်တို့ကို ကူညီတိုးတက်စေပါ! ကျွန်ုပ်တို့၏ မိတ်ဆက်ဗီဒီယိုကို ကြည့်ရှုပြီး သင့်ထင်မြင်ချက်များကို မျှဝေပါ။](https://github.com/owu/wsl-dashboard/discussions/9)



## 💻 စနစ် လိုအပ်ချက်များ

- WSL ဖွင့်ထားသော Windows 10 သို့မဟုတ် Windows 11 (WSL 2 အသုံးပြုရန် အကြံပြုထားသည်)။
- WSL distro အနည်းဆုံးတစ်ခု ထည့်သွင်းထားခြင်း သို့မဟုတ် distro အသစ် ထည့်သွင်းခွင့် ရှိခြင်း။
- 64-bit CPU; ချောမွေ့သော multi-distro အသုံးပြုမှုအတွက် 4 GB သို့မဟုတ် ၎င်းထက်ပိုသော RAM အကြံပြုထားသည်။

## 📦 ထည့်သွင်းမှု လမ်းညွှန်

### နည်းလမ်း 1: Pre-compiled Binary ဒေါင်းလုဒ်လုပ်ခြင်း

အလွယ်ဆုံးနည်းလမ်းမှာ ကြိုတင် compile လုပ်ထားသော version ကို အသုံးပြုခြင်းဖြစ်သည်:

1. [GitHub Releases](https://github.com/owu/wsl-dashboard/releases) စာမျက်နှာသို့ သွားပါ။
2. Windows အတွက် `wsldashboard` executable နောက်ဆုံး version ကို ဒေါင်းလုဒ်လုပ်ပါ။
3. ဖြည်ပါ (zip file ဖြစ်ပါက) ပြီး `wsldashboard.exe` ကို run ပါ။

ထည့်သွင်းရန် မလိုအပ်ပါ၊ ဤ app သည် တစ်ခုတည်းသော file portable program ဖြစ်သည်။

### နည်းလမ်း 2: Source Code မှ Build လုပ်ခြင်း

Rust toolchain (Rust 1.92+ သို့မဟုတ် နောက်ဆုံး version) ထည့်သွင်းထားကြောင်း သေချာပါစေ။

1. Repository ကူးယူပါ:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. Build လုပ်ပြီး run ပါ:

   - Development debug:

     ```powershell
     cargo run
     ```
   - Build script ဖြင့် optimize လုပ်ထားသော release version build လုပ်ခြင်း:

     > Build script သည် `x86_64-pc-windows-msvc` toolchain လိုအပ်သည်။

     ```powershell
     .\build\portable\build.ps1
     ```


## 🛠️ နည်းပညာ အစုနှင့် စွမ်းဆောင်ရည်

- **Kernel**: Rust ဖြင့် အကောင်အထည်ဖော်ထားပြီး မမ်မိုရီ လုံခြုံရေးနှင့် zero-cost abstraction ကို သေချာစေသည်။
- **UI Framework**: မြင့်မားသော စွမ်းဆောင်ရည် **Skia** rendering backend ပါဝင်သော Slint။
- **Asynchronous Runtime**: non-blocking system commands နှင့် I/O အတွက် Tokio။
- **စွမ်းဆောင်ရည် ထူးခြားချက်များ**:
  - **တုံ့ပြန်မှု အမြန်နှုန်း**: ချက်ချင်း စတင်ခြင်း၊ နှင့် WSL အခြေအနေကို အချိန်နှင့်တပြေးညီ စောင့်ကြည့်ခြင်း။
  - **အရင်းအမြစ် ထိရောက်မှု**: အလွန်နည်းပါးသော အရင်းအမြစ် သုံးစွဲမှု ([အဓိကလုပ်ဆောင်ချက်များ](#-အဓိကလုပ်ဆောင်ချက်များနှင့်-အသုံးပြုပုံ) တွင် ကြည့်ရှုပါ)။
  - **Portable**: optimize လုပ်ထားသော release version သည် တစ်ခုတည်းသော compact executable ဖိုင်ကို ဖန်တီးသည်။



## 🤝 အသိုင်းအဝိုင်း ပံ့ပိုးမှု

အောက်ပါ အသိုင်းအဝိုင်းများ၏ ပံ့ပိုးမှုအတွက် ကျေးဇူးတင်ပါသည်:

- [Rust Programming Language](https://www.rust-lang.org) - အစွမ်းထက်ပြီး လုံခြုံသော programming language
- [Slint | Declarative GUI for Rust, C++, JavaScript & Python](https://slint.dev) - ခေတ်မီ UI framework
- [WSL: Windows Subsystem for Linux](https://github.com/microsoft/WSL) - အကောင်းဆုံး Windows Subsystem for Linux
- [Tokio - An asynchronous Rust runtime](https://tokio.rs) - ထိရောက်သော asynchronous runtime
- [Windows Developer Community](https://developer.microsoft.com/en-us/windows/community) - ဆက်လက်ဖြစ်ပေါ်နေသော platform တိုးတက်မှုများ
- [Reddit](https://www.reddit.com) - ကမ္ဘာလုံးဆိုင်ရာ အသိုင်းအဝိုင်း ဆွေးနွေးမှုနှင့် ပံ့ပိုးမှု
- [Hacker News](https://news.ycombinator.com) - ကမ္ဘာလုံးဆိုင်ရာ အသိုင်းအဝိုင်း ဆွေးနွေးမှုနှင့် ပံ့ပိုးမှု
- [Linux.do](https://linux.do) - IT ကျွမ်းကျင်သူများအတွက် လူကြိုက်များသော အသိုင်းအဝိုင်း
- [V2EX](https://www.v2ex.com) - တရုတ် နည်းပညာ အသိုင်းအဝိုင်း ဆွေးနွေးမှု

သင့်ပံ့ပိုးမှုနှင့် feedback များက ဤစီမံကိန်းကို ဖြစ်နိုင်စေသည်!


## ❤️ ဤစီမံကိန်းကို ပံ့ပိုးပါ

- ဤစီမံကိန်းသည် GPL-3.0 open source license ကို အသုံးပြုထားပြီး အသုံးပြုသူအားလုံးအတွက် အခမဲ့ဖြစ်သည်။
- Feature development မှ daily testing နှင့် bug fixes အထိ လုပ်ငန်းအားလုံးသည် အားလပ်ချိန်မှလာသည်။ Open source လမ်းကြောင်းသည် တစ်ယောက်တည်းလျှောက်ရန် မလွယ်ကူပါ၊ သင့်အသိအမှတ်ပြုမှုနှင့် ပံ့ပိုးမှုသည် စီမံကိန်း ဆက်လက်ရှင်သန်ရန် အင်အားဖြစ်သည်။
- ဤကိရိယာက သင့်ကို အမှန်တကယ် ကူညီသည်ဟု ခံစားရပါက ကူညီပါ။ လှူဒါန်းမှုအားလုံးကို server ကုန်ကျစရိတ်၊ version iteration နှင့် feature optimization အတွက် အသုံးပြုမည်ဖြစ်ပြီး စီမံကိန်း ဆက်လက် update ဖြစ်စေမည်။
- ကောင်းသောအရာ အနည်းငယ်သည် ကြယ်များဖြစ်သည်။ သင့်နားလည်မှုနှင့် ရက်ရောမှုအတွက် ထပ်မံကျေးဇူးတင်ပါသည်!

ကျွန်ုပ်တို့၏ လှူဒါန်းမှု စာမျက်နှာကို ဝင်ကြည့်ပါ: [https://www.wslui.com/donate/](https://www.wslui.com/donate/)


## ⭐️ ချစ်ခြင်းဖြင့်

ဤစီမံကိန်းက သင့်ကို ကူညီသည်ဟု ခံစားရပါက GitHub တွင် star တစ်ပွင့် ပေးနိုင်ပါက ကျေးဇူးတင်ပါမည်။ သင့်အသိအမှတ်ပြုမှုက စီမံကိန်းကို ပိုမိုကျယ်ပြန့်သော အသုံးပြုသူများထံ ရောက်ရှိစေမည်ဖြစ်ပြီး ကျေးဇူးတင်ပါသည်။ ဤအားပေးမှုကပင် ကျွန်ုပ်ကို ဆက်လက်လုပ်ဆောင်စေသည်။


## 📄 အိုင်ပင်းဆို့စ် လိုင်စင်

ဤစီမံကိန်းကို GPL-3.0 license ဖြင့် ခွင့်ပြုထားသည် – အသေးစိတ်အတွက် [LICENSE](../LICENSE) ဖိုင်ကို ကြည့်ရှုပါ။


---

Built with ❤️ for the WSL Community.