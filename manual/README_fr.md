# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" />
</p>

Un tableau de bord moderne, performant et léger pour la gestion des instances WSL (Windows Subsystem for Linux). Conçu avec Rust et Slint pour une expérience native haut de gamme.

---

[![Rust](https://img.shields.io/badge/rust-v1.92+-orange.svg)](https://www.rust-lang.org)
[![Slint](https://img.shields.io/badge/UI-Slint-blue.svg)](https://slint.dev)
[![License](https://img.shields.io/badge/license-GPL--3.0-blue.svg)](LICENSE)

I18N: [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [English](../README.md) | [日本語](./README_ja.md) | Français | [Español](./README_es.md) | [Русский](./README_ru.md) | [Português](./README_pt.md) | [Deutsch](./README_de.md) | [Italiano](./README_it.md) | [Türkçe](./README_tr.md) | [Bahasa Indonesia](./README_id.md) | [हिन्दी](./README_hi.md) | [বাংলা](./README_bn.md)

---

## 🖼️ Captures d'écran

### Accueil (Mode Clair & Sombre)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

### Ajouter une Instance & Paramètres
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>

### À propos et menu réduit
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

## 🎬 Démonstration

Voici une démonstration de WSL Dashboard en action :

![Démonstration WSL Dashboard](../assets/screenshot/demo.gif)

## 🚀 Fonctionnalités Clés

- Interface graphique intuitive avec support du mode sombre et animations fluides.
- Gestion en un clic de toutes vos distributions WSL (Démarrer, Arrêter, Terminer, Désenregistrer).
- Accès rapide aux terminaux des distributions, à VS Code et à l'Explorateur de fichiers.
- Surveillance et affichage en temps réel de l'état des instances WSL.
- Exportation et sauvegarde vers des archives `.tar` ou `.tar.gz` compressées.
- Importation et clonage d'instances à partir de sauvegardes ou de distributions existantes.
- Relocalisation des instances WSL volumineuses (migration VHDX) vers d'autres disques pour libérer de l'espace sur C:.
- Installation intelligente de distributions depuis le Microsoft Store ou GitHub.
- Assistant de téléchargement RootFS intégré pour les installations manuelles.
- Informations détaillées sur l'emplacement du fichier VHDX, la taille du disque virtuel et l'utilisation réelle du disque.

## Configuration Requise

- Windows 10 ou Windows 11 avec WSL activé (WSL 2 recommandé).
- Au moins une distribution WSL installée, ou l'autorisation d'en installer de nouvelles.
- Processeur 64 bits ; 4 Go de RAM ou plus recommandés pour une utilisation fluide de plusieurs distributions.

## 📦 Installation

### Option 1 : Télécharger l'exécutable précompilé

La méthode la plus simple est d'utiliser la version déjà compilée :

1. Allez sur la page des [GitHub Releases](https://github.com/owu/wsl-dashboard/releases).
2. Téléchargez le dernier exécutable `wsldashboard` pour Windows.
3. Extrayez (si nécessaire) et lancez `wsldashboard.exe`.

Aucun installateur n'est requis ; l'application est un binaire portable unique.

### Option 2 : Compiler à partir des sources

Assurez-vous d'avoir installé la chaîne d'outils Rust (Rust 1.92+ ou plus récent).

1. Clonez le dépôt :

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. Compilez et lancez :

   - Pour le développement :

     ```powershell
     cargo run
     ```

   - Build de production optimisé :

     ```powershell
     cargo run --release
     ```

   - Utilisation du script de build (recommandé pour la production) :

     > Le script de build nécessite la chaîne d'outils `x86_64-pc-windows-gnu`.

     ```powershell
     .\build\scripts\build.ps1
     ```

## 🧭 Aperçu de l'utilisation

- **Gérer les distributions existantes** : Démarrez, arrêtez, terminez ou désenregistrez n'importe quelle distribution WSL depuis la vue principale.
- **Ouvrir les outils rapidement** : Lancez une distribution dans votre terminal, VS Code ou l'Explorateur de fichiers en un seul clic.
- **Créer de nouvelles instances** : Utilisez la vue d'ajout d'instance pour installer depuis le Microsoft Store, télécharger des images RootFS ou cloner des distributions existantes.
- **Sauvegarde et restauration** : Exportez des distributions vers des archives `.tar` / `.tar.gz` et importez-les plus tard ou sur une autre machine.
- **Déplacer le stockage** : Relocalisez les fichiers VHDX volumineux vers un autre lecteur pour économiser de l'espace sur le disque C:.
- **Surveiller l'état** : Gardez un œil sur l'état des distributions et l'utilisation du stockage en temps réel.

## ⚙️ Configuration & Logs

Toute la configuration est gérée via la vue Paramètres :

- Choisissez le répertoire d'installation par défaut pour les nouvelles instances WSL.
- Configurez le répertoire des logs et le niveau de log (Error / Warn / Info / Debug / Trace).
- Choisissez la langue de l'interface ou laissez-la suivre la langue du système.
- Basculez entre le mode clair et sombre, et déterminez si l'application peut arrêter automatiquement WSL après certaines opérations.
- Configurez la fréquence de vérification des mises à jour (quotidienne, hebdomadaire, bimensuelle, mensuelle).

Les fichiers de log sont écrits dans le répertoire configuré et peuvent être joints lors du signalement de problèmes.

## 🛠️ Stack Technique & Performance

- **Cœur** : Implémenté en Rust pour la sécurité mémoire et des abstractions à coût nul.
- **Framework UI** : Slint, un toolkit UI moderne accéléré par GPU (backend : `winit`).
- **Runtime Async** : Tokio pour des commandes système et des E/S hautement concurrentes et non bloquantes.
- **Performance** :
  - **Usage mémoire** : Généralement autour de 60–80 Mo de RAM.
  - **Réactivité** : Démarrage quasi instantané et mises à jour de l'état WSL en temps réel.
  - **Taille binaire** : Le build de production optimisé produit un exécutable compact unique.

## 🌍 Langues Supportées

Le support complet de l'internationalisation est fourni pour les langues suivantes :

| Langue | Code | Emoji |
| :--- | :---: | :---: |
| Chinois Simplifié | `zh-CN` | 🇨🇳 |
| Chinois Traditionnel | `zh-TW` | 🇭🇰 / 🇹🇼 |
| Anglais | `en` | 🇺🇸 |
| Japonais | `ja` | 🇯🇵 |
| Français | `fr` | 🇫🇷 |
| Espagnol | `es` | 🇪🇸 |
| Russe | `ru` | 🇷🇺 |
| Portugais | `pt` | 🇵🇹 |
| Allemand | `de` | 🇩🇪 |
| Italien | `it` | 🇮🇹 |
| Turc | `tr` | 🇹🇷 |
| Indonésien | `id` | 🇮🇩 |
| Hindi | `hi` | 🇮🇳 |
| Bengali | `bn` | 🇧🇩 |

## 📄 Licence

Ce projet est sous licence GPL-3.0 – voir le fichier [LICENSE](LICENSE) pour plus de détails.

---

Built with ❤️ for the WSL Community.
