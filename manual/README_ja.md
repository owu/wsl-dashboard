# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

モダンで高性能、軽量、かつ低メモリの WSL (Windows Subsystem for Linux) インスタンス管理ダッシュボード。プレミアムなネイティブ体験のために Rust と Slint で構築されています。

---

```diff
注意:

- WSL Dashboard は Microsoft Store を通じて配布されていません。
- 「WSL Dashboard」という名前でそこにリストされているアプリケーションは承認されておらず、偽物である可能性があります。
- 詐欺を避けるためにダウンロードしないでください。
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

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | 日本語 | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 目次
- [🌍 多言語対応](#-多言語対応)
- [🚀 主な機能と使用方法](#-主な機能と使用方法)
- [⚙️ 設定とログ](#️-設定とログ)
- [🖼️ スクリーンショット](#️-スクリーンショット)
- [🎬 操作デモ](#-操作デモ)
- [💻 システム要件](#-システム要件)
- [📦 インストール指南](#-インストール指南)
- [🛠️ 技術スタックとパフォーマンス](#️-技术スタックとパフォーマンス)
- [🤝 コミュニティサポート](#-コミュニティサポート)
- [❤️ このプロジェクトを支援する](#️-このプロジェクトを支援する)
- [⭐️ 愛の結晶](#️-愛の結晶)
- [📄 ライセンス](#-ライセンス)

---

## 🌍 多言語対応

英語, 中国語 (簡体字), 中国語 (繁体字), ヒンディー語, スペイン語, フランス語, アラビア語, ベンガル語, ポルトガル語, ロシア語, ウルドゥー語, インドネシア語, ドイツ語, 日本語, トルコ語, 韓国語, イタリア語, オランダ語, スウェーデン語, チェコ語, ギリシャ語, ハンガリー語, ヘブライ語, ノルウェー語, デンマーク語, フィンランド語, スロバキア語, スロベニア語, アイスランド語, ベトナム語, テルグ語, ジャワ語, タイ語, タミル語, フィリピン語, パンジャーブ語, マレー語, ポーランド語, ウクライナ語, ペルシア語, カンナダ語, マラーティー語, ハウサ語, ビルマ語, ウズベク語, アゼルバイジャン語, セブアノ語, マラヤーラム語, シンド語, アムハラ語

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="英語" alt="英語" />
  <img src="../assets/flags/cn.svg" width="32" title="中国語 (簡体字)" alt="中国語 (簡体字)" />
  <img src="../assets/flags/tw.svg" width="32" title="中国語 (繁体字)" alt="中国語 (繁体字)" />
  <img src="../assets/flags/in.svg" width="32" title="ヒンディー語" alt="ヒンディー語" />
  <img src="../assets/flags/es.svg" width="32" title="スペイン語" alt="スペイン語" />
  <img src="../assets/flags/fr.svg" width="32" title="フランス語" alt="フランス語" />
  <img src="../assets/flags/sa.svg" width="32" title="アラビア語" alt="アラビア語" />
  <img src="../assets/flags/bd.svg" width="32" title="ベンガル語" alt="ベンガル語" />
  <img src="../assets/flags/pt.svg" width="32" title="ポルトガル語" alt="ポルトガル語" />
  <img src="../assets/flags/ru.svg" width="32" title="ロシア語" alt="ロシア語" />
  <img src="../assets/flags/pk.svg" width="32" title="ウルドゥー語" alt="ウルドゥー語" />
  <img src="../assets/flags/id.svg" width="32" title="インドネシア語" alt="インドネシア語" />
  <img src="../assets/flags/de.svg" width="32" title="ドイツ語" alt="ドイツ語" />
  <img src="../assets/flags/jp.svg" width="32" title="日本語" alt="日本語" />
  <img src="../assets/flags/tr.svg" width="32" title="トルコ語" alt="トルコ語" />
  <img src="../assets/flags/kr.svg" width="32" title="韓国語" alt="韓国語" />
  <img src="../assets/flags/it.svg" width="32" title="イタリア語" alt="イタリア語" />
  <img src="../assets/flags/nl.svg" width="32" title="オランダ語" alt="オランダ語" />
  <img src="../assets/flags/se.svg" width="32" title="スウェーデン語" alt="スウェーデン語" />
  <img src="../assets/flags/cz.svg" width="32" title="チェコ語" alt="チェコ語" />
  <img src="../assets/flags/gr.svg" width="32" title="ギリシャ語" alt="ギリシャ語" />
  <img src="../assets/flags/hu.svg" width="32" title="ハンガリー語" alt="ハンガリー語" />
  <img src="../assets/flags/il.svg" width="32" title="ヘブライ語" alt="ヘブライ語" />
  <img src="../assets/flags/no.svg" width="32" title="ノルウェー語" alt="ノルウェー語" />
  <img src="../assets/flags/dk.svg" width="32" title="デンマーク語" alt="デンマーク語" />
  <img src="../assets/flags/fi.svg" width="32" title="フィンランド語" alt="フィンランド語" />
  <img src="../assets/flags/sk.svg" width="32" title="スロバキア語" alt="スロバキア語" />
  <img src="../assets/flags/si.svg" width="32" title="スロベニア語" alt="スロベニア語" />
  <img src="../assets/flags/is.svg" width="32" title="アイスランド語" alt="アイスランド語" />
  <img src="../assets/flags/vn.svg" width="32" title="ベトナム語" alt="ベトナム語" />
  <img src="../assets/flags/in.svg" width="32" title="テルグ語" alt="テルグ語" />
  <img src="../assets/flags/id.svg" width="32" title="ジャワ語" alt="ジャワ語" />
  <img src="../assets/flags/th.svg" width="32" title="タイ語" alt="タイ語" />
  <img src="../assets/flags/in.svg" width="32" title="タミル語" alt="タミル語" />
  <img src="../assets/flags/ph.svg" width="32" title="フィリピン語" alt="フィリピン語" />
  <img src="../assets/flags/pk.svg" width="32" title="パンジャーブ語" alt="パンジャーブ語" />
  <img src="../assets/flags/my.svg" width="32" title="マレー語" alt="マレー語" />
  <img src="../assets/flags/pl.svg" width="32" title="ポーランド語" alt="ポーランド語" />
  <img src="../assets/flags/ua.svg" width="32" title="ウクライナ語" alt="ウクライナ語" />
  <img src="../assets/flags/ir.svg" width="32" title="ペルシア語" alt="ペルシア語" />
  <img src="../assets/flags/in.svg" width="32" title="カンナダ語" alt="カンナダ語" />
  <img src="../assets/flags/in.svg" width="32" title="マラーティー語" alt="マラーティー語" />
  <img src="../assets/flags/ng.svg" width="32" title="ハウサ語" alt="ハウサ語" />
  <img src="../assets/flags/mm.svg" width="32" title="ビルマ語" alt="ビルマ語" />
  <img src="../assets/flags/uz.svg" width="32" title="ウズベク語" alt="ウズベク語" />
  <img src="../assets/flags/az.svg" width="32" title="アゼルバイジャン語" alt="アゼルバイジャン語" />
  <img src="../assets/flags/ph.svg" width="32" title="セブアノ語" alt="セブアノ語" />
  <img src="../assets/flags/in.svg" width="32" title="マラヤーラム語" alt="マラヤーラム語" />
  <img src="../assets/flags/pk.svg" width="32" title="シンド語" alt="シンド語" />
  <img src="../assets/flags/et.svg" width="32" title="アムハラ語" alt="アムハラ語" />
</p>


## 🚀 主な機能と使用方法

- **モダンなネイティブ UI**：直感的でダーク/ライトモードに対応した GUI、スムーズなアニメーション、**Skia** による高性能なレンダリング。
- **システムトレイ統合**：システムトレイへの最小化をフルサポート（約 10MB RAM 使用）、ダブルクリックでの表示切り替え、多機能な右クリックメニュー。
- **インテリジェントな起動**：Windows 起動時の自動開始、トレイへの最小化起動（`/silent` オプション）、終了時の自動シャットダウンを設定可能。
- **包括的なインスタンス管理**：ワンクリックでの起動、停止、終了、登録解除。リアルタイムの状態監視と、ディスク使用量やファイル場所の詳細表示。
- **ディストリビューション管理**：デフォルト設定、ディレクトリ移動（VHDX の別ドライブへの移行）、`.tar` / `.tar.gz` 形式でのエクスポートとクローン。
- **クイック統合**：ターミナル、VS Code、ファイルエクスプローラーを即座に起動。作業ディレクトリのカスタマイズや起動スクリプトのフックに対応。
- **ディストリビューションのインストール**：Microsoft Store、GitHub、ローカルファイル（RootFS/VHDX）、またはオンラインミラーから Linux ディストリビューションをインストール（自動速度テストで最速ミラーを選択、RootFS ダウンロードヘルパー内蔵）。
- **グローバルな安全性**：ミューテックスロックによる移行/バックアップの同時実行制御、削除時の Appx 自動クリーンアップ。
- **超低メモリフットプリント**：効率を極限まで最適化。トレイ待機時は約 **10MB**、ウィンドウモードでも言語に応じ **18MB**（標準）〜 **38MB**（中日韓などの複雑なフォント）で動作。
- **高度なネットワーク**: シームレスなポート転送管理（ファイアウォールルールの自動作成機能付き）および統合接続のためのグローバルHTTPプロキシ設定。
- **USBデバイス管理**: `usbipd-win`との完全な統合により、ダッシュボードから直接、すべてのWSLインスタンスにわたってローカルUSBデバイスを簡単にバインド、アタッチ、および管理できます。


## ⚙️ 設定とログ

すべての設定は「設定」ビューで管理されます：

- 新しい WSL インスタンスのデフォルトインストール先を選択。
- ログディレクトリとログレベル (Error / Warn / Info / Debug / Trace) の設定。
- UI 言語の選択（またはシステム言語に従う）。
- ダークモードの切り替え、および操作後に WSL を自動シャットダウンするかどうかの設定。
- アップデート確認の頻度（毎日、毎週、隔週、毎月）の設定。
- Windows 起動時の自動開始（パス自動修復機能付き）。
- 起動時にシステムトレイへ最小化する設定。
- 閉じるボタンで終了せずトレイに最小化する設定。
- 特定の機能タブの表示/非表示を切り替えてサイドバーをカスタマイズします。

ログファイルは指定されたディレクトリに書き込まれ、問題報告時に添付できます。


## 🖼️ スクリーンショット

### ホーム (ライト＆ダークモード)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB ＆ メニューの折りたたみ
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### ネットワーク
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### インスタンス追加 ＆ 設定
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>
<p align="center">
  <img src="../assets/screenshot/settings-advanced.png" width="48%" />
  <img src="../assets/screenshot/settings-interface.png" width="48%" />
</p>

### バージョン情報 & 寄付
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/donate.png" width="48%" />
</p>

## 🎬 操作デモ

[改善にご協力ください！紹介ビデオをご覧いただき、ご意見をお聞かせください。](https://github.com/owu/wsl-dashboard/discussions/9)



## 💻 システム要求

- WSL が有効な Windows 10 または Windows 11（WSL 2 推奨）。
- 少なくとも 1 つの WSL ディストリビューションがインストールされているか、新しいものをインストールする権限があること。
- 64ビット CPU。複数のディストリビューションをスムーズに使用するために 4 GB 以上の RAM を推奨。

## 📦 インストール指南

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
   - 専用スクリプトを使用してリリースビルドを作成：

     > ビルドスクリプトには `x86_64-pc-windows-msvc` ツールチェーンが必要です。

     ```powershell
     .\build\portable\build.ps1
     ```


## 🛠️ 技術スタックとパフォーマンス

- **コア**: メモリ安全性とゼロコスト抽象化のために Rust で実装。
- **UI フレームワーク**: 高性能な **Skia** レンダリングバックエンドを備えた Slint。
- **非同期ランタイム**: Tokio。非ブロッキングなシステムコマンドと I/O を実現。
- **パフォーマンスハイライト**:
  - **レスポンス**: ほぼ瞬時の起動とリアルタイムの WSL 状態監視。
  - **効率性**: 超低リソース使用量（詳細は [主な機能](#-主な機能と使用方法) を参照）。
  - **ポータビリティ**: 単一のコンパクトな実行ファイルを生成。



## 🤝 コミュニティサポート

以下のコミュニティのサポートに心から感謝します：

- [Rust Programming Language](https://www.rust-lang.org) - 強力で安全なプログラミング言語
- [Slint | Declarative GUI for Rust, C++, JavaScript & Python](https://slint.dev) - モダンなUIフレームワーク
- [WSL: Windows Subsystem for Linux](https://github.com/microsoft/WSL) - 素晴らしいWindows Subsystem for Linux
- [Tokio - An asynchronous Rust runtime](https://tokio.rs) - 効率的な非同期ランタイム
- [Windows Developer Community](https://developer.microsoft.com/en-us/windows/community) - 継続的なプラットフォームの改善
- [Reddit](https://www.reddit.com) - グローバルコミュニティの議論とサポート
- [Hacker News](https://news.ycombinator.com) - グローバルコミュニティの議論とサポート
- [Linux.do](https://linux.do) - ITプロフェッショナル向けの人気コミュニティ
- [V2EX](https://www.v2ex.com) - 中国語テクノロジーコミュニティの議論

皆様の貢献とフィードバックがこのプロジェクトを可能にしています！


## ❤️ このプロジェクトを支援する

- 本プロジェクトはGPL-3.0のオープンソースライセンスの下、すべてのユーザーに無料で提供されています。
- 機能開発、日常テスト、バグ修正まで、すべての作業は余暇の時間に行われています。オープンソースの道を一人で歩むのは容易ではありません。あなたの認知と支援が、プロジェクトを持続させる力となります。
- このツールがあなたの役に立ったと感じたら、ぜひご支援をお願いいたします。すべての寄付はサーバー費用、バージョンアップ、機能改善に充てられ、プロジェクトの継続的な更新と着実な進歩を支えます。
- ささやかな善意も、星の光となります。ご理解とご支援に心より感謝いたします！

寄付ページにアクセスする：[https://www.wslui.com/donate/](https://www.wslui.com/donate/)


## ⭐️ 愛の結晶

このプロジェクトが役に立ったと感じられたなら、GitHub でスターを付けていただければ幸いです。皆様の応援がより多くの人に届く助けとなり、心から感謝しております。この励ましこそが、開発を続ける原動力です。


## 📄 ライセンス

このプロジェクトは GPL-3.0 の下でライセンスされています。詳細は [LICENSE](../LICENSE) ファイルをご覧ください。


---

Built with ❤️ for the WSL Community.

