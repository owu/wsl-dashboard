# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logó" />
</p>

Nútímalegt, afkastamikið, létt og minnissparandi mælaborð fyrir WSL (Windows Subsystem for Linux) tilviksstjórnun. Byggt með Rust og Slint fyrir frábæra innfædda upplifun.

```diff
Tilkynning:

- WSL Dashboard er ekki dreift í gegnum Microsoft Store.
- Allar forrit sem eru skráðar þar undir nafninu "WSL Dashboard" eru óheimilar og geta verið falsaðar.
- Vinsamlegast ekki hala því niður til að forðast hugsanleg svindl.
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

README I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | Íslenska | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 Efnisyfirlit
[🌍 Tungumálastuðningur](#-tungumálastuðningur) | [🚀 Helstu eiginleikar og notkun](#-helstu-eiginleikar-og-notkun) | [🖼️ Skjámyndir](#️-skjámyndir) | [📦 Uppsetningarleiðbeiningar](#-uppsetningarleiðbeiningar) | [📢 Opinber samfélags hópar](#-opinber-samfélags-hópar) | [❤️ Styðja þetta verkefni](#️-styðja-þetta-verkefni) | [⭐️ Gert af ást](#️-gert-af-ást) | [📄 Leyfi](#-leyfi)

---

## 🌍 Tungumálastuðningur

Enska, einfölduð kínverska, hefðbundin kínverska, hindí, spænska, franska, arabíska, bengalska, portúgalska, rússneska, úrdú, indónesíska, þýska, japanska, tyrkneska, kóreska, ítalska, hollenska, sænska, tékkneska, gríska, ungverska, hebreska, norska, danska, finnska, slóvakíska, slóvenska, íslenska, víetnamska, telúgú, javanska, taílenska, tamílska, filippínska, pundžabí, malajiska, pólska, úkraínska, persneska, kannada, marathí, haúsa, barmneska, úsbekska, aserbædsjanska, cebúanó, malajalam, sindhí, amharíska.

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="Enska" alt="Enska" />
  <img src="../assets/flags/cn.svg" width="32" title="Kínverska (Einfölduð)" alt="Kínverska (Einfölduð)" />
  <img src="../assets/flags/tw.svg" width="32" title="Kínverska (Hefðbundin)" alt="Kínverska (Hefðbundin)" />
  <img src="../assets/flags/in.svg" width="32" title="Hindí" alt="Hindí" />
  <img src="../assets/flags/es.svg" width="32" title="Spænska" alt="Spænska" />
  <img src="../assets/flags/fr.svg" width="32" title="Franska" alt="Franska" />
  <img src="../assets/flags/sa.svg" width="32" title="Arabíska" alt="Arabíska" />
  <img src="../assets/flags/bd.svg" width="32" title="Bengalska" alt="Bengalska" />
  <img src="../assets/flags/pt.svg" width="32" title="Portúgalska" alt="Portúgalska" />
  <img src="../assets/flags/ru.svg" width="32" title="Rússneska" alt="Rússneska" />
  <img src="../assets/flags/pk.svg" width="32" title="Úrdú" alt="Úrdú" />
  <img src="../assets/flags/id.svg" width="32" title="Indónesíska" alt="Indónesíska" />
  <img src="../assets/flags/de.svg" width="32" title="Þýska" alt="Þýska" />
  <img src="../assets/flags/jp.svg" width="32" title="Japanska" alt="Japanska" />
  <img src="../assets/flags/tr.svg" width="32" title="Tyrkneska" alt="Tyrkneska" />
  <img src="../assets/flags/kr.svg" width="32" title="Kóreska" alt="Kóreska" />
  <img src="../assets/flags/it.svg" width="32" title="Ítalska" alt="Ítalska" />
  <img src="../assets/flags/nl.svg" width="32" title="Hollenska" alt="Hollenska" />
  <img src="../assets/flags/se.svg" width="32" title="Sænska" alt="Sænska" />
  <img src="../assets/flags/cz.svg" width="32" title="Tékkneska" alt="Tékkneska" />
  <img src="../assets/flags/gr.svg" width="32" title="Gríska" alt="Gríska" />
  <img src="../assets/flags/hu.svg" width="32" title="Ungverska" alt="Ungverska" />
  <img src="../assets/flags/il.svg" width="32" title="Hebreska" alt="Hebreska" />
  <img src="../assets/flags/no.svg" width="32" title="Norska" alt="Norska" />
  <img src="../assets/flags/dk.svg" width="32" title="Danska" alt="Danska" />
  <img src="../assets/flags/fi.svg" width="32" title="Finnska" alt="Finnska" />
  <img src="../assets/flags/sk.svg" width="32" title="Slóvakíska" alt="Slóvakíska" />
  <img src="../assets/flags/si.svg" width="32" title="Slóvenska" alt="Slóvenska" />
  <img src="../assets/flags/is.svg" width="32" title="Íslenska" alt="Íslenska" />
  <img src="../assets/flags/vn.svg" width="32" title="Víetnamska" alt="Víetnamska" />
  <img src="../assets/flags/in.svg" width="32" title="Telúgú" alt="Telúgú" />
  <img src="../assets/flags/id.svg" width="32" title="Javanska" alt="Javanska" />
  <img src="../assets/flags/th.svg" width="32" title="Taílenska" alt="Taílenska" />
  <img src="../assets/flags/in.svg" width="32" title="Tamílska" alt="Tamílska" />
  <img src="../assets/flags/ph.svg" width="32" title="Filippínska" alt="Filippínska" />
  <img src="../assets/flags/pk.svg" width="32" title="Pundžabí" alt="Pundžabí" />
  <img src="../assets/flags/my.svg" width="32" title="Malajiska" alt="Malajiska" />
  <img src="../assets/flags/pl.svg" width="32" title="Pólska" alt="Pólska" />
  <img src="../assets/flags/ua.svg" width="32" title="Úkraínska" alt="Úkraínska" />
  <img src="../assets/flags/ir.svg" width="32" title="Persneska" alt="Persneska" />
  <img src="../assets/flags/in.svg" width="32" title="Kannada" alt="Kannada" />
  <img src="../assets/flags/in.svg" width="32" title="Marathí" alt="Marathí" />
  <img src="../assets/flags/ng.svg" width="32" title="Haúsa" alt="Haúsa" />
  <img src="../assets/flags/mm.svg" width="32" title="Barmneska" alt="Barmneska" />
  <img src="../assets/flags/uz.svg" width="32" title="Úsbekska" alt="Úsbekska" />
  <img src="../assets/flags/az.svg" width="32" title="Aserbædsjanska" alt="Aserbædsjanska" />
  <img src="../assets/flags/ph.svg" width="32" title="Cebúanó" alt="Cebúanó" />
  <img src="../assets/flags/in.svg" width="32" title="Malajalam" alt="Malajalam" />
  <img src="../assets/flags/pk.svg" width="32" title="Sindhí" alt="Sindhí" />
  <img src="../assets/flags/et.svg" width="32" title="Amharíska" alt="Amharíska" />
</p>

## 🚀 Helstu eiginleikar og notkun

- **Nútímalegt viðmót**: Innsæi GUI með stuðningi fyrir dökka og ljósa stillingu, mjúkar hreyfingar og afkastamikil teiknun knúin af **Skia**.
- **Samþætting við kerfisbakka (Tray)**: Fullur stuðningur við að fela forritið í kerfisbakkanum (~10MB vinnsluminni í notkun), tvísmella til að sýna/fela og hagnýtur hægri-smellismatseðill.
- **Snjöll ræsistýring**: Stilla stjórnborðið til að ræsast með Windows, lágmarkast í bakka (hljóðlaus ræsing með `/silent`) og slökkva sjálfkrafa á Linux-kerfum við lokun.
- **Verkefnaáætlun**: Sveigjanleg stjórnun verkefnaáætlunar, styður sjálfvirka framkvæmd skripta og skipana í WSL tilvikum á ákveðnum tíma eða með reglubundnu millibili fyrir sjálfvirkar aðgerðir.
- **Víðtæk tilvikastýring**: Ræsa, stöðva, loka og afskrá með einum smelli. Rauntímavöktun á stöðu og nákvæmar upplýsingar um disknotkun og staðsetningu skráa.
- **Umsjón með kerfum**: Setja sem sjálfgefið, flutningur (færa VHDX á aðra drif) og útflutningur/afritun á `.tar` eða `.tar.gz` sniði.
- **Diskatenging**: Tengdu og stjórnaðu VHDX sýndardiskum WSL tilvika beint, styður tengingu, aftengingu og diskastjórnun, auðveldar skráaflutning milli hýsil og WSL.
- **Hröð samþætting**: Ræsa beint í Terminal, VS Code eða skráakönnuð með sérsniðnum vinnuefnum og ræsiforritaskriftum.
- **Uppsetning dreifingar**: Settu upp Linux dreifingar í gegnum Microsoft Store, GitHub, staðbundnar skrár (RootFS/VHDX) eða netspeglar (með sjálfvirkri hraðaprófun til að velja hraðasta spegilinn og innbyggðu RootFS niðurhalshjálpartæki).
- **Alþjóðlegt öryggi**: Mutex-lásar fyrir örugga samtímaflutninga/afritun og sjálfvirk Appx-hreinsun við eyðingu.
- **Mjög lítil minnisnotkun**: Mjög fínstillt fyrir skilvirkni. Hljóðlaus ræsing (í bakka) notar aðeins um **~10MB** vinnsluminni. Notkun í gluggaham fer eftir leturgerðum: **~18MB** fyrir stöðluð tungumál og **~38MB** fyrir tungumál með stóra táknasett (kínverska, japanska, kóreska).
- **Ítarleg netkerfi**: Óaðfinnanleg stjórnun á áframsendingu gátta (með sjálfvirkri gerð eldveggsreglna) og alþjóðleg uppsetning HTTP-umboðsþjóns fyrir sameinaða tengingu.
- **Stjórnun USB-tækja**: Full samþætting við `usbipd-win` til að binda, tengja og stjórna staðbundnum USB-tækjum áreynslulaust í öllum WSL-tilvikum þínum beint af mælaborðinu.
- **Stillingar og annálar**: Stýra í Stillingum sjálfgefinni uppsetningarmöppu, annálamöppu og -stigi, viðmótsmáli, dökku ham, tíðni uppfærsluleitrar, sjálfvirkri ræsingu og hegðun við lokun, auk hliðarspjaltsins.
- **Halda dreifingum í gangi**: Einn smellur á „Ræsa“ heldur dreifingunni í gangi í stað þess að hún lokist eftir ~60 s aðgerðaleysi; þú getur líka virkjað „Ræsa þessa dreifingu við ræsingu Windows“.
- **Altökkvalar**: Skilgreindu altökkvala til að sýna/fela aðalgluggann eða opna fljótt skjáhermi valinnar dreifingar.


## 🖼️ Skjámyndir
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

Viltu sjá stjórnborðið í notkun? Heimsæktu [opinberu vefsíðuna](https://www.wslui.com/) til að skoða heila skjáskotsmyndasafnið.

## 📦 Uppsetningarleiðbeiningar

### Valkostur 1: Heimsækja vefsíðu verkefnisins (Mælt með)

Við mælum með að heimsækja opinberu vefsíðuna til að sækja, þar sem hún býður upp á marga speglatengla fyrir sléttari upplifun:

Farðu á [Niðurhalssíðuna](https://www.wslui.com/download/) og veldu spegilinn sem hentar þínu svæði.

### Valkostur 2: Uppsetning í gegnum winget

Þú getur sett upp WSLDashboard beint frá Windows Package Manager (winget), með því að nota annað hvort moniker eða fulla pakkaauðkennið:

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

> Winget pakkaauðkennið er `Owu.WSLDashboard` og moniker er `wsl-dashboard` (háð ekki stafastöðu). Hvor tveggja virkar.

Fyrir frekari upplýsingar, heimsæktu [WinGet samfélagsgeymsluna](https://github.com/microsoft/winget-pkgs/tree/master/manifests/o/Owu/WSLDashboard).

### Valkostur 3: Smíða úr frumkóða

Gakktu úr skugga um að þú hafir Rust tækjapakkann (Rust 1.92+ eða nýrri) uppsettan.

1. Afritaðu geymsluna:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. Smíða og keyra:

   - Fyrir þróun:

     ```powershell
     cargo run
     ```
   - Fínstillt útgáfusmíð með því að nota smíðaskriftuna:

     > Smíðaskriftan krefst `x86_64-pc-windows-msvc` tækjapakkans.

     ```powershell
     .\build\portable\build.ps1 -Arch x64
     ```


## 📢 Opinber samfélags hópar

Velkominn að heita í opinberu samfélags hópunum WSL UI! Aðgangi til að umræða ábendingar, tilkynna vandamál, fá uppfærslur og tengjast þróunaraðilum og öðrum notendum.

<div align="center">

| Alþjóðlegur hópur (Telegram) | Kínverskur hópur (WeChat) |
| :---: | :---: |
| <img src="https://cdn1.wslui.com/static/images/group/group-telegram.png" width="240" alt="Telegram Group QR Code" /> | <img src="https://cdn1.wslui.com/static/images/group/group-wechat.png" width="240" alt="WeChat Group QR Code" /> |
| Skanna til að ganga í | Skanna til að ganga í |

</div>


## ❤️ Styðja þetta verkefni

- Þetta verkefni er undir GPL-3.0 leyfi og er ókeypis fyrir alla notendur.
- Frá eiginleikaþróun og daglegum prófunum til villuleiðréttinga — öll vinna er unnin í frítíma. Leið opins hugbúnaðar er ekki auðvelt að ganga ein. Viðurkenning þín og stuðningur gefur verkefninu sjálfstraust til að halda áfram.
- Ef þetta tól hefur raunverulega hjálpað þér, íhugaðu að leggja hönd á plóg. Allar gjafir fara í þjónnuskostnað, útgáfuuppfærslur og eiginleikabætur, sem heldur verkefninu stöðugt uppfærðu og stöðugt framfarandi.
- Hver lítil góðvild er geisli stjörnuljósi. Takk aftur fyrir skilning þinn og rausnarsemi！ [Heimsæktu framlags síðuna okkar](https://www.wslui.com/donate/).


## ⭐️ Gert af ást

Ef þér finnst þetta verkefni gagnlegt væri ég þakklátur ef þú gætir gefið því stjörnu á GitHub. Stuðningur þinn hjálpar því að ná til breiðari hóps og er mikils metinn. Það er þessi hvatning sem dregur mig áfram í þróuninni.


## 📄 Leyfi

Þetta verkefni er gefið út undir GPL-3.0 leyfinu – sjá [LICENSE](../LICENSE) skrána fyrir nánari upplýsingar.
