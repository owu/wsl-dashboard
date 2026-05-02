# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

Moderní, vysoce výkonný a lehký ovládací panel pro správu instancí WSL (Windows Subsystem for Linux). Postaveno na jazyce Rust a frameworku Slint pro prvotřídní nativní zážitek.

---

```diff
Upozornění:

- Tento software není distribuován prostřednictvím Microsoft Store.
- Jakákoli aplikace uvedená tam pod stejným názvem je neoprávněná a může být padělaná.
- Prosím, nestahujte ji, abyste se vyhnuli potenciálním podvodům.
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

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | Čeština | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md)

---

## 📑 Obsah
- [🌍 Podporované jazyky](#-podporované-jazyky)
- [🚀 Klíčové vlastnosti a použití](#-klíčové-vlastnosti-a-použití)
- [⚙️ Konfigurace a protokoly](#️-konfigurace-a-protokoly)
- [🖼️ Snímky obrazovky](#️-snímky-obrazovky)
- [🎬 Ukázka provozu](#-ukázka-provozu)
- [💻 Systémové požadavky](#-systémové-požadavky)
- [📦 Instalační příručka](#-instalační-příručka)
- [🛠️ Technologický zásobník a výkon](#️-technologický-zásobník-a-výkon)
- [⭐️ Dílo z lásky](#️-dílo-z-lásky)
- [📄 Licence](#-licence)

---

## 🌍 Podporované jazyky

Angličtina, zjednodušená čínština, tradiční čínština, hindština, španělština, francouzština, arabština, bengálština, portugalština, ruština, urdština, indonéština, němčina, japonština, turečtina, korejština, italština, nizozemština, švédština, čeština, řečtina, maďarština, hebrejština, norština, dánština, finština, slovenština, slovinština, islandština.

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="Angličtina" alt="Angličtina" />
  <img src="../assets/flags/cn.svg" width="32" title="Čínština (Zjednodušená)" alt="Čínština (Zjednodušená)" />
  <img src="../assets/flags/tw.svg" width="32" title="Čínština (Tradiční)" alt="Čínština (Tradiční)" />
  <img src="../assets/flags/in.svg" width="32" title="Hindština" alt="Hindština" />
  <img src="../assets/flags/es.svg" width="32" title="Španělština" alt="Španělština" />
  <img src="../assets/flags/fr.svg" width="32" title="Francouzština" alt="Francouzština" />
  <img src="../assets/flags/sa.svg" width="32" title="Arabština" alt="Arabština" />
  <img src="../assets/flags/bd.svg" width="32" title="Bengálština" alt="Bengálština" />
  <img src="../assets/flags/pt.svg" width="32" title="Portugalština" alt="Portugalština" />
  <img src="../assets/flags/ru.svg" width="32" title="Ruština" alt="Ruština" />
  <img src="../assets/flags/pk.svg" width="32" title="Urdu" alt="Urdu" />
  <img src="../assets/flags/id.svg" width="32" title="Indonéština" alt="Indonéština" />
  <img src="../assets/flags/de.svg" width="32" title="Němčina" alt="Němčina" />
  <img src="../assets/flags/jp.svg" width="32" title="Japonština" alt="Japonština" />
  <img src="../assets/flags/tr.svg" width="32" title="Turečtina" alt="Turečtina" />
  <img src="../assets/flags/kr.svg" width="32" title="Korejština" alt="Korejština" />
  <img src="../assets/flags/it.svg" width="32" title="Italština" alt="Italština" />
  <img src="../assets/flags/nl.svg" width="32" title="Nizozemština" alt="Nizozemština" />
  <img src="../assets/flags/se.svg" width="32" title="Švédština" alt="Švédština" />
  <img src="../assets/flags/cz.svg" width="32" title="Čeština" alt="Čeština" />
  <img src="../assets/flags/gr.svg" width="32" title="Řečtina" alt="Řečtina" />
  <img src="../assets/flags/hu.svg" width="32" title="Maďarština" alt="Maďarština" />
  <img src="../assets/flags/il.svg" width="32" title="Hebrejština" alt="Hebrejština" />
  <img src="../assets/flags/no.svg" width="32" title="Norština" alt="Norština" />
  <img src="../assets/flags/dk.svg" width="32" title="Dánština" alt="Dánština" />
  <img src="../assets/flags/fi.svg" width="32" title="Finština" alt="Finština" />
  <img src="../assets/flags/sk.svg" width="32" title="Slovenština" alt="Slovenština" />
  <img src="../assets/flags/si.svg" width="32" title="Slovinština" alt="Slovinština" />
  <img src="../assets/flags/is.svg" width="32" title="Islandština" alt="Islandština" />
</p>


## 🚀 Klíčové vlastnosti a použití

- **Moderní nativní UI**: Intuitivní grafické rozhraní s podporou tmavého/světlého režimu, plynulými animacemi a vysoce výkonným vykreslováním pomocí engine **Skia**.
- **Integrace do systémové lišty (Tray)**: Plná podpora minimalizace do lišty (využití RAM ~10 MB), obnovení poklepáním a funkční kontextová nabídka pravým tlačítkem.
- **Inteligentní spouštění**: Nakonfigurujte panel tak, aby se spouštěl se systémem Windows, minimalizoval se do lišty (tichý režim s parametrem `/silent`) a automaticky ukončoval distribuce při ukončení.
- **Komplexní správa instancí**: Spuštění, zastavení, ukončení a zrušení registrace jedním kliknutím. Sledování stavu v reálném čase a podrobné informace o zaplnění disku a umístění souborů.
- **Správa distribucí**: Nastavení jako výchozí, migrace (přesun VHDX na jiné disky) a export/klonování do archivů `.tar` nebo `.tar.gz`.
- **Rychlá integrace**: Okamžité spouštění Terminálu, VS Code nebo Průzkumníka souborů s přizpůsobitelnými pracovními adresáři a háčky pro spouštěcí skripty.
- **Chytrá instalace**: Instalace z Microsoft Store, GitHubu nebo lokálních souborů (RootFS/VHDX). Obsahuje vestavěného pomocníka pro stahování RootFS.
- **Bezpečnost**: Zámky mutex pro bezpečné souběžné migrační a zálohovací operace a automatické čištění Appx při odebírání.
- **Extrémně nízké nároky na paměť**: Vysoce optimalizováno pro efektivitu. Tichý start (v liště) využívá pouze **~10 MB** RAM. Využití v režimu okna se liší podle složitosti písma: **~18 MB** pro standardní jazyky a **~38 MB** pro jazyky s rozsáhlými znakovými sadami (čínština, japonština, korejština).
- **Pokročilé sítě**: Bezproblémová správa přesměrování portů (s automatickým vytvářením pravidel brány firewall) a globální konfigurace HTTP proxy pro sjednocené připojení.
- **Správa zařízení USB**: Plná integrace s `usbipd-win` pro snadné vázání, připojování a správu místních zařízení USB napříč instancemi WSL přímo z uživatelského rozhraní řídicího panelu.


## ⚙️ Konfigurace a protokoly

Veškerá konfigurace se spravuje prostřednictvím zobrazení Nastavení:

- Výběr výchozího instalačního adresáře pro nové instance WSL.
- Konfigurace adresáře pro protokoly a úrovně protokolování (Error / Warn / Info / Debug / Trace).
- Výběr jazyka rozhraní nebo nastavení podle systému.
- Přepínání tmavého režimu a nastavení automatického ukončování WSL po operacích.
- Konfigurace četnosti kontroly aktualizací (denně, týdně, čtrnáctidenně, měsíčně).
- Povolení automatického spouštění při startu systému (s automatickou opravou cesty).
- Nastavení minimalizace do lišty při spuštění.
- Nastavení tlačítka zavřít pro minimalizaci do lišty namísto ukončení programu.
- Přizpůsobte si postranní panel přepínáním viditelnosti konkrétních karet funkcí.

Soubory protokolů se zapisují do nakonfigurovaného adresáře a lze je přiložit při hlášení problémů.


## 🖼️ Snímky obrazovky

### Domů (Světlý a tmavý režim)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB & sbalené menu
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### sieť
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### Přidat instanci a Nastavení
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>

### O aplikaci
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
</p>

## 🎬 Ukázka provozu

Níže je ukázka WSL Dashboard v akci:

![Ukázka WSL Dashboard](../assets/screenshot/demo.gif)



## 💻 Systémové požadavky

- Windows 10 nebo Windows 11 s povoleným WSL (doporučeno WSL 2).
- Alespoň jedna nainstalovaná distribuce WSL nebo oprávnění k instalaci nových.
- 64bitový procesor; pro plynulé používání více distribucí doporučeno 4 GB RAM nebo více.

## 📦 Instalační příručka

### Možnost 1: Stažení předem sestaveného binárního souboru

Nejjednodušší způsob, jak začít, je použít předkompilovanou verzi:

1. Přejděte na stránku [GitHub Releases](https://github.com/owu/wsl-dashboard/releases).
2. Stáhněte si nejnovější spustitelný soubor `wsldashboard` pro Windows.
3. Rozbalte jej (pokud je v archivu) a spusťte `wsldashboard.exe`.

Není vyžadován žádný instalátor; aplikace je jediný přenosný binární soubor.

### Možnost 2: Sestavení ze zdrojového kódu

Ujistěte se, že máte nainstalovanou sadu nástrojů Rust (Rust 1.92+ nebo novější).

1. Naklonujte repozitář:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   ```

