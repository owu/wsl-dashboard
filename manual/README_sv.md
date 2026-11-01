# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logotyp" />
</p>

En modern, högpresterande, lättviktig och minnessnål kontrollpanel för WSL-instanshantering (Windows Subsystem for Linux). Byggd med Rust och Slint för en förstklassig inbyggd upplevelse.

```diff
Meddelande:

- WSL Dashboard distribueras inte via Microsoft Store.
- Alla applikationer som listas där under namnet "WSL Dashboard" är oauktoriserade och kan vara förfalskade.
- Vänligen ladda inte ner det för att undvika potentiella bedrägerier.
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

README I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | Svenska | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 Innehållsförteckning
[🌍 Språkstöd](#-språkstöd) | [🚀 Huvudfunktioner & Användning](#-huvudfunktioner--användning) | [🖼️ Skärmbilder](#️-skärmbilder) | [📦 Installationsguide](#-installationsguide) | [📢 Officiella samhällsgrupper](#-officiella-samhällsgrupper) | [❤️ Stöd detta projekt](#️-stöd-detta-projekt) | [⭐️ Ett kärleksprojekt](#️-ett-kärleksprojekt) | [📄 Licens](#-licens)

---

## 🌍 Språkstöd

Engelska, Kinesiska (Förenklad), Kinesiska (Traditionell), Hindi, Spanska, Franska, Arabiska, Bengaliska, Portugisiska, Ryska, Urdu, Indonesiska, Tyska, Japanska, Turkiska, Koreanska, Italienska, Nederländska, Svenska, Tjeckiska, Grekiska, Ungerska, Hebreiska, Norska, Danska, Finska, Slovakiska, Slovenska, Isländska, Vietnamesiska, Telugu, Javanska, Thailändska, Tamil, Filippinska, Punjabi, Malajiska, Polska, Ukrainska, Persiska, Kannada, Marathi, Hausa, Burmesiska, Uzbekiska, Azerbajdzjanska, Cebuano, Malayalam, Sindhi, Amhariska.

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="Engelska" alt="Engelska" />
  <img src="../assets/flags/cn.svg" width="32" title="Kinesiska (Förenklad)" alt="Kinesiska (Förenklad)" />
  <img src="../assets/flags/tw.svg" width="32" title="Kinesiska (Traditionell)" alt="Kinesiska (Traditionell)" />
  <img src="../assets/flags/in.svg" width="32" title="Hindi" alt="Hindi" />
  <img src="../assets/flags/es.svg" width="32" title="Spanska" alt="Spanska" />
  <img src="../assets/flags/fr.svg" width="32" title="Franska" alt="Franska" />
  <img src="../assets/flags/sa.svg" width="32" title="Arabiska" alt="Arabiska" />
  <img src="../assets/flags/bd.svg" width="32" title="Bengaliska" alt="Bengaliska" />
  <img src="../assets/flags/pt.svg" width="32" title="Portugisiska" alt="Portugisiska" />
  <img src="../assets/flags/ru.svg" width="32" title="Ryska" alt="Ryska" />
  <img src="../assets/flags/pk.svg" width="32" title="Urdu" alt="Urdu" />
  <img src="../assets/flags/id.svg" width="32" title="Indonesiska" alt="Indonesiska" />
  <img src="../assets/flags/de.svg" width="32" title="Tyska" alt="Tyska" />
  <img src="../assets/flags/jp.svg" width="32" title="Japanska" alt="Japanska" />
  <img src="../assets/flags/tr.svg" width="32" title="Turkiska" alt="Turkiska" />
  <img src="../assets/flags/kr.svg" width="32" title="Koreanska" alt="Koreanska" />
  <img src="../assets/flags/it.svg" width="32" title="Italienska" alt="Italienska" />
  <img src="../assets/flags/nl.svg" width="32" title="Nederländska" alt="Nederländska" />
  <img src="../assets/flags/se.svg" width="32" title="Svenska" alt="Svenska" />
  <img src="../assets/flags/cz.svg" width="32" title="Tjeckiska" alt="Tjeckiska" />
  <img src="../assets/flags/gr.svg" width="32" title="Grekiska" alt="Grekiska" />
  <img src="../assets/flags/hu.svg" width="32" title="Ungerska" alt="Ungerska" />
  <img src="../assets/flags/il.svg" width="32" title="Hebreiska" alt="Hebreiska" />
  <img src="../assets/flags/no.svg" width="32" title="Norska" alt="Norska" />
  <img src="../assets/flags/dk.svg" width="32" title="Danska" alt="Danska" />
  <img src="../assets/flags/fi.svg" width="32" title="Finska" alt="Finska" />
  <img src="../assets/flags/sk.svg" width="32" title="Slovakiska" alt="Slovakiska" />
  <img src="../assets/flags/si.svg" width="32" title="Slovenska" alt="Slovenska" />
  <img src="../assets/flags/is.svg" width="32" title="Isländska" alt="Isländska" />
  <img src="../assets/flags/vn.svg" width="32" title="Vietnamesiska" alt="Vietnamesiska" />
  <img src="../assets/flags/in.svg" width="32" title="Telugu" alt="Telugu" />
  <img src="../assets/flags/id.svg" width="32" title="Javanska" alt="Javanska" />
  <img src="../assets/flags/th.svg" width="32" title="Thailändska" alt="Thailändska" />
  <img src="../assets/flags/in.svg" width="32" title="Tamil" alt="Tamil" />
  <img src="../assets/flags/ph.svg" width="32" title="Filippinska" alt="Filippinska" />
  <img src="../assets/flags/in.svg" width="32" title="Punjabi" alt="Punjabi" />
  <img src="../assets/flags/my.svg" width="32" title="Malajiska" alt="Malajiska" />
  <img src="../assets/flags/pl.svg" width="32" title="Polska" alt="Polska" />
  <img src="../assets/flags/ua.svg" width="32" title="Ukrainska" alt="Ukrainska" />
  <img src="../assets/flags/ir.svg" width="32" title="Persiska" alt="Persiska" />
  <img src="../assets/flags/in.svg" width="32" title="Kannada" alt="Kannada" />
  <img src="../assets/flags/in.svg" width="32" title="Marathi" alt="Marathi" />
  <img src="../assets/flags/ng.svg" width="32" title="Hausa" alt="Hausa" />
  <img src="../assets/flags/mm.svg" width="32" title="Burmesiska" alt="Burmesiska" />
  <img src="../assets/flags/uz.svg" width="32" title="Uzbekiska" alt="Uzbekiska" />
  <img src="../assets/flags/az.svg" width="32" title="Azerbajdzjanska" alt="Azerbajdzjanska" />
  <img src="../assets/flags/ph.svg" width="32" title="Cebuano" alt="Cebuano" />
  <img src="../assets/flags/in.svg" width="32" title="Malayalam" alt="Malayalam" />
  <img src="../assets/flags/pk.svg" width="32" title="Sindhi" alt="Sindhi" />
  <img src="../assets/flags/et.svg" width="32" title="Amhariska" alt="Amhariska" />
</p>

## 🚀 Huvudfunktioner & Användning

- **Modernt nativt UI**: Intuitivt GUI med stöd för mörkt/ljust läge, mjuka animationer och högpresterande rendering via **Skia**.
- **Systemfältsintegration (Tray)**: Fullt stöd för minimering till systemfältet (~10 MB RAM-användning), dubbelklicka för att växla mellan fönster och en funktionell högerklicksmeny.
- **Intelligent start**: Konfigurera panelen för att starta med Windows, minimera till systemfältet (tyst läge med `/silent`) och stänga av distributioner automatiskt vid avslut.
- **Uppgiftsschemaläggning**: Flexibel hantering av uppgiftsscheman som stöder automatisk körning av skript och kommandon i WSL-instanser vid angivna tider eller intervall för automatiserade operationer.
- **Omfattande instanskontroll**: Starta, stoppa, avsluta och avregistrera med ett klick. Statusövervakning i realtid och detaljerad information om diskanvändning och filplatser.
- **Hantering av distributioner**: Ställ in som standard, migrering (flytta VHDX till andra enheter) och exportera/klona till `.tar` eller `.tar.gz`-arkiv.
- **Diskmontering**: Montera och hantera VHDX-virtuella diskar för WSL-instanser direkt, med stöd för monterings-, avmonterings- och diskanteringsoperationer, vilket gör det enkelt att överföra filer mellan värddatorn och WSL.
- **Snabb integration**: Starta omedelbart i Terminal, VS Code eller Utforskaren med anpassningsbara arbetskataloger och startskriptshooks.
- **Distributioninstallation**: Installera Linuxdistributioner via Microsoft Store, GitHub, lokala filer (RootFS/VHDX) eller onlinespeglar (med automatisk hastighetstest för att välja den snabbaste spegeln och inbyggd RootFS-nedladdningsassistent).
- **Säkerhet**: Mutex-lås för säkra samtidiga migrerings-/backup-operationer och automatisk Appx-rensning vid borttagning.
- **Minimalt minnesavtryck**: Högt optimerad för effektivitet. Tyst start (systemfältet) använder endast **~10 MB** RAM. Användning i fönsterläge varierar beroende på tecknens komplexitet: **~18 MB** för standardspråk och **~38 MB** för språk med stora teckenuppsättningar (kinesiska, japanska, koreanska).
- **Avancerat nätverk**: Sömlös hantering av portvidarebefordran (med automatiskt skapande av brandväggsregler) och global HTTP-proxykonfiguration för enhetlig anslutning.
- **Hantering av USB-enheter**: Fullständig integrering med `usbipd-win` för att smidigt binda, ansluta och hantera lokala USB-enheter över dina WSL-instanser direkt från kontrollpanelen.
- **Konfiguration och loggar**: Hantera i Inställningar standardinstallationsmapp, loggmapp och loggnivå, gränssnittsspråk, mörkt läge, uppdateringsfrekvens, autostart och stängningsbeteende samt sidofältet.
- **Hålla distributioner aktiva**: Ett klick på "Starta" håller distributionen igång i stället för att stängas efter ~60 s inaktivitet; du kan också aktivera "Starta denna distribution när Windows startar".
- **Globala genvägar**: Ange globala snabbtangenter för att visa/dölja huvudfönstret eller öppna en distributions terminal direkt.


## 🖼️ Skärmbilder
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

Vill du se instrumentpanelen i aktion? Besök den [officiella webbplatsen](https://www.wslui.com/) för det fullständiga skärmbildsarkivet.

## 📦 Installationsguide

### Alternativ 1: Besök projektets webbplats (Rekommenderas)

Vi rekommenderar att du besöker den officiella webbplatsen för att ladda ner, eftersom den erbjuder flera speglar för en smidigare upplevelse:

Gå till [Nedladdningssidan](https://www.wslui.com/download/) och välj den spegel som passar din region.

### Alternativ 2: Installera via winget

Du kan installera WSLDashboard direkt från Windows Package Manager (winget), med antingen monikern eller det fullständiga paketidentifieraren:

```powershell
# Sök (skiftlägesokänsligt)
winget search wsl-dashboard
# eller
winget search WSLDashboard

