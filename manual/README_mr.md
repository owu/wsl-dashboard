# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

आधुनिक, उच्च-कार्यक्षमता, हलके व कम मेमरी वापर असलेले WSL (Windows Subsystem for Linux) उदाहरण व्यवस्थापन डॅशबोर्ड. Rust आणि Slint वर आधारित, उत्कृष्ट मूळ अनुभव प्रदान करते.

---

```diff
सूचना:

- WSL Dashboard Microsoft Store मार्फत वितरित केले जात नाही.
- तिथे "WSL Dashboard" नावाने सूचीबद्ध कोणतेही अॅप्लिकेशन अनधिकृत आहे आणि बनावट असू शकते.
- संभाव्य फसवणुकी टाळण्यासाठी कृपया ते डाउनलोड करू नका.
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

I18N : [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | मराठी | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 अनुक्रमणिका
- [🌍 भाषा समर्थन](#-भाषा-समर्थन)
- [🚀 मुख्य वैशिष्ट्ये व वापर](#-मुख्य-वैशिष्ट्ये-व-वापर)
- [⚙️ कॉन्फिगरेशन व लॉग्स](#️-कॉन्फिगरेशन-व-लॉग्स)
- [🖼️ सॉफ्टवेअर स्क्रीनशॉट्स](#️-सॉफ्टवेअर-स्क्रीनशॉट्स)
- [🎬 ऑपरेशन डेमो](#-ऑपरेशन-डेमो)
- [💻 सिस्टम आवश्यकता](#-सिस्टम-आवश्यकता)
- [📦 स्थापना मार्गदर्शक](#-स्थापना-मार्गदर्शक)
- [🛠️ तंत्रज्ञान स्टॅक व कार्यक्षमता](#️-तंत्रज्ञान-स्टॅक-व-कार्यक्षमता)
- [🤝 समुदाय समर्थन](#-समुदाय-समर्थन)
- [❤️ या प्रकल्पाला समर्थन द्या](#️-या-प्रकल्पाला-समर्थन-द्या)
- [⭐️ प्रेमाने चालणारा प्रकल्प](#️-प्रेमाने-चालणारा-प्रकल्प)
- [📄 मुक्त स्रोत परवाना](#-मुक्त-स्रोत-परवाना)

---

## 🌍 भाषा समर्थन

इंग्रजी, सरलीकृत चिनी, पारंपरिक चिनी, हिंदी, स्पॅनिश, फ्रेंच, अरबी, बंगाली, पोर्तुगीज, रशियन, उर्दू, इंडोनेशियन, जर्मन, जपानी, तुर्की, कोरियन, इटालियन, डच, स्वीडिश, झेक, ग्रीक, हंगेरियन, हिब्रू, नॉर्वेजियन, डॅनिश, फिन्निश, स्लोव्हाक, स्लोव्हेनियन, आइसलँडिक, व्हिएतनामी, तेलुगू, जावानीज, थाई, तमिळ, फिलिपिनो, पंजाबी, मलय, पोलिश, उक्रेनियन, पर्शियन, कन्नड, मराठी, हौसा, बर्मीझ, उझ्बेक, अझरबैजानी, सेबुआनो, मल्याळम, सिंधी, अम्हारिक

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="इंग्रजी" alt="इंग्रजी" />
  <img src="../assets/flags/cn.svg" width="32" title="सरलीकृत चिनी" alt="सरलीकृत चिनी" />
  <img src="../assets/flags/tw.svg" width="32" title="पारंपरिक चिनी" alt="पारंपरिक चिनी" />
  <img src="../assets/flags/in.svg" width="32" title="हिंदी" alt="हिंदी" />
  <img src="../assets/flags/es.svg" width="32" title="स्पॅनिश" alt="स्पॅनिश" />
  <img src="../assets/flags/fr.svg" width="32" title="फ्रेंच" alt="फ्रेंच" />
  <img src="../assets/flags/sa.svg" width="32" title="अरबी" alt="अरबी" />
  <img src="../assets/flags/bd.svg" width="32" title="बंगाली" alt="बंगाली" />
  <img src="../assets/flags/pt.svg" width="32" title="पोर्तुगीज" alt="पोर्तुगीज" />
  <img src="../assets/flags/ru.svg" width="32" title="रशियन" alt="रशियन" />
  <img src="../assets/flags/pk.svg" width="32" title="उर्दू" alt="उर्दू" />
  <img src="../assets/flags/id.svg" width="32" title="इंडोनेशियन" alt="इंडोनेशियन" />
  <img src="../assets/flags/de.svg" width="32" title="जर्मन" alt="जर्मन" />
  <img src="../assets/flags/jp.svg" width="32" title="जपानी" alt="जपानी" />
  <img src="../assets/flags/tr.svg" width="32" title="तुर्की" alt="तुर्की" />
  <img src="../assets/flags/kr.svg" width="32" title="कोरियन" alt="कोरियन" />
  <img src="../assets/flags/it.svg" width="32" title="इटालियन" alt="इटालियन" />
  <img src="../assets/flags/nl.svg" width="32" title="डच" alt="ड�" />
  <img src="../assets/flags/se.svg" width="32" title="स्वीडिश" alt="स्वीडिश" />
  <img src="../assets/flags/cz.svg" width="32" title="झेक" alt="झेक" />
  <img src="../assets/flags/gr.svg" width="32" title="ग्रीक" alt="ग्रीक" />
  <img src="../assets/flags/hu.svg" width="32" title="हंगेरियन" alt="हंगेरियन" />
  <img src="../assets/flags/il.svg" width="32" title="हिब्रू" alt="हिब्रू" />
  <img src="../assets/flags/no.svg" width="32" title="नॉर्वेजियन" alt="नॉर्वेजियन" />
  <img src="../assets/flags/dk.svg" width="32" title="डॅनिश" alt="डॅनिश" />
  <img src="../assets/flags/fi.svg" width="32" title="फिन्निश" alt="फिन्निश" />
  <img src="../assets/flags/sk.svg" width="32" title="स्लोव्हाक" alt="स्लोव्हाक" />
  <img src="../assets/flags/si.svg" width="32" title="स्लोव्हेनियन" alt="स्लोव्हेनियन" />
  <img src="../assets/flags/is.svg" width="32" title="आइसलँडिक" alt="आइसलँडिक" />
  <img src="../assets/flags/vn.svg" width="32" title="व्हिएतनामी" alt="व्हिएतनामी" />
  <img src="../assets/flags/in.svg" width="32" title="तेलुगू" alt="तेलुगू" />
  <img src="../assets/flags/id.svg" width="32" title="जावानीज" alt="जावानीज" />
  <img src="../assets/flags/th.svg" width="32" title="थाई" alt="थाई" />
  <img src="../assets/flags/in.svg" width="32" title="तमिळ" alt="तमिळ" />
  <img src="../assets/flags/ph.svg" width="32" title="फिलिपिनो" alt="फिलिपिनो" />
  <img src="../assets/flags/in.svg" width="32" title="पंजाबी" alt="पंजाबी" />
  <img src="../assets/flags/my.svg" width="32" title="मलय" alt="मलय" />
  <img src="../assets/flags/pl.svg" width="32" title="पोलिश" alt="पोलिश" />
  <img src="../assets/flags/ua.svg" width="32" title="उक्रेनियन" alt="उक्रेनियन" />
  <img src="../assets/flags/ir.svg" width="32" title="पर्शियन" alt="पर्शियन" />
  <img src="../assets/flags/in.svg" width="32" title="कन्नड" alt="कन्नड" />
  <img src="../assets/flags/in.svg" width="32" title="मराठी" alt="मराठी" />
  <img src="../assets/flags/ng.svg" width="32" title="हौसा" alt="हौसा" />
  <img src="../assets/flags/mm.svg" width="32" title="बर्मीझ" alt="बर्मीझ" />
  <img src="../assets/flags/uz.svg" width="32" title="उझ्बेक" alt="उझ्बेक" />
  <img src="../assets/flags/az.svg" width="32" title="अझरबैजानी" alt="अझरबैजानी" />
  <img src="../assets/flags/ph.svg" width="32" title="सेबुआनो" alt="सेबुआनो" />
  <img src="../assets/flags/in.svg" width="32" title="मल्याळम" alt="मल्याळम" />
  <img src="../assets/flags/pk.svg" width="32" title="सिंधी" alt="सिंधी" />
  <img src="../assets/flags/et.svg" width="32" title="अम्हारिक" alt="अम्हारिक" />
</p>


## 🚀 मुख्य वैशिष्ट्ये व वापर

- **आधुनिक मूळ UI**: **Skia** द्वारे चालवलेली उच्च-कार्यक्षमता रेंडरिंग, डार्क/लाइट मोड, सुलभ अॅनिमेशन्ससह सहज GUI.
- **सिस्टम ट्रे एकत्रीकरण**: सर्वसमावेशक ट्रे समर्थन (सुमारे 10MB मेमरी वापर), डबल-क्लिकने दाखवा/लपवा टॉगल आणि पूर्ण-वैशिष्ट्य राइट-क्लिक मेनू.
- **स्मार्ट स्टार्टअप**: बूटवर ऑटो-स्टार्ट, ट्रेवर मिनिमाइझ करणे (`/silent` वादाने सायलेंट लॉन्च), आणि बाहेर पडताना डिस्ट्रो ऑटो-क्लोज.
- **सर्वसमावेशक उदाहरण नियंत्रण**: एका क्लिकने सुरू करा, थांबवा, समाप्त करा आणि नोंदणी रद्द करा. रिअल-टाइम स्थिती निरीक्षण, डिस्क वापर व फाइल स्थानांवर सखोल नजर.
- **डिस्ट्रो व्यवस्थापन**: डीफॉल्ट म्हणून सेट करणे, भौतिक स्थलांतर (VHDX दुसऱ्या डिस्कवर हलवणे), आणि `.tar` किंवा `.tar.gz` आर्काइव्हम्हणून निर्यात/क्लोन.
- **जलद एकत्रीकरण**: टर्मिनल, VS Code किंवा फाइल एक्सप्लोररला एका क्लिकने प्रवेश, सानुकूल कार्य निर्देशिका व स्टार्टअप स्क्रिप्ट हुक्स समर्थन.
- **डिस्ट्रो स्थापना**: Microsoft Store, GitHub, स्थानिक फाइली (RootFS/VHDX), किंवा ऑनलाइन मिरर द्वारे Linux वितरणे स्थापित करा (सर्वात जलद मिरर निवडण्यासाठी ऑटो स्पीड टेस्ट आणि अंतर्निहित RootFS डाउनलोड सहाय्यक).
- **जागतिक सुरक्षा**: एकाच वेळी स्थलांतर/बॅकअप ऑपरेशन्सच्या सुरक्षेसाठी म्युटेक्स लॉक्स वापरणे, आणि काढून टाकताना Appx पॅकेजेसचे ऑटो-क्लीनअप.
- **अत्यंत कम मेमरी वापर**: अत्यधिक ऑप्टिमाइझ केलेली संसाधन कार्यक्षमता. सायलेंट स्टार्टअप (सिस्टम ट्रे) सुमारे **10MB** मेमरी. विंडो मोडमध्ये फॉन्ट गुंतागुंतीनुसार सुमारे **18MB** (इंग्रजी, जर्मन सारखी मानक भाषा) ते **38MB** (चिनी-जपानी-कोरियन सारखे मोठे अक्षर संच).
- **प्रगत नेटवर्क व्यवस्थापन**: पोर्ट फॉरवर्डिंग (ऑटो-क्रिएट फायरवॉल नियम) आणि जागतिक HTTP प्रॉक्सी कॉन्फिगरेशन सुलभतेने व्यवस्थापित करणे, एकत्रित कनेक्टिव्हिटी अनुभव साध्य करणे.
- **USB उपकरण व्यवस्थापन**: `usbipd-win` शी सखोल एकत्रीकरण, डॅशबोर्ड UI मध्ये थेट सर्व WSL उदाहरणांमधील स्थानिक USB उपकरणे सहजतेने बाइंड, अटॅच व व्यवस्थापित करता येतात.


## ⚙️ कॉन्फिगरेशन व लॉग्स

सर्व कॉन्फिगरेशन "सेटिंग्ज" दृश्याद्वारे व्यवस्थापित केली जातात:

- नवीन WSL उदाहरणांची डीफॉल्ट स्थापना निर्देशिका निवडा.
- लॉग निर्देशिका आणि लॉग स्तर कॉन्फिगर करा (Error / Warn / Info / Debug / Trace).
- UI भाषा निवडा किंवा सिस्टम भाषा अनुसरा.
- डार्क मोड टॉगल करा, आणि ऑपरेशननंतर ॲप WSL ऑटो-क्लोज करते का.
- अपडेट तपासणी वारंवारता कॉन्फिगर करा (दररोज, साप्ताहिक, पाक्षिक, मासिक).
- बूटवर ऑटो-स्टार्ट सक्षम करा (मार्ग ऑटो-फिक्स वैशिष्ट्यासह).
- स्टार्टअपवर ट्रेवर मिनिमाइझ करणे सेट करा.
- बंद बटण वर्तन कॉन्फिगर करा (प्रोग्रामवरून बाहेर पडण्याऐवजी ट्रेवर मिनिमाइझ करणे).
- विशिष्ट वैशिष्ट्य टॅब्सची दृश्यमानता टॉगल करून साइडबार सानुकूलित करा.

लॉग फाइल्स कॉन्फिगर केलेल्या लॉग निर्देशिकेत लिहिल्या जातात, समस्या नोंदवताना लॉग्स जोडता येतात.


## 🖼️ सॉफ्टवेअर स्क्रीनशॉट्स

### मुख्य इंटरफेस (डार्क & लाइट मोड)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB & मेनू कोलॅप्स
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### नेटवर्क व्यवस्थापन
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### उदाहरण जोडा & सेटिंग्ज
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>
<p align="center">
  <img src="../assets/screenshot/settings-advanced.png" width="48%" />
  <img src="../assets/screenshot/settings-interface.png" width="48%" />
</p>

### बद्दल & देणगी
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/donate.png" width="48%" />
</p>

## 🎬 ऑपरेशन डेमो

[आम्हाला सुधारण्यात मदत करा! आमचे परिचय व्हिडिओ पहा आणि आपले विचार शेअर करा.](https://github.com/owu/wsl-dashboard/discussions/9)



## 💻 सिस्टम आवश्यकता

- WSL सक्षम असलेले Windows 10 किंवा Windows 11 (WSL 2 शिफारस).
- किमान एक WSL वितरण स्थापित असावे, किंवा नवीन वितरण स्थापित करण्याची परवानगी असावी.
- 64-बिट CPU; बहु डिस्ट्रो वापरासाठी 4 GB किंवा अधिक RAM शिफारस.

## 📦 स्थापना मार्गदर्शक

### पद्धत 1: पूर्व-निर्मित बायनरी डाउनलोड करा

सर्वात सोपी पद्धत बिल्ट आवृत्ती वापरणे:

1. [GitHub Releases](https://github.com/owu/wsl-dashboard/releases) पृष्ठावर जा.
2. Windows साठी नवीनतम `wsldashboard` एक्झिक्युटेबल डाउनलोड करा.
3. डीकंप्रेस करा (कम्प्रेस्ड फाइल असल्यास) आणि `wsldashboard.exe` चालवा.

स्थापना आवश्यक नाही, हे ॲप सिंगल-फाइल पोर्टेबल प्रोग्राम आहे.

### पद्धत 2: स्रोतावरून बिल्ड करा

Rust टूलचेन (Rust 1.92+ किंवा नवीनतम) स्थापित असल्याची खात्री करा.

1. रिपॉझिटरी क्लोन करा:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. बिल्ड करा आणि चालवा:

   - विकास डीबग:

     ```powershell
     cargo run
     ```
   - ऑप्टिमाइझ केलेले रिलीज बिल्डसाठी बिल्ड स्क्रिप्ट वापरा:

     > बिल्ड स्क्रिप्टला `x86_64-pc-windows-msvc` टूलचेन आवश्यक आहे.

     ```powershell
     .\build\portable\build.ps1
     ```


## 🛠️ तंत्रज्ञान स्टॅक व कार्यक्षमता

- **कर्नल**: मेमरी सुरक्षा आणि झीरो-कॉस्ट अॅब्स्ट्रॅक्शन्स सुनिश्चित करण्यासाठी Rust मध्ये अंमलबजावणी.
- **UI फ्रेमवर्क**: उच्च-कार्यक्षमता **Skia** रेंडरिंग बॅकएंडसह Slint.
- **असिंक्रोनस रनटाइम**: नॉन-ब्लॉकिंग सिस्टम कमांड व I/O साठी Tokio.
- **कार्यक्षमता ठळक वैशिष्ट्ये**:
  - **प्रतिसाद गती**: जवळजवळ तत्काल स्टार्टअप गती, WSL स्थिती रिअल-टाइममध्ये निरीक्षण.
  - **संसाधन कार्यक्षमता**: अत्यंत कम संसाधन वापर (तपशीलांसाठी [मुख्य वैशिष्ट्ये](#-मुख्य-वैशिष्ट्ये-व-वापर) पहा).
  - **पोर्टेबिलिटी**: ऑप्टिमाइझ केलेले रिलीज बिल्ड एक संकुचित एक्झिक्युटेबल तयार करते.



## 🤝 समुदाय समर्थन

या समुदायांच्या समर्थनाबद्दल हृदयपूर्वक आभार:

- [Rust Programming Language](https://www.rust-lang.org) - शक्तिशाली व सुरक्षित प्रोग्रामिंग भाषा
- [Slint | Declarative GUI for Rust, C++, JavaScript & Python](https://slint.dev) - आधुनिक UI फ्रेमवर्क
- [WSL: Windows Subsystem for Linux](https://github.com/microsoft/WSL) - उत्कृष्ट Windows Subsystem for Linux
- [Tokio - An asynchronous Rust runtime](https://tokio.rs) - कार्यक्षम असिंक्रोनस रनटाइम
- [Windows Developer Community](https://developer.microsoft.com/en-us/windows/community) - सतत प्लॅटफॉर्म सुधारणा
- [Reddit](https://www.reddit.com) - जागतिक समुदाय चर्चा व समर्थन
- [Hacker News](https://news.ycombinator.com) - जागतिक समुदाय चर्चा व समर्थन
- [Linux.do](https://linux.do) - IT व्यावसायिकांसाठी लोकप्रिय समुदाय
- [V2EX](https://www.v2ex.com) - चिनी तंत्रज्ञान समुदाय चर्चा

तुमचे योगदान व अभिप्राय या प्रकल्पाला शक्य बनवतात!


## ❤️ या प्रकल्पाला समर्थन द्या

- हा प्रकल्प GPL-3.0 मुक्त स्रोत परवान्याचे अनुसरण करतो, सर्व वापरकर्त्यांसाठी मोफत.
- वैशिष्ट्य विकास, दैनिक चाचण्यांपासून बग निवारणापर्यंत, सर्व काम मोकळ्या वेळेतील सातत्यातून येते. मुक्त स्रोत मार्ग एकट्याने सोपा नाही, तुमची मान्यता व समर्थन प्रकल्प दीर्घकाल चालू ठेवण्यासाठी बळ आहे.
- हे साधन तुम्हाला खरोखर उपयोगी वाटल्यास, कृपया हातभार लावा. सर्व देणग्या सर्व्हर खर्च, आवृत्ती सुधारणा व वैशिष्ट्य ऑप्टिमायझेशनसाठी वापरल्या जातात.
- छोटी कृपाही तारा म्हणून चमकते. तुमच्या समजूतदारपणाबद्दल व उदारतेबद्दल पुन्हा धन्यवाद!

आमचे देणगी पृष्न भेट द्या: [https://www.wslui.com/donate/](https://www.wslui.com/donate/)


## ⭐️ प्रेमाने चालणारा प्रकल्प

हा प्रकल्प तुम्हाला उपयोगी वाटल्यास, GitHub वर तारा देऊन तुमची मान्यता द्याल अशी मी विनंती करतो. तुमची मान्यता प्रकल्पाला व्यापक वापरकर्त्यांपर्यंत पोहोचण्यात मदत करेल. हा प्रोत्साहन मला सतत पुढे नेतो.


## 📄 मुक्त स्रोत परवाना

हा प्रकल्प GPL-3.0 परवान्यांतर्गत परवानाकृत आहे – तपशीलांसाठी [LICENSE](../LICENSE) फाइल पहा.


---

Built with ❤️ for the WSL Community.