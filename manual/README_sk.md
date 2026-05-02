# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

Moderný, vysoko výkonný a ľahký ovládací panel na správu inštancií WSL (Windows Subsystem for Linux). Postavený v jazyku Rust a frameworku Slint pre prvotriedny natívny zážitok.

---

```diff
Oznámenie:

- Tento softvér sa ne distribuuje prostredníctvom Microsoft Store.
- Akákoľvek aplikácia uvedená tam pod rovnakým názvom je neoprávnená a môže být falošná.
- Prosím, nestahujte ju, aby ste sa vyhli potenciálnym podvodom.
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

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | Slovenčina | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md)

---

## 📑 Obsah
- [🌍 Podporované jazyky](#-podporované-jazyky)
- [🚀 Kľúčové vlastnosti a použitie](#-kľúčové-vlastnosti-a-použitie)
- [⚙️ Konfigurácia a protokoly](#️-konfigurácia-a-protokoly)
- [🖼️ Snímky obrazovky](#️-snímky-obrazovky)
- [🎬 Ukážka prevádzky](#-ukážka-prevádzky)
- [💻 Systémové požiadavky](#-systémové-požiadavky)
- [📦 Inštalačná príručka](#-inštalačná-príručka)
- [🛠️ Technologický zásobník a výkon](#️-technologický-zásobník-a-výkon)
- [⭐️ Dielo z lásky](#️-dielo-z-lásky)
- [📄 Licencia](#-licencia)

---

## 🌍 Podporované jazyky

Angličtina, zjednodušená čínština, tradičná čínština, hindčina, španielčina, francúzština, arabčina, bengálčina, portugalčina, ruština, urdčina, indonézština, nemčina, japončina, turečtina, kórejčina, taliančina, holandčina, švédčina, čeština, gréčtina, maďarčina, hebrejčina, nórčina, dánčina, fínčina, slovenčina, slovinčina, islandčina.

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="Angličtina" alt="Angličtina" />
  <img src="../assets/flags/cn.svg" width="32" title="Čínština (Zjednodušená)" alt="Čínština (Zjednodušená)" />
  <img src="../assets/flags/tw.svg" width="32" title="Čínština (Tradičná)" alt="Čínština (Tradičná)" />
  <img src="../assets/flags/in.svg" width="32" title="Hindčina" alt="Hindčina" />
  <img src="../assets/flags/es.svg" width="32" title="Španielčina" alt="Španielčina" />
  <img src="../assets/flags/fr.svg" width="32" title="Francúzština" alt="Francúzština" />
  <img src="../assets/flags/sa.svg" width="32" title="Arabčina" alt="Arabčina" />
  <img src="../assets/flags/bd.svg" width="32" title="Bengálčina" alt="Bengálčina" />
  <img src="../assets/flags/pt.svg" width="32" title="Portugalčina" alt="Portugalčina" />
  <img src="../assets/flags/ru.svg" width="32" title="Ruština" alt="Ruština" />
  <img src="../assets/flags/pk.svg" width="32" title="Urdu" alt="Urdu" />
  <img src="../assets/flags/id.svg" width="32" title="Indonézština" alt="Indonézština" />
  <img src="../assets/flags/de.svg" width="32" title="Nemčina" alt="Nemčina" />
  <img src="../assets/flags/jp.svg" width="32" title="Japončina" alt="Japončina" />
  <img src="../assets/flags/tr.svg" width="32" title="Turečtina" alt="Turečtina" />
  <img src="../assets/flags/kr.svg" width="32" title="Korejčina" alt="Korejčina" />
  <img src="../assets/flags/it.svg" width="32" title="Taliančina" alt="Taliančina" />
  <img src="../assets/flags/nl.svg" width="32" title="Holandčina" alt="Holandčina" />
  <img src="../assets/flags/se.svg" width="32" title="Švédčina" alt="Švédčina" />
  <img src="../assets/flags/cz.svg" width="32" title="Čeština" alt="Čeština" />
  <img src="../assets/flags/gr.svg" width="32" title="Gréčtina" alt="Gréčtina" />
  <img src="../assets/flags/hu.svg" width="32" title="Maďarčina" alt="Maďarčina" />
  <img src="../assets/flags/il.svg" width="32" title="Hebrejčina" alt="Hebrejčina" />
  <img src="../assets/flags/no.svg" width="32" title="Nórčina" alt="Nórčina" />
  <img src="../assets/flags/dk.svg" width="32" title="Dánčina" alt="Dánčina" />
  <img src="../assets/flags/fi.svg" width="32" title="Fínčina" alt="Fínčina" />
  <img src="../assets/flags/sk.svg" width="32" title="Slovenčina" alt="Slovenčina" />
  <img src="../assets/flags/si.svg" width="32" title="Slovinčina" alt="Slovinčina" />
  <img src="../assets/flags/is.svg" width="32" title="Islandčina" alt="Islandčina" />
</p>


## 🚀 Kľúčové vlastnosti a použitie

- **Moderné natívne UI**: Intuitívne grafické rozhranie s podporou tmavého/svetlého režimu, plynulými animáciami a vysoko výkonným vykresľovaním pomocou engine **Skia**.
- **Integrácia do systémovej lišty (Tray)**: Úplná podpora minimalizácie do lišty (využitie RAM ~10 MB), obnovenie dvojitým kliknutím a funkčná kontextová ponuka pravým tlačidlom.
- **Inteligentné spúšťanie**: Nakonfigurujte panel tak, aby sa spúšťal so systémom Windows, minimalizoval sa do lišty (tichý režim s parametrom `/silent`) a automaticky ukončoval distribúcie pri ukončení.
- **Komplexná správa inštancií**: Spustenie, zastavenie, ukončenie a zrušenie registrácie jedným kliknutím. Sledovanie stavu v reálnom čase a podrobné informácie o zaplnení disku a umiestnení súborov.
- **Správa distribúcií**: Nastavenie ako predvolené, migrácia (presun VHDX na iné disky) a export/klonovanie do archívov `.tar` alebo `.tar.gz`.
- **Rýchla integrácia**: Okamžité spúšťanie Terminálu, VS Code alebo Prieskumníka súborov s prispôsobiteľnými pracovnými adresármi a háčikmi pre spúšťacie skripty.
- **Inteligentná inštalácia**: Inštalácia z Microsoft Store, GitHubu alebo lokálnych súborov (RootFS/VHDX). Obsahuje vstavaného pomocníka na sťahovanie RootFS.
- **Bezpečnosť**: Zámky mutex pre bezpečné súbežné migračné a zálohovacie operácie a automatické čistenie Appx pri odoberaní.
- **Extrémne nízke nároky na pamäť**: Vysoko optimalizované pre efektivitu. Tichý štart (v lište) využíva iba **~10 MB** RAM. Využitie v režime okna sa líši podľa zložitosti písma: **~18 MB** pre štandardné jazyky a **~38 MB** pre jazyky s rozsiahlymi znakovými sadami (čínština, japončina, kórejčina).
- **Pokročilé siete**: Plynulá správa presmerovania portov (s automatickým vytváraním pravidiel brány firewall) a globálna konfigurácia HTTP proxy na zjednotené pripojenie.
- **Správa zariadení USB**: Úplná integrácia s aplikáciou `usbipd-win` na bezproblémové viazanie, pripájanie a správu lokálnych zariadení USB vo vašich inštanciách WSL priamo z používateľského rozhrania.


## ⚙️ Konfigurácia a protokoly

Všetka konfigurácia sa spravuje prostredníctvom zobrazenia Nastavenia:

- Výber predvoleného inštalačného adresára pre nové inštancie WSL.
- Konfigurácia adresára pre protokoly a úrovne protokolovania (Error / Warn / Info / Debug / Trace).
- Výber jazyka rozhrania alebo nastavenie podľa systému.
- Prepínanie tmavého režimu a nastavenie automatického ukončovania WSL po operáciách.
- Konfigurácia frekvencie kontroly aktualizácií (denne, týždenne, dvojtýždenne, mesačne).
- Povolenie automatického spúšťania pri štarte systému (s automatickou opravou cesty).
- Nastavenie minimalizácie do lišty pri spustení pre nerušený zážitok.
- Nastavenie tlačidla zavrieť pre minimalizáciu do lišty namiesto ukončenia programu.
- Prispôsobte si bočný panel prepínaním viditeľnosti konkrétnych kariet funkcií.

Súbory protokolov sa zapisujú do nakonfigurovaného adresára a možno ich priložiť pri hlásení problémov.


## 🖼️ Snímky obrazovky

### Domov (Svetlý a tmavý režim)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB  a zbalené menu
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### sieť
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### Pridať inštanciu a Nastavenia
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>

### O aplikácii
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
</p>

## 🎬 Ukážka prevádzky

Nižšie je ukážka WSL Dashboard v akcii:

![Ukážka WSL Dashboard](../assets/screenshot/demo.gif)



## 💻 Systémové požiadavky

- Windows 10 alebo Windows 11 s povoleným WSL (odporúča sa WSL 2).
- Aspoň jedna nainštalovaná distribúcia WSL alebo oprávnenie na inštaláciu nových.
- 64-bitový procesor; pre plynulé používanie viacerých distribúcií sa odporúča 4 GB RAM alebo viac.

## 📦 Inštalačná príručka

### Možnosť 1: Stiahnutie vopred zostaveného binárneho súboru

Najjednoduchší spôsob, ako začať, je použiť predkompilovanú verziu:

1. Prejdite na stránku [GitHub Releases](https://github.com/owu/wsl-dashboard/releases).
2. Stiahnite si najnovší spustiteľný súbor `wsldashboard` pre Windows.
3. Rozbaľte ho (ak je v archíve) a spustite `wsldashboard.exe`.

Nevyžaduje sa žiadny inštalátor; aplikácia je jediný prenosný binárny súbor.

### Možnosť 2: Zostavenie zo zdrojového kódu

Uistite sa, že máte nainštalovanú sadu nástrojov Rust (Rust 1.92+ alebo novšiu).

1. Naklonujte repozitár:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. Zostavenie a spustenie:

   - Pre vývoj:

     ```powershell
     cargo run
     ```
   - Optimalizované produkčné zostavenie pomocou zostavovacieho skriptu:

     > Zostavovací skript vyžaduje sadu nástrojov `x86_64-pc-windows-msvc`.

     ```powershell
     .\build\scripts\build.ps1
     ```


## 🛠️ Technologický zásobník a výkon

- **Jadro**: Implementované v jazyku Rust pre bezpečnosť pamäte a nulové náklady na abstrakcie.
- **UI Framework**: Slint s vysoko výkonným vykresľovacím enginom **Skia**.
- **Asynchrónne behové prostredie**: Tokio pre neblokujúce systémové príkazy a I/O.
- **Hlavné výhody výkonu**:
  - **Responzivita**: Takmer okamžité spustenie a sledovanie stavu WSL v reálnom čase.
  - **Efektivita**: Extrémne nízke využitie zdrojov (podrobnosti pozri [Kľúčové vlastnosti](#-kľúčové-vlastnosti-a-použitie)).
  - **Prenositeľnosť**: Optimalizované zostavenie produkuje jediný kompaktný spustiteľný súbor.



## ⭐️ Dielo z lásky

Ak považujete tento projekt za užitočný, bol by som vďačný, keby ste mu mohli nechať hviezdičku na GitHub-e. Vaša podpora mu pomáha osloviť širšie publikum a hlboko si ju vážim. Práve toto povzbudenie ma motivuje pokračovať v budovaní.

## 📄 Licencia

Tento projekt je licencovaný pod GPL-3.0 – podrobnosti nájdete v súbore [LICENSE](../LICENSE).

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