2. Sestavení a spuštění:

   - Pro vývoj:

     ```powershell
     cargo run
     ```
   - Optimalizované produkční sestavení pomocí sestavovacího skriptu:

     > Sestavovací skript vyžaduje sadu nástrojů `x86_64-pc-windows-msvc`.

     ```powershell
     .\build\scripts\build.ps1
     ```


## 🛠️ Technologický zásobník a výkon

- **Jádro**: Implementováno v jazyce Rust pro bezpečnost paměti a nulové náklady na abstrakce.
- **UI Framework**: Slint s vysoce výkonným vykreslovacím enginem **Skia**.
- **Asynchronní běhové prostředí**: Tokio pro neblokující systémové příkazy a I/O.
- **Hlavní výhody výkonu**:
  - **Responzivita**: Téměř okamžité spuštění a sledování stavu WSL v reálném čase.
  - **Efektivita**: Extrémně nízké využití zdrojů (podrobnosti viz [Klíčové vlastnosti](#-klíčové-vlastnosti-a-použití)).
  - **Přenositelnost**: Optimalizované sestavení produkuje jediný kompaktní spustitelný soubor.



## ⭐️ Dílo z lásky

Pokud vám tento projekt připadá užitečný, byl bych vděčný, kdybyste mu mohli nechat hvězdičku na GitHubu. Vaše podpora mu pomáhá oslovit širší publikum a hluboce si jí vážím. Právě toto povzbuzení mě motivuje k dalšímu budování.

## 📄 Licence

Tento projekt je licencován pod GPL-3.0 – podrobnosti naleznete v souboru [LICENSE](../LICENSE).

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
