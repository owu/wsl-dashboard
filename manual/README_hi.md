# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

एक आधुनिक, उच्च-प्रदर्शन, हल्का और कम-मेमोरी वाला WSL (लिनक्स के लिए विंडोज़ सबसिस्टम) इंस्टेंस प्रबंधन डैशबोर्ड। प्रीमियम नेटिव अनुभव के लिए Rust और Slint के साथ बनाया गया।

```diff
सूचना:

- WSL Dashboard Microsoft Store के माध्यम से वितरित नहीं किया जाता है।
- वहां "WSL Dashboard" नाम से सूचीबद्ध कोई भी एप्लिकेशन अनधिकृत है और नकली हो सकता है।
- संभावित घोटालों से बचने के लिए कृपया इसे डाउनलोड न करें।
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

README I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | हिन्दी | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 विषय-सूची
[🌍 भाषा समर्थन](#-भाषा-समर्थन) | [🚀 मुख्य विशेषताएं और उपयोग](#-मुख्य-विशेषताएं-और-उपयोग) | [🖼️ स्क्रीनशॉट](#️-स्क्रीनशॉट) | [📦 इंस्टॉलेशन गाइड](#-इंस्टॉलेशन-गाइड) | [📢 आधिकारिक सामूदायिक समूह](#-आधिकारिक-सामुदायिक-समूह) | [❤️ इस प्रोजेक्ट को सपोर्ट करें](#️-इस-प्रोजेक्ट-को-सपोर्ट-करें) | [⭐️ प्यार का श्रम](#️-प्यार-का-श्रम) | [📄 लाइसेंस](#-लाइसेंस)

---

## 🌍 भाषा समर्थन

अंग्रेज़ी, चीनी (सरल), चीनी (पारंपरिक), हिन्दी, स्पेनिश, फ्रेंच, अरबी, बंगाली, पुर्तगाली, रूसी, उर्दू, इंडोनेशियाई, जर्मन, जापानी, तुर्की, कोरियाई, इतालवी, डच, स्वीडिश, चेक, ग्रीक, हंगेरियन, हिब्रू, नार्वेजियन, डेनिश, फिनिश, स्लोवाक, स्लोवेनियाई, आइसलैंडिक, वियेतनामी, तेलुगु, जावानीज़, थाई, तमिल, फिलिपिनो, पंजाबी, मलय, पोलिश, यूक्रेनी, फ़ारसी, कन्नड़, मराठी, हौसा, बर्मी, उज़्बेक, अज़रबैजानी, सेबुआनो, मलयालम, सिंधी, अम्हारिक

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
  <img src="../assets/flags/vn.svg" width="32" title="वियेतनामी" alt="वियेतनामी" />
  <img src="../assets/flags/in.svg" width="32" title="तेलुगु" alt="तेलुगु" />
  <img src="../assets/flags/id.svg" width="32" title="जावानीज़" alt="जावानीज़" />
  <img src="../assets/flags/th.svg" width="32" title="थाई" alt="थाई" />
  <img src="../assets/flags/in.svg" width="32" title="तमिल" alt="तमिल" />
  <img src="../assets/flags/ph.svg" width="32" title="फिलिपिनो" alt="फिलिपिनो" />
  <img src="../assets/flags/pk.svg" width="32" title="पंजाबी" alt="पंजाबी" />
  <img src="../assets/flags/my.svg" width="32" title="मलय" alt="मलय" />
  <img src="../assets/flags/pl.svg" width="32" title="पोलिश" alt="पोलिश" />
  <img src="../assets/flags/ua.svg" width="32" title="यूक्रेनी" alt="यूक्रेनी" />
  <img src="../assets/flags/ir.svg" width="32" title="फ़ारसी" alt="फ़ारसी" />
  <img src="../assets/flags/in.svg" width="32" title="कन्नड़" alt="कन्नड़" />
  <img src="../assets/flags/in.svg" width="32" title="मराठी" alt="मराठी" />
  <img src="../assets/flags/ng.svg" width="32" title="हौसा" alt="हौसा" />
  <img src="../assets/flags/mm.svg" width="32" title="बर्मी" alt="बर्मी" />
  <img src="../assets/flags/uz.svg" width="32" title="उज़्बेक" alt="उज़्बेक" />
  <img src="../assets/flags/az.svg" width="32" title="अज़रबैजानी" alt="अज़रबैजानी" />
  <img src="../assets/flags/ph.svg" width="32" title="सेबुआनो" alt="सेबुआनो" />
  <img src="../assets/flags/in.svg" width="32" title="मलयालम" alt="मलयालम" />
  <img src="../assets/flags/pk.svg" width="32" title="सिंधी" alt="सिंधी" />
  <img src="../assets/flags/et.svg" width="32" title="अम्हारिक" alt="अम्हारिक" />
</p>

## 🚀 मुख्य विशेषताएं और उपयोग

- **आधुनिक नेटिव UI**: सहज ज्ञान युक्त GUI, डार्क/लाइट मोड सपोर्ट के साथ, सहज एनिमेशन, और **Skia** द्वारा संचालित उच्च-प्रदर्शन रेंडरिंग।
- **सिस्टम ट्रे एकीकरण**: सिस्टम ट्रे मिनिमाइज का पूर्ण समर्थन (केवल ~10MB रैम उपयोग), डबल-क्लिक से टॉगल करें, और एक कार्यात्मक राइट-क्लिक मेनू।
- **इंटेलिजेंट स्टार्टअप**: विंडोज के साथ डैशबोर्ड शुरू करना, ट्रे में मिनिमाइज करना (`/silent` विकल्प के साथ साइलेंट मोड), और बाहर निकलने पर वितरण को स्वचालित रूप से बंद करना।
- **कार्य अनुसूची (टास्क शेड्यूलिंग)**: लचीला कार्य अनुसूची प्रबंधन, निर्दिष्ट समय या अंतराल पर WSL इंस्टेंस में स्क्रिप्ट और कमांड के स्वचालित निष्पादन का समर्थन करता है।
- **व्यापक इंस्टेंस नियंत्रण**: वन-क्लिक प्रारंभ, रोकें, समाप्त करना और अनरजिस्टर। रीयल-टाइम स्थिति निगरानी और डिस्क उपयोग एवं फ़ाइल स्थान के बारे में विस्तृत जानकारी।
- **डिस्ट्रो प्रबंधन**: डिफ़ॉल्ट के रूप में सेट करें, माइग्रेशन (VHDX को अन्य ड्राइव पर ले जाना), और `.tar` या `.tar.gz` अभिलेखागार में निर्यात/क्लोन करें।
- **डिस्क माउंट**: WSL इंस्टेंस की VHDX वर्चुअल डिस्क को सीधे माउंट और प्रबंधित करें, माउंट, अनमाउंट और डिस्क प्रबंधन संचालन का समर्थन करता है, जिससे होस्ट और WSL के बीच फ़ाइलों का स्थानांतरण आसान हो जाता है।
- **त्वरित एकीकरण**: टर्मिनल, VS Code या फ़ाइल एक्सप्लोरर में तुरंत लॉन्च, अनुकूलन योग्य कार्य निर्देशिकाओं और स्टार्टअप स्क्रिप्ट हुक के साथ।
- **डिस्ट्रो इंस्टॉलेशन**: Microsoft Store, GitHub, स्थानीय फ़ाइलों (RootFS/VHDX), या ऑनलाइन मिरर के माध्यम से Linux डिस्ट्रीब्यूशन इंस्टॉल करें (स्वचालित गति परीक्षण के साथ सबसे तेज़ मिरर चुनें और अंतर्निहित RootFS डाउनलोड सहायक)।
- **वैश्विक सुरक्षा**: सुरक्षित समवर्ती माइग्रेशन/बैकअप संचालन के लिए म्यूटेक्स लॉक और हटाए जाने पर स्वचालित Appx सफाई।
- **अल्ट्रा-लो मेमोरी फुटप्रिंट**: दक्षता के लिए अत्यधिक अनुकूलित। साइलेंट स्टार्टअप (सिस्टम ट्रे) केवल **~10MB** रैम उपयोग करता है। विंडो मोड उपयोग फ़ॉन्ट जटिलता के अनुसार बदलता है: मानक भाषाओं के लिए **~18MB** और जटिल फ़ॉन्ट वाली भाषाओं (चीनी, जापानी, कोरियाई) के लिए **~38MB**।
- **उन्नत नेटवर्किंग**: निर्बाध पोर्ट फ़ॉरवर्डिंग प्रबंधन (स्वचालित फ़ायरवॉल नियम निर्माण के साथ) और एकीकृत कनेक्टिविटी के लिए वैश्विक HTTP प्रॉक्सी कॉन्फ़िगरेशन।
- **USB डिवाइस प्रबंधन**: डैशबोर्ड UI से सीधे आपके WSL उदाहरणों में स्थानीय USB उपकरणों को आसानी से बाइंड करने, संलग्न करने और प्रबंधित करने के लिए `usbipd-win` के साथ पूर्ण एकीकरण।
- **कॉन्फ़िगरेशन और लॉग**: सेटिंग्स से डिफ़ॉल्ट इंस्टॉल निर्देशिका, लॉग निर्देशिका और स्तर, UI भाषा, डार्क मोड, अपडेट जाँच आवृत्ति, ऑटोस्टार्ट और बंद होने का व्यवहार, तथा साइडबार प्रबंधित करें।
- **डिस्ट्रो को सक्रिय रखें**: एक क्लिक पर «प्रारंभ» डिस्ट्रो को ~60 सेकंड निष्क्रियता के बाद बंद होने के बजाय चलता रखता है; आप «Windows प्रारंभ होने पर यह डिस्ट्रो शुरू करें» भी सक्षम कर सकते हैं।
- **वैश्विक शॉर्टकट**: मुख्य विंडो दिखाने/छिपाने या किसी डिस्ट्रो का टर्मिनल तुरंत खोलने के लिए वैश्विक शॉर्टकट सेट करें।


## 🖼️ स्क्रीनशॉट
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

डैशबोर्ड को काम करते हुए देखना चाहते हैं? पूरी स्क्रीनशॉट गैलरी के लिए [आधिकारिक वेबसाइट](https://www.wslui.com/) पर जाएँ।

## 📦 इंस्टॉलेशन गाइड

### विकल्प 1: प्रोजेक्ट वेबसाइट पर जाएँ (अनुशंसित)

हम आधिकारिक वेबसाइट पर जाकर डाउनलोड करने की सलाह देते हैं, क्योंकि इसमें सुगम अनुभव के लिए कई मिरर लिंक उपलब्ध हैं:

[डाउनलोड पेज](https://www.wslui.com/download/) पर जाएँ और अपने क्षेत्र के लिए उपयुक्त मिरर चुनें।

### विकल्प 2: winget के माध्यम से इंस्टॉल करें

आप Windows पैकेज मैनेजर (winget) से सीधे WSLDashboard इंस्टॉल कर सकते हैं, मॉनिकर या पूर्ण पैकेज आइडेंटिफ़ायर का उपयोग करके:

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

> winget पैकेज आइडेंटिफ़ायर `Owu.WSLDashboard` है और मॉनिकर `wsl-dashboard` है (केस-इनसेंसिटिव)। दोनों में से कोई भी काम करेगा।

अधिक जानकारी के लिए, [WinGet कम्युनिटी रिपॉजिटरी](https://github.com/microsoft/winget-pkgs/tree/master/manifests/o/Owu/WSLDashboard) पर जाएँ।

### विकल्प 3: स्रोत से निर्माण करें

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
     .\build\portable\build.ps1 -Arch x64
     ```


