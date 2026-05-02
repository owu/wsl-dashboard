# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

WSL (Windows Subsystem for Linux) इंस्टेंस प्रबंधन के लिए एक आधुनिक, उच्च-प्रदर्शन और हल्का डैशबोर्ड। प्रीमियम नेटिव अनुभव के लिए Rust और Slint के साथ निर्मित।

---

```diff
ध्यान दें:

- इस सॉफ़्टवेयर को Microsoft Store के माध्यम से वितरित नहीं किया जाता।
- वहाँ समान नाम के तहत सूचीबद्ध कोई भी एप्लिकेशन अनधिकृत है और जाली हो सकता है।
- कृपया इसे डाउनलोड न करें।
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

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | हिन्दी | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md)

---

## 📑 विषय-सूची
- [🌍 भाषा समर्थन](#-भाषा-समर्थन)
- [🚀 मुख्य विशेषताएं और उपयोग](#-मुख्य-विशेषताएं-और-उपयोग)
- [⚙️ कॉन्फ़िगरेशन और लॉग](#️-कॉन्फ़िगरेशन-और-लॉग)
- [🖼️ स्क्रीनशॉट](#️-स्क्रीनशॉट)
- [🎬 ऑपरेशन डेमो](#-ऑपरेशन-डेमो)
- [💻 सिस्टम आवश्यकताएं](#-सिस्टम-आवश्यकताएं)
- [📦 इंस्टॉलेशन गाइड](#-इंस्टॉलेशन-गाइड)
- [🛠️ टेक स्टैक और प्रदर्शन](#️-टेक-स्टैक-और-प्रदर्शन)
- [⭐️ प्यार का श्रम](#️-प्यार-का-श्रम)
- [📄 लाइसेंस](#-लाइसेन्स)

---

## 🌍 भाषा समर्थन

अंग्रेज़ी, चीनी (सरल), चीनी (पारंपरिक), हिन्दी, स्पेनिश, फ्रेंच, अरबी, बंगाली, पुर्तगाली, रूसी, उर्दू, इंडोनेशियाई, जर्मन, जापानी, तुर्की, कोरियाई, इतालवी, डच, स्वीडिश, चेक, ग्रीक, हंगेरियन, हिब्रू, नार्वेजियन, डेनिश, फिनिश, स्लोवाक, स्लोवेनियाई, आइसलैंडिक

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="अंग्रेज़ी" alt="अंग्रेज़ी" />
  <img src="../assets/flags/cn.svg" width="32" title="चीनी (सरल)" alt="चीनी (सरल)" />
  <img src="../assets/flags/tw.svg" width="32" title="चीनी (पारंपरिक)" alt="चीनी (पारंपरिक)" />
  <img src="../assets/flags/in.svg" width="32" title="हिन्दी" alt="हिन्दी" />
  <img src="../assets/flags/es.svg" width="32" title="स्पेनिश" alt="स्पेनिश" />
  <img src="../assets/flags/fr.svg" width="32" title="फ्रेंच" alt="फ्रेंच" />
  <img src="../assets/flags/sa.svg" width="32" title="अरबी" alt="अरबी" />
  <img src="../assets/flags/bd.svg" width="32" title="बंगाली" alt="बंगाली" />
  <img src="../assets/flags/pt.svg" width="32" title="पुर्तगाली" alt="पुर्तगाली" />
  <img src="../assets/flags/ru.svg" width="32" title="रूसी" alt="रूसी" />
  <img src="../assets/flags/pk.svg" width="32" title="उर्दू" alt="उर्दू" />
  <img src="../assets/flags/id.svg" width="32" title="इंडोनेशियाई" alt="इंडोनेशियाई" />
  <img src="../assets/flags/de.svg" width="32" title="जर्मन" alt="जर्मन" />
  <img src="../assets/flags/jp.svg" width="32" title="जापानी" alt="जापानी" />
  <img src="../assets/flags/tr.svg" width="32" title="तुर्की" alt="तुर्की" />
  <img src="../assets/flags/kr.svg" width="32" title="कोरियाई" alt="कोरियाई" />
  <img src="../assets/flags/it.svg" width="32" title="इतालवी" alt="इतालवी" />
  <img src="../assets/flags/nl.svg" width="32" title="डच" alt="डच" />
  <img src="../assets/flags/se.svg" width="32" title="स्वीडिश" alt="स्वीडिश" />
  <img src="../assets/flags/cz.svg" width="32" title="चेक" alt="चेक" />
  <img src="../assets/flags/gr.svg" width="32" title="ग्रीक" alt="ग्रीक" />
  <img src="../assets/flags/hu.svg" width="32" title="हंगेरियन" alt="हंगेरियन" />
  <img src="../assets/flags/il.svg" width="32" title="हिब्रू" alt="हिब्रू" />
  <img src="../assets/flags/no.svg" width="32" title="नार्वेजियन" alt="नार्वेजियन" />
  <img src="../assets/flags/dk.svg" width="32" title="डेनिश" alt="डेनिश" />
  <img src="../assets/flags/fi.svg" width="32" title="फिनिश" alt="फिनिश" />
  <img src="../assets/flags/sk.svg" width="32" title="स्लोवाक" alt="स्लोवाक" />
  <img src="../assets/flags/si.svg" width="32" title="स्लोवेनियाई" alt="स्लोवेनियाई" />
  <img src="../assets/flags/is.svg" width="32" title="आइसलैंडिक" alt="आइसलैंडिक" />
</p>


## 🚀 मुख्य विशेषताएं और उपयोग

- **आधुनिक नेटिव UI**: सहज ज्ञान युक्त GUI, डार्क/लाइट मोड सपोर्ट के साथ, सहज एनिमेशन, और **Skia** द्वारा संचालित उच्च-प्रदर्शन रेंडरिंग।
- **सिस्टम ट्रे एकीकरण**: सिस्टम ट्रे मिनिमाइज का पूर्ण समर्थन (केवल ~10MB रैम उपयोग), डबल-क्लिक से टॉगल करें, और एक कार्यात्मक राइट-क्लिक मेनू।
- **इंटेलिजेंट स्टार्टअप**: विंडोज के साथ डैशबोर्ड शुरू करना, ट्रे में मिनिमाइज करना (`/silent` विकल्प के साथ साइलेंट मोड), और बाहर निकलने पर वितरण को स्वचालित रूप से बंद करना।
- **व्यापक इंस्टेंस नियंत्रण**: वन-क्लिक प्रारंभ, रोकें, समाप्त करना और अनरजिस्टर। रीयल-टाइम स्थिति निगरानी और डिस्क उपयोग एवं फ़ाइल स्थान के बारे में विस्तृत जानकारी।
- **डिस्ट्रो प्रबंधन**: डिफ़ॉल्ट के रूप में सेट करें, माइग्रेशन (VHDX को अन्य ड्राइव पर ले जाना), और `.tar` या `.tar.gz` अभिलेखागार में निर्यात/क्लोन करें।
- **त्वरित एकीकरण**: टर्मिनल, VS Code या फ़ाइल एक्सप्लोरर में तुरंत लॉन्च, अनुकूलन योग्य कार्य निर्देशिकाओं और स्टार्टअप स्क्रिप्ट हुक के साथ।
- **स्मार्ट इंस्टॉलेशन**: Microsoft Store, GitHub या स्थानीय फ़ाइलों (RootFS/VHDX) से इंस्टॉल करें। अंतर्निहित RootFS डाउनलोड सहायक शामिल है।
- **वैश्विक सुरक्षा**: सुरक्षित समवर्ती माइग्रेशन/बैकअप संचालन के लिए म्यूटेक्स लॉक और हटाए जाने पर स्वचालित Appx सफाई।
- **अल्ट्रा-लो मेमोरी फुटप्रिंट**: दक्षता के लिए अत्यधिक अनुकूलित। साइलेंट स्टार्टअप (सिस्टम ट्रे) केवल **~10MB** रैम उपयोग करता है। विंडो मोड उपयोग फ़ॉन्ट जटिलता के अनुसार बदलता है: मानक भाषाओं के लिए **~18MB** और जटिल फ़ॉन्ट वाली भाषाओं (चीनी, जापानी, कोरियाई) के लिए **~38MB**।
- **उन्नत नेटवर्किंग**: निर्बाध पोर्ट फ़ॉरवर्डिंग प्रबंधन (स्वचालित फ़ायरवॉल नियम निर्माण के साथ) और एकीकृत कनेक्टिविटी के लिए वैश्विक HTTP प्रॉक्सी कॉन्फ़िगरेशन।
- **USB डिवाइस प्रबंधन**: डैशबोर्ड UI से सीधे आपके WSL उदाहरणों में स्थानीय USB उपकरणों को आसानी से बाइंड करने, संलग्न करने और प्रबंधित करने के लिए `usbipd-win` के साथ पूर्ण एकीकरण।


## ⚙️ कॉन्फ़िगरेशन और लॉग

सभी कॉन्फ़िगरेशन सेटिंग्स दृश्य के माध्यम से प्रबंधित किए जाते हैं:

- नए WSL इंस्टेंस के लिए डिफ़ॉルト स्थापित निर्देशिका चुनें।
- लॉग निर्देशिका और लॉग स्तर (Error / Warn / Info / Debug / Trace) कॉन्फ़िगर करें।
- UI भाषा चुनें या इसे सिस्टम भाषा का पालन करने दें।
- डार्क मोड टॉगल करें और क्या ऐप बन्द होने पर WSL को स्वचालित रूप से बंद करे।
- कॉन्फ़िगर करें कि ऐप अपडेट के लिए कितनी बार जाँच करता है (प्रतिदिन, साप्ताहिक, पाक्षिक, मासिक)।
- सिस्टम स्टार्टअप पर स्वचालित शुरुआत सक्षम करें (स्वचालित पथ मरम्मत के साथ)।
- ऐप को स्टार्टअप पर सिस्टम ट्रे में मिनिमाइज करने के लिए सेट करें।
- क्लोज बटन को प्रोग्राम बंद करने के बजाय सिस्टम ट्रे में मिनिमाइज करने के लिए सेट करें।
- विशिष्ट सुविधा टैब की दृश्यता को टॉगल करके साइडबार को अनुकूलित करें।

लॉग फ़ाइलें कॉन्फ़िगर की गई लॉग निर्देशिका में लिखी जाती हैं और समस्याओं की रिपोर्ट करते समय संलग्न की जा सकती हैं।


## 🖼️ स्क्रीनशॉट

### होम (डार्क और लाइट मोड)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB & सरलीकृत मेनू
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### नेटवर्क
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### इंस्टेंस जोड़ें और सेटिंग्स
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>

### परिचय
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
</p>

## 🎬 ऑपरेशन डेमो

नीचे कार्य करते हुए WSL Dashboard का प्रदर्शन दिया गया है:

![WSL Dashboard Demo](../assets/screenshot/demo.gif)



## 💻 सिस्टम आवश्यकताएं

- WSL सक्षम के साथ Windows 10 या Windows 11 (WSL 2 अनुशंसित)।
- कम से कम एक WSL वितरण स्थापित, या नए स्थापित करने की अनुमति।
- 64-बिट CPU; सुचारू मल्टी-डिस्ट्रो उपयोग के लिए 4 GB रैम या अधिक अनुशंसित।

## 📦 इंस्टॉलेशन गाइड

### विकल्प 1: प्रीबिल्ट बाइनरी डाउनलोड करें

शुरू करने का सबसे आसान तरीका प्री-कंपाइल्ड रिलीज़ का उपयोग करना है:

1. [GitHub Releases](https://github.com/owu/wsl-dashboard/releases) पेज पर जाएं।
2. Windows के लिए नवीनतम `wsldashboard` निष्पादन योग्य फ़ाइल डाउनलोड करें।
3. निकालें (यदि संकुलित है) और `wsldashboard.exe` चलाएँ।

किसी इंस्टॉलर की आवश्यकता नहीं है; ऐप एक एकल पोर्टेबल बाइनरी है।

### विकल्प 2: स्रोत से निर्माण करें

सुनिश्चित करें कि आपके पास Rust टूलचेन (Rust 1.92+ या नया) स्थापित है।

1. रिपॉजिटरी क्लोन करें:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. निर्माण करें और चलाएँ:

   - विकास के लिए:

     ```powershell
     cargo run
     ```
   - बिल्ड स्क्रिप्ट का उपयोग करके ऑप्टिमाइज्ड रिलीज़ निर्माण करें:

     > बिल्ड स्क्रिप्ट के लिए `x86_64-pc-windows-msvc` टूलचेन की आवश्यकता होती है।

     ```powershell
     .\build\scripts\build.ps1
     ```


## 🛠️ टेक स्टैक और प्रदर्शन

- **कोर**: मेमोरी सुरक्षा और शून्य-लागत अमूर्तता के लिए Rust में कार्यान्वित।
- **UI फ्रेमवर्क**: उच्च-प्रदर्शन **Skia** रेंडरिंग बैकएंड के साथ Slint।
- **एसिंक रनटाइम**: गैर-अवरुद्ध सिस्टम कमांड और I/O के लिए Tokio।
- **प्रदर्शन मुख्य विशेषताएं**:
  - **जवाबदेही**: लगभग त्वरित स्टार्टअप और रीयल-टाइम WSL स्थिति की निगरानी।
  - **दक्षता**: अल्ट्रा-लो रिसोर्स उपयोग (विवरण के लिए [मुख्य विशेषताएं](#-मुख्य-विशेषताएं-और-उपयोग) देखें)।
  - **पोर्टेबिलिटी**: अनुकूलित रिलीज़ निर्माण एक एकल कॉम्पैक्ट निष्पादन योग्य बनाता है।



## ⭐️ प्यार का श्रम

यदि आपको यह प्रोजेक्ट उपयोगी लगा, तो मैं आभारी रहूँगा यदि आप GitHub पर एक स्टार छोड़ सकें। आपका समर्थन इसे व्यापक दर्शकों तक पहुँचाने में मदद करता है और इसकी गहराई से सराहना की जाती है। यही प्रोत्साहन मुझे निर्माण जारी रखने के लिए प्रेरित करता है।

## 📄 लाइसेंस

यह प्रोजेक्ट GPL-3.0 के तहत लाइसेंस प्राप्त है - विवरण के लिए [LICENSE](../LICENSE) फ़ाइल देखें।

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
