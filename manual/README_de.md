# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" />
</p>

---

Ein modernes, leistungsstarkes und leichtgewichtiges Dashboard zur Verwaltung von WSL-Instanzen (Windows Subsystem for Linux). Entwickelt mit Rust und Slint für ein erstklassiges natives Erlebnis.

---

[![Rust](https://img.shields.io/badge/rust-v1.92+-orange.svg)](https://www.rust-lang.org)
[![Slint](https://img.shields.io/badge/UI-Slint-blue.svg)](https://slint.dev)
[![License](https://img.shields.io/badge/license-GPL--3.0-blue.svg)](LICENSE)

I18N: [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [English](../README.md) | [日本語](./README_ja.md) | [Français](./README_fr.md) | [Español](./README_es.md) | [Русский](./README_ru.md) | [Português](./README_pt.md) | Deutsch | [Italiano](./README_it.md) | [Türkçe](./README_tr.md) | [Bahasa Indonesia](./README_id.md) | [हिन्दी](./README_hi.md) | [বাংলা](./README_bn.md)

---

## 🖼️ Screenshots

### Home (Hell- & Dunkelmodus)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

### Instanz hinzufügen & Einstellungen
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>

### Über & Menü einklappen
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

## 🎬 Bedienungs-Demo

Hier ist eine Demonstration des WSL Dashboards in Aktion:

![WSL Dashboard Demo](../assets/screenshot/demo.gif)

## 🚀 Hauptmerkmale

- Intuitive Benutzeroberfläche mit Unterstützung für den Dunkelmodus und flüssigen Animationen.
- Ein-Klick-Verwaltung für alle Ihre WSL-Distributionen (Starten, Stoppen, Beenden, Deregistrieren).
- Schneller Zugriff auf Distributions-Terminals, VS Code und den Datei-Explorer.
- Echtzeit-Statusüberwachung und -anzeige der WSL-Instanzen.
- Export und Backup in `.tar`- oder komprimierte `.tar.gz`-Archive.
- Import und Klonen von Instanzen aus Backups oder bestehenden Distributionen.
- Verschieben großer WSL-Instanzen (VHDX-Migration) auf andere Festplatten, um Platz auf Laufwerk C: zu sparen.
- Intelligente Installation von Distributionen aus dem Microsoft Store oder von GitHub.
- Integrierter RootFS-Download-Assistent für manuelle Installationen.
- Detaillierte Einblicke in den VHDX-Dateispeicherort, die Größe des virtuellen Datenträgers und die tatsächliche Festplattenbelegung.

## Systemvoraussetzungen

- Windows 10 oder Windows 11 mit aktiviertem WSL (WSL 2 empfohlen).
- Mindestens eine installierte WSL-Distribution oder die Berechtigung, neue zu installieren.
- 64-Bit-CPU; 4 GB RAM oder mehr empfohlen für reibungslose Nutzung mehrerer Distributionen.

## 📦 Installation

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

   - Optimierter Release-Build:

     ```powershell
     cargo run --release
     ```

   - Verwendung des Build-Skripts (empfohlen für Release-Binärdateien):

     > Das Build-Skript erfordert die `x86_64-pc-windows-gnu` Toolchain.

     ```powershell
     .\build\scripts\build.ps1
     ```

## 🧭 Nutzungsübersicht

- **Vorhandene Distributionen verwalten**: Starten, Stoppen, Beenden oder Deregistrieren einer WSL-Distribution aus der Hauptansicht.
- **Werkzeuge schnell öffnen**: Starten Sie eine Distribution in Ihrem Terminal, in VS Code oder im Datei-Explorer mit einem einzigen Klick.
- **Neue Instanzen erstellen**: Verwenden Sie die Ansicht „Instanz hinzufügen“, um aus dem Microsoft Store zu installieren, RootFS-Images herunterzuladen oder vorhandene Distributionen zu klonen.
- **Backup und Wiederherstellung**: Exportieren Sie Distributionen in `.tar` / `.tar.gz`-Archive und importieren Sie diese später oder auf einem anderen Rechner.
- **Speicherort verschieben**: Verschieben Sie große VHDX-Dateien auf ein anderes Laufwerk, um Platz auf Laufwerk C: zu sparen.
- **Status überwachen**: Behalten Sie den Echtzeit-Status der Distributionen und die Speichernutzung im Auge, während das WSL Dashboard läuft.

## ⚙️ Konfiguration & Protokolle

Die gesamte Konfiguration wird über die Einstellungsansicht verwaltet:

- Wählen Sie das Standard-Installationsverzeichnis für neue WSL-Instanzen.
- Konfigurieren Sie das Protokollverzeichnis und die Protokollstufe (Error / Warn / Info / Debug / Trace).
- Wählen Sie die UI-Sprache oder lassen Sie sie der Systemsprache folgen.
- Schalten Sie den Dunkelmodus um und legen Sie fest, ob die App WSL nach Vorgängen automatisch herunterfahren kann.
- Konfigurieren Sie, wie oft die App nach Updates sucht (täglich, wöchentlich, zweiwöchentlich, monatlich).

Protokolldateien werden in das konfigurierte Protokollverzeichnis geschrieben und können bei der Meldung von Problemen angehängt werden.

## 🛠️ Tech Stack & Leistung

- **Kern**: Implementiert in Rust für Speichersicherheit und Zero-Cost-Abstraktionen.
- **UI-Framework**: Slint, ein modernes GPU-beschleunigtes UI-Toolkit (Backend: `winit`).
- **Async-Runtime**: Tokio für hochparallele, nicht blockierende Systembefehle und I/O.
- **Leistung**:
  - **Speichernutzung**: In der Regel etwa 60–80 MB RAM.
  - **Reaktionsfähigkeit**: Nahezu sofortiger Start und Status-Updates in Echtzeit durch Streaming-Technologie.
  - **Binärgröße**: Optimierter Release-Build erzeugt eine einzige kompakte ausführbare Datei.

## 🌍 Sprachunterstützung

Vollständige Internationalisierung wird für die folgenden Sprachen angeboten:

| Sprache | Code | Emoji |
| :--- | :---: | :---: |
| Vereinfachtes Chinesisch | `zh-CN` | 🇨🇳 |
| Traditionelles Chinesisch | `zh-TW` | 🇭🇰 / 🇹🇼 |
| Englisch | `en` | 🇺🇸 |
| Japanisch | `ja` | 🇯🇵 |
| Französisch | `fr` | 🇫🇷 |
| Spanisch | `es` | 🇪🇸 |
| Russisch | `ru` | 🇷🇺 |
| Portugiesisch | `pt` | 🇵🇹 |
| Deutsch | `de` | 🇩🇪 |
| Italienisch | `it` | 🇮🇹 |
| Türkisch | `tr` | 🇹🇷 |
| Indonesisch | `id` | 🇮🇩 |
| Hindi | `hi` | 🇮🇳 |
| Bengalisch | `bn` | 🇧🇩 |

## 📄 Lizenz

Dieses Projekt ist unter der GPL-3.0 lizenziert – weitere Details finden Sie in der Datei [LICENSE](LICENSE).

---

Built with ❤️ for the WSL Community.
