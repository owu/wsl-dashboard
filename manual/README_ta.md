# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

நவீன, உயர் செயல்திறன், இலகுரக மற்றும் குறைந்த நினைவக பயன்பாடு கொண்ட WSL (Windows Subsystem for Linux) நிகழ்வு மேலாண்மை டாஷ்போர்டு. Rust மற்றும் Slint அடிப்படையில் உருவாக்கப்பட்டது, சிறந்த சொந்த அனுபவத்தை வழங்குகிறது.

---

```diff
குறிப்பு:

- இந்த மென்பொருள் Microsoft Store மூலம் விநியோகிக்கப்படாது.
- இங்கே அதே பெயரில் உள்ள பயன்பாடுகள் அங்கீகரிக்கப்படாத பைரேட் மென்பொருள்.
- மோசடியிலிருந்து தப்பிக்க தயவுசெய்து பதிவிறக்க வேண்டாம்.
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

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [తెలుగు](./README_te.md) | தமிழ் | [Tiếng Việt](./README_vi.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 உள்ளடக்கம்
- [🌍 மொழி ஆதரவு](#-மொழி-ஆதரவு)
- [🚀 முக்கிய அம்சங்கள் & பயன்பாடு](#-முக்கிய-அம்சங்கள்--பயன்பாடு)
- [⚙️ கட்டமைப்பு & பதிவுகள்](#️-கட்டமைப்பு--பதிவுகள்)
- [🖼️ மென்பொருள் ஸ்கிரீன்ஷாட்கள்](#️-மென்பொருள்-ஸ்கிரீன்ஷாட்கள்)
- [🎬 செயல்பாட்டு டெமோ](#-செயல்பாட்டு-டெமோ)
- [💻 கணினி தேவைகள்](#-கணினி-தேவைகள்)
- [📦 நிறுவல் வழிகாட்டி](#-நிறுவல்-வழிகாட்டி)
- [🛠️ தொழில்நுட்ப அடுக்கு & செயல்திறன்](#️-தொழில்நுட்ப-அடுக்கு--செயல்திறன்)
- [🤝 சமூக ஆதரவு](#-சமூக-ஆதரவு)
- [❤️ இந்த திட்டத்தை ஆதரிக்கவும்](#️-இந்த-திட்டத்தை-ஆதரிக்கவும்)
- [⭐️ அன்பால் இயங்கும் திட்டம்](#️-அன்பால்-இயங்கும்-திட்டம்)
- [📄 திறந்த மூல உரிமம்](#-திறந்த-மூல-உரிமம்)

---

## 🌍 மொழி ஆதரவு

ஆங்கிலம், எளிமைப்படுத்தப்பட்ட சீனம், பாரம்பரிய சீனம், இந்தி, ஸ்பானிஷ், பிரெஞ்சு, அரபு, வங்காளம், போர்த்துகீசியம், ரஷ்யன், உருது, இந்தோனேசியன், ஜெர்மன், ஜப்பானிய, துருக்கிய, கொரிய, இத்தாலிய, டச்சு, ஸ்வீடிஷ், செக், கிரேக்க, ஹங்கேரிய, ஹீப்ரு, நார்வேஜியன், டேனிஷ், பின்னிஷ், ஸ்லோவாக், ஸ்லோவேனிய, ஐஸ்லாந்திய, வியட்நாம், தெலுங்கு, ஜாவானிய, தாய், தமிழ், பிலிப்பினோ, பஞ்சாபி, மலாய், போலிஷ், உக்ரேனியன், பாரசீகம், கன்னடம், மராத்தி, ஹவுசா, பர்மிய, உஸ்பெக், அசர்பைஜானி, செபுவானோ, மலையாளம், சிந்தி, அம்ஹாரிக்

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="ஆங்கிலம்" alt="ஆங்கிலம்" />
  <img src="../assets/flags/cn.svg" width="32" title="எளிமைப்படுத்தப்பட்ட சீனம்" alt="எளிமைப்படுத்தப்பட்ட சீனம்" />
  <img src="../assets/flags/tw.svg" width="32" title="பாரம்பரிய சீனம்" alt="பாரம்பரிய சீனம்" />
  <img src="../assets/flags/in.svg" width="32" title="இந்தி" alt="இந்தி" />
  <img src="../assets/flags/es.svg" width="32" title="ஸ்பானிஷ்" alt="ஸ்பானிஷ்" />
  <img src="../assets/flags/fr.svg" width="32" title="பிரெஞ்சு" alt="பிரெஞ்சு" />
  <img src="../assets/flags/sa.svg" width="32" title="அரபு" alt="அரபு" />
  <img src="../assets/flags/bd.svg" width="32" title="வங்காளம்" alt="வங்காளம்" />
  <img src="../assets/flags/pt.svg" width="32" title="போர்த்துகீசியம்" alt="போர்த்துகீசியம்" />
  <img src="../assets/flags/ru.svg" width="32" title="ரஷ்யன்" alt="ரஷ்யன்" />
  <img src="../assets/flags/pk.svg" width="32" title="உருது" alt="உருது" />
  <img src="../assets/flags/id.svg" width="32" title="இந்தோனேசியன்" alt="இந்தோனேசியன்" />
  <img src="../assets/flags/de.svg" width="32" title="ஜெர்மன்" alt="ஜெர்மன்" />
  <img src="../assets/flags/jp.svg" width="32" title="ஜப்பானிய" alt="ஜப்பானிய" />
  <img src="../assets/flags/tr.svg" width="32" title="துருக்கிய" alt="துருக்கிய" />
  <img src="../assets/flags/kr.svg" width="32" title="கொரிய" alt="கொரிய" />
  <img src="../assets/flags/it.svg" width="32" title="இத்தாலிய" alt="இத்தாலிய" />
  <img src="../assets/flags/nl.svg" width="32" title="டச்சு" alt="டச்சு" />
  <img src="../assets/flags/se.svg" width="32" title="ஸ்வீடிஷ்" alt="ஸ்வீடிஷ்" />
  <img src="../assets/flags/cz.svg" width="32" title="செக்" alt="செக்" />
  <img src="../assets/flags/gr.svg" width="32" title="கிரேக்க" alt="கிரேக்க" />
  <img src="../assets/flags/hu.svg" width="32" title="ஹங்கேரிய" alt="ஹங்கேரிய" />
  <img src="../assets/flags/il.svg" width="32" title="ஹீப்ரு" alt="ஹீப்ரு" />
  <img src="../assets/flags/no.svg" width="32" title="நார்வேஜியன்" alt="நார்வேஜியன்" />
  <img src="../assets/flags/dk.svg" width="32" title="டேனிஷ்" alt="டேனிஷ்" />
  <img src="../assets/flags/fi.svg" width="32" title="பின்னிஷ்" alt="பின்னிஷ்" />
  <img src="../assets/flags/sk.svg" width="32" title="ஸ்லோவாக்" alt="ஸ்லோவாக்" />
  <img src="../assets/flags/si.svg" width="32" title="ஸ்லோவேனிய" alt="ஸ்லோவேனிய" />
  <img src="../assets/flags/is.svg" width="32" title="ஐஸ்லாந்திய" alt="ஐஸ்லாந்திய" />
  <img src="../assets/flags/vn.svg" width="32" title="வியட்நாம்" alt="வியட்நாம்" />
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


## 🚀 முக்கிய அம்சங்கள் & பயன்பாடு

- **நவீன சொந்த UI**: **Skia** மூலம் இயக்கப்படும் உயர் செயல்திறன் ரெண்டரிங், இருண்ட/ஒளி பயன்முறை, மென்மையான அனிமேஷன்களுடன் நேரடியான GUI.
- **கணினி தட்டு ஒருங்கிணைப்பு**: விரிவான தட்டு ஆதரவு (சுமார் 10MB நினைவக பயன்பாடு), இருமுறை கிளிக் மூலம் காட்டு/மறை மாற்றம் மற்றும் முழு அம்ச வலது-கிளிக் மெனு.
- **ஸ்மார்ட் தொடக்கம்**: பூட்டில் தானியங்கி தொடக்கம், தட்டுக்கு குறைத்தல் (`/silent` வாதத்துடன் அமைதியான தொடக்கம்), மற்றும் வெளியேறும் போது டிஸ்ட்ரோக்களை தானியங்கி மூடுதல்.
- **விரிவான நிகழ்வு கட்டுப்பாடு**: ஒரே கிளிக்கில் தொடங்கு, நிறுத்து, முடிவுறுத்து மற்றும் பதிவு நீக்கு. நிகழ்நேர நிலை கண்காணிப்பு, வட்டு பயன்பாடு மற்றும் கோப்பு இடங்களில் ஆழமான பார்வை.
- **டிஸ்ட்ரோ நிர்வாகம்**: இயல்புநிலையாக அமை, இயற்பியல் இடம்பெயர்வு (VHDX ஐ மற்றொரு வட்டுக்கு நகர்த்து), மற்றும் `.tar` அல்லது `.tar.gz` ஆர்கைவ்வாக ஏற்றுமதி/குளோன்.
- **விரைவு ஒருங்கிணைப்பு**: டெர்மினல், VS Code அல்லது கோப்பு எக்ஸ்ப்ளோரருக்கு ஒரே கிளிக்கில் அணுகல், தனிப்பயன் பணி அடைவு மற்றும் தொடக்க ஸ்கிரிப்ட் ஹூக்குகள் ஆதரவு.
- **டிஸ்ட்ரோ நிறுவல்**: Microsoft Store, GitHub, உள்ளூர் கோப்புகள் (RootFS/VHDX), அல்லது ஆன்லைன் மிரர்கள் மூலம் Linux விநியோகங்களை நிறுவவும் (வேகமான மிரரைத் தேர்ந்தெடுக்க தானியங்கி வேக சோதனை மற்றும் உள்ளமைக்கப்பட்ட RootFS பதிவிறக்க உதவியாளர்).
- **உலகளாவிய பாதுகாப்பு**: ஒரே நேரத்தில் இடம்பெயர்வு/காப்பு செயல்பாடுகளுக்கான பாதுகாப்பிற்கு மியூடெக்ஸ் பூட்டுகள் பயன்படுத்துதல், மற்றும் நீக்கும் போது Appx தொகுப்புகளின் தானியங்கி தூய்மைப்படுத்தல்.
- **மிகக் குறைந்த நினைவக பயன்பாடு**: மிக உயர்ந்த மட்டத்தில் மேம்படுத்தப்பட்ட வள செயல்திறன். அமைதியான தொடக்கம் (கணினி தட்டு) சுமார் **10MB** நினைவகம். சாளர பயன்முறையில் எழுத்துரு சிக்கலான தன்மையின் அடிப்படையில் சுமார் **18MB** (ஆங்கிலம், ஜெர்மன் போன்ற நிலையான மொழிகள்) முதல் **38MB** (சீன-ஜப்பானிய-கொரிய போன்ற பெரிய எழுத்து தொகுப்புகள்) வரை.
- **மேம்பட்ட நெட்வொர்க் நிர்வாகம்**: போர்ட் ஃபார்வேர்டிங் (தானியங்கி ஃபயர்வால் விதிகள் உருவாக்கம்) மற்றும் உலகளாவி HTTP ப்ராக்ஸி கட்டமைப்பை மென்மையாக நிர்வகித்தல், ஒருங்கிணைந்த இணைப்பு அனுபவத்தை அடைதல்.
- **USB சாதன நிர்வாகம்**: `usbipd-win` உடன் ஆழமான ஒருங்கிணைப்பு, டாஷ்போர்டு UI இல் நேரடியாக அனைத்து WSL நிகழ்வுகளிலும் உள்ளூர் USB சாதனங்களை எளிதாக பைண்ட், இணை மற்றும் நிர்வகிக்க முடியும்.


## ⚙️ கட்டமைப்பு & பதிவுகள்

அனைத்து கட்டமைப்புகளும் "அமைப்புகள்" காட்சி மூலம் நிர்வகிக்கப்படுகின்றன:

- புதிய WSL நிகழ்வுகளுக்கான இயல்புநிலை நிறுவல் அடைவைத் தேர்ந்தெடுக்கவும்.
- பதிவு அடைவு மற்றும் பதிவு நிலையை கட்டமைக்கவும் (Error / Warn / Info / Debug / Trace).
- UI மொழியைத் தேர்ந்தெடுக்கவும் அல்லது கணினி மொழியைப் பின்பற்றவும்.
- இருண்ட பயன்முறையை மாற்றவும், மற்றும் செயல்பாட்டிற்குப் பிறகு பயன்பாடு WSL ஐ தானியங்கி மூடுமா.
- புதுப்பிப்பு சரிபார்ப்பு அதிர்வெண்ணை கட்டமைக்கவும் (தினசரி, வாராந்திர, இரு வாராந்திர, மாதாந்திர).
- பூட்டில் தானியங்கி தொடக்கத்தை இயக்கவும் (பாதை தானியங்கி சரிசெய்தல் அம்சத்துடன்).
- தொடக்கத்தில் தட்டுக்கு குறைத்தலை அமைக்கவும்.
- மூடு பொத்தான் நடத்தையை கட்டமைக்கவும் (நிரலிலிருந்து வெளியேறுவதற்கு பதிலாக தட்டுக்கு குறைத்தல்).
- குறிப்பிட்ட அம்ச தாவல்களின் தெரிவுநிலையை மாற்றுவதன் மூலம் பக்கப்பட்டியை தனிப்பயனாக்கவும்.

பதிவு கோப்புகள் கட்டமைக்கப்பட்ட பதிவு அடைவில் எழுதப்படுகின்றன, சிக்கல்களைப் புகாரளிக்கும் போது பதிவுகளை இணைக்கலாம்.


## 🖼️ மென்பொருள் ஸ்கிரீன்ஷாட்கள்

### முகப்பு இடைமுகம் (இருண்ட & ஒளி பயன்முறை)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB & மெனு மடிப்பு
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### நெட்வொர்க் நிர்வாகம்
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### நிகழ்வு சேர் & அமைப்புகள்
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>
<p align="center">
  <img src="../assets/screenshot/settings-advanced.png" width="48%" />
  <img src="../assets/screenshot/settings-interface.png" width="48%" />
</p>

### பற்றி & நன்கொடை
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/donate.png" width="48%" />
</p>

## 🎬 செயல்பாட்டு டெமோ

[எங்களை மேம்படுத்த உதவுங்கள்! எங்கள் அறிமுக வீடியோவைப் பாருங்கள் மற்றும் உங்கள் கருத்துக்களைப் பகிருங்கள்.](https://github.com/owu/wsl-dashboard/discussions/9)



## 💻 கணினி தேவைகள்

- WSL இயக்கப்பட்ட Windows 10 அல்லது Windows 11 (WSL 2 பரிந்துரைக்கப்படுகிறது).
- குறைந்தது ஒரு WSL விநியோகம் நிறுவப்பட்டிருக்க வேண்டும், அல்லது புதிய விநியோகத்தை நிறுவுவதற்கான அனுமதி இருக்க வேண்டும்.
- 64-பிட் CPU; பல டிஸ்ட்ரோ பயன்பாட்டிற்கு 4 GB அல்லது அதிகமான RAM பரிந்துரைக்கப்படுகிறது.

## 📦 நிறுவல் வழிகாட்டி

### முறை 1: முன்கட்டமைக்கப்பட்ட பைனரிகளைப் பதிவிறக்கவும்

எளிதான வழி கட்டமைக்கப்பட்ட பதிப்பைப் பயன்படுத்துவது:

1. [GitHub Releases](https://github.com/owu/wsl-dashboard/releases) பக்கத்திற்குச் செல்லுங்கள்.
2. Windows க்கான சமீபத்திய `wsldashboard` இயங்கக்கூடிய கோப்பைப் பதிவிறக்கவும்.
3. டிகம்ப்ரஸ் செய்யுங்கள் (சுருக்கப்பட்ட கோப்பு என்றால்) மற்றும் `wsldashboard.exe` ஐ இயக்குங்கள்.

நிறுவல் தேவையில்லை, இந்த பயன்பாடு ஒற்றை கோப்பு போர்ட்டபிள் நிரல்.

### முறை 2: மூலத்திலிருந்து கட்டமைக்கவும்

Rust கருவித்தொகுப்பு (Rust 1.92+ அல்லது புதிய பதிப்பு) நிறுவப்பட்டிருப்பதை உறுதிசெய்யுங்கள்.

1. களஞ்சியத்தை குளோன் செய்யுங்கள்:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. கட்டமைத்து இயக்குங்கள்:

   - மேம்பாட்டு பிழைத்திருத்தம்:

     ```powershell
     cargo run
     ```
   - மேம்படுத்தப்பட்ட வெளியீட்டு கட்டத்திற்கு கட்டமைப்பு ஸ்கிரிப்ட் பயன்படுத்தவும்:

     > கட்டமைப்பு ஸ்கிரிப்ட்டிற்கு `x86_64-pc-windows-msvc` கருவித்தொகுப்பு தேவை.

     ```powershell
     .\build\portable\build.ps1
     ```


## 🛠️ தொழில்நுட்ப அடுக்கு & செயல்திறன்

- **கெர்னல்**: நினைவக பாதுகாப்பு மற்றும் பூஜ்ஜிய செலவு சுருக்கங்களை உறுதிசெய்ய Rust இல் செயல்படுத்தப்பட்டது.
- **UI கட்டமைப்பு**: உயர் செயல்திறன் **Skia** ரெண்டரிங் பின்புலத்துடன் Slint.
- **ஒத்திசைவற்ற இயக்க நேரம்**: தடுக்காத கணினி கட்டளைகள் மற்றும் I/O க்கான Tokio.
- **செயல்திறன் சிறப்பம்சங்கள்**:
  - **மறுமொழி வேகம்**: கிட்டத்தட்ட உடனடி தொடக்க வேகம், WSL நிலையை நிகழ்நேரத்தில் கண்காணித்தல்.
  - **வள செயல்திறன்**: மிகக் குறைந்த வள பயன்பாடு (விவரங்களுக்கு [முக்கிய அம்சங்கள்](#-முக்கிய-அம்சங்கள்--பயன்பாடு) பார்க்கவும்).
  - **பெயர்த்தகவு**: மேம்படுத்தப்பட்ட வெளியீட்டு கட்டம் ஒற்றை சிறிய இயங்கக்கூடிய கோப்பை உருவாக்குகிறது.



## 🤝 சமூக ஆதரவு

இந்த சமூகங்களின் ஆதரவுக்கு மனமார்ந்த நன்றிகள்:

- [Rust Programming Language](https://www.rust-lang.org) - சக்திவாய்ந்த மற்றும் பாதுகாப்பான நிரலாக்க மொழி
- [Slint | Declarative GUI for Rust, C++, JavaScript & Python](https://slint.dev) - நவீன UI கட்டமைப்பு
- [WSL: Windows Subsystem for Linux](https://github.com/microsoft/WSL) - சிறந்த Windows Subsystem for Linux
- [Tokio - An asynchronous Rust runtime](https://tokio.rs) - திறமையான ஒத்திசைவற்ற இயக்க நேரம்
- [Windows Developer Community](https://developer.microsoft.com/en-us/windows/community) - தொடர்ச்சியான தள மேம்பாடுகள்
- [Reddit](https://www.reddit.com) - உலகளாவிய சமூக விவாதம் & ஆதரவு
- [Hacker News](https://news.ycombinator.com) - உலகளாவிய சமூக விவாதம் & ஆதரவு
- [Linux.do](https://linux.do) - IT நிபுணர்களுக்கான பிரபலமான சமூகம்
- [V2EX](https://www.v2ex.com) - சீன தொழில்நுட்ப சமூக விவாதம்

உங்கள் பங்களிப்பு மற்றும் கருத்து இந்த திட்டத்தை சாத்தியமாக்கியது!


## ❤️ இந்த திட்டத்தை ஆதரிக்கவும்

- இந்த திட்டம் GPL-3.0 திறந்த மூல உரிமத்தை பின்பற்றுகிறது, அனைத்து பயனர்களுக்கும் இலவசமாக கிடைக்கிறது.
- அம்ச மேம்பாடு, தினசரி சோதனை முதல் பிழை தீர்வு வரை, அனைத்து பணிகளும் ஓய்வு நேரத்தில் நிலையான முயற்சியிலிருந்து வருகின்றன. திறந்த மூல பாதை தனியாக எளிதானது அல்ல, உங்கள் அங்கீகாரம் மற்றும் ஆதரவு திட்டம் நீண்ட காலம் தொடர்வதற்கான உந்துதல்.
- இந்த கருவி உங்களுக்கு உண்மையாக உதவியதாக உணர்ந்தால், தயவுசெய்து உதவிக்கரம் நீட்டுங்கள். அனைத்து நன்கொடைகளும் சேவையக செலவுகள், பதிப்பு மேம்பாடுகள் மற்றும் அம்ச மேம்படுத்தல்களுக்கு பயன்படுத்தப்படும்.
- சிறிய கருணையும் நட்சத்திரம் போல பிரகாசிக்கிறது. உங்கள் புரிதல் மற்றும் தாராள மனப்பான்மைக்கு மீண்டும் நன்றி!

எங்கள் நன்கொடை பக்கத்தைப் பார்வையிடுங்கள்: [https://www.wslui.com/donate/](https://www.wslui.com/donate/)


## ⭐️ அன்பால் இயங்கும் திட்டம்

இந்த திட்டம் உங்களுக்கு உதவியாக இருந்ததாக உணர்ந்தால், GitHub இல் ஒரு நட்சத்திரம் வழங்குமாறு நான் நன்றியுடன் இருப்பேன். உங்கள் அங்கீகாரம் திட்டத்தை பரந்த பயனர் குழுவை அடைய உதவும். இந்த ஊக்கம் என்னை தொடர்ந்து முன்னோக்கி நகர்த்துகிறது.


## 📄 திறந்த மூல உரிமம்

இந்த திட்டம் GPL-3.0 உரிமத்தின் கீழ் உரிமம் பெற்றது – விவரங்களுக்கு [LICENSE](../LICENSE) கோப்பைப் பார்க்கவும்.


---

Built with ❤️ for the WSL Community.