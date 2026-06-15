# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

ఆధునిక, అధిక-పనితీరు, తేలికపాటి మరియు తక్కువ మెమరీ వినియోగం కలిగిన WSL (Windows Subsystem for Linux) ఇన్‌స్టాన్స్ మేనేజ్‌మెంట్ డాష్‌బోర్డ్. Rust మరియు Slint ఆధారంగా నిర్మించబడింది, అత్యుత్తమ నేటివ్ అనుభవాన్ని అందిస్తుంది.

---

```diff
గమనిక:

- ఈ సాఫ్ట్‌వేర్ Microsoft Store ద్వారా పంపిణీ చేయబడదు.
- ఇక్కడ అదే పేరుతో ఉన్న అనువర్తనాలు అనధికారిక పైరేటెడ్ సాఫ్ట్‌వేర్.
- మోసపోకుండా దయచేసి డౌన్‌లోడ్ చేయవద్దు.
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

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | తెలుగు | [Tiếng Việt](./README_vi.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 విషయ సూచిక
- [🌍 భాషా మద్దతు](#-భాషా-మద్దతు)
- [🚀 ప్రధాన లక్షణాలు & వినియోగం](#-ప్రధాన-లక్షణాలు--వినియోగం)
- [⚙️ కాన్ఫిగరేషన్ & లాగ్‌లు](#️-కాన్ఫిగరేషన్--లాగ్లు)
- [🖼️ సాఫ్ట్‌వేర్ స్క్రీన్‌షాట్‌లు](#️-సాఫ్ట్వేర్-స్క్రీన్షాట్లు)
- [🎬 ఆపరేషన్ డెమో](#-ఆపరేషన్-డెమో)
- [💻 సిస్టమ్ అవసరాలు](#-సిస్టమ్-అవసరాలు)
- [📦 ఇన్‌స్టాలేషన్ గైడ్](#-ఇన్స్టాలేషన్-గైడ్)
- [🛠️ టెక్ స్టాక్ & పనితీరు](#️-టెక్-స్టాక్--పనితీరు)
- [🤝 కమ్యూనిటీ మద్దతు](#-కమ్యూనిటీ-మద్దతు)
- [❤️ ప్రాజెక్ట్‌కు మద్దతు ఇవ్వండి](#️-ప్రాజెక్ట్కు-మద్దతు-ఇవ్వండి)
- [⭐️ ప్రేమతో నడిచే ప్రాజెక్ట్](#️-ప్రేమతో-నడిచే-ప్రాజెక్ట్)
- [📄 ఓపెన్ సోర్స్ లైసెన్స్](#-ఓపెన్-సోర్స్-లైసెన్స్)

---

## 🌍 భాషా మద్దతు

ఇంగ్లీష్, సరళీకృత చైనీస్, సాంప్రదాయ చైనీస్, హిందీ, స్పానిష్, ఫ్రెంచ్, అరబిక్, బెంగాలీ, పోర్చుగీస్, రష్యన్, ఉర్దూ, ఇండోనేషియన్, జర్మన్, జపనీస్, టర్కిష్, కొరియన్, ఇటాలియన్, డచ్, స్వీడిష్, చెక్, గ్రీక్, హంగేరియన్, హీబ్రూ, నార్వేజియన్, డానిష్, ఫిన్నిష్, స్లోవాక్, స్లోవేనియన్, ఐస్లాండిక్, వియత్నామీస్, తెలుగు, జావానీస్, థాయ్, తమిళం, ఫిలిపినో, పంజాబీ, మలయ్, పోలిష్, ఉక్రేనియన్, పర్షియన్, కన్నడ, మరాఠీ, హౌసా, బర్మీస్, ఉజ్బెక్, అజర్బైజానీ, సెబువానో, మలయాళం, సింధీ, అమ్హారిక్

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="ఇంగ్లీష్" alt="ఇంగ్లీష్" />
  <img src="../assets/flags/cn.svg" width="32" title="సరళీకృత చైనీస్" alt="సరళీకృత చైనీస్" />
  <img src="../assets/flags/tw.svg" width="32" title="సాంప్రదాయ చైనీస్" alt="సాంప్రదాయ చైనీస్" />
  <img src="../assets/flags/in.svg" width="32" title="హిందీ" alt="హిందీ" />
  <img src="../assets/flags/es.svg" width="32" title="స్పానిష్" alt="స్పానిష్" />
  <img src="../assets/flags/fr.svg" width="32" title="ఫ్రెంచ్" alt="ఫ్రెంచ్" />
  <img src="../assets/flags/sa.svg" width="32" title="అరబిక్" alt="అరబిక్" />
  <img src="../assets/flags/bd.svg" width="32" title="బెంగాలీ" alt="బెంగాలీ" />
  <img src="../assets/flags/pt.svg" width="32" title="పోర్చుగీస్" alt="పోర్చుగీస్" />
  <img src="../assets/flags/ru.svg" width="32" title="రష్యన్" alt="రష్యన్" />
  <img src="../assets/flags/pk.svg" width="32" title="ఉర్దూ" alt="ఉర్దూ" />
  <img src="../assets/flags/id.svg" width="32" title="ఇండోనేషియన్" alt="ఇండోనేషియన్" />
  <img src="../assets/flags/de.svg" width="32" title="జర్మన్" alt="జర్మన్" />
  <img src="../assets/flags/jp.svg" width="32" title="జపనీస్" alt="జపనీస్" />
  <img src="../assets/flags/tr.svg" width="32" title="టర్కిష్" alt="టర్కిష్" />
  <img src="../assets/flags/kr.svg" width="32" title="కొరియన్" alt="కొరియన్" />
  <img src="../assets/flags/it.svg" width="32" title="ఇటాలియన్" alt="ఇటాలియన్" />
  <img src="../assets/flags/nl.svg" width="32" title="డచ్" alt="డచ్" />
  <img src="../assets/flags/se.svg" width="32" title="స్వీడిష్" alt="స్వీడిష్" />
  <img src="../assets/flags/cz.svg" width="32" title="చెక్" alt="చెక్" />
  <img src="../assets/flags/gr.svg" width="32" title="గ్రీక్" alt="గ్రీక్" />
  <img src="../assets/flags/hu.svg" width="32" title="హంగేరియన్" alt="హంగేరియన్" />
  <img src="../assets/flags/il.svg" width="32" title="హీబ్రూ" alt="హీబ్రూ" />
  <img src="../assets/flags/no.svg" width="32" title="నార్వేజియన్" alt="నార్వేజియన్" />
  <img src="../assets/flags/dk.svg" width="32" title="డానిష్" alt="డానిష్" />
  <img src="../assets/flags/fi.svg" width="32" title="ఫిన్నిష్" alt="ఫిన్నిష్" />
  <img src="../assets/flags/sk.svg" width="32" title="స్లోవాక్" alt="స్లోవాక్" />
  <img src="../assets/flags/si.svg" width="32" title="స్లోవేనియన్" alt="స్లోవేనియన్" />
  <img src="../assets/flags/is.svg" width="32" title="ఐస్లాండిక్" alt="ఐస్లాండిక్" />
  <img src="../assets/flags/vn.svg" width="32" title="వియత్నామీస్" alt="వియత్నామీస్" />
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


## 🚀 ప్రధాన లక్షణాలు & వినియోగం

- **ఆధునిక నేటివ్ UI**: **Skia** ద్వారా శక్తివంతమైన అధిక-పనితీరు రెండరింగ్‌తో, డార్క్/లైట్ మోడ్, సజావుగా యానిమేషన్‌లతో సహజమైన GUI.
- **సిస్టమ్ ట్రే ఇంటిగ్రేషన్**: సమగ్ర ట్రే మద్దతు (దాదాపు 10MB మెమరీ వినియోగం), డబుల్-క్లిక్ ద్వారా చూపించు/దాచు టోగుల్ మరియు పూర్తి-ఫీచర్ రైట్-క్లిక్ మెనూ.
- **స్మార్ట్ స్టార్టప్**: బూట్‌పై ఆటో-స్టార్ట్, ట్రేకు మినిమైజ్ చేయడం (`/silent` ఆర్గ్యుమెంట్‌తో సైలెంట్ లాంచ్), మరియు నిష్క్రమణపై డిస్ట్రోలను ఆటో-క్లోజ్ చేయడం.
- **సమగ్ర ఇన్‌స్టాన్స్ నియంత్రణ**: ఒకే క్లిక్‌తో ప్రారంభించండి, ఆపండి, టెర్మినేట్ చేయండి మరియు రిజిస్టర్ తొలగించండి. నిజ-సమయ స్థితి పర్యవేక్షణ, డిస్క్ వినియోగం మరియు ఫైల్ స్థానాలపై లోతైన అవగాహన.
- **డిస్ట్రో నిర్వహణ**: డిఫాల్ట్‌గా సెట్ చేయడం, భౌతిక మైగ్రేషన్ (VHDX ను ఇతర డిస్క్‌కు తరలించడం), మరియు `.tar` లేదా `.tar.gz` ఆర్కైవ్‌గా ఎగుమతి/క్లోన్ చేయడం.
- **త్వరిత ఇంటిగ్రేషన్**: టెర్మినల్, VS Code లేదా ఫైల్ ఎక్స్‌ప్లోరర్‌కు ఒకే క్లిక్‌తో ప్రవేశం, అనుకూల వర్కింగ్ డైరెక్టరీ మరియు స్టార్టప్ స్క్రిప్ట్ హుక్స్ మద్దతు.
- **డిస్ట్రో ఇన్‌స్టాలేషన్**: Microsoft Store, GitHub, స్థానిక ఫైల్‌లు (RootFS/VHDX), లేదా ఆన్‌లైన్ మిర్రర్‌ల ద్వారా Linux డిస్ట్రిబ్యూషన్‌లను ఇన్‌స్టాల్ చేయండి (వేగవంతమైన మిర్రర్‌ను ఎంచుకోవడానికి ఆటో స్పీడ్ టెస్ట్ మరియు అంతర్నిర్మిత RootFS డౌన్‌లోడ్ సహాయకుడు).
- **గ్లోబల్ సేఫ్టీ**: ఏకకాల మైగ్రేషన్/బ్యాకప్ ఆపరేషన్ల భద్రత కోసం మ్యూటెక్స్ లాక్స్ ఉపయోగించడం, మరియు తొలగింపుపై Appx ప్యాకేజీల ఆటో-క్లీనప్.
- **అత్యంత తక్కువ మెమరీ వినియోగం**: అత్యధికంగా ఆప్టిమైజ్ చేయబడిన వనరుల సామర్థ్యం. సైలెంట్ స్టార్టప్ (సిస్టమ్ ట్రే) దాదాపు **10MB** మెమరీ. విండో మోడ్‌లో ఫాంట్ సంక్లిష్టత ఆధారంగా దాదాపు **18MB** (ఇంగ్లీష్, జర్మన్ వంటి ప్రామాణిక భాషలు) నుండి **38MB** (చైనీస్-జపనీస్-కొరియన్ వంటి పెద్ద క్యారెక్టర్ సెట్‌లు).
- **అధునాతన నెట్‌వర్క్ మేనేజ్‌మెంట్**: పోర్ట్ ఫార్వార్డింగ్ (ఆటో-క్రియేట్ ఫైర్‌వాల్ నియమాలు) మరియు గ్లోబల్ HTTP ప్రాక్సీ కాన్ఫిగరేషన్‌ను సజావుగా నిర్వహించడం, ఏకీకృత కనెక్టివిటీ అనుభవాన్ని సాధించడం.
- **USB పరికర నిర్వహణ**: `usbipd-win` తో లోతైన ఇంటిగ్రేషన్, డాష్‌బోర్డ్ UI లో నేరుగా అన్ని WSL ఇన్‌స్టాన్స్‌లలో స్థానిక USB పరికరాలను సులభంగా బైండ్, అటాచ్ మరియు నిర్వహించగలగడం.


## ⚙️ కాన్ఫిగరేషన్ & లాగ్‌లు

అన్ని కాన్ఫిగరేషన్లు "సెట్టింగ్స్" వీక్షణ ద్వారా నిర్వహించబడతాయి:

- కొత్త WSL ఇన్‌స్టాన్స్‌ల డిఫాల్ట్ ఇన్‌స్టాల్ డైరెక్టరీని ఎంచుకోండి.
- లాగ్ డైరెక్టరీ మరియు లాగ్ స్థాయిని కాన్ఫిగర్ చేయండి (Error / Warn / Info / Debug / Trace).
- UI భాషను ఎంచుకోండి లేదా సిస్టమ్ భాషను అనుసరించండి.
- డార్క్ మోడ్ టోగుల్ చేయండి, మరియు ఆపరేషన్ తర్వాత అనువర్తనం WSL ని ఆటో-క్లోజ్ చేస్తుందా లేదా.
- నవీకరణ తనిఖీ పౌనఃపున్యాన్ని కాన్ఫిగర్ చేయండి (రోజువారీ, వారానికి ఒకసారి, రెండు వారాలకు ఒకసారి, నెలవారీ).
- బూట్‌పై ఆటో-స్టార్ట్ ప్రారంభించండి (పాత్ ఆటో-ఫిక్స్ ఫీచర్‌తో).
- స్టార్టప్‌పై ట్రేకు మినిమైజ్ చేయడాన్ని సెట్ చేయండి.
- క్లోజ్ బటన్ ప్రవర్తనను కాన్ఫిగర్ చేయండి (ప్రోగ్రామ్ నుండి నిష్క్రమించడానికి బదులుగా ట్రేకు మినిమైజ్ చేయడం).
- నిర్దిష్ట ఫీచర్ ట్యాబ్‌ల దృశ్యమానతను టోగుల్ చేయడం ద్వారా సైడ్‌బార్‌ను అనుకూలీకరించండి.

లాగ్ ఫైల్స్ కాన్ఫిగర్ చేసిన లాగ్ డైరెక్టరీలో రాయబడతాయి, సమస్యలను నివేదించేటప్పుడు లాగ్‌లను జత చేయవచ్చు.


## 🖼️ సాఫ్ట్‌వేర్ స్క్రీన్‌షాట్‌లు

### హోమ్ ఇంటర్‌ఫేస్ (డార్క్ & లైట్ మోడ్)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB & మెనూ కుదింపు
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### నెట్‌వర్క్ మేనేజ్‌మెంట్
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### ఇన్‌స్టాన్స్ జోడించండి & సెట్టింగ్స్
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>
<p align="center">
  <img src="../assets/screenshot/settings-advanced.png" width="48%" />
  <img src="../assets/screenshot/settings-interface.png" width="48%" />
</p>

### గురించి & విరాళం
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/donate.png" width="48%" />
</p>

## 🎬 ఆపరేషన్ డెమో

[మాకు మెరుగుపరచడంలో సహాయపడండి! మా పరిచయ వీడియోను చూడండి మరియు మీ ఆలోచనలను పంచుకోండి.](https://github.com/owu/wsl-dashboard/discussions/9)



## 💻 సిస్టమ్ అవసరాలు

- WSL ప్రారంభించబడిన Windows 10 లేదా Windows 11 (WSL 2 సిఫార్సు చేయబడింది).
- కనీసం ఒక WSL డిస్ట్రిబ్యూషన్ ఇన్‌స్టాల్ చేయబడి ఉండాలి, లేదా కొత్త డిస్ట్రిబ్యూషన్ ఇన్‌స్టాల్ చేయడానికి అనుమతి ఉండాలి.
- 64-బిట్ CPU; బహుళ డిస్ట్రో వినియోగం కోసం 4 GB లేదా అంతకంటే ఎక్కువ RAM సిఫార్సు చేయబడింది.

## 📦 ఇన్‌స్టాలేషన్ గైడ్

### మార్గం 1: ప్రీ-బిల్ట్ బైనరీలను డౌన్‌లోడ్ చేయండి

అత్యంత సులభమైన మార్గం బిల్డ్ చేయబడిన వెర్షన్‌ను ఉపయోగించడం:

1. [GitHub Releases](https://github.com/owu/wsl-dashboard/releases) పేజీకి వెళ్ళండి.
2. Windows కోసం తాజా `wsldashboard` ఎగ్జిక్యూటబుల్‌ను డౌన్‌లోడ్ చేయండి.
3. డీకంప్రెస్ చేయండి (కంప్రెస్డ్ ఫైల్ అయితే) మరియు `wsldashboard.exe` ను రన్ చేయండి.

ఇన్‌స్టాలేషన్ అవసరం లేదు, ఈ అనువర్తనం సింగిల్-ఫైల్ పోర్టబుల్ ప్రోగ్రామ్.

### మార్గం 2: సోర్స్ నుండి బిల్డ్ చేయండి

Rust టూల్‌చైన్ (Rust 1.92+ లేదా తాజా వెర్షన్) ఇన్‌స్టాల్ చేయబడిందని నిర్ధారించుకోండి.

1. రిపోజిటరీని క్లోన్ చేయండి:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. బిల్డ్ చేసి రన్ చేయండి:

   - డెవలప్‌మెంట్ డీబగ్:

     ```powershell
     cargo run
     ```
   - ఆప్టిమైజ్ చేయబడిన రిలీజ్ బిల్డ్ కోసం బిల్డ్ స్క్రిప్ట్ ఉపయోగించండి:

     > బిల్డ్ స్క్రిప్ట్‌కు `x86_64-pc-windows-msvc` టూల్‌చైన్ అవసరం.

     ```powershell
     .\build\portable\build.ps1
     ```


## 🛠️ టెక్ స్టాక్ & పనితీరు

- **కెర్నల్**: మెమరీ సేఫ్టీ మరియు జీరో-కాస్ట్ అబ్‌స్ట్రాక్షన్స్ నిర్ధారించడానికి Rust లో అమలు చేయబడింది.
- **UI ఫ్రేమ్‌వర్క్**: అధిక-పనితీరు **Skia** రెండరింగ్ బ్యాకెండ్‌తో Slint.
- **అసింక్రోనస్ రన్‌టైమ్**: నాన్-బ్లాకింగ్ సిస్టమ్ కమాండ్‌లు మరియు I/O కోసం Tokio.
- **పనితీరు హైలైట్స్**:
  - **ప్రతిస్పందన వేగం**: దాదాపు తక్షణ స్టార్టప్ వేగం, WSL స్థితిని నిజ-సమయంలో పర్యవేక్షించడం.
  - **వనరుల సామర్థ్యం**: అత్యంత తక్కువ వనరుల వినియోగం (వివరాల కోసం [ప్రధాన లక్షణాలు](#-ప్రధాన-లక్షణాలు--వినియోగం) చూడండి).
  - **పోర్టబిలిటీ**: ఆప్టిమైజ్ చేయబడిన రిలీజ్ బిల్డ్ ఒకే కాంపాక్ట్ ఎగ్జిక్యూటబుల్‌ను ఉత్పత్తి చేస్తుంది.



## 🤝 కమ్యూనిటీ మద్దతు

ఈ కమ్యూనిటీల మద్దతుకు హృదయపూర్వక కృతజ్ఞతలు:

- [Rust Programming Language](https://www.rust-lang.org) - శక్తివంతమైన మరియు సురక్షితమైన ప్రోగ్రామింగ్ భాష
- [Slint | Declarative GUI for Rust, C++, JavaScript & Python](https://slint.dev) - ఆధునిక UI ఫ్రేమ్‌వర్క్
- [WSL: Windows Subsystem for Linux](https://github.com/microsoft/WSL) - అద్భుతమైన Windows Subsystem for Linux
- [Tokio - An asynchronous Rust runtime](https://tokio.rs) - సమర్థవంతమైన అసింక్రోనస్ రన్‌టైమ్
- [Windows Developer Community](https://developer.microsoft.com/en-us/windows/community) - నిరంతర ప్లాట్‌ఫారమ్ మెరుగుదలలు
- [Reddit](https://www.reddit.com) - ప్రపంచ కమ్యూనిటీ చర్చ & మద్దతు
- [Hacker News](https://news.ycombinator.com) - ప్రపంచ కమ్యూనిటీ చర్చ & మద్దతు
- [Linux.do](https://linux.do) - IT నిపుణుల కోసం ప్రముఖ కమ్యూనిటీ
- [V2EX](https://www.v2ex.com) - చైనీస్ టెక్ కమ్యూనిటీ చర్చ

మీ సహకారం మరియు అభిప్రాయం ఈ ప్రాజెక్ట్‌ను సాధ్యం చేశాయి!


## ❤️ ప్రాజెక్ట్‌కు మద్దతు ఇవ్వండి

- ఈ ప్రాజెక్ట్ GPL-3.0 ఓపెన్ సోర్స్ లైసెన్స్‌ను అనుసరిస్తుంది, అన్ని వినియోగదారులకు ఉచితంగా అందుబాటులో ఉంటుంది.
- ఫీచర్ డెవలప్‌మెంట్, రోజువారీ పరీక్షల నుండి బగ్ పరిష్కారం వరకు, అన్ని పనులు విరామ సమయంలో నిరంతరంగా జరుగుతాయి. ఓపెన్ సోర్స్ మార్గం ఒంటరిగా సులభం కాదు, మీ గుర్తింపు మరియు మద్దతు ప్రాజెక్ట్ దీర్ఘకాలం కొనసాగడానికి బలం.
- ఈ సాధనం మీకు నిజంగా సహాయపడిందని మీకు అనిపిస్తే, దయచేసి సహాయం అందించండి. అన్ని విరాళాలు సర్వర్ ఖర్చులు, వెర్షన్ అప్‌డేట్‌లు మరియు ఫీచర్ ఆప్టిమైజేషన్ కోసం ఉపయోగించబడతాయి.
- చిన్న దయ కూడా నక్షత్రంలా ప్రకాశిస్తుంది. మీ అవగాహన మరియు ఔదార్యానికి మరోసారి ధన్యవాదాలు!

మా విరాళ పేజీని సందర్శించండి: [https://www.wslui.com/donate/](https://www.wslui.com/donate/)


## ⭐️ ప్రేమతో నడిచే ప్రాజెక్ట్

ఈ ప్రాజెక్ట్ మీకు సహాయకరంగా ఉందని మీకు అనిపిస్తే, GitHub లో నక్షత్రం ఇవ్వమని మిమ్మల్ని అభ్యర్థిస్తున్నాను. మీ గుర్తింపు ప్రాజెక్ట్‌ను విస్తృత వినియోగదారులకు చేరుకోవడంలో సహాయపడుతుంది. ఈ ప్రోత్సాహం నన్ను నిరంతరం ముందుకు నడిపిస్తుంది.


## 📄 ఓపెన్ సోర్స్ లైసెన్స్

ఈ ప్రాజెక్ట్ GPL-3.0 లైసెన్స్ కింద లైసెన్స్ పొందింది – వివరాల కోసం [LICENSE](../LICENSE) ఫైల్ చూడండి.


---

Built with ❤️ for the WSL Community.
