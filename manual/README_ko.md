# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard 로고" />
</p>

현대적이고, 고성능이며, 가볍고, 메모리 사용량이 적은 WSL (Windows Subsystem for Linux) 인스턴스 관리 대시보드입니다. 프리미엄 네이티브 환경을 위해 Rust 및 Slint로 구축되었습니다.

```diff
알림:

- WSL Dashboard는 Microsoft Store를 통해 배포되지 않습니다.
- "WSL Dashboard"라는 이름으로 거기에 나열된 모든 애플리케이션은 무단이며 위조되었을 수 있습니다.
- 사기를 피하려면 다운로드하지 마세요.
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

README I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | 한국어 | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 목차
[🌍 지원 언어](#-지원-언어) | [🚀 주요 기능 및 사용법](#-주요-기능-및-사용법) | [🖼️ 스크린샷](#️-스크린샷) | [📦 설치 가이드](#-설치-가이드) | [📢 공식 커뮤니티 그룹](#-공식-커뮤니티-그룹) | [❤️ 이 프로젝트 지원하기](#️-이-프로젝트-지원하기) | [⭐️ 사랑의 결실](#️-사랑의-결실) | [📄 라이선스](#-라이선스)

---

## 🌍 지원 언어

영어, 중국어(간체/번체), 힌디어, 스페인어, 프랑스어, 아랍어, 벵골어, 포르투갈어, 러시아어, 우르두어, 인도네시아어, 독일어, 일본어, 터키어, 한국어, 이탈리아어, 네덜란드어, 스웨덴어, 체코어, 그리스어, 헝가리어, 히브리어, 노르웨이어, 덴마크어, 핀란드어, 슬로바키아어, 슬로베니아어, 아이슬란드어, 베트남어, 텔루구어, 자바어, 태국어, 타밀어, 필리핀어, 펀잡어, 말레이어, 폴란드어, 우크라이나어, 페르시아어, 칸나다어, 마라티어, 하우사어, 버마어, 우즈베크어, 아제르바이잔어, 세부아노어, 말라얄람어, 신드어, 암하라어.

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="영어" alt="영어" />
  <img src="../assets/flags/cn.svg" width="32" title="중국어 (간체)" alt="중국어 (간체)" />
  <img src="../assets/flags/tw.svg" width="32" title="중국어 (번체)" alt="중국어 (번체)" />
  <img src="../assets/flags/in.svg" width="32" title="힌디어" alt="힌디어" />
  <img src="../assets/flags/es.svg" width="32" title="스페인어" alt="스페인어" />
  <img src="../assets/flags/fr.svg" width="32" title="프랑스어" alt="프랑스어" />
  <img src="../assets/flags/sa.svg" width="32" title="아랍어" alt="아랍어" />
  <img src="../assets/flags/bd.svg" width="32" title="벵골어" alt="벵골어" />
  <img src="../assets/flags/pt.svg" width="32" title="포르투갈어" alt="포르투갈어" />
  <img src="../assets/flags/ru.svg" width="32" title="러시아어" alt="러시아어" />
  <img src="../assets/flags/pk.svg" width="32" title="우르두어" alt="우르두어" />
  <img src="../assets/flags/id.svg" width="32" title="인도네시아어" alt="인도네시아어" />
  <img src="../assets/flags/de.svg" width="32" title="독일어" alt="독일어" />
  <img src="../assets/flags/jp.svg" width="32" title="일본어" alt="일본어" />
  <img src="../assets/flags/tr.svg" width="32" title="터키어" alt="터키어" />
  <img src="../assets/flags/kr.svg" width="32" title="한국어" alt="한국어" />
  <img src="../assets/flags/it.svg" width="32" title="이탈리아어" alt="이탈리아어" />
  <img src="../assets/flags/nl.svg" width="32" title="네덜란드어" alt="네덜란드어" />
  <img src="../assets/flags/se.svg" width="32" title="스웨덴어" alt="스웨덴어" />
  <img src="../assets/flags/cz.svg" width="32" title="체코어" alt="체코어" />
  <img src="../assets/flags/gr.svg" width="32" title="그리스어" alt="그리스어" />
  <img src="../assets/flags/hu.svg" width="32" title="헝가리어" alt="헝가리어" />
  <img src="../assets/flags/il.svg" width="32" title="히브리어" alt="히브리어" />
  <img src="../assets/flags/no.svg" width="32" title="노르웨이어" alt="노르웨이어" />
  <img src="../assets/flags/dk.svg" width="32" title="덴마크어" alt="덴마크어" />
  <img src="../assets/flags/fi.svg" width="32" title="핀란드어" alt="핀란드어" />
  <img src="../assets/flags/sk.svg" width="32" title="슬로바키아어" alt="슬로바키아어" />
  <img src="../assets/flags/si.svg" width="32" title="슬로베니아어" alt="슬로베니아어" />
  <img src="../assets/flags/is.svg" width="32" title="아이슬란드어" alt="아이슬란드어" />
  <img src="../assets/flags/vn.svg" width="32" title="베트남어" alt="베트남어" />
  <img src="../assets/flags/in.svg" width="32" title="텔루구어" alt="텔루구어" />
  <img src="../assets/flags/id.svg" width="32" title="자바어" alt="자바어" />
  <img src="../assets/flags/th.svg" width="32" title="태국어" alt="태국어" />
  <img src="../assets/flags/in.svg" width="32" title="타밀어" alt="타밀어" />
  <img src="../assets/flags/ph.svg" width="32" title="필리핀어" alt="필리핀어" />
  <img src="../assets/flags/in.svg" width="32" title="펀잡어" alt="펀잡어" />
  <img src="../assets/flags/my.svg" width="32" title="말레이어" alt="말레이어" />
  <img src="../assets/flags/pl.svg" width="32" title="폴란드어" alt="폴란드어" />
  <img src="../assets/flags/ua.svg" width="32" title="우크라이나어" alt="우크라이나어" />
  <img src="../assets/flags/ir.svg" width="32" title="페르시아어" alt="페르시아어" />
  <img src="../assets/flags/in.svg" width="32" title="칸나다어" alt="칸나다어" />
  <img src="../assets/flags/in.svg" width="32" title="마라티어" alt="마라티어" />
  <img src="../assets/flags/ng.svg" width="32" title="하우사어" alt="하우사어" />
  <img src="../assets/flags/mm.svg" width="32" title="버마어" alt="버마어" />
  <img src="../assets/flags/uz.svg" width="32" title="우즈베크어" alt="우즈베크어" />
  <img src="../assets/flags/az.svg" width="32" title="아제르바이잔어" alt="아제르바이잔어" />
  <img src="../assets/flags/ph.svg" width="32" title="세부아노어" alt="세부아노어" />
  <img src="../assets/flags/in.svg" width="32" title="말라얄람어" alt="말라얄람어" />
  <img src="../assets/flags/pk.svg" width="32" title="신드어" alt="신드어" />
  <img src="../assets/flags/et.svg" width="32" title="암하라어" alt="암하라어" />
</p>

## 🚀 주요 기능 및 사용법

- **현대적인 네이티브 UI**: 라이트/다크 모드 지원, 부드러운 애니메이션, **Skia** 기반의 고성능 렌더링을 갖춘 직관적인 GUI.
- **시스템 트레이 통합**: 트레이 최소화 지원(메모리 사용량 약 10MB), 더블 클릭으로 창 전환, 실용적인 우클릭 메뉴 제공.
- **지능형 시작 설정**: 윈도우 시작 시 자동 실행, 트레이로 최소화하여 시작(정숙 모드 `/silent`), 종료 시 배포판 자동 종료 기능을 설정할 수 있습니다.
- **작업 스케줄링**: 유연한 작업 일정 관리를 통해 지정된 시간 또는 간격으로 WSL 인스턴스에서 스크립트와 명령을 자동으로 실행하여 자동화된 운영을 지원합니다.
- **포괄적인 인스턴스 제어**: 클릭 한 번으로 시작, 중지, 강제 종료 및 등록 해제가 가능합니다. 실시간 상태 모니터링과 디스크 사용량 및 파일 위치에 대한 상세 정보를 제공합니다.
- **배포판 관리**: 기본값 설정, 마이그레이션(VHDX 파일을 다른 드라이브로 이동), `.tar` 또는 `.tar.gz` 아카이브로 내보내기 및 복제가 가능합니다.
- **디스크 마운트**: WSL 인스턴스의 VHDX 가상 디스크를 직접 마운트하고 관리합니다. 마운트, 마운트 해제, 디스크 관리 작업을 지원하며 호스트와 WSL 간의 파일 전송을 용이하게 합니다.
- **빠른 통합**: 사용자 지정 작업 디렉토리 및 시작 스크립트 훅을 통해 터미널, VS Code 또는 파일 탐색기를 즉시 실행할 수 있습니다.
- **배포판 설치**: Microsoft Store, GitHub, 로컬 파일(RootFS/VHDX) 또는 온라인 미러를 통해 Linux 배포판을 설치할 수 있습니다 (자동 속도 테스트로 가장 빠른 미러 선택, 내장 RootFS 다운로드 도우미 포함).
- **글로벌 안전성**: 안전한 동시 마이그레이션/백업 작업을 위한 뮤텍스(Mutex) 잠금 및 삭제 시 자동 Appx 정리를 지원합니다.
- **초저전력 메모리 점유**: 효율성을 위해 고도로 최적화되었습니다. 트레이 정숙 모드 시작 시 약 **10MB**의 RAM만 사용합니다. 창 모드 사용 시 폰트 복잡성에 따라 메모리 사용량이 달라집니다. 표준 언어(영어, 독일어, 스페인어 등)는 **약 18MB**, 한중일 등 큰 문자 세트 언어는 **약 38MB**를 사용합니다.
- **고급 네트워킹**: 원활한 포트 포워딩 관리(방화벽 규칙 자동 생성 포함) 및 통합된 연결을 위한 전역 HTTP 프록시 구성.
- **USB 장치 관리**: 대시보드 UI에서 직접 `usbipd-win` 과의 완전한 통합을 통해 모든 WSL 인스턴스에서 로컬 USB 장치를 손쉽게 바인드, 연결 및 관리할 수 있습니다.
- **설정 및 로그**: 설정에서 기본 설치 폴더, 로그 폴더와 수준, UI 언어, 어두운 모드, 업데이트 확인 주기, 자동 시작 및 종료 동작, 사이드바를 한곳에서 관리합니다.
- **배포판 상시 실행**: «시작»을 한 번 클릭하면 약 60초 유휴 후 자동 종료되는 대신 계속 실행됩니다. «Windows 시작 시 이 배포판 자동 시작»도 켤 수 있습니다.
- **전역 단축키**: 전역 단축키를 지정해 기본 창을 표시/숨기거나 특정 배포판의 터미널을 즉시 엽니다.


## 🖼️ 스크린샷
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

대시보드가 실제로 동작하는 모습을 보고 싶으신가요? 전체 스크린샷 갤러리는 [공식 웹사이트](https://www.wslui.com/)에서 확인하세요.

## 📦 설치 가이드

### 옵션 1: 프로젝트 웹사이트 방문 (권장)

더 원활한 다운로드 경험을 위해 여러 미러 링크를 제공하는 공식 웹사이트를 방문할 것을 권장합니다:

[다운로드 페이지](https://www.wslui.com/download/)로 이동하여 해당 지역에 맞는 미러를 선택하세요.

### 옵션 2: winget으로 설치

모니커(moniker) 또는 전체 패키지 식별자를 사용하여 Windows 패키지 관리자(winget)에서 직접 WSLDashboard를 설치할 수 있습니다:

```powershell
# 검색 (대소문자 구분 없음)
winget search wsl-dashboard
# 또는
winget search WSLDashboard

