# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" />
</p>

モダンで高性能、かつ軽量な WSL (Windows Subsystem for Linux) インスタンス管理ダッシュボードです。Rust と Slint で構築され、プレミアムなネイティブ体験を提供します。

---

[![Rust](https://img.shields.io/badge/rust-v1.92+-orange.svg)](https://www.rust-lang.org)
[![Slint](https://img.shields.io/badge/UI-Slint-blue.svg)](https://slint.dev)
[![License](https://img.shields.io/badge/license-GPL--3.0-blue.svg)](LICENSE)

I18N: [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [English](../README.md) | 日本語 | [Français](./README_fr.md) | [Español](./README_es.md) | [Русский](./README_ru.md) | [Português](./README_pt.md) | [Deutsch](./README_de.md) | [Italiano](./README_it.md) | [Türkçe](./README_tr.md) | [Bahasa Indonesia](./README_id.md) | [हिन्दी](./README_hi.md) | [বাংলা](./README_bn.md)

---

## 🖼️ スクリーンショット

### ホーム (ライト＆ダークモード)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

### インスタンス追加 ＆ 設定
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>

### バージョン情報 ＆ メニューの折りたたみ
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

## 🎬 操作デモ

以下は WSL Dashboard の動作デモです：

![WSL Dashboard Demo](../assets/screenshot/demo.gif)

## 🚀 主な機能

- ダークモード対応とスムーズなアニメーションを備えた直感的な GUI。
- すべての WSL ディストリビューションをワンクリックで管理（起動、停止、終了、登録解除）。
- ディストリビューションのターミナル、VS Code、ファイルエクスプローラーへのクイックアクセス。
- WSL インスタンスの状態をリアルタイムで監視および表示。
- `.tar` または圧縮された `.tar.gz` アーカイブへのエクスポートとバックアップ。
- バックアップや既存のディストリビューションからのインスタンスのインポートとクローン作成。
- 大容量の WSL インスタンス（VHDX）を他のディスクへ移動し、Cドライブの空き容量を確保。
- Microsoft Store または GitHub からのスマートなディストリビューション作成。
- 手動インストール用の内蔵 RootFS ダウンロードヘルパー。
- VHDX ファイルの場所、仮想ディスクサイズ、Linux 内の実ディスク使用量の詳細表示。

## システム要件

- WSL が有効な Windows 10 または Windows 11（WSL 2 推奨）。
- 少なくとも 1 つの WSL ディストリビューションがインストールされているか、新しいものをインストールする権限があること。
- 64ビット CPU。複数のディストリビューションをスムーズに使用するために 4 GB 以上の RAM を推奨。

## 📦 インストール

### 方法 1: ビルド済みバイナリをダウンロードする

最も簡単な方法は、コンパイル済みのリリースを使用することです：

1. [GitHub Releases](https://github.com/owu/wsl-dashboard/releases) ページにアクセスします。
2. Windows 用の最新の `wsldashboard` 実行ファイルをダウンロードします。
3. 必要に応じて解凍し、`wsldashboard.exe` を実行します。

インストーラーは不要です。アプリは単一のポータブルバイナリです。

### 方法 2: ソースからビルドする

Rust ツールチェーン (Rust 1.92+ 以降) がインストールされていることを確認してください。

1. リポジトリをクローンします：

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. ビルドして実行します：

   - 開発用：

     ```powershell
     cargo run
     ```

   - 最適化されたリリースビルド：

     ```powershell
     cargo run --release
     ```

   - ビルドスクリプトを使用する場合 (推奨)：

     > ビルドスクリプトには `x86_64-pc-windows-gnu` ツールチェーンが必要です。

     ```powershell
     .\build\scripts\build.ps1
     ```

## 🧭 使用方法の概要

- **既存のディストリビューションの管理**：メインビューから任意の WSL ディストリビューションを起動、停止、終了、または登録解除できます。
- **ツールの素早い起動**：ワンクリックでディストリビューションをターミナル、VS Code、またはファイルエクスプローラーで開きます。
- **新しいインスタンスの作成**：Microsoft Store からのインストール、RootFS イメージのダウンロード、または既存のディストリビューションのクローン作成が可能です。
- **バックアップと復元**：ディストリビューションを `.tar` / `.tar.gz` アーカイブとしてエクスポートし、後で、または別のマシンでインポートできます。
- **保存場所の移動**：大きな VHDX ファイルを別のドライブに移動して、C ドライブの容量を節約できます。
- **状態の監視**：WSL Dashboard の実行中、リアルタイムのディストリビューション状態とストレージ使用量を確認できます。

## ⚙️ 設定とログ

すべての設定は設定ビューで管理されます：

- 新しい WSL インスタンスのデフォルトインストール先を選択。
- ログディレクトリとログレベル (Error / Warn / Info / Debug / Trace) の設定。
- UI 言語の選択（またはシステム言語に従う）。
- ダークモードの切り替え、および操作後に WSL を自動シャットダウンするかどうかの設定。
- アップデート確認の頻度（毎日、毎週、隔週、毎月）の設定。

ログファイルは指定されたディレクトリに書き込まれ、問題報告時に添付できます。

## 🛠️ 技術スタックとパフォーマンス

- **コア**: メモリ安全性とゼロコスト抽象化のために Rust で実装。
- **UI フレームワーク**: Slint。モダンな GPU 加速 UI ツールキット（バックエンド: `winit`）。
- **非同期ランタイム**: Tokio。高度に並列で非ブロッキングなシステムコマンドと I/O を実現。
- **パフォーマンス**:
  - **メモリ使用量**: 通常 60–80 MB 程度の RAM。
  - **レスポンス**: ストリーミング技術によるほぼ瞬時の起動とリアルタイムの WSL 状態更新。
  - **バイナリサイズ**: 最適化されたリリースビルドにより、単一のコンパクトな実行ファイルを生成。

## 🌍 多言語対応

以下の言語で完全な国際化をサポートしています：

| 言語 | コード | 絵文字 |
| :--- | :---: | :---: |
| 简体中文 | `zh-CN` | 🇨🇳 |
| 繁體中文 | `zh-TW` | 🇭🇰 / 🇹🇼 |
| 英語 | `en` | 🇺🇸 |
| 日本語 | `ja` | 🇯🇵 |
| フランス語 | `fr` | 🇫🇷 |
| スペイン語 | `es` | 🇪🇸 |
| ロシア語 | `ru` | 🇷🇺 |
| ポルトガル語 | `pt` | 🇵🇹 |
| ドイツ語 | `de` | 🇩🇪 |
| イタリア語 | `it` | 🇮🇹 |
| トルコ語 | `tr` | 🇹🇷 |
| インドネシア語 | `id` | 🇮🇩 |
| ヒンディー語 | `hi` | 🇮🇳 |
| ベンガル語 | `bn` | 🇧🇩 |

## 📄 ライセンス

このプロジェクトは GPL-3.0 の下でライセンスされています。詳細は [LICENSE](LICENSE) ファイルをご覧ください。

---

Built with ❤️ for the WSL Community.
