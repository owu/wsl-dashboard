# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" />
</p>

Una dashboard moderna, performante e leggera per la gestione delle istanze WSL (Windows Subsystem for Linux). Realizzata con Rust e Slint per un'esperienza nativa di alto livello.

---

[![Rust](https://img.shields.io/badge/rust-v1.92+-orange.svg)](https://www.rust-lang.org)
[![Slint](https://img.shields.io/badge/UI-Slint-blue.svg)](https://slint.dev)
[![License](https://img.shields.io/badge/license-GPL--3.0-blue.svg)](LICENSE)

I18N: [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [English](../README.md) | [日本語](./README_ja.md) | [Français](./README_fr.md) | [Español](./README_es.md) | [Русский](./README_ru.md) | [Português](./README_pt.md) | [Deutsch](./README_de.md) | Italiano | [Türkçe](./README_tr.md) | [Bahasa Indonesia](./README_id.md) | [हिन्दी](./README_hi.md) | [বাংলা](./README_bn.md)

---

## 🖼️ Screenshot

### Home (Modalità Chiara e Scura)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

### Aggiungi Istanza e Impostazioni
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>

### Informazioni e menu compresso
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

## 🎬 Demo di Funzionamento

Di seguito è riportata una dimostrazione di WSL Dashboard in azione:

![Demo WSL Dashboard](../assets/screenshot/demo.gif)

## 🚀 Funzionalità Principali

- GUI intuitiva con supporto alla modalità scura e animazioni fluide.
- Gestione in un clic di tutte le distribuzioni WSL (Avvio, Stop, Terminazione, Disinstallazione).
- Accesso rapido ai terminali delle distribuzioni, a VS Code e a Esplora File.
- Monitoraggio e visualizzazione dello stato delle istanze WSL in tempo reale.
- Esportazione e backup in archivi `.tar` o `.tar.gz` compressi.
- Importazione e clonazione di istanze da backup o distribuzioni esistenti.
- Ricollocazione di istanze WSL di grandi dimensioni (migrazione VHDX) su altri dischi per risparmiare spazio nell'unità C:.
- Installazione intelligente di distribuzioni dal Microsoft Store o da GitHub.
- Assistente al download di RootFS integrato per installazioni manuali.
- Informazioni dettagliate sulla posizione del file VHDX, sulle dimensioni del disco virtuale e sull'utilizzo effettivo del disco.

## Requisiti di Sistema

- Windows 10 o Windows 11 con WSL abilitato (consigliato WSL 2).
- Almeno una distribuzione WSL installata, o il permesso di installarne di nuove.
- CPU a 64 bit; consigliati 4 GB di RAM o più per un utilizzo fluido di più distribuzioni.

## 📦 Installazione

### Opzione 1: Scarica il binario precompilato

Il modo più semplice per iniziare è utilizzare la versione precompilata:

1. Vai alla pagina [GitHub Releases](https://github.com/owu/wsl-dashboard/releases).
2. Scarica l'ultimo eseguibile `wsldashboard` per Windows.
3. Estrai (se necessario) ed esegui `wsldashboard.exe`.

Non è richiesto alcun programma di installazione; l'app è un unico binario portatile.

### Opzione 2: Compila dai sorgenti

Assicurati di avere installato la toolchain Rust (Rust 1.92+ o successivo).

1. Clona il repository:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. Compila ed esegui:

   - Per lo sviluppo:

     ```powershell
     cargo run
     ```

   - Build di rilascio ottimizzata:

     ```powershell
     cargo run --release
     ```

   - Utilizzo dello script di build (consigliato per produrre binari di rilascio):

     > Lo script di build richiede la toolchain `x86_64-pc-windows-gnu`.

     ```powershell
     .\build\scripts\build.ps1
     ```

## 🧭 Panoramica d'Uso

- **Gestione distribuzioni esistenti**: avvia, ferma, termina o disinstalla qualsiasi distribuzione WSL dalla vista principale.
- **Apertura rapida strumenti**: lancia una distribuzione nel terminale, in VS Code o in Esplora File con un singolo clic.
- **Creazione nuove istanze**: usa la vista Aggiungi Istanza per installare dal Microsoft Store, scaricare immagini RootFS o clonare distribuzioni esistenti.
- **Backup e ripristino**: esporta le distribuzioni in archivi `.tar` / `.tar.gz` e importale in seguito o su un altro computer.
- **Spostamento storage**: ricolloca file VHDX di grandi dimensioni in un'unità diversa per risparmiare spazio sul disco C:.
- **Monitoraggio stato**: tieni d'occhio lo stato delle distribuzioni e l'utilizzo dello storage in tempo reale mentre WSL Dashboard è in esecuzione.

## ⚙️ Configurazione e Log

Tutta la configurazione è gestita tramite la vista Impostazioni:

- Scegli la directory di installazione predefinita per le nuove istanze WSL.
- Configura la directory dei log e il livello di log (Error / Warn / Info / Debug / Trace).
- Scegli la lingua dell'interfaccia o lascia che segua la lingua di sistema.
- Attiva o disattiva la modalità scura e consenti all'app di arrestare automaticamente WSL dopo le operazioni.
- Configura la frequenza con cui l'app controlla gli aggiornamenti (giornaliera, settimanale, bisettimanale, mensile).

I file di log vengono scritti nella directory configurata e possono essere allegati quando si segnalano problemi.

## 🛠️ Stack Tecnologico e Performance

- **Core**: implementato in Rust per sicurezza della memoria e astrazioni a costo zero.
- **UI framework**: Slint, un toolkit UI moderno accelerato via GPU (backend: `winit`).
- **Async runtime**: Tokio per comandi di sistema e I/O altamente concorrenti e non bloccanti.
- **Performance**:
  - **Uso memoria**: solitamente intorno a 60–80 MB di RAM.
  - **Reattività**: avvio quasi istantaneo e aggiornamenti dello stato WSL in tempo real tramite streaming.
  - **Dimensione binario**: la build di rilascio ottimizzata produce un unico eseguibile compatto.

## 🌍 Lingue Supportate

Il supporto completo all'internazionalizzazione è fornito per le seguenti lingue:

| Lingua | Codice | Emoji |
| :--- | :---: | :---: |
| Cinese Semplificato | `zh-CN` | 🇨🇳 |
| Cinese Tradizionale | `zh-TW` | 🇭🇰 / 🇹🇼 |
| Inglese | `en` | 🇺🇸 |
| Giapponese | `ja` | 🇯🇵 |
| Francese | `fr` | 🇫🇷 |
| Spagnolo | `es` | 🇪🇸 |
| Russo | `ru` | 🇷🇺 |
| Portoghese | `pt` | 🇵🇹 |
| Tedesco | `de` | 🇩🇪 |
| Italiano | `it` | 🇮🇹 |
| Turco | `tr` | 🇹🇷 |
| Indonesiano | `id` | 🇮🇩 |
| Hindi | `hi` | 🇮🇳 |
| Bengalese | `bn` | 🇧🇩 |

## 📄 Licenza

Questo progetto è rilasciato sotto licenza GPL-3.0 – consulta il file [LICENSE](LICENSE) per i dettagli.

---

Built with ❤️ for the WSL Community.
