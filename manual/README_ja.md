# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

モダンで高性能、軽量、かつ低メモリの WSL (Windows Subsystem for Linux) インスタンス管理ダッシュボード。プレミアムなネイティブ体験のために Rust と Slint で構築されています。

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

README I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | 日本語 | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 目次
[🌍 多言語対応](#-多言語対応) | [🚀 主な機能と使用方法](#-主な機能と使用方法) | [🖼️ スクリーンショット](#️-スクリーンショット) | [📦 インストール指南](#-インストール指南) | [📢 公式コミュニティグループ](#-公式コミュニティグループ) | [❤️ このプロジェクトを支援する](#️-このプロジェクトを支援する) | [⭐️ 愛の結晶](#️-愛の結晶) | [📄 ライセンス](#-ライセンス)

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
- **タスクスケジューリング**: 柔軟なタスクスケジュール管理により、指定した時間または間隔で WSL インスタンス内のスクリプトやコマンドを自動実行し、自動運用を実現します。
- **包括的なインスタンス管理**：ワンクリックでの起動、停止、終了、登録解除。リアルタイムの状態監視と、ディスク使用量やファイル場所の詳細表示。
- **ディストリビューション管理**：デフォルト設定、ディレクトリ移動（VHDX の別ドライブへの移行）、`.tar` / `.tar.gz` 形式でのエクスポートとクローン。
- **ディスクマウント**: WSL インスタンスの VHDX 仮想ディスクを直接マウントして管理します。マウント、アンマウント、ディスク管理操作をサポートし、ホストと WSL 間のファイル転送を容易にします。
- **クイック統合**：ターミナル、VS Code、ファイルエクスプローラーを即座に起動。作業ディレクトリのカスタマイズや起動スクリプトのフックに対応。
- **ディストリビューションのインストール**：Microsoft Store、GitHub、ローカルファイル（RootFS/VHDX）、またはオンラインミラーから Linux ディストリビューションをインストール（自動速度テストで最速ミラーを選択、RootFS ダウンロードヘルパー内蔵）。
- **グローバルな安全性**：ミューテックスロックによる移行/バックアップの同時実行制御、削除時の Appx 自動クリーンアップ。
- **超低メモリフットプリント**：効率を極限まで最適化。トレイ待機時は約 **10MB**、ウィンドウモードでも言語に応じ **18MB**（標準）〜 **38MB**（中日韓などの複雑なフォント）で動作。
- **高度なネットワーク**: シームレスなポート転送管理（ファイアウォールルールの自動作成機能付き）および統合接続のためのグローバルHTTPプロキシ設定。
- **USBデバイス管理**: `usbipd-win`との完全な統合により、ダッシュボードから直接、すべてのWSLインスタンスにわたってローカルUSBデバイスを簡単にバインド、アタッチ、および管理できます。
- **設定とログ**：設定画面で既定のインストール先、ログの保存先とレベル、UI 言語、ダークモード、アップデート確認の頻度、自動起動と終了時の動作、サイドバーの表示を一括管理できます。
- **ディストロの常駐**：［起動］をワンクリックするだけで、約 60 秒アイドルで自動終了せず稼働を継続します。「Windows 起動時にこのディストロを起動する」を有効にすると常時稼働も可能です。
- **グローバルショートカット**：グローバルホットキーを設定し、メインウィンドウの表示/非表示や、指定ディストロのターミナルを即座に開けます。


## 🖼️ スクリーンショット
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

ダッシュボードの実際の動作を見たいですか？完全なスクリーンショットギャラリーは[公式サイト](https://www.wslui.com/)でご覧ください。

## 📦 インストール指南

### 方法 1: プロジェクトのウェブサイトを参照 (推奨)

公式ウェブサイトは複数のミラーリンクを提供しており、よりスムーズなダウンロード体験が得られるため、アクセスすることをお勧めします：

[ダウンロードページ](https://www.wslui.com/download/) にアクセスし、お住まいの地域に適したミラーを選択してください。

### 方法 2: winget でインストール

Windows Package Manager (winget) を使用して、モニカーまたは完全なパッケージ識別子のいずれかで WSLDashboard を直接インストールできます：

```powershell
# 検索 (大文字小文字の区別なし)
winget search wsl-dashboard
# または
winget search WSLDashboard

# インストール (いずれかを選択)
winget install wsl-dashboard
# または
winget install Owu.WSLDashboard
```

> winget パッケージ識別子は `Owu.WSLDashboard`、モニカーは `wsl-dashboard` です (大文字小文字の区別なし)。どちらでも使用できます。

詳細については、[WinGet コミュニティリポジトリ](https://github.com/microsoft/winget-pkgs/tree/master/manifests/o/Owu/WSLDashboard) を参照してください。

### 方法 3: ソースからビルドする

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
     .\build\portable\build.ps1 -Arch x64
     ```


## 📢 公式コミュニティグループ

WSL UI の公式コミュニティグループへようこそ！使い方のお tips、問題の報告、最新情報の入手、開発者や他のユーザーとのつながりにご活用ください。

<div align="center">

| グローバルグループ (Telegram) | 中国語グループ (WeChat) |
| :---: | :---: |
| <img src="https://cdn1.wslui.com/static/images/group/group-telegram.png" width="240" alt="Telegram Group QR Code" /> | <img src="https://cdn1.wslui.com/static/images/group/group-wechat.png" width="240" alt="WeChat Group QR Code" /> |
| QR コードをスキャン | QR コードをスキャン |

</div>


## ❤️ このプロジェクトを支援する

- 本プロジェクトはGPL-3.0のオープンソースライセンスの下、すべてのユーザーに無料で提供されています。
- 機能開発、日常テスト、バグ修正まで、すべての作業は余暇の時間に行われています。オープンソースの道を一人で歩むのは容易ではありません。あなたの認知と支援が、プロジェクトを持続させる力となります。
- このツールがあなたの役に立ったと感じたら、ぜひご支援をお願いいたします。すべての寄付はサーバー費用、バージョンアップ、機能改善に充てられ、プロジェクトの継続的な更新と着実な進歩を支えます。
- ささやかな善意も、星の光となります。ご理解とご支援に心より感謝いたします！[寄付ページにアクセスする](https://www.wslui.com/donate/)。


## ⭐️ 愛の結晶

このプロジェクトが役に立ったと感じられたなら、GitHub でスターを付けていただければ幸いです。皆様の応援がより多くの人に届く助けとなり、心から感謝しております。この励ましこそが、開発を続ける原動力です。


## 📄 ライセンス

このプロジェクトは GPL-3.0 の下でライセンスされています。詳細は [LICENSE](../LICENSE) ファイルをご覧ください。
