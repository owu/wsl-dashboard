# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logotyp" />
</p>

En modern, högpresterande, lättviktig och minnessnål kontrollpanel för WSL-instanshantering (Windows Subsystem for Linux). Byggd med Rust och Slint för en förstklassig inbyggd upplevelse.

---

```diff
Avisering:

- Denna programvara distribueras inte via Microsoft Store.
- Alla program som listas där under samma namn är obehöriga och kan vara förfalskade.
- Vänligen ladda inte ner den för att undvika potentiella bedrägerier.
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

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | Svenska | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 Innehållsförteckning
- [🌍 Språkstöd](#-språkstöd)
- [🚀 Huvudfunktioner & Användning](#-huvudfunktioner--användning)
- [⚙️ Konfiguration & Loggar](#️-konfiguration--loggar)
- [🖼️ Skärmbilder](#️-skärmbilder)
- [🎬 Demonstration](#-demonstration)
- [💻 Systemkrav](#-systemkrav)
- [📦 Installationsguide](#-installationsguide)
- [🛠️ Plattform & Prestanda](#️-plattform--prestanda)
- [🤝 Gemenskapsstöd](#-gemenskapsstöd)
- [❤️ Stöd detta projekt](#️-stöd-detta-projekt)
- [⭐️ Ett kärleksprojekt](#️-ett-kärleksprojekt)
- [📄 Licens](#-licens)

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
- **Omfattande instanskontroll**: Starta, stoppa, avsluta och avregistrera med ett klick. Statusövervakning i realtid och detaljerad information om diskanvändning och filplatser.
- **Hantering av distributioner**: Ställ in som standard, migrering (flytta VHDX till andra enheter) och exportera/klona till `.tar` eller `.tar.gz`-arkiv.
- **Snabb integration**: Starta omedelbart i Terminal, VS Code eller Utforskaren med anpassningsbara arbetskataloger och startskriptshooks.
- **Distributioninstallation**: Installera Linuxdistributioner via Microsoft Store, GitHub, lokala filer (RootFS/VHDX) eller onlinespeglar (med automatisk hastighetstest för att välja den snabbaste spegeln och inbyggd RootFS-nedladdningsassistent).
- **Säkerhet**: Mutex-lås för säkra samtidiga migrerings-/backup-operationer och automatisk Appx-rensning vid borttagning.
- **Minimalt minnesavtryck**: Högt optimerad för effektivitet. Tyst start (systemfältet) använder endast **~10 MB** RAM. Användning i fönsterläge varierar beroende på tecknens komplexitet: **~18 MB** för standardspråk och **~38 MB** för språk med stora teckenuppsättningar (kinesiska, japanska, koreanska).
- **Avancerat nätverk**: Sömlös hantering av portvidarebefordran (med automatiskt skapande av brandväggsregler) och global HTTP-proxykonfiguration för enhetlig anslutning.
- **Hantering av USB-enheter**: Fullständig integrering med `usbipd-win` för att smidigt binda, ansluta och hantera lokala USB-enheter över dina WSL-instanser direkt från kontrollpanelen.


## ⚙️ Konfiguration & Loggar

All konfiguration hanteras via inställningsvyn:

- Välj standardkatalog för installation av nya WSL-instanser.
- Konfigurera loggkatalog och loggnivå (Error / Warn / Info / Debug / Trace).
- Välj UI-språk eller låt det följa systemspråket.
- Växla mörkt läge och välj om appen ska stänga av WSL automatiskt efter operationer.
- Konfigurera hur ofta appen söker efter uppdateringar (dagligen, veckovis, varannan vecka, månadsvis).
- Aktivera automatisk start vid systemstart (med automatisk reparation av sökvägar).
- Ställ in appen att minimera till systemfältet vid start.
- Konfigurera stängningsknappen att minimera till systemfältet istället för att avsluta.
- Anpassa sidofältet genom att växla synligheten för specifika funktionsflikar.

Loggfiler skrivs till den konfigurerade loggkatalogen och kan bifogas vid felrapportering.


## 🖼️ Skärmbilder

### Hem (Ljust & Mörkt läge)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB & Komprimera menyn
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### nätverk
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### Lägg till instans & Inställningar
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>
<p align="center">
  <img src="../assets/screenshot/settings-advanced.png" width="48%" />
  <img src="../assets/screenshot/settings-interface.png" width="48%" />
</p>

### Om & Donera
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/donate.png" width="48%" />
</p>

## 🎬 Demonstration

[Hjälp oss att bli bättre! Titta på vår introduktionsvideo och dela dina tankar.](https://github.com/owu/wsl-dashboard/discussions/9)



## 💻 Systemkrav

- Windows 10 eller Windows 11 med WSL aktiverat (WSL 2 rekommenderas).
- Minst en WSL-distribution installerad, eller behörighet att installera nya.
- 64-bitars CPU; 4 GB RAM eller mer rekommenderas för smidig användning av flera distributioner.

## 📦 Installationsguide

### Alternativ 1: Ladda ner färdigkompilerad binär

Det enklaste sättet att komma igång är att använda den förkompilerade versionen:

1. Gå till sidan för [GitHub Releases](https://github.com/owu/wsl-dashboard/releases).
2. Ladda ner den senaste körbara filen `wsldashboard` för Windows.
3. Packa upp (om den är i ett arkiv) och kör `wsldashboard.exe`.

Ingen installation krävs; appen är en enda portabel binärfil.

### Alternativ 2: Bygg från källkod

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
     .\build\portable\build.ps1
     ```


