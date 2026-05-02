# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

Ein modernes, leistungsstarkes und leichtgewichtiges Dashboard zur Verwaltung von WSL-Instanzen (Windows Subsystem for Linux). Entwickelt mit Rust und Slint für ein erstklassiges natives Erlebnis.

---

```diff
Hinweis:

- Diese Software wird nicht über den Microsoft Store vertrieben.
- Jede dort unter demselben Namen aufgeführte Anwendung ist nicht autorisiert und kann gefälscht sein.
- Bitte laden Sie es nicht herunter, um Betrug zu vermeiden.
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

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | Deutsch | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md)

---

## 📑 Inhaltsverzeichnis
- [🌍 Sprachunterstützung](#-sprachunterstützung)
- [🚀 Hauptmerkmale & Nutzung](#-hauptmerkmale--nutzung)
- [⚙️ Konfiguration & Protokolle](#️-konfiguration--protokolle)
- [🖼️ Screenshots](#️-screenshots)
- [🎬 Bedienungs-Demo](#-bedienungs-demo)
- [💻 Systemvoraussetzungen](#-systemvoraussetzungen)
- [📦 Installationsanleitung](#-installationsanleitung)
- [🛠️ Tech Stack & Leistung](#️-tech-stack--leistung)
- [⭐️ Herzensprojekt](#️-herzensprojekt)
- [📄 Lizenz](#-lizenz)

---

## 🌍 Sprachunterstützung

Englisch, Chinesisch (Vereinfacht), Chinesisch (Traditionell), Hindi, Spanisch, Französisch, Arabisch, Bengalisch, Portugiesisch, Russisch, Urdu, Indonesisch, Deutsch, Japanisch, Türkisch, Koreanisch, Italienisch, Niederländisch, Schwedisch, Tschechisch, Griechisch, Ungarisch, Hebräisch, Norwegisch, Dänisch, Finnisch, Slowakisch, Slowenisch, Isländisch

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="Englisch" alt="Englisch" />
  <img src="../assets/flags/cn.svg" width="32" title="Chinesisch (Vereinfacht)" alt="Chinesisch (Vereinfacht)" />
  <img src="../assets/flags/tw.svg" width="32" title="Chinesisch (Traditionell)" alt="Chinesisch (Traditionell)" />
  <img src="../assets/flags/in.svg" width="32" title="Hindi" alt="Hindi" />
  <img src="../assets/flags/es.svg" width="32" title="Spanisch" alt="Spanisch" />
  <img src="../assets/flags/fr.svg" width="32" title="Französisch" alt="Französisch" />
  <img src="../assets/flags/sa.svg" width="32" title="Arabisch" alt="Arabisch" />
  <img src="../assets/flags/bd.svg" width="32" title="Bengalisch" alt="Bengalisch" />
  <img src="../assets/flags/pt.svg" width="32" title="Portugiesisch" alt="Portugiesisch" />
  <img src="../assets/flags/ru.svg" width="32" title="Russisch" alt="Russisch" />
  <img src="../assets/flags/pk.svg" width="32" title="Urdu" alt="Urdu" />
  <img src="../assets/flags/id.svg" width="32" title="Indonesisch" alt="Indonesisch" />
  <img src="../assets/flags/de.svg" width="32" title="Deutsch" alt="Deutsch" />
  <img src="../assets/flags/jp.svg" width="32" title="Japanisch" alt="Japanisch" />
  <img src="../assets/flags/tr.svg" width="32" title="Türkisch" alt="Türkisch" />
  <img src="../assets/flags/kr.svg" width="32" title="Koreanisch" alt="Koreanisch" />
  <img src="../assets/flags/it.svg" width="32" title="Italienisch" alt="Italienisch" />
  <img src="../assets/flags/nl.svg" width="32" title="Niederländisch" alt="Niederländisch" />
  <img src="../assets/flags/se.svg" width="32" title="Schwedisch" alt="Schwedisch" />
  <img src="../assets/flags/cz.svg" width="32" title="Tschechisch" alt="Tschechisch" />
  <img src="../assets/flags/gr.svg" width="32" title="Griechisch" alt="Griechisch" />
  <img src="../assets/flags/hu.svg" width="32" title="Ungarisch" alt="Ungarisch" />
  <img src="../assets/flags/il.svg" width="32" title="Hebräisch" alt="Hebräisch" />
  <img src="../assets/flags/no.svg" width="32" title="Norwegisch" alt="Norwegisch" />
  <img src="../assets/flags/dk.svg" width="32" title="Dänisch" alt="Dänisch" />
  <img src="../assets/flags/fi.svg" width="32" title="Finnisch" alt="Finnisch" />
  <img src="../assets/flags/sk.svg" width="32" title="Slowakisch" alt="Slowakisch" />
  <img src="../assets/flags/si.svg" width="32" title="Slowenisch" alt="Slowenisch" />
  <img src="../assets/flags/is.svg" width="32" title="Isländisch" alt="Isländisch" />
</p>


## 🚀 Hauptmerkmale & Nutzung

- **Moderne native UI**: Intuitive Benutzeroberfläche mit Unterstützung für Hell-/Dunkelmodus, flüssigen Animationen und leistungsstarkem Rendering durch **Skia**.
- **System-Tray-Integration**: Volle Unterstützung für die Minimierung in den Infobereich (~10MB RAM-Verbrauch), Doppelklick zum Umschalten und ein funktionales Kontextmenü.
- **Intelligenter Autostart**: Dashboard mit Windows starten, minimiert im Tray (Silent-Modus mit `/silent`) und automatisches Herunterfahren der Distributionen beim Beenden.
- **Umfassende Instanzverwaltung**: Ein-Klick-Start, Stopp, Beenden und Deregistrieren. Echtzeit-Statusüberwachung und detaillierte Einblicke in Speichernutzung und Dateipfade.
- **Distributions-Management**: Als Standard festlegen, Migration (VHDX auf andere Laufwerke verschieben) sowie Export/Klonen in `.tar` oder `.tar.gz`.
- **Schnelle Integration**: Sofortiger Start von Terminal, VS Code oder Datei-Explorer mit anpassbaren Arbeitsverzeichnissen und Startup-Skript-Hooks.
- **Smarte Installation**: Installation aus dem Microsoft Store, von GitHub oder lokalen Dateien (RootFS/VHDX). Integrierter RootFS-Download-Assistent.
- **Globale Sicherheit**: Mutex-Sperren für sichere gleichzeitige Migrations-/Backup-Vorgänge und automatische Appx-Bereinigung beim Entfernen.
- **Ultra-niedriger Speicherverbrauch**: Hochgradig auf Effizienz optimiert. Silent-Start (System Tray) verbraucht nur **~10MB** RAM. Der Verbrauch im Fenstermodus variiert nach Schriftkomplexität: **~18MB** für Standardsprachen (Englisch, Deutsch, Spanisch usw.) und **~38MB** für Sprachen mit großen Zeichensätzen (Chinesisch, Japanisch, Koreanisch).
- **Erweiterte Netzwerke**: Nahtlose Verwaltung der Portweiterleitung (mit automatischer Erstellung von Firewall-Regeln) und globale HTTP-Proxy-Konfiguration für einheitliche Konnektivität.
- **USB-Geräteverwaltung**: Vollständige Integration mit `usbipd-win` zur mühelosen Bindung, Ausführung und Verwaltung lokaler USB-Geräte über Ihre WSL-Instanzen hinweg direkt über die Dashboard-Benutzeroberfläche.


## ⚙️ Konfiguration & Protokolle

Die gesamte Konfiguration wird über die Einstellungsansicht verwaltet:

- Wählen Sie das Standard-Installationsverzeichnis für neue WSL-Instanzen.
- Konfigurieren Sie das Protokollverzeichnis und die Protokollstufe (Error / Warn / Info / Debug / Trace).
- Wählen Sie die UI-Sprache oder lassen Sie sie der Systemsprache folgen.
- Schalten Sie den Dunkelmodus um und legen Sie fest, ob die App WSL nach Vorgängen automatisch herunterfahren kann.
- Konfigurieren Sie, wie oft die App nach Updates sucht (täglich, wöchentlich, zweiwöchentlich, monatlich).
- Aktivieren Sie den automatischen Start beim Systemboot (mit automatischer Pfadreparatur).
- Stellen Sie die App so ein, dass sie beim Start in den System-Tray minimiert wird.
- Konfigurieren Sie die Schließen-Schaltfläche so, dass sie in den Tray minimiert wird, anstatt das Programm zu beenden.
- Passen Sie die Seitenleiste an, indem Sie die Sichtbarkeit bestimmter Funktionsregisterkarten umschalten.

Protokolldateien werden in das konfigurierte Protokollverzeichnis geschrieben und können bei der Meldung von Problemen angehängt werden.


## 🖼️ Screenshots

### Home (Hell- & Dunkelmodus)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB & Menü einklappen
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### Netzwerk
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### Instanz hinzufügen & Einstellungen
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>

### Über 
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
</p>

## 🎬 Bedienungs-Demo

Hier ist eine Demonstration des WSL Dashboards in Aktion:

![WSL Dashboard Demo](../assets/screenshot/demo.gif)



## 💻 Systemvoraussetzungen

- Windows 10 oder Windows 11 mit aktiviertem WSL (WSL 2 empfohlen).
- Mindestens eine installierte WSL-Distribution oder die Berechtigung, neue zu installieren.
- 64-Bit-CPU; 4 GB RAM oder mehr empfohlen für reibungslose Nutzung mehrerer Distributionen.

## 📦 Installationsanleitung

### Option 1: Vorkompilierte Binärdatei herunterladen

Der einfachste Weg, um zu starten, ist die Verwendung des vorkompilierten Releases:

1. Gehen Sie zur Seite [GitHub Releases](https://github.com/owu/wsl-dashboard/releases).
2. Laden Sie die neueste `wsldashboard`-Ausführungsdatei für Windows herunter.
3. Entpacken Sie diese (falls gepackt) und führen Sie `wsldashboard.exe` aus.

Es ist kein Installer erforderlich; die App ist eine einzelne portable Binärdatei.

### Option 2: Aus dem Quellcode erstellen

Stellen Sie sicher, dass Sie die Rust-Toolchain (Rust 1.92+ oder neuer) installiert haben.

1. Klonen Sie das Repository:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. Erstellen und ausführen:

   - Für die Entwicklung:

     ```powershell
     cargo run
     ```
   - Optimierten Release-Build mit dem Build-Skript erstellen:

     > Das Build-Skript erfordert die `x86_64-pc-windows-msvc` Toolchain.

     ```powershell
     .\build\scripts\build.ps1
     ```


## 🛠️ Tech Stack & Leistung

- **Kern**: Implementiert in Rust für Speichersicherheit und Zero-Cost-Abstraktionen.
- **UI-Framework**: Slint mit leistungsstarkem **Skia**-Rendering-Backend.
- **Async-Runtime**: Tokio für nicht blockierende Systembefehle und I/O.
- **Leistungs-Highlights**:
  - **Reaktionsfähigkeit**: Nahezu sofortiger Start und Echtzeit-WSL-Statusüberwachung.
  - **Effizienz**: Ultra-niedriger Ressourcenverbrauch (Details siehe [Hauptmerkmale](#-hauptmerkmale--nutzung)).
  - **Portabilität**: Optimierter Release-Build erzeugt eine einzige kompakte ausführbare Datei.



## ⭐️ Herzensprojekt

Wenn Sie dieses Projekt nützlich fanden, wäre ich Ihnen dankbar, wenn Sie einen Stern auf GitHub hinterlassen könnten. Ihre Unterstützung hilft dabei, ein breiteres Publikum zu erreichen, und wird sehr geschätzt. Es ist diese Ermutigung, die mich motiviert, weiter zu bauen.

## 📄 Lizenz

Dieses Projekt ist unter der GPL-3.0 lizenziert – weitere Details finden Sie in der Datei [LICENSE](../LICENSE).

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

