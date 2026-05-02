# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard 로고" />
</p>

현대적이고 강력하며 가벼운 WSL(Windows Subsystem for Linux) 인스턴스 관리 대시보드입니다. 프리미엄 네이티브 경험을 위해 Rust와 Slint로 제작되었습니다.

---

```diff
알림:

- 이 소프트웨어는 Microsoft Store를 통해 배포되지 않습니다.
- 같은 이름으로 거기에 나열된 모든 애플리케이션은 무단이며 위조되었을 수 있습니다.
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

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | 한국어 | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md)

---

## 📑 목차
- [🌍 지원 언어](#-지원-언어)
- [🚀 주요 기능 및 사용법](#-주요-기능-및-사용법)
- [⚙️ 설정 및 로그](#️-설정-및-로그)
- [🖼️ 스크린샷](#️-스크린샷)
- [🎬 작동 데모](#-작동-데모)
- [💻 시스템 요구 사항](#-시스템-요구-사항)
- [📦 설치 가이드](#-설치-가이드)
- [🛠️ 기술 스택 및 성능](#️-기술-스택-및-성능)
- [⭐️ 사랑의 결실](#️-사랑의-결실)
- [📄 라이선스](#-라이선스)

---

## 🌍 지원 언어

영어, 중국어(간체/번체), 힌디어, 스페인어, 프랑스어, 아랍어, 벵골어, 포르투갈어, 러시아어, 우르두어, 인도네시아어, 독일어, 일본어, 터키어, 한국어, 이탈리아어, 네덜란드어, 스웨덴어, 체코어, 그리스어, 헝가리어, 히브리어, 노르웨이어, 덴마크어, 핀란드어, 슬로바키아어, 슬로베니아어, 아이슬란드어.

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
</p>


## 🚀 주요 기능 및 사용법

- **현대적인 네이티브 UI**: 라이트/다크 모드 지원, 부드러운 애니메이션, **Skia** 기반의 고성능 렌더링을 갖춘 직관적인 GUI.
- **시스템 트레이 통합**: 트레이 최소화 지원(메모리 사용량 약 10MB), 더블 클릭으로 창 전환, 실용적인 우클릭 메뉴 제공.
- **지능형 시작 설정**: 윈도우 시작 시 자동 실행, 트레이로 최소화하여 시작(정숙 모드 `/silent`), 종료 시 배포판 자동 종료 기능을 설정할 수 있습니다.
- **포괄적인 인스턴스 제어**: 클릭 한 번으로 시작, 중지, 강제 종료 및 등록 해제가 가능합니다. 실시간 상태 모니터링과 디스크 사용량 및 파일 위치에 대한 상세 정보를 제공합니다.
- **배포판 관리**: 기본값 설정, 마이그레이션(VHDX 파일을 다른 드라이브로 이동), `.tar` 또는 `.tar.gz` 아카이브로 내보내기 및 복제가 가능합니다.
- **빠른 통합**: 사용자 지정 작업 디렉토리 및 시작 스크립트 훅을 통해 터미널, VS Code 또는 파일 탐색기를 즉시 실행할 수 있습니다.
- **스마트 설치**: Microsoft Store, GitHub 또는 로컬 파일(RootFS/VHDX)에서 설치할 수 있습니다. 내장된 RootFS 다운로드 도우미가 포함되어 있습니다.
- **글로벌 안전성**: 안전한 동시 마이그레이션/백업 작업을 위한 뮤텍스(Mutex) 잠금 및 삭제 시 자동 Appx 정리를 지원합니다.
- **초저전력 메모리 점유**: 효율성을 위해 고도로 최적화되었습니다. 트레이 정숙 모드 시작 시 약 **10MB**의 RAM만 사용합니다. 창 모드 사용 시 폰트 복잡성에 따라 메모리 사용량이 달라집니다. 표준 언어(영어, 독일어, 스페인어 등)는 **약 18MB**, 한중일 등 큰 문자 세트 언어는 **약 38MB**를 사용합니다.
- **고급 네트워킹**: 원활한 포트 포워딩 관리(방화벽 규칙 자동 생성 포함) 및 통합된 연결을 위한 전역 HTTP 프록시 구성.
- **USB 장치 관리**: 대시보드 UI에서 직접 `usbipd-win` 과의 완전한 통합을 통해 모든 WSL 인스턴스에서 로컬 USB 장치를 손쉽게 바인드, 연결 및 관리할 수 있습니다.


## ⚙️ 설정 및 로그

모든 설정은 '설정' 화면에서 관리할 수 있습니다.

- 새로운 WSL 인스턴스의 기본 설치 디렉토리를 선택합니다.
- 로그 디렉토리 및 로그 레벨(Error / Warn / Info / Debug / Trace)을 구성합니다.
- UI 언어를 선택하거나 시스템 언어를 따르도록 설정합니다.
- 다크 모드 전환 및 작업 후 WSL 자동 종료 여부를 설정합니다.
- 업데이트 확인 주기(매일, 매주, 격주, 매월)를 구성합니다.
- 시스템 부팅 시 자동 시작을 활성화합니다(자동 경로 복구 포함).
- 시작 시 앱을 시스템 트레이로 최소화하도록 설정하여 방해 없는 환경을 제공합니다.
- 닫기 버튼을 눌렀을 때 종료 대신 시스템 트레이로 최소화되도록 설정합니다.
- 특정 기능 탭의 표시 여부를 전환하여 사이드바를 사용자 지정합니다.

로그 파일은 구성된 로그 디렉토리에 기록되며, 문제 보고 시 첨부할 수 있습니다.


## 🖼️ 스크린샷

### 홈 (라이트 및 다크 모드)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB & 메뉴를 접으세요
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### 회로망
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### 인스턴스 추가 및 설정
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>

### ~에 대한
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
</p>

## 🎬 작동 데모

아래는 WSL Dashboard가 작동하는 배포 데모입니다.

![WSL Dashboard 데모](../assets/screenshot/demo.gif)



## 💻 시스템 요구 사항

- WSL이 활성화된 Windows 10 또는 Windows 11 (WSL 2 권장).
- 최소 하나 이상의 WSL 배포판이 설치되어 있거나, 새 배포판을 설치할 수 있는 권한이 있어야 합니다.
- 64비트 CPU, 여러 배포판을 원활하게 사용하기 위해 4GB 이상의 RAM을 권장합니다.

## 📦 설치 가이드

### 옵션 1: 빌드된 바이너리 다운로드

가장 쉬운 방법은 미리 컴파일된 릴리스 인스턴스를 사용하는 것입니다.

1. [GitHub Releases](https://github.com/owu/wsl-dashboard/releases) 페이지로 이동합니다.
2. Windows용 최신 `wsldashboard` 실행 파일을 다운로드합니다.
3. 압축을 풀고 `wsldashboard.exe`를 실행합니다.

별도의 설치 프로그램이 필요하지 않으며, 앱은 단일 포터블 바이너리로 제공됩니다.

### 옵션 2: 소스에서 빌드

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
     .\build\scripts\build.ps1
     ```


