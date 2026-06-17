# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

Et moderne, højtydende, letvægts og hukommelsesbesparende kontrolpanel til administration af WSL-instanser (Windows Subsystem for Linux). Bygget med Rust og Slint for en førsteklasses indfødt oplevelse.

---

```diff
Bemærk:

- WSL Dashboard distribueres ikke via Microsoft Store.
- Enhver applikation, der er anført der under navnet "WSL Dashboard", er uautoriseret og kan være forfalsket.
- Venligst download det ikke for at undgå potentielle svindelnumre.
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

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | Dansk | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 Indholdsfortegnelse
- [🌍 Sprogstøtte](#-sprogstøtte)
- [🚀 Nøglefunktioner og brug](#-nøglefunktioner-og-brug)
- [⚙️ Konfiguration og logfiler](#️-konfiguration-og-logfiler)
- [🖼️ Skærmbilleder](#️-skærmbilleder)
- [🎬 Demonstration](#-demonstration)
- [💻 Systemkrav](#-systemkrav)
- [📦 Installationsvejledning](#-installationsvejledning)
- [🛠️ Teknologistak og ydeevne](#️-teknologistak-og-ydeevne)
- [🤝 Fællesskabsstøtte](#-fællesskabsstøtte)
- [❤️ Støt dette projekt](#️-støt-dette-projekt)
- [⭐️ Arbejde udført med kærlighed](#️-arbejde-udført-med-kærlighed)
- [📄 Licens](#-licens)

---

## 🌍 Sprogstøtte

Engelsk, forenklet kinesisk, traditionelt kinesisk, hindi, spansk, fransk, arabisk, bengalsk, portugisisk, russisk, urdu, indonesisk, tysk, japansk, tyrkisk, koreansk, italiensk, hollandsk, svensk, tjekkisk, græsk, ungarsk, hebraisk, norsk, dansk, finsk, slovakisk, slovensk, islandsk, vietnamesisk, telugu, javanesisk, thai, tamil, filippinsk, punjabi, malay, polsk, ukrainsk, persisk, kannada, marathi, hausa, burmesisk, usbekisk, aserbajdsjansk, cebuano, malayalam, sindhi, amharisk.

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="Engelsk" alt="Engelsk" />
  <img src="../assets/flags/cn.svg" width="32" title="Kinesisk (Forenklet)" alt="Kinesisk (Forenklet)" />
  <img src="../assets/flags/tw.svg" width="32" title="Kinesisk (Traditionelt)" alt="Kinesisk (Traditionelt)" />
  <img src="../assets/flags/in.svg" width="32" title="Hindi" alt="Hindi" />
  <img src="../assets/flags/es.svg" width="32" title="Spansk" alt="Spansk" />
  <img src="../assets/flags/fr.svg" width="32" title="Fransk" alt="Fransk" />
  <img src="../assets/flags/sa.svg" width="32" title="Arabisk" alt="Arabisk" />
  <img src="../assets/flags/bd.svg" width="32" title="Bengalsk" alt="Bengalsk" />
  <img src="../assets/flags/pt.svg" width="32" title="Portugisisk" alt="Portugisisk" />
  <img src="../assets/flags/ru.svg" width="32" title="Russisk" alt="Russisk" />
  <img src="../assets/flags/pk.svg" width="32" title="Urdu" alt="Urdu" />
  <img src="../assets/flags/id.svg" width="32" title="Indonesisk" alt="Indonesisk" />
  <img src="../assets/flags/de.svg" width="32" title="Tysk" alt="Tysk" />
  <img src="../assets/flags/jp.svg" width="32" title="Japansk" alt="Japansk" />
  <img src="../assets/flags/tr.svg" width="32" title="Tyrkisk" alt="Tyrkisk" />
  <img src="../assets/flags/kr.svg" width="32" title="Koreansk" alt="Koreansk" />
  <img src="../assets/flags/it.svg" width="32" title="Italiensk" alt="Italiensk" />
  <img src="../assets/flags/nl.svg" width="32" title="Hollandsk" alt="Hollandsk" />
  <img src="../assets/flags/se.svg" width="32" title="Svensk" alt="Svensk" />
  <img src="../assets/flags/cz.svg" width="32" title="Tjekkisk" alt="Tjekkisk" />
  <img src="../assets/flags/gr.svg" width="32" title="Græsk" alt="Græsk" />
  <img src="../assets/flags/hu.svg" width="32" title="Ungarsk" alt="Ungarsk" />
  <img src="../assets/flags/il.svg" width="32" title="Hebraisk" alt="Hebraisk" />
  <img src="../assets/flags/no.svg" width="32" title="Norsk" alt="Norsk" />
  <img src="../assets/flags/dk.svg" width="32" title="Dansk" alt="Dansk" />
  <img src="../assets/flags/fi.svg" width="32" title="Finsk" alt="Finsk" />
  <img src="../assets/flags/sk.svg" width="32" title="Slovakisk" alt="Slovakisk" />
  <img src="../assets/flags/si.svg" width="32" title="Slovensk" alt="Slovensk" />
  <img src="../assets/flags/is.svg" width="32" title="Islandsk" alt="Islandsk" />
  <img src="../assets/flags/vn.svg" width="32" title="Vietnamesisk" alt="Vietnamesisk" />
  <img src="../assets/flags/in.svg" width="32" title="Telugu" alt="Telugu" />
  <img src="../assets/flags/id.svg" width="32" title="Javanesisk" alt="Javanesisk" />
  <img src="../assets/flags/th.svg" width="32" title="Thai" alt="Thai" />
  <img src="../assets/flags/in.svg" width="32" title="Tamil" alt="Tamil" />
  <img src="../assets/flags/ph.svg" width="32" title="Filippinsk" alt="Filippinsk" />
  <img src="../assets/flags/pk.svg" width="32" title="Punjabi" alt="Punjabi" />
  <img src="../assets/flags/my.svg" width="32" title="Malay" alt="Malay" />
  <img src="../assets/flags/pl.svg" width="32" title="Polsk" alt="Polsk" />
  <img src="../assets/flags/ua.svg" width="32" title="Ukrainsk" alt="Ukrainsk" />
  <img src="../assets/flags/ir.svg" width="32" title="Persisk" alt="Persisk" />
  <img src="../assets/flags/in.svg" width="32" title="Kannada" alt="Kannada" />
  <img src="../assets/flags/in.svg" width="32" title="Marathi" alt="Marathi" />
  <img src="../assets/flags/ng.svg" width="32" title="Hausa" alt="Hausa" />
  <img src="../assets/flags/mm.svg" width="32" title="Burmesisk" alt="Burmesisk" />
  <img src="../assets/flags/uz.svg" width="32" title="Usbekisk" alt="Usbekisk" />
  <img src="../assets/flags/az.svg" width="32" title="Aserbajdsjansk" alt="Aserbajdsjansk" />
  <img src="../assets/flags/ph.svg" width="32" title="Cebuano" alt="Cebuano" />
  <img src="../assets/flags/in.svg" width="32" title="Malayalam" alt="Malayalam" />
  <img src="../assets/flags/pk.svg" width="32" title="Sindhi" alt="Sindhi" />
  <img src="../assets/flags/et.svg" width="32" title="Amharisk" alt="Amharisk" />
</p>


## 🚀 Nøglefunktioner og brug

- **Moderne nativ brugerflade**: Intuitiv GUI med understøttelse af mørk/lys tilstand, jævne animationer og højtydende rendering drevet af **Skia**.
- **Integration med systembakke (Tray)**: Fuld understøttelse af minimering til systembakken (~10 MB RAM-forbrug), dobbeltklik for at skifte vindue og en funktionel højrekliksmenu.
- **Intelligent opstart**: Konfigurer kontrolpanelet til at starte med Windows, minimere til bakken (lydløs tilstand med `/silent`) og automatisk lukning af distributioner ved afslutning.
- **Omfattende instansstyring**: Start, stop, terminer og afregistrer med ét klik. Statusovervågning i realtid og detaljeret indsigt i diskforbrug og filplaceringer.
- **Distro-styring**: Indstil som standard, migrering (flyt VHDX til andre drev) og eksport/klonering til `.tar` eller `.tar.gz`-arkiver.
- **Hurtig integration**: Øjeblikkelig start i Terminal, VS Code eller Stifinder med brugerdefinerede arbejdsmapper og opstartsscript-hooks.
- **Distributioninstallation**: Installer Linux-distributioner via Microsoft Store, GitHub, lokale filer (RootFS/VHDX) eller onlinespejle (med automatisk hastighedstest for at vælge det hurtigste spejl og indbygget RootFS-downloadhjælper).
- **Global sikkerhed**: Mutex-låse til sikre samtidige migrerings-/backupoperationer og automatisk Appx-oprydning ved fjernelse.
- **Ultra-lavt hukommelsesaftryk**: Højt optimeret for effektivitet. Lydløs opstart (systembakke) bruger kun **~10 MB** RAM. Brug i vinduestilstand varierer efter skrifttypekompleksitet: **~18 MB** for standardsprog og **~38 MB** for sprog med store tegnsæt (kinesisk, japansk, koreansk).
- **Avanceret netværk**: Problemfri styring af portvideresendelse (med automatisk oprettelse af firewallregler) og global HTTP-proxykonfiguration for samlet forbindelse.
- **Håndtering af USB-enheder**: Fuld integration med `usbipd-win` for ubesværet binding, vedhæftning og administration af lokale USB-enheder på tværs af dine WSL-instanser direkte fra dashboardet.


## ⚙️ Konfiguration og logfiler

Al konfiguration administreres via visningen Indstillinger:

- Vælg standardinstallationsmappe for nye WSL-instanser.
- Konfigurer logmappe og logniveau (Error / Warn / Info / Debug / Trace).
- Vælg brugerfladesprog eller lad det følge systemsproget.
- Skift mellem mørk tilstand og om appen skal lukke WSL automatisk efter operationer.
- Konfigurer hvor ofte appen skal søge efter opdateringer (dagligt, ugentligt, hver anden uge, månedligt).
- Aktiver automatisk start ved systemstart (med automatisk reparation af sti).
- Indstil appen til at minimere til systembakken ved opstart.
- Konfigurer lukkeknappen til at minimere til systembakken i stedet for at afslutte.
- Tilpas sidepanelet ved at slå synligheden af specifikke funktionfaner til eller fra.

Logfiler skrives til den konfigurerede logmappe og kan vedlægges ved rapportering af problemer.


## 🖼️ Skærmbilleder

### Hjem (Lys og mørk tilstand)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB og minimeret menu
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### Nettverk
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### Tilføj instans og indstillinger
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>
<p align="center">
  <img src="../assets/screenshot/settings-advanced.png" width="48%" />
  <img src="../assets/screenshot/settings-interface.png" width="48%" />
</p>

### Om & Doner
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/donate.png" width="48%" />
</p>

## 🎬 Demonstration

[Hjælp os med at blive bedre! Se vores introduktionsvideo og del dine tanker.](https://github.com/owu/wsl-dashboard/discussions/9)



## 💻 Systemkrav

- Windows 10 eller Windows 11 med WSL aktiveret (WSL 2 anbefales).
- Mindst én WSL-distribution installeret, eller tilladelse til at installere nye.
- 64-bit CPU; 4 GB RAM eller mere anbefales til smidig brug af flere distroer.

## 📦 Installationsvejledning

### Mulighed 1: Download færdigbygget binær fil

Den nemmeste måde at komme i gang på er at bruge den prækompilerede udgivelse:

1. Gå til siden for [GitHub Releases](https://github.com/owu/wsl-dashboard/releases).
2. Download den seneste `wsldashboard` eksekverbare fil til Windows.
3. Udpak (hvis den er pakket) og kør `wsldashboard.exe`.

Der kræves ingen installation; appen er en enkelt bærbar binær fil.

### Mulighed 2: Byg fra kildekode

Sørg for, at du har Rust-værktøjskæden (Rust 1.92+ eller nyere) installeret.

1. Klôn lageret:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. Byg og kør:

   - Til udvikling:

     ```powershell
     cargo run
     ```
   - Optimeret udgivelsesbyg ved hjælp af byggescriptet:

     > Byggescriptet kræver `x86_64-pc-windows-msvc` værktøjskæden.

     ```powershell
     .\build\portable\build.ps1
     ```


## 🛠️ Teknologistak og ydeevne

- **Kerne**: Implementeret i Rust for hukommelsessikkerhed og nulomkostningsabstraktioner.
- **Brugerflade-framework**: Slint med højtydende **Skia** renderings-backend.
- **Async Runtime**: Tokio til ikke-blokerende systemkommandoer og I/O.
- **Højdepunkter for ydeevne**:
  - **Responstid**: Næsten øjeblikkelig opstart og realtidsovervågning af WSL-status.
  - **Effektivitet**: Ultra-lavt ressourceforbrug (se [Nøglefunktioner](#-nøglefunktioner-og-brug) for detaljer).
  - **Portabilitet**: Optimeret udgivelsesbyg producerer en enkelt kompakt eksekverbar fil.



## 🤝 Fællesskabsstøtte

En stor tak til følgende fællesskaber for deres støtte:

- [Rust Programming Language](https://www.rust-lang.org) - For det kraftfulde og sikre programmeringssprog
- [Slint | Declarative GUI for Rust, C++, JavaScript & Python](https://slint.dev) - For det moderne UI-rammeværk
- [WSL: Windows Subsystem for Linux](https://github.com/microsoft/WSL) - For det fantastiske Windows Subsystem for Linux
- [Tokio - An asynchronous Rust runtime](https://tokio.rs) - For den effektive asynkrone kørselstid
- [Windows Developer Community](https://developer.microsoft.com/en-us/windows/community) - For løbende platformforbedringer
- [Reddit](https://www.reddit.com) - For globale fællesskabsdiskussioner og støtte
- [Hacker News](https://news.ycombinator.com) - For globale fællesskabsdiskussioner og støtte
- [Linux.do](https://linux.do) - For det populære fællesskab for IT-fagfolk
- [V2EX](https://www.v2ex.com) - For diskussioner i det kinesiske teknologifællesskab

Dine bidrag og tilbagemeldinger gør dette projekt muligt！


## ❤️ Støt dette projekt

- Dette projekt er licenseret under GPL-3.0 og er gratis for alle brugere.
- Fra funktionsudvikling og daglige tests til fejlrettelser — alt arbejde udføres i fritiden. Vejen for open source er ikke let at gå alene. Din anerkendelse og støtte giver projektet tilliden til at fortsætte.
- Hvis dette værktøj virkelig har hjulpet dig, så overvej at give en hånd. Alle donationer går til serveromkostninger, versionsopdateringer og funktionsforbedringer, så projektet holdes løbende opdateret og stabilt fremadskridende.
- Hver lille venlighed er en stråle af stjernelys. Tak igen for din forståelse og generøsitet！

Besøg vores donationsside：[https://www.wslui.com/donate/](https://www.wslui.com/donate/)


## ⭐️ Arbejde udført med kærlighed

Hvis du har fundet dette projekt nyttigt, ville jeg være taknemmelig, hvis du ville efterlade en stjerne på GitHub. Din støtte hjælper det med at nå ud til et bredere publikum og værdsættes dybt. Det er denne opmuntring, der motiverer mig til at fortsætte med at bygge.


## 📄 Licens

Dette projekt er licenseret under GPL-3.0 – se [LICENSE](../LICENSE) filen for detaljer.


---

Built with ❤️ for the WSL Community.