## 📢 आधिकारिक सामुदायिक समूह

WSL UI के आधिकारिक सामुदायिक समूहों में आपका स्वागत है! उपयोग के टिप्स पर चर्चा करने, मुद्डों की रिपोर्ट करने, अपडेट प्राप्त करने, और डेवलपरस और अन्य उपयोगकर्ताओं से जुडने केलिए हमारे चैनल में शामिल हों।

<div align="center">

| वैश्विक समूह (Telegram) | चीनी समूह (WeChat) |
| :---: | :---: |
| <img src="https://cdn1.wslui.com/static/images/group/group-telegram.png" width="240" alt="Telegram Group QR Code" /> | <img src="https://cdn1.wslui.com/static/images/group/group-wechat.png" width="240" alt="WeChat Group QR Code" /> |
| जोडने केलिए स्कैन करें | जोडनة केलिए स्कैन करें |

</div>


## ❤️ इस प्रोजेक्ट को सपोर्ट करें

- यह प्रोजेक्ट GPL-3.0 ओपन सोर्स लाइसेंस के तहत है और सभी उपयोगकर्ताओं के लिए निःशुल्क है।
- फ़ीचर डेवलपमेंट, दैनिक परीक्षण से लेकर बग फ़िक्स तक, सभी काम खाली समय में किया जाता है। ओपन सोर्स का रास्ता अकेले चलना आसान नहीं है — आपकी मान्यता और समर्थन प्रोजेक्ट को आगे बढ़ने का हौसला देते हैं।
- अगर इस टूल ने वास्तव में आपकी मदद की है, तो कृपया सहयोग करें। सभी दान सर्वर खर्च, वर्शन अपडेट और फ़ीचर सुधार में उपयोग किए जाते हैं, जिससे प्रोजेक्ट लगातार अपडेट और स्थिर प्रगति बनी रहती है।
- हर छोटी दयालुता तारों की रोशनी है। आपकी समझ और उदारता के लिए पुनः धन्यवाद！ [हमारे दान पृष्ठ पर जाएं](https://www.wslui.com/donate/).


## ⭐️ प्यार का श्रम

यदि आपको यह प्रोजेक्ट उपयोगी लगा, तो मैं आभारी रहूँगा यदि आप GitHub पर एक स्टार छोड़ सकें। आपका समर्थन इसे व्यापक दर्शकों तक पहुँचाने में मदद करता है और इसकी गहराई से सराहना की जाती है। यही प्रोत्साहन मुझे निर्माण जारी रखने के लिए प्रेरित करता है।


## 📄 लाइसेंस

यह प्रोजेक्ट GPL-3.0 के तहत लाइसेंस प्राप्त है - विवरण के लिए [LICENSE](../LICENSE) फ़ाइल देखें।