## 🛠️ Plattform & Prestanda

- **Kärna**: Implementerad i Rust för minnessäkerhet och abstraktioner utan extra kostnad.
- **UI-ramverk**: Slint med högpresterande **Skia**-renderingsmotor.
- **Asynkron körtid**: Tokio för icke-blockerande systemkommandon och I/O.
- **Prestandahöjdpunkter**:
  - **Responsivitet**: Nästan omedelbar start och statusövervakning av WSL i realtid.
  - **Effektivitet**: Ultralåg resursförbrukning (se [Huvudfunktioner](#-huvudfunktioner--användning) för detaljer).
  - **Portabilitet**: Optimerad release-build skapar en enda kompakt körbar fil.



## 🤝 Gemenskapsstöd

Ett stort tack till följande gemenskaper för deras stöd:

- [Rust Programming Language](https://www.rust-lang.org) - För det kraftfulla och säkra programmeringsspråket
- [Slint | Declarative GUI for Rust, C++, JavaScript & Python](https://slint.dev) - För det moderna UI-ramverket
- [WSL: Windows Subsystem for Linux](https://github.com/microsoft/WSL) - För det fantastiska Windows Subsystem for Linux
- [Tokio - An asynchronous Rust runtime](https://tokio.rs) - För den effektiva asynkrona körtiden
- [Windows Developer Community](https://developer.microsoft.com/en-us/windows/community) - För kontinuerliga plattformsförbättringar
- [Reddit](https://www.reddit.com) - För globala gemenskapsdiskussioner och stöd
- [Hacker News](https://news.ycombinator.com) - För globala gemenskapsdiskussioner och stöd
- [Linux.do](https://linux.do) - För den populära gemenskapen för IT-proffs
- [V2EX](https://www.v2ex.com) - För diskussioner i den kinesiska teknikgemenskapen

Dina bidrag och feedback gör detta projekt möjligt！


## ❤️ Stöd detta projekt

- Detta projekt är licensierat under GPL-3.0 och är gratis för alla användare.
- Från funktionsutveckling och dagliga tester till buggfixar — allt arbete görs på fritiden. Vägen för öppen källkod är inte lätt att vandra ensam. Ditt erkännande och stöd ger projektet förtroendet att fortsätta.
- Om detta verktyg verkligen har hjälpt dig, överväg att ge ett bidrag. Alla donationer går till serverkostnader, versionsuppdateringar och funktionsförbättringar, vilket håller projektet kontinuerligt uppdaterat och stadigt framåtgående.
- Varje liten vänlighet är en strimma av stjärnljus. Tack igen för din förståelse och generositet！

Besök vår donationssida：[https://www.wslui.com/donate/](https://www.wslui.com/donate/)


## ⭐️ Ett kärleksprojekt

Om du har tyckt att det här projektet är användbart skulle jag vara tacksam om du kunde lämna en stjärna på GitHub. Ditt stöd hjälper det att nå en bredare publik och uppskattas djupt. Det är denna uppmuntran som motiverar mig att fortsätta bygga.


## 📄 Licens

Detta projekt är licensierat under GPL-3.0 – se filen [LICENSE](../LICENSE) för detaljer.


---

Built with ❤️ for the WSL Community.