# 설치 (하나를 선택)
winget install wsl-dashboard
# 또는
winget install Owu.WSLDashboard
```

> winget 패키지 식별자는 `Owu.WSLDashboard`이고 모니커는 `wsl-dashboard`입니다 (대소문자 구분 없음). 둘 다 사용 가능합니다.

자세한 내용은 [WinGet 커뮤니티 리포지토리](https://github.com/microsoft/winget-pkgs/tree/master/manifests/o/Owu/WSLDashboard)를 방문하세요.

### 옵션 3: 소스에서 빌드

Rust 툴체인(Rust 1.92 이상)이 설치되어 있는지 확인하세요.

1. 리포지토리 클론:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. 빌드 및 실행:

   - 개발용:

     ```powershell
     cargo run
     ```
   - 빌드 스크립트를 사용한 최적화된 릴리스 빌드:

     > 빌드 스크립트는 `x86_64-pc-windows-msvc` 툴체인이 필요합니다.

     ```powershell
     .\build\portable\build.ps1 -Arch x64
     ```


## 📢 공식 커뮤니티 그룹

WSL UI 공식 커뮤니티 그룹에 오신 것을 환영합니다! 사용 팁 논의, 문제 보고, 최신 업데이트 파악, 개발자 및 다른 사용자들과 소통하기 위해 채널에 참여하세요.

<div align="center">

| 글로벌 그룹 (Telegram) | 중국 그룹 (WeChat) |
| :---: | :---: |
| <img src="https://cdn1.wslui.com/static/images/group/group-telegram.png" width="240" alt="Telegram Group QR Code" /> | <img src="https://cdn1.wslui.com/static/images/group/group-wechat.png" width="240" alt="WeChat Group QR Code" /> |
| 스캔하여 참여 | 스캔하여 참여 |

</div>


## ❤️ 이 프로젝트 지원하기

- 이 프로젝트는 GPL-3.0 오픈 소스 라이선스 하에 모든 사용자에게 무료로 제공됩니다.
- 기능 개발, 일상 테스트부터 버그 수정까지, 모든 작업은 여가 시간에 이루어집니다. 오픈 소스의 길을 혼자 걷기는 쉽지 않습니다. 여러분의 인정과 지원이 프로젝트가 오래 지속될 수 있는 원동력입니다.
- 이 도구가 실제로 도움이 되었다면, 작은 후원을 부탁드립니다. 모든 기부금은 서버 비용, 버전 업데이트, 기능 개선에 사용되어 프로젝트의 지속적인 업데이트와 안정적인 발전을 지원합니다.
- 작은 친절도 별빛이 됩니다. 여러분의 이해와 관대함에 다시 한번 감사드립니다！ [기부 페이지 방문하기](https://www.wslui.com/donate/).


## ⭐️ 사랑의 결실

이 프로젝트가 유용하다고 생각하신다면 GitHub에서 별(star)을 남겨주시면 감사하겠습니다. 여러분의 지지는 더 많은 사용자에게 도달하는 데 도움이 되며 진심으로 감사드리고 있습니다. 이러한 격려가 계속해서 개발할 수 있는 동기가 됩니다.


## 📄 라이선스

이 프로젝트는 GPL-3.0 라이선스 하에 배포됩니다. 자세한 내용은 [LICENSE](../LICENSE) 파일을 참조하세요.
