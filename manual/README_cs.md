# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

Moderní, vysoce výkonný, lehký a na paměť nenáročný panel pro správu instancí WSL (Windows Subsystem for Linux). Postavený s Rust a Slint pro prémiový nativní zážitek.

```diff
Oznámení:

- WSL Dashboard není distribuován prostřednictvím Microsoft Store.
- Jakákoli aplikace uvedená tam pod názvem "WSL Dashboard" je neautorizovaná a může být padělaná.
- Prosím, nestahujte ji, abyste se vyhnuli případným podvodům.
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

README I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | Čeština | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 Obsah
[🌍 Podporované jazyky](#-podporované-jazyky) | [🚀 Klíčové vlastnosti a použití](#-klíčové-vlastnosti-a-použití) | [🖼️ Snímky obrazovky](#️-snímky-obrazovky) | [📦 Instalační příručka](#-instalační-příručka) | [📢 Oficiální komunitní skupiny](#-oficiální-komunitní-skupiny) | [❤️ Podpořit tento projekt](#️-podpořit-tento-projekt) | [⭐️ Dílo z lásky](#️-dílo-z-lásky) | [📄 Licence](#-licence)

---

## 🌍 Podporované jazyky

Angličtina, zjednodušená čínština, tradiční čínština, hindština, španělština, francouzština, arabština, bengálština, portugalština, ruština, urdština, indonéština, němčina, japonština, turečtina, korejština, italština, nizozemština, švédština, čeština, řečtina, maďarština, hebrejština, norština, dánština, finština, slovenština, slovinština, islandština, vietnamština, telugština, javánština, thajština, tamilština, filipínština, pandžábština, malajština, polština, ukrajinština, perština, kannadština, maráthština, hauština, barmština, uzbečtina, ázerbájdžánština, cebuánština, malayálamština, sindhština, amharština.

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
  <img src="../assets/flags/vn.svg" width="32" title="Vietnamština" alt="Vietnamština" />
  <img src="../assets/flags/in.svg" width="32" title="Telugština" alt="Telugština" />
  <img src="../assets/flags/id.svg" width="32" title="Javánština" alt="Javánština" />
  <img src="../assets/flags/th.svg" width="32" title="Thajština" alt="Thajština" />
  <img src="../assets/flags/in.svg" width="32" title="Tamilština" alt="Tamilština" />
  <img src="../assets/flags/ph.svg" width="32" title="Filipínština" alt="Filipínština" />
  <img src="../assets/flags/in.svg" width="32" title="Paňdžábština" alt="Paňdžábština" />
  <img src="../assets/flags/my.svg" width="32" title="Malajština" alt="Malajština" />
  <img src="../assets/flags/pl.svg" width="32" title="Polština" alt="Polština" />
  <img src="../assets/flags/ua.svg" width="32" title="Ukrajinština" alt="Ukrajinština" />
  <img src="../assets/flags/ir.svg" width="32" title="Perština" alt="Perština" />
  <img src="../assets/flags/in.svg" width="32" title="Kannadština" alt="Kannadština" />
  <img src="../assets/flags/in.svg" width="32" title="Maráthština" alt="Maráthština" />
  <img src="../assets/flags/ng.svg" width="32" title="Hauština" alt="Hauština" />
  <img src="../assets/flags/mm.svg" width="32" title="Barmština" alt="Barmština" />
  <img src="../assets/flags/uz.svg" width="32" title="Uzbečtina" alt="Uzbečtina" />
  <img src="../assets/flags/az.svg" width="32" title="Ázerbájdžánština" alt="Ázerbájdžánština" />
  <img src="../assets/flags/ph.svg" width="32" title="Cebuánština" alt="Cebuánština" />
  <img src="../assets/flags/in.svg" width="32" title="Malajálamština" alt="Malajálamština" />
  <img src="../assets/flags/pk.svg" width="32" title="Sindhština" alt="Sindhština" />
  <img src="../assets/flags/et.svg" width="32" title="Amharština" alt="Amharština" />
</p>

## 🚀 Klíčové vlastnosti a použití

- **Moderní nativní UI**: Intuitivní grafické rozhraní s podporou tmavého/světlého režimu, plynulými animacemi a vysoce výkonným vykreslováním pomocí engine **Skia**.
- **Integrace do systémové lišty (Tray)**: Plná podpora minimalizace do lišty (využití RAM ~10 MB), obnovení poklepáním a funkční kontextová nabídka pravým tlačítkem.
- **Inteligentní spouštění**: Nakonfigurujte panel tak, aby se spouštěl se systémem Windows, minimalizoval se do lišty (tichý režim s parametrem `/silent`) a automaticky ukončoval distribuce při ukončení.
- **Plánování úloh**: Flexibilní správa plánování úloh, podporující automatické spouštění skriptů a příkazů v instancích WSL v určených časech nebo intervalech pro automatizované operace.
- **Komplexní správa instancí**: Spuštění, zastavení, ukončení a zrušení registrace jedním kliknutím. Sledování stavu v reálném čase a podrobné informace o zaplnění disku a umístění souborů.
- **Správa distribucí**: Nastavení jako výchozí, migrace (přesun VHDX na jiné disky) a export/klonování do archivů `.tar` nebo `.tar.gz`.
- **Připojení disku**: Přímé připojení a správa VHDX virtuálních disků instancí WSL, podpora připojení, odpojení a operací správy disků, usnadňující přenos souborů mezi hostitelským systémem a WSL.
- **Rychlá integrace**: Okamžité spouštění Terminálu, VS Code nebo Průzkumníka souborů s přizpůsobitelnými pracovními adresáři a háčky pro spouštěcí skripty.
- **Instalace distribuce**: Instalujte Linux distribuce přes Microsoft Store, GitHub, lokální soubory (RootFS/VHDX) nebo online zrcadla (s automatickým testem rychlosti pro výběr nejrychlejšího zrcadla a vestavěným pomocníkem pro stahování RootFS).
- **Bezpečnost**: Zámky mutex pro bezpečné souběžné migrační a zálohovací operace a automatické čištění Appx při odebírání.
- **Extrémně nízké nároky na paměť**: Vysoce optimalizováno pro efektivitu. Tichý start (v liště) využívá pouze **~10 MB** RAM. Využití v režimu okna se liší podle složitosti písma: **~18 MB** pro standardní jazyky a **~38 MB** pro jazyky s rozsáhlými znakovými sadami (čínština, japonština, korejština).
- **Pokročilé sítě**: Bezproblémová správa přesměrování portů (s automatickým vytvářením pravidel brány firewall) a globální konfigurace HTTP proxy pro sjednocené připojení.
- **Správa zařízení USB**: Plná integrace s `usbipd-win` pro snadné vázání, připojování a správu místních zařízení USB napříč instancemi WSL přímo z uživatelského rozhraní řídicího panelu.
- **Konfigurace a protokoly**: V Nastavení spravujte výchozí instalační složku, složku a úroveň protokolů, jazyk rozhraní, tmavý režim, frekvenci kontroly aktualizací, automatické spuštění a chování při zavření i postranní panel.
- **Udržování distribuce v chodu**: Jedním kliknutím na „Spustit" necháte distribuci běžet místo vypnutí po ~60 s nečinnosti; lze zapnout i „Spouštět tuto distribuci při startu Windows".
- **Globální klávesové zkratky**: Nastavte globální zkratky pro zobrazení/skrytí hlavního okna nebo otevření terminálu zvolené distribuce.


## 🖼️ Snímky obrazovky
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

Chcete vidět panel v akci? Navštivte [oficiální web](https://www.wslui.com/) a prohlédněte si kompletní galerii snímků obrazovky.

## 📦 Instalační příručka

### Možnost 1: Navštivte web projektu (Doporučeno)

Doporučujeme navštívit oficiální webové stránky pro stažení, protože nabízejí několik zrcadlových odkazů pro plynulejší zážitek:

Přejděte na [stránku pro stažení](https://www.wslui.com/download/) a vyberte zrcadlo vhodné pro vaši oblast.

### Možnost 2: Instalace přes winget

Můžete nainstalovat WSLDashboard přímo z Windows Package Manager (winget) pomocí monikeru nebo úplného identifikátoru balíčku:

```powershell
# Vyhledávání (nezáleží na velikosti písmen)
winget search wsl-dashboard
# nebo
winget search WSLDashboard