## 🛠️ 기술 스택 및 성능

- **코어**: 메모리 안전성과 제로 비용 추상화를 위해 Rust로 구현되었습니다.
- **UI 프레임워크**: 고성능 **Skia** 렌더링 백엔드를 사용하는 Slint.
- **비동기 런타임**: 비차단 시스템 명령 및 I/O를 위한 Tokio.
- **성능 하이라이트**:
  - **응답성**: 거의 즉각적인 시작과 실시간 WSL 상태 모니터링을 지원합니다.
  - **효율성**: 초저전력 리소스 사용(자세한 내용은 [주요 기능](#-주요-기능-및-사용법) 참조).
  - **이식성**: 최적화된 릴리스 빌드는 단일 압축 실행 파일을 생성합니다.



## ⭐️ 사랑의 결실

이 프로젝트가 유용하다고 생각하신다면 GitHub에서 별(star)을 남겨주시면 감사하겠습니다. 여러분의 지지는 더 많은 사용자에게 도달하는 데 도움이 되며 진심으로 감사드리고 있습니다. 이러한 격려가 계속해서 개발할 수 있는 동기가 됩니다.

## 📄 라이선스

이 프로젝트는 GPL-3.0 라이선스 하에 배포됩니다. 자세한 내용은 [LICENSE](../LICENSE) 파일을 참조하세요.

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