# Installera (välj ett)
winget install wsl-dashboard
# eller
winget install Owu.WSLDashboard
```

> Winget-paketidentifieraren är `Owu.WSLDashboard` och monikern är `wsl-dashboard` (skiftlägesokänsligt). Båda fungerar.

För mer information, besök [WinGet-communityförvaret](https://github.com/microsoft/winget-pkgs/tree/master/manifests/o/Owu/WSLDashboard).

### Alternativ 3: Bygg från källkod

Se till att du har Rust-verktygskedjan (Rust 1.92+ eller nyare) installerad.

1. Klona arkivet:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. Bygg och kör:

   - För utveckling:

     ```powershell
     cargo run
     ```
   - Optimerad release-build med byggskriptet:

     > Byggskriptet kräver verktygskedjan `x86_64-pc-windows-msvc`.

     ```powershell
     .\build\portable\build.ps1 -Arch x64
     ```


## 📢 Officiella samhällsgrupper

Välkommen till de officiella WSL UI-samhällsgrupperna! Gå med i våra kanaler för att diskutera användningstips, rapportera problem, få de senaste uppdateringarna och koppla upp dig mot utvecklare och andra användare.

<div align="center">

| Global grupp (Telegram) | Kinesisk grupp (WeChat) |
| :---: | :---: |
| <img src="https://cdn1.wslui.com/static/images/group/group-telegram.png" width="240" alt="Telegram Group QR Code" /> | <img src="https://cdn1.wslui.com/static/images/group/group-wechat.png" width="240" alt="WeChat Group QR Code" /> |
| Skanna för att gå med | Skanna för att gå med |

</div>


## ❤️ Stöd detta projekt

- Detta projekt är licensierat under GPL-3.0 och är gratis för alla användare.
- Från funktionsutveckling och dagliga tester till buggfixar — allt arbete görs på fritiden. Vägen för öppen källkod är inte lätt att vandra ensam. Ditt erkännande och stöd ger projektet förtroendet att fortsätta.
- Om detta verktyg verkligen har hjälpt dig, överväg att ge ett bidrag. Alla donationer går till serverkostnader, versionsuppdateringar och funktionsförbättringar, vilket håller projektet kontinuerligt uppdaterat och stadigt framåtgående.
- Varje liten vänlighet är en strimma av stjärnljus. Tack igen för din förståelse och generositet！ [Besök vår donationssida](https://www.wslui.com/donate/).


## ⭐️ Ett kärleksprojekt

Om du har tyckt att det här projektet är användbart skulle jag vara tacksam om du kunde lämna en stjärna på GitHub. Ditt stöd hjälper det att nå en bredare publik och uppskattas djupt. Det är denna uppmuntran som motiverar mig att fortsätta bygga.


## 📄 Licens

Detta projekt är licensierat under GPL-3.0 – se filen [LICENSE](../LICENSE) för detaljer.