# Instalace (vyberte jednu)
winget install wsl-dashboard
# nebo
winget install Owu.WSLDashboard
```

> Identifikátor balíčku winget je `Owu.WSLDashboard` a moniker je `wsl-dashboard` (nezáleží na velikosti písmen). Oba fungují.

Další informace naleznete v [komunitním repozitáři WinGet](https://github.com/microsoft/winget-pkgs/tree/master/manifests/o/Owu/WSLDashboard).

### Možnost 3: Sestavení ze zdrojového kódu

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
     .\build\portable\build.ps1 -Arch x64
     ```


## 📢 Oficiální komunitní skupiny

Vítejte v oficiálních komunitních skupinách WSL UI! Připojte se k našim kanálům k diskusi o tipy pro použití, hlášení problémů, získávání nejnovějších aktualizací a propojení s vývojáři a dalšími uživateli.

<div align="center">

| Globální skupina (Telegram) | Čínská skupina (WeChat) |
| :---: | :---: |
| <img src="https://cdn1.wslui.com/static/images/group/group-telegram.png" width="240" alt="Telegram Group QR Code" /> | <img src="https://cdn1.wslui.com/static/images/group/group-wechat.png" width="240" alt="WeChat Group QR Code" /> |
| Naskenujte pro připojení | Naskenujte pro připojení |

</div>


## ❤️ Podpořit tento projekt

- Tento projekt je licencován pod GPL-3.0 a je zdarma pro všechny uživatele.
- Od vývoje funkcí a denního testování po opravy chyb — veškerá práce je vykonávána ve volném čase. Cesta open source není snadná, když jdete sami. Vaše uznání a podpora dávají projektu sebevědomí pokračovat.
- Pokud vám tento nástroj skutečně pomohl, zvažte přispění. Všechny dary jdou na náklady na server, aktualizace verzí a vylepšení funkcí, což udržuje projekt neustále aktualizovaný a stabilně postupující.
- Každý malý skutek laskavosti je paprsek hvězdného světla. Děkujeme ještě jednou za vaše pochopení a štědrost！ [Navštivte naši stránku pro darování](https://www.wslui.com/donate/).


## ⭐️ Dílo z lásky

Pokud vám tento projekt připadá užitečný, byl bych vděčný, kdybyste mu mohli nechat hvězdičku na GitHubu. Vaše podpora mu pomáhá oslovit širší publikum a hluboce si jí vážím. Právě toto povzbuzení mě motivuje k dalšímu budování.


## 📄 Licence

Tento projekt je licencován pod GPL-3.0 – podrobnosti naleznete v souboru [LICENSE](../LICENSE).
