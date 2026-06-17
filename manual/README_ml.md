# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

ആധുനിക, ഉയർന്ന-പ്രവർത്തനക്ഷമത, ലഘുവും കുറഞ്ഞ മെമ്മറി ഉപയോഗവുമുള്ള WSL (Windows Subsystem for Linux) ഇൻസ്റ്റാൻസ് മാനേജ്മെന്റ് ഡാഷ്ബോർഡ്. Rust, Slint എന്നിവയിൽ നിർമ്മിച്ചത്, മികച്ച നേറ്റീവ് അനുഭവം നൽകുന്നു.

---

```diff
അറിയിപ്പ്:

- WSL Dashboard Microsoft Store വഴി വിതരണം ചെയ്യുന്നില്ല.
- "WSL Dashboard" എന്ന പേരിൽ അവിടെ ലിസ്റ്റ് ചെയ്തിട്ടുള്ള ഏത് ആപ്ലിക്കേഷനും അനധികൃതമാണ്, വ്യാജമായിരിക്കാം.
- സാധ്യമായ തട്ടിപ്പുകൾ ഒഴിവാക്കാൻ ദയവായി അത് ഡൗൺലോഡ് ചെയ്യരുത്.
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

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | മലയാളം | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 ഉള്ളടക്കം
- [🌍 ഭാഷാ പിന്തുണ](#-ഭാഷാ-പിന്തുണ)
- [🚀 പ്രധാന സവിശേഷതകളും ഉപയോഗവും](#-പ്രധാന-സവിശേഷതകളും-ഉപയോഗവും)
- [⚙️ കോൺഫിഗറേഷനും ലോഗുകളും](#️-കോൺഫിഗറേഷനും-ലോഗുകളും)
- [🖼️ സോഫ്റ്റ്‌വെയർ സ്ക്രീൻഷോട്ടുകൾ](#️-സോഫ്റ്റ്‌വെയർ-സ്ക്രീൻഷോട്ടുകൾ)
- [🎬 പ്രവർത്തന ഡെമോ](#-പ്രവർത്തന-ഡെമോ)
- [💻 സിസ്റ്റം ആവശ്യകതകൾ](#-സിസ്റ്റം-ആവശ്യകതകൾ)
- [📦 ഇൻസ്റ്റാളേഷൻ ഗൈഡ്](#-ഇൻസ്റ്റാളേഷൻ-ഗൈഡ്)
- [🛠️ സാങ്കേതിക സ്റ്റാക്കും പ്രവർത്തനക്ഷമതയും](#️-സാങ്കേതിക-സ്റ്റാക്കും-പ്രവർത്തനക്ഷമതയും)
- [🤝 കമ്മ്യൂണിറ്റി പിന്തുണ](#-കമ്മ്യൂണിറ്റി-പിന്തുണ)
- [❤️ ഈ പ്രോജക്ടിനെ പിന്തുണയ്ക്കുക](#️-ഈ-പ്രോജക്ടിനെ-പിന്തുണയ്ക്കുക)
- [⭐️ സ്നേഹത്തോടെ പ്രവർത്തിക്കുന്ന പ്രോജക്ട്](#️-സ്നേഹത്തോടെ-പ്രവർത്തിക്കുന്ന-പ്രോജക്ട്)
- [📄 ഓപ്പൺ സോഴ്സ് ലൈസൻസ്](#-ഓപ്പൺ-സോഴ്സ്-ലൈസൻസ്)

---

## 🌍 ഭാഷാ പിന്തുണ

ഇംഗ്ലീഷ്, ലളിതവൽക്കരിച്ച ചൈനീസ്, പരമ്പരാഗത ചൈനീസ്, ഹിന്ദി, സ്പാനിഷ്, ഫ്രഞ്ച്, അറബിക്, ബംഗാളി, പോർച്ചുഗീസ്, റഷ്യൻ, ഉറുദു, ഇന്തോനേഷ്യൻ, ജർമ്മൻ, ജാപ്പനീസ്, തുർക്കിഷ്, കൊറിയൻ, ഇറ്റാലിയൻ, ഡച്ച്, സ്വീഡിഷ്, ചെക്ക്, ഗ്രീക്ക്, ഹംഗേറിയൻ, ഹീബ്രൂ, നോർവീജിയൻ, ഡാനിഷ്, ഫിന്നിഷ്, സ്ലോവാക്, സ്ലോവേനിയൻ, ഐസ്‌ലാൻഡിക്, വിയറ്റ്നാമീസ്, തെലുങ്ക്, ജാവാനീസ്, തായ്, തമിഴ്, ഫിലിപ്പിനോ, പഞ്ചാബി, മലായ്, പോളിഷ്, യുക്രേനിയൻ, പേർഷ്യൻ, കന്നഡ, മറാത്തി, ഹൗസ, മ്യാൻമാർ, ഉസ്‌ബെക്ക്, അസർ‌ബൈജാനി, സെബുവാനോ, മലയാളം, സിന്ധി, അംഹാരിക്

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="ഇംഗ്ലീഷ്" alt="ഇംഗ്ലീഷ്" />
  <img src="../assets/flags/cn.svg" width="32" title="ലളിതവൽക്കരിച്ച ചൈനീസ്" alt="ലളിതവൽക്കരിച്ച ചൈനീസ്" />
  <img src="../assets/flags/tw.svg" width="32" title="പരമ്പരാഗത ചൈനീസ്" alt="പരമ്പരാഗത ചൈനീസ്" />
  <img src="../assets/flags/in.svg" width="32" title="ഹിന്ദി" alt="ഹിന്ദി" />
  <img src="../assets/flags/es.svg" width="32" title="സ്പാനിഷ്" alt="സ്പാനിഷ്" />
  <img src="../assets/flags/fr.svg" width="32" title="ഫ്രഞ്ച്" alt="ഫ്രഞ്ച്" />
  <img src="../assets/flags/sa.svg" width="32" title="അറബിക്" alt="അറബിക്" />
  <img src="../assets/flags/bd.svg" width="32" title="ബംഗാളി" alt="ബംഗാളി" />
  <img src="../assets/flags/pt.svg" width="32" title="പോർച്ചുഗീസ്" alt="പോർച്ചുഗീസ്" />
  <img src="../assets/flags/ru.svg" width="32" title="റഷ്യൻ" alt="റഷ്യൻ" />
  <img src="../assets/flags/pk.svg" width="32" title="ഉറുദു" alt="ഉറുദു" />
  <img src="../assets/flags/id.svg" width="32" title="ഇന്തോനേഷ്യൻ" alt="ഇന്തോനേഷ്യൻ" />
  <img src="../assets/flags/de.svg" width="32" title="ജർമ്മൻ" alt="ജർമ്മൻ" />
  <img src="../assets/flags/jp.svg" width="32" title="ജാപ്പനീസ്" alt="ജാപ്പനീസ്" />
  <img src="../assets/flags/tr.svg" width="32" title="തുർക്കിഷ്" alt="തുർക്കിഷ്" />
  <img src="../assets/flags/kr.svg" width="32" title="കൊറിയൻ" alt="കൊറിയൻ" />
  <img src="../assets/flags/it.svg" width="32" title="ഇറ്റാലിയൻ" alt="ഇറ്റാലിയൻ" />
  <img src="../assets/flags/nl.svg" width="32" title="ഡച്ച്" alt="ഡച്ച്" />
  <img src="../assets/flags/se.svg" width="32" title="സ്വീഡിഷ്" alt="സ്വീഡിഷ്" />
  <img src="../assets/flags/cz.svg" width="32" title="ചെക്ക്" alt="ചെക്ക്" />
  <img src="../assets/flags/gr.svg" width="32" title="ഗ്രീക്ക്" alt="ഗ്രീക്ക്" />
  <img src="../assets/flags/hu.svg" width="32" title="ഹംഗേറിയൻ" alt="ഹംഗേറിയൻ" />
  <img src="../assets/flags/il.svg" width="32" title="ഹീബ്രൂ" alt="ഹീബ്രൂ" />
  <img src="../assets/flags/no.svg" width="32" title="നോർവീജിയൻ" alt="നോർവീജിയൻ" />
  <img src="../assets/flags/dk.svg" width="32" title="ഡാനിഷ്" alt="ഡാനിഷ്" />
  <img src="../assets/flags/fi.svg" width="32" title="ഫിന്നിഷ്" alt="ഫിന്നിഷ്" />
  <img src="../assets/flags/sk.svg" width="32" title="സ്ലോവാക്" alt="സ്ലോവാക്" />
  <img src="../assets/flags/si.svg" width="32" title="സ്ലോവേനിയൻ" alt="സ്ലോവേനിയൻ" />
  <img src="../assets/flags/is.svg" width="32" title="ഐസ്‌ലാൻഡിക്" alt="ഐസ്‌ലാൻഡിക്" />
  <img src="../assets/flags/vn.svg" width="32" title="വിയറ്റ്നാമീസ്" alt="വിയറ്റ്നാമീസ്" />
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


## 🚀 പ്രധാന സവിശേഷതകളും ഉപയോഗവും

- **ആധുനിക നേറ്റീവ് UI**: **Skia** പവർഡ് ഹൈ-പെർഫോമൻസ് റെൻഡറിംഗ്, ഡാർക്ക്/ലൈറ്റ് മോഡ്, മൃദുവായ ആനിമേഷനുകളുള്ള സ്വാഭാവിക GUI.
- **സിസ്റ്റം ട്രേ ഇന്റഗ്രേഷൻ**: സമഗ്രമായ ട്രേ പിന്തുണ (ഏകദേശം 10MB മെമ്മറി ഉപയോഗം), ഡബിൾ-ക്ലിക്ക് ഉപയോഗിച്ച് കാണിക്കുക/മറയ്ക്കുക ടോഗിളും പൂർണ്ണ-സവിശേഷത റൈറ്റ്-ക്ലിക്ക് മെനുവും.
- **സ്മാർട്ട് സ്റ്റാർട്ടപ്പ്**: ബൂട്ടിൽ ഓട്ടോ-സ്റ്റാർട്ട്, ട്രേയിലേക്ക് മിനിമൈസ് (`/silent` ആർഗ്യുമെന്റ് ഉപയോഗിച്ച് സൈലന്റ് ലോഞ്ച്), എക്സിറ്റിൽ ഡിസ്ട്രോ ഓട്ടോ-ക്ലോസ്.
- **സമഗ്രമായ ഇൻസ്റ്റാൻസ് നിയന്ത്രണം**: ഒറ്റ ക്ലിക്കിൽ ആരംഭിക്കുക, നിർത്തുക, അവസാനിപ്പിക്കുക, രജിസ്ട്രി നീക്കം ചെയ്യുക. തത്സമയ സ്ഥിതി നിരീക്ഷണം, ഡിസ്ക് ഉപയോഗവും ഫയൽ സ്ഥാനങ്ങളും സംബന്ധിച്ച ആഴത്തിലുള്ള ഉൾക്കാഴ്ച.
- **ഡിസ്ട്രോ മാനേജ്മെന്റ്**: ഡീഫോൾട്ടായി സജ്ജമാക്കുക, ഭൗതിക മൈഗ്രേഷൻ (VHDX മറ്റൊരു ഡിസ്കിലേക്ക് നീക്കുക), `.tar` അല്ലെങ്കിൽ `.tar.gz` ആർക്കൈവായി എക്സ്പോർട്ട്/ക്ലോൺ ചെയ്യുക.
- **ദ്രുത ഇന്റഗ്രേഷൻ**: ടെർമിനൽ, VS Code അല്ലെങ്കിൽ ഫയൽ എക്സ്പ്ലോററിലേക്ക് ഒറ്റ ക്ലിക്കിൽ ആക്സസ്, കസ്റ്റം വർക്കിംഗ് ഡയറക്ടറിയും സ്റ്റാർട്ടപ്പ് സ്ക്രിപ്റ്റ് ഹുക്കുകളും പിന്തുണ.
- **ഡിസ്ട്രോ ഇൻസ്റ്റാളേഷൻ**: Microsoft Store, GitHub, ലോക്കൽ ഫയലുകൾ (RootFS/VHDX), അല്ലെങ്കിൽ ഓൺലൈൻ മിററുകൾ വഴി Linux ഡിസ്ട്രിബ്യൂഷനുകൾ ഇൻസ്റ്റാൾ ചെയ്യുക (ഏറ്റവും വേഗമുള്ള മിറർ തിരഞ്ഞെടുക്കാൻ ഓട്ടോ സ്പീഡ് ടെസ്റ്റും ബിൽറ്റ്-ഇൻ RootFS ഡൗൺലോഡ് അസിസ്റ്റന്റും).
- **ഗ്ലോബൽ സുരക്ഷ**: ഏകകാല മൈഗ്രേഷൻ/ബാക്കപ്പ് പ്രവർത്തനങ്ങളുടെ സുരക്ഷയ്ക്കായി മ്യൂട്ടക്സ് ലോക്കുകൾ ഉപയോഗിക്കുക, നീക്കം ചെയ്യുമ്പോൾ Appx പാക്കേജുകളുടെ ഓട്ടോ-ക്ലീനപ്പ്.
- **അതീവ കുറഞ്ഞ മെമ്മറി ഉപയോഗം**: അത്യധികം ഒപ്റ്റിമൈസ് ചെയ്ത റിസോഴ്സ് കാര്യക്ഷമത. സൈലന്റ് സ്റ്റാർട്ടപ്പ് (സിസ്റ്റം ട്രേ) ഏകദേശം **10MB** മെമ്മറി. വിൻഡോ മോഡിൽ ഫോണ്ട് സങ്കീർണ്ണത അനുസരിച്ച് ഏകദേശം **18MB** (ഇംഗ്ലീഷ്, ജർമ്മൻ പോലുള്ള സ്റ്റാൻഡേർഡ് ഭാഷകൾ) മുതൽ **38MB** (ചൈനീസ്-ജാപ്പനീസ്-കൊറിയൻ പോലുള്ള വലിയ ക്യാരക്ടർ സെറ്റുകൾ) വരെ.
- **വിപുലമായ നെറ്റ്‌വർക്ക് മാനേജ്മെന്റ്**: പോർട്ട് ഫോർവേഡിംഗ് (ഓട്ടോ-ക്രിയേറ്റ് ഫയർവാൾ നിയമങ്ങൾ) കൂടാതെ ഗ്ലോബൽ HTTP പ്രോക്സി കോൺഫിഗറേഷനും തടസ്സമില്ലാതെ കൈകാര്യം ചെയ്യുക, ഏകീകൃത കണക്റ്റിവിറ്റി അനുഭവം കൈവരിക്കുക.
- **USB ഉപകരണ മാനേജ്മെന്റ്**: `usbipd-win` മായി ആഴത്തിലുള്ള ഇന്റഗ്രേഷൻ, ഡാഷ്ബോർഡ് UI-യിൽ നേരിട്ട് എല്ലാ WSL ഇൻസ്റ്റാൻസുകളിലും ലോക്കൽ USB ഉപകരണങ്ങൾ എളുപ്പത്തിൽ ബൈൻഡ്, അറ്റാച്ച്, മാനേജ് ചെയ്യുക.


## ⚙️ കോൺഫിഗറേഷനും ലോഗുകളും

എല്ലാ കോൺഫിഗറേഷനുകളും "സെറ്റിംഗ്സ്" വ്യൂ വഴി കൈകാര്യം ചെയ്യുന്നു:

- പുതിയ WSL ഇൻസ്റ്റാൻസുകളുടെ ഡീഫോൾട്ട് ഇൻസ്റ്റാൾ ഡയറക്ടറി തിരഞ്ഞെടുക്കുക.
- ലോഗ് ഡയറക്ടറിയും ലോഗ് ലെവലും കോൺഫിഗർ ചെയ്യുക (Error / Warn / Info / Debug / Trace).
- UI ഭാഷ തിരഞ്ഞെടുക്കുക അല്ലെങ്കിൽ സിസ്റ്റം ഭാഷ പിന്തുടരുക.
- ഡാർക്ക് മോഡ് ടോഗിൾ ചെയ്യുക, പ്രവർത്തനത്തിന് ശേഷം ആപ്പ് WSL ഓട്ടോ-ക്ലോസ് ചെയ്യുമോ.
- അപ്ഡേറ്റ് പരിശോധന ആവൃത്തി കോൺഫിഗർ ചെയ്യുക (ദിനംപ്രതി, ആഴ്ചതോറും, രണ്ടാഴ്ച കൂടുമ്പോൾ, മാസത്തിലൊരിക്കൽ).
- ബൂട്ടിൽ ഓട്ടോ-സ്റ്റാർട്ട് പ്രവർത്തനക്ഷമമാക്കുക (പാത്ത് ഓട്ടോ-ഫിക്സ് സവിശേഷതയോടെ).
- സ്റ്റാർട്ടപ്പിൽ ട്രേയിലേക്ക് മിനിമൈസ് ചെയ്യുക.
- ക്ലോസ് ബട്ടൺ പെരുമാറ്റം കോൺഫിഗർ ചെയ്യുക (പ്രോഗ്രാമിൽ നിന്ന് പുറത്തുകടക്കുന്നതിന് പകരം ട്രേയിലേക്ക് മിനിമൈസ് ചെയ്യുക).
- നിർദ്ദിഷ്ട ഫീച്ചർ ടാബുകളുടെ ദൃശ്യപരത ടോഗിൾ ചെയ്ത് സൈഡ്ബാർ കസ്റ്റമൈസ് ചെയ്യുക.

ലോഗ് ഫയലുകൾ കോൺഫിഗർ ചെയ്ത ലോഗ് ഡയറക്ടറിയിൽ എഴുതുന്നു, പ്രശ്നങ്ങൾ റിപ്പോർട്ട് ചെയ്യുമ്പോൾ ലോഗുകൾ ചേർക്കാം.


## 🖼️ സോഫ്റ്റ്‌വെയർ സ്ക്രീൻഷോട്ടുകൾ

### ഹോം ഇന്റർഫേസ് (ഡാർക്ക് & ലൈറ്റ് മോഡ്)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB & മെനു കൊളാപ്സ്
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### നെറ്റ്‌വർക്ക് മാനേജ്മെന്റ്
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### ഇൻസ്റ്റാൻസ് ചേർക്കുക & സെറ്റിംഗ്സ്
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>
<p align="center">
  <img src="../assets/screenshot/settings-advanced.png" width="48%" />
  <img src="../assets/screenshot/settings-interface.png" width="48%" />
</p>

### കുറിച്ച് & സംഭാവന
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/donate.png" width="48%" />
</p>

## 🎬 പ്രവർത്തന ഡെമോ

[ഞങ്ങളെ മെച്ചപ്പെടുത്താൻ സഹായിക്കൂ! ഞങ്ങളുടെ ആമുഖ വീഡിയോ കാണുക, നിങ്ങളുടെ ആശയങ്ങൾ പങ്കുവെക്കുക.](https://github.com/owu/wsl-dashboard/discussions/9)



## 💻 സിസ്റ്റം ആവശ്യകതകൾ

- WSL പ്രവർത്തനക്ഷമമായ Windows 10 അല്ലെങ്കിൽ Windows 11 (WSL 2 ശുപാർശ ചെയ്യുന്നു).
- കുറഞ്ഞത് ഒരു WSL ഡിസ്ട്രിബ്യൂഷൻ ഇൻസ്റ്റാൾ ചെയ്തിരിക്കണം, അല്ലെങ്കിൽ പുതിയ ഡിസ്ട്രിബ്യൂഷൻ ഇൻസ്റ്റാൾ ചെയ്യാനുള്ള അനുമതി ഉണ്ടായിരിക്കണം.
- 64-ബിറ്റ് CPU; ബഹു-ഡിസ്ട്രോ ഉപയോഗത്തിന് 4 GB അല്ലെങ്കിൽ അതിലധികം RAM ശുപാർശ.

## 📦 ഇൻസ്റ്റാളേഷൻ ഗൈഡ്

### രീതി 1: പ്രീ-ബിൽട്ട് ബൈനറികൾ ഡൗൺലോഡ് ചെയ്യുക

ഏറ്റവും എളുപ്പമുള്ള മാർഗ്ഗം ബിൽട്ട് പതിപ്പ് ഉപയോഗിക്കുക:

1. [GitHub Releases](https://github.com/owu/wsl-dashboard/releases) പേജിലേക്ക് പോകുക.
2. Windows-നായുള്ള ഏറ്റവും പുതിയ `wsldashboard` എക്സിക്യൂട്ടബിൾ ഡൗൺലോഡ് ചെയ്യുക.
3. ഡീകംപ്രസ് ചെയ്യുക (കംപ്രസ്ഡ് ഫയൽ ആണെങ്കിൽ) `wsldashboard.exe` പ്രവർത്തിപ്പിക്കുക.

ഇൻസ്റ്റാളേഷൻ ആവശ്യമില്ല, ഈ ആപ്പ് സിംഗിൾ-ഫയൽ പോർട്ടബിൾ പ്രോഗ്രാം ആണ്.

### രീതി 2: സോഴ്സിൽ നിന്ന് ബിൽഡ് ചെയ്യുക

Rust ടൂൾചെയിൻ (Rust 1.92+ അല്ലെങ്കിൽ ഏറ്റവും പുതിയത്) ഇൻസ്റ്റാൾ ചെയ്തിട്ടുണ്ടെന്ന് ഉറപ്പാക്കുക.

1. റെപ്പോസിറ്ററി ക്ലോൺ ചെയ്യുക:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. ബിൽഡ് ചെയ്ത് പ്രവർത്തിപ്പിക്കുക:

   - ഡെവലപ്പ്മെന്റ് ഡീബഗ്:

     ```powershell
     cargo run
     ```
   - ഒപ്റ്റിമൈസ് ചെയ്ത റിലീസ് ബിൽഡിനായി ബിൽഡ് സ്ക്രിപ്റ്റ് ഉപയോഗിക്കുക:

     > ബിൽഡ് സ്ക്രിപ്റ്റിന് `x86_64-pc-windows-msvc` ടൂൾചെയിൻ ആവശ്യമാണ്.

     ```powershell
     .\build\portable\build.ps1
     ```


## 🛠️ സാങ്കേതിക സ്റ്റാക്കും പ്രവർത്തനക്ഷമതയും

- **കെർണൽ**: മെമ്മറി സുരക്ഷയും സീറോ-കോസ്റ്റ് ആബ്സ്ട്രാക്ഷനുകളും ഉറപ്പാക്കാൻ Rust-ൽ നടപ്പിലാക്കിയത്.
- **UI ഫ്രെയിംവർക്ക്**: ഹൈ-പെർഫോമൻസ് **Skia** റെൻഡറിംഗ് ബാക്കെൻഡുള്ള Slint.
- **അസിൻക്രോണസ് റൺടൈം**: നോൺ-ബ്ലോക്കിംഗ് സിസ്റ്റം കമാൻഡുകൾക്കും I/O-ക്കുമായി Tokio.
- **പ്രവർത്തനക്ഷമത ഹൈലൈറ്റുകൾ**:
  - **പ്രതികരണ വേഗത**: ഏകദേശം തൽക്ഷണ സ്റ്റാർട്ടപ്പ് വേഗത, WSL സ്ഥിതിയുടെ തത്സമയ നിരീക്ഷണം.
  - **റിസോഴ്സ് കാര്യക്ഷമത**: അതീവ കുറഞ്ഞ റിസോഴ്സ് ഉപയോഗം (കൂടുതൽ വിവരങ്ങൾക്ക് [പ്രധാന സവിശേഷതകൾ](#-പ്രധാന-സവിശേഷതകളും-ഉപയോഗവും) കാണുക).
  - **പോർട്ടബിലിറ്റി**: ഒപ്റ്റിമൈസ് ചെയ്ത റിലീസ് ബിൽഡ് ഒരു കോംപാക്ട് എക്സിക്യൂട്ടബിൾ സൃഷ്ടിക്കുന്നു.



## 🤝 കമ്മ്യൂണിറ്റി പിന്തുണ

ഈ കമ്മ്യൂണിറ്റികളുടെ പിന്തുണയ്ക്ക് ഹൃദയംഗമമായ നന്ദി:

- [Rust Programming Language](https://www.rust-lang.org) - ശക്തവും സുരക്ഷിതവുമായ പ്രോഗ്രാമിംഗ് ഭാഷ
- [Slint | Declarative GUI for Rust, C++, JavaScript & Python](https://slint.dev) - ആധുനിക UI ഫ്രെയിംവർക്ക്
- [WSL: Windows Subsystem for Linux](https://github.com/microsoft/WSL) - മികച്ച Windows Subsystem for Linux
- [Tokio - An asynchronous Rust runtime](https://tokio.rs) - കാര്യക്ഷമമായ അസിൻക്രോണസ് റൺടൈം
- [Windows Developer Community](https://developer.microsoft.com/en-us/windows/community) - തുടർച്ചയായ പ്ലാറ്റ്‌ഫോം മെച്ചപ്പെടുത്തലുകൾ
- [Reddit](https://www.reddit.com) - ആഗോള കമ്മ്യൂണിറ്റി ചർച്ച & പിന്തുണ
- [Hacker News](https://news.ycombinator.com) - ആഗോള കമ്മ്യൂണിറ്റി ചർച്ച & പിന്തുണ
- [Linux.do](https://linux.do) - IT പ്രൊഫഷണലുകൾക്കുള്ള ജനപ്രിയ കമ്മ്യൂണിറ്റി
- [V2EX](https://www.v2ex.com) - ചൈനീസ് ടെക്ക് കമ്മ്യൂണിറ്റി ചർച്ച

നിങ്ങളുടെ സംഭാവനയും ഫീഡ്ബാക്കും ഈ പ്രോജക്ട് സാധ്യമാക്കി!


## ❤️ ഈ പ്രോജക്ടിനെ പിന്തുണയ്ക്കുക

- ഈ പ്രോജക്ട് GPL-3.0 ഓപ്പൺ സോഴ്സ് ലൈസൻസ് പിന്തുടരുന്നു, എല്ലാ ഉപയോക്താക്കൾക്കും സൗജന്യം.
- ഫീച്ചർ ഡെവലപ്പ്മെന്റ്, ദൈനംദിന പരിശോധന മുതൽ ബഗ് പരിഹാരം വരെ, എല്ലാ പ്രവർത്തനങ്ങളും ഒഴിവുസമയത്തെ നിരന്തര പരിശ്രമത്തിൽ നിന്ന് വരുന്നു. ഓപ്പൺ സോഴ്സ് പാത ഒറ്റയ്ക്ക് എളുപ്പമല്ല, നിങ്ങളുടെ അംഗീകാരവും പിന്തുണയും പ്രോജക്ട് ദീർഘകാലം തുടരാൻ ശക്തി.
- ഈ ഉപകരണം നിങ്ങൾക്ക് യഥാർത്ഥത്തിൽ സഹായകമാണെന്ന് തോന്നുന്നുവെങ്കിൽ, ദയവായി സഹായഹസ്തം നീട്ടുക. എല്ലാ സംഭാവനകളും സർവർ ചെലവ്, പതിപ്പ് അപ്ഡേറ്റുകൾ, ഫീച്ചർ ഒപ്റ്റിമൈസേഷൻ എന്നിവയ്ക്ക് ഉപയോഗിക്കും.
- ചെറിയ ദയയും നക്ഷത്രം പോലെ തിളങ്ങുന്നു. നിങ്ങളുടെ മനസ്സിലാക്കലിനും ഔദാര്യത്തിനും വീണ്ടും നന്ദി!

ഞങ്ങളുടെ സംഭാവന പേജ് സന്ദർശിക്കുക: [https://www.wslui.com/donate/](https://www.wslui.com/donate/)


## ⭐️ സ്നേഹത്തോടെ പ്രവർത്തിക്കുന്ന പ്രോജക്ട്

ഈ പ്രോജക്ട് നിങ്ങൾക്ക് സഹായകമാണെന്ന് തോന്നുന്നുവെങ്കിൽ, GitHub-ൽ ഒരു നക്ഷത്രം നൽകി നിങ്ങളുടെ അംഗീകാരം നൽകാൻ ഞാൻ നന്ദിയുള്ളവനാണ്. നിങ്ങളുടെ അംഗീകാരം പ്രോജക്ടിനെ വിശാലമായ ഉപയോക്താക്കളിലേക്ക് എത്തിക്കാൻ സഹായിക്കും. ഈ പ്രോത്സാഹനം എന്നെ നിരന്തരം മുന്നോട്ട് നയിക്കുന്നു.


## 📄 ഓപ്പൺ സോഴ്സ് ലൈസൻസ്

ഈ പ്രോജക്ട് GPL-3.0 ലൈസൻസിന് കീഴിൽ ലൈസൻസ് ചെയ്തിരിക്കുന്നു – കൂടുതൽ വിവരങ്ങൾക്ക് [LICENSE](../LICENSE) ഫയൽ കാണുക.


---

Built with ❤️ for the WSL Community.