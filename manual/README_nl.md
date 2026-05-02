# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

Een modern, krachtig en lichtgewicht dashboard voor het beheer van WSL-instances (Windows Subsystem for Linux). Gebouwd met Rust en Slint voor een hoogwaardige native ervaring.

---

```diff
Kennisgeving:

- Deze software wordt niet gedistribueerd via de Microsoft Store.
- Elke applicatie die daar vermeld staat onder dezelfde naam is niet geautoriseerd en kan namaak zijn.
- Gelieve het niet te downloaden om oplichting te voorkomen.
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

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | Nederlands | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md)

---

## 📑 Inhoudsopgave
- [🌍 Ondersteunde talen](#-ondersteunde-talen)
- [🚀 Belangrijkste kenmerken & Gebruik](#-belangrijkste-kenmerken--gebruik)
- [⚙️ Configuratie & Logboeken](#️-configuratie--logboeken)
- [🖼️ Screenshots](#️-screenshots)
- [🎬 Demonstratie](#-demonstratie)
- [💻 Systeemvereisten](#-systeemvereisten)
- [📦 Installatiehandleiding](#-installatiehandleiding)
- [🛠️ Tech Stack & Prestaties](#️-tech-stack--prestaties)
- [⭐️ Liefdeswerk](#️-liefdeswerk)
- [📄 Licentie](#-licentie)

---

## 🌍 Ondersteunde talen

Engels, Vereenvoudigd Chinees, Traditioneel Chinees, Hindi, Spaans, Frans, Arabisch, Bengaals, Portugees, Russisch, Urdu, Indonesisch, Duits, Japans, Turks, Koreaans, Italiaans, Nederlands, Zweeds, Tsjechisch, Grieks, Hongaars, Hebreeuws, Noors, Deens, Fins, Slowaaks, Sloveens, IJslands.

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="Engels" alt="Engels" />
  <img src="../assets/flags/cn.svg" width="32" title="Chinees (Vereenvoudigd)" alt="Chinees (Vereenvoudigd)" />
  <img src="../assets/flags/tw.svg" width="32" title="Chinees (Traditioneel)" alt="Chinees (Traditioneel)" />
  <img src="../assets/flags/in.svg" width="32" title="Hindi" alt="Hindi" />
  <img src="../assets/flags/es.svg" width="32" title="Spaans" alt="Spaans" />
  <img src="../assets/flags/fr.svg" width="32" title="Frans" alt="Frans" />
  <img src="../assets/flags/sa.svg" width="32" title="Arabisch" alt="Arabisch" />
  <img src="../assets/flags/bd.svg" width="32" title="Bengalees" alt="Bengalees" />
  <img src="../assets/flags/pt.svg" width="32" title="Portugees" alt="Portugees" />
  <img src="../assets/flags/ru.svg" width="32" title="Russisch" alt="Russisch" />
  <img src="../assets/flags/pk.svg" width="32" title="Urdu" alt="Urdu" />
  <img src="../assets/flags/id.svg" width="32" title="Indonesisch" alt="Indonesisch" />
  <img src="../assets/flags/de.svg" width="32" title="Duits" alt="Duits" />
  <img src="../assets/flags/jp.svg" width="32" title="Japans" alt="Japans" />
  <img src="../assets/flags/tr.svg" width="32" title="Turks" alt="Turks" />
  <img src="../assets/flags/kr.svg" width="32" title="Koreaans" alt="Koreaans" />
  <img src="../assets/flags/it.svg" width="32" title="Italiaans" alt="Italiaans" />
  <img src="../assets/flags/nl.svg" width="32" title="Nederlands" alt="Nederlands" />
  <img src="../assets/flags/se.svg" width="32" title="Zweeds" alt="Zweeds" />
  <img src="../assets/flags/cz.svg" width="32" title="Tsjechisch" alt="Tsjechisch" />
  <img src="../assets/flags/gr.svg" width="32" title="Grieks" alt="Grieks" />
  <img src="../assets/flags/hu.svg" width="32" title="Hongaars" alt="Hongaars" />
  <img src="../assets/flags/il.svg" width="32" title="Hebreeuws" alt="Hebreeuws" />
  <img src="../assets/flags/no.svg" width="32" title="Noors" alt="Noors" />
  <img src="../assets/flags/dk.svg" width="32" title="Deens" alt="Deens" />
  <img src="../assets/flags/fi.svg" width="32" title="Fins" alt="Fins" />
  <img src="../assets/flags/sk.svg" width="32" title="Slowaaks" alt="Slowaaks" />
  <img src="../assets/flags/si.svg" width="32" title="Sloveens" alt="Sloveens" />
  <img src="../assets/flags/is.svg" width="32" title="IJslands" alt="IJslands" />
</p>


## 🚀 Belangrijkste kenmerken & Gebruik

- **Moderne Native UI**: Intuïtieve GUI met ondersteuning voor Donkere/Lichte modus, vloeiende animaties en hoogwaardige rendering aangedreven door **Skia**.
- **Systeemvak-integratie (Tray)**: Volledige ondersteuning voor minimaliseren naar het systeemvak (geheugengebruik ~10 MB), dubbelklikken om te schakelen en een functioneel rechtsklikmenu.
- **Intelligente Startup**: Configureer het dashboard om met Windows te starten, te minimaliseren naar het systeemvak (stille modus met `/silent`), en distributies automatisch af te sluiten bij het afsluiten.
- **Uitgebreid beheer**: Start, Stop, Beëindig en Verwijder registratie met één klik. Realtime statusbewaking en gedetailleerd inzicht in schijfgebruik en bestandslocaties.
- **Distro beheer**: Instellen als standaard, migratie (VHDX verplaatsen naar andere schijven), en exporteren/klonen naar `.tar` of `.tar.gz` archieven.
- **Snelle integratie**: Direct starten in Terminal, VS Code of Verkenner met aanpasbare werkmappen en startup script-hooks.
- **Slimme installatie**: Installeren vanuit Microsoft Store, GitHub of lokale bestanden (RootFS/VHDX). Inclusief een ingebouwde RootFS download-helper.
- **Veiligheid**: Mutex-locks voor veilige gelijktijdige migratie-/backup-operaties en automatische opschoning van Appx bij verwijdering.
- **Ultra-laag geheugengebruik**: Sterk geoptimaliseerd voor efficiëntie. Stille startup (systeemvak) gebruikt slechts **~10 MB** RAM. Gebruik in venstermodus varieert per fontcomplexiteit: **~18 MB** voor standaardtalen en **~38 MB** voor talen met grote tekensets (Chinees, Japans, Koreaans).
- **Geavanceerde netwerken**: Naadloos beheer van port forwarding (met automatische aanmaak van firewallregels) en wereldwijde HTTP-proxyconfiguratie voor geünificeerde connectiviteit.
- **USB-apparaatbeheer**: Volledige integratie met `usbipd-win` voor het moeiteloos binden, aankoppelen en beheren van lokale USB-apparaten in al uw WSL-instanties direct vanaf het dashboard.


## ⚙️ Configuratie & Logboeken

Alle configuratie wordt beheerd via de Instellingen-weergave:

- Kies de standaard installatiemap voor nieuwe WSL-instances.
- Configureer de logmap en het logniveau (Error / Warn / Info / Debug / Trace).
- Kies de UI-taal of laat deze de systeemtaal volgen.
- Schakel donkere modus in/uit en stel in of de app WSL automatisch mag afsluiten na operaties.
- Configureer hoe vaak de app op updates controleert (dagelijks, wekelijks, tweewekelijks, maandelijks).
- Schakel automatisch starten bij systeemopstart in (met automatisch padherstel).
- Stel de app in om bij opstarten te minimaliseren naar het systeemvak.
- Stel de sluitknop in om te minimaliseren naar het systeemvak in plaats van af te sluiten.
- Pas de zijbalk aan door de zichtbaarheid van specifieke functietabbladen in of uit te schakelen.

Logbestanden worden naar de geconfigureerde logmap geschreven en kunnen worden bijgevoegd bij het melden van problemen.


## 🖼️ Screenshots

### Home (Lichte & Donkere modus)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB & menu inklappen
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### netwerk
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### Instance toevoegen & Instellingen
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>

### Over
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
</p>

## 🎬 Demonstratie

Hieronder ziet u een demonstratie van het WSL Dashboard in actie:

![WSL Dashboard Demo](../assets/screenshot/demo.gif)



## 💻 Systeemvereisten

- Windows 10 of Windows 11 met WSL ingeschakeld (WSL 2 aanbevolen).
- Minimaal één WSL-distributie geïnstalleerd, of toestemming om nieuwe te installeren.
- 64-bit CPU; 4 GB RAM of meer aanbevolen voor soepel gebruik van meerdere distro's.

## 📦 Installatiehandleiding

### Optie 1: Download de voorgecompileerde binary

De eenvoudigste manier om aan de slag te gaan is door de voorgecompileerde release te gebruiken:

1. Ga naar de [GitHub Releases](https://github.com/owu/wsl-dashboard/releases) pagina.
2. Download het nieuwste `wsldashboard` uitvoerbare bestand voor Windows.
3. Pak het uit (indien verpakt) en voer `wsldashboard.exe` uit.

Er is geen installatieprogramma vereist; de app is een enkele draagbare binary.

### Optie 2: Bouwen vanuit de broncode

Zorg ervoor dat de Rust-toolchain (Rust 1.92 of nieuwer) is geïnstalleerd.

1. Kloon de repository:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. Bouwen en uitvoeren:

   - Voor ontwikkeling:

     ```powershell
     cargo run
     ```
   - Geoptimaliseerde release-build met het build-script:

     > Het build-script vereist de `x86_64-pc-windows-msvc` toolchain.

     ```powershell
     .\build\scripts\build.ps1
     ```


## 🛠️ Tech Stack & Prestaties

- **Kern**: Geïmplementeerd in Rust voor geheugenveiligheid en zero-cost abstracties.
- **UI Framework**: Slint met hoogwaardige **Skia** rendering backend.
- **Async Runtime**: Tokio voor niet-blokkerende systeemcommando's en I/O.
- **Prestatiehoogtepunten**:
  - **Responsiviteit**: Bijna onmiddellijke opstart en realtime WSL-statusbewaking.
  - **Efficiëntie**: Zeer laag bronnengebruik (zie [Belangrijkste kenmerken](#-belangrijkste-kenmerken--gebruik) voor details).
  - **Portabiliteit**: Geoptimaliseerde release-build produceert een enkele compacte executable.



## ⭐️ Liefdeswerk

Als u dit project nuttig heeft gevonden, zou ik het op prijs stellen als u een ster achterlaat op GitHub. Uw steun helpt het een breder publiek te bereiken en wordt zeer gewaardeerd. Het is deze aanmoediging die mij motiveert om door te gaan met bouwen.

## 📄 Licentie

Dit project is gelicenseerd onder de GPL-3.0 – zie het [LICENSE](../LICENSE) bestand voor details.

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

