# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

Nowoczesny, wydajny, lekki i oszczędny pulpit zarządzania instancjami WSL (Windows Subsystem for Linux). Zbudowany w Rust i Slint, zapewniający doskonałe natywne doświadczenie.

---

```diff
Ogłoszenie:

- WSL Dashboard nie jest dystrybuowany za pośrednictwem Microsoft Store.
- Wszelkie aplikacje wymienione tam pod nazwą "WSL Dashboard" są nieautoryzowane i mogą być fałszywe.
- Proszę go nie pobierać, aby uniknąć potencjalnych oszustw.
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

I18N : [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | Polski | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 Spis treści
- [🌍 Obsługiwane języki](#-obsługiwane-języki)
- [🚀 Kluczowe funkcje i użytkowanie](#-kluczowe-funkcje-i-użytkowanie)
- [⚙️ Konfiguracja i logi](#️-konfiguracja-i-logi)
- [🖼️ Zrzuty ekranu](#️-zrzuty-ekranu)
- [🎬 Demonstracja](#-demonstracja)
- [💻 Wymagania systemowe](#-wymagania-systemowe)
- [📦 Instalacja](#-instalacja)
- [🛠️ Stos technologiczny i wydajność](#️-stos-technologiczny-i-wydajność)
- [🤝 Wsparcie społeczności](#-wsparcie-społeczności)
- [❤️ Wsparcie projektu](#️-wsparcie-projektu)
- [⭐️ Gwiazdka wsparcia](#️-gwiazdka-wsparcia)
- [📄 Licencja](#-licencja)

---

## 🌍 Obsługiwane języki

Angielski, Chiński uproszczony, Chiński tradycyjny, Hindi, Hiszpański, Francuski, Arabski, Bengalski, Portugalski, Rosyjski, Urdu, Indonezyjski, Niemiecki, Japoński, Turecki, Koreański, Włoski, Holenderski, Szwedzki, Czeski, Grecki, Węgierski, Hebrajski, Norweski, Duński, Fiński, Słowacki, Słoweński, Islandzki, Wietnamski, Telugu, Jawajski, Tajski, Tamil, Filipiński, Pendżabski, Malajski, Polski, Ukraiński, Perski, Kannada, Marathi, Hausa, Birmański, Uzbecki, Azerski, Cebuano, Malajalam, Sindhi, Amharski

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="Angielski" alt="Angielski" />
  <img src="../assets/flags/cn.svg" width="32" title="Chiński uproszczony" alt="Chiński uproszczony" />
  <img src="../assets/flags/tw.svg" width="32" title="Chiński tradycyjny" alt="Chiński tradycyjny" />
  <img src="../assets/flags/in.svg" width="32" title="Hindi" alt="Hindi" />
  <img src="../assets/flags/es.svg" width="32" title="Hiszpański" alt="Hiszpański" />
  <img src="../assets/flags/fr.svg" width="32" title="Francuski" alt="Francuski" />
  <img src="../assets/flags/sa.svg" width="32" title="Arabski" alt="Arabski" />
  <img src="../assets/flags/bd.svg" width="32" title="Bengalski" alt="Bengalski" />
  <img src="../assets/flags/pt.svg" width="32" title="Portugalski" alt="Portugalski" />
  <img src="../assets/flags/ru.svg" width="32" title="Rosyjski" alt="Rosyjski" />
  <img src="../assets/flags/pk.svg" width="32" title="Urdu" alt="Urdu" />
  <img src="../assets/flags/id.svg" width="32" title="Indonezyjski" alt="Indonezyjski" />
  <img src="../assets/flags/de.svg" width="32" title="Niemiecki" alt="Niemiecki" />
  <img src="../assets/flags/jp.svg" width="32" title="Japoński" alt="Japoński" />
  <img src="../assets/flags/tr.svg" width="32" title="Turecki" alt="Turecki" />
  <img src="../assets/flags/kr.svg" width="32" title="Koreański" alt="Koreański" />
  <img src="../assets/flags/it.svg" width="32" title="Włoski" alt="Włoski" />
  <img src="../assets/flags/nl.svg" width="32" title="Holenderski" alt="Holenderski" />
  <img src="../assets/flags/se.svg" width="32" title="Szwedzki" alt="Szwedzki" />
  <img src="../assets/flags/cz.svg" width="32" title="Czeski" alt="Czeski" />
  <img src="../assets/flags/gr.svg" width="32" title="Grecki" alt="Grecki" />
  <img src="../assets/flags/hu.svg" width="32" title="Węgierski" alt="Węgierski" />
  <img src="../assets/flags/il.svg" width="32" title="Hebrajski" alt="Hebrajski" />
  <img src="../assets/flags/no.svg" width="32" title="Norweski" alt="Norweski" />
  <img src="../assets/flags/dk.svg" width="32" title="Duński" alt="Duński" />
  <img src="../assets/flags/fi.svg" width="32" title="Fiński" alt="Fiński" />
  <img src="../assets/flags/sk.svg" width="32" title="Słowacki" alt="Słowacki" />
  <img src="../assets/flags/si.svg" width="32" title="Słoweński" alt="Słoweński" />
  <img src="../assets/flags/is.svg" width="32" title="Islandzki" alt="Islandzki" />
  <img src="../assets/flags/vn.svg" width="32" title="Wietnamski" alt="Wietnamski" />
  <img src="../assets/flags/in.svg" width="32" title="Telugu" alt="Telugu" />
  <img src="../assets/flags/id.svg" width="32" title="Javanese" alt="Javanese" />
  <img src="../assets/flags/th.svg" width="32" title="Tajski" alt="Tajski" />
  <img src="../assets/flags/in.svg" width="32" title="Tamil" alt="Tamil" />
  <img src="../assets/flags/ph.svg" width="32" title="Filipiński" alt="Filipiński" />
  <img src="../assets/flags/pk.svg" width="32" title="Punjabi" alt="Punjabi" />
  <img src="../assets/flags/my.svg" width="32" title="Malajski" alt="Malajski" />
  <img src="../assets/flags/pl.svg" width="32" title="Polski" alt="Polski" />
  <img src="../assets/flags/ua.svg" width="32" title="Ukraiński" alt="Ukraiński" />
  <img src="../assets/flags/ir.svg" width="32" title="Perski" alt="Perski" />
  <img src="../assets/flags/in.svg" width="32" title="Kannada" alt="Kannada" />
  <img src="../assets/flags/in.svg" width="32" title="Marathi" alt="Marathi" />
  <img src="../assets/flags/ng.svg" width="32" title="Hausa" alt="Hausa" />
  <img src="../assets/flags/mm.svg" width="32" title="Burmese" alt="Burmese" />
  <img src="../assets/flags/uz.svg" width="32" title="Uzbek" alt="Uzbek" />
  <img src="../assets/flags/az.svg" width="32" title="Azerbaijani" alt="Azerbaijani" />
  <img src="../assets/flags/ph.svg" width="32" title="Cebuano" alt="Cebuano" />
  <img src="../assets/flags/in.svg" width="32" title="Malayalam" alt="Malayalam" />
  <img src="../assets/flags/pk.svg" width="32" title="Sindhi" alt="Sindhi" />
  <img src="../assets/flags/et.svg" width="32" title="Amharic" alt="Amharic" />
</p>


## 🚀 Kluczowe funkcje i użytkowanie

- **Nowoczesny natywny interfejs**: Intuicyjne GUI z obsługą trybu ciemnego/jasnego, płynne animacje i wydajne renderowanie napędzane przez **Skia**.
- **Integracja z tacą systemową**: Pełne wsparcie tacy systemowej (zużycie pamięci ~10MB), podwójne kliknięcie do pokazania/ukrycia oraz w pełni funkcjonalne menu kontekstowe.
- **Inteligentne uruchamianie**: Obsługa autostartu, minimalizacji do tacy (cichy start z parametrem `/silent`) oraz automatycznego wyłączania dystrybucji przy zamknięciu.
- **Kompleksowa kontrola instancji**: Uruchamianie, zatrzymywanie, wymuszanie zatrzymania i wyrejestrowanie jednym kliknięciem. Monitorowanie statusu w czasie rzeczywistym, szczegółowe informacje o użyciu dysku i lokalizacji plików.
- **Zarządzanie dystrybucjami**: Ustawianie jako domyślnej, migracja fizyczna (przenoszenie VHDX na inny dysk) oraz eksport/klonowanie do `.tar` lub `.tar.gz`.
- **Szybka integracja**: Otwieranie terminala, VS Code lub eksploratora plików jednym kliknięciem, z obsługą niestandardowych katalogów roboczych i skryptów uruchomieniowych.
- **Instalacja dystrybucji**: Instaluj dystrybucje Linuxa przez Microsoft Store, GitHub, pliki lokalne (RootFS/VHDX) lub lustra online (z automatycznym testem prędkości wyboru najszybszego lustra i wbudowanym asystentem pobierania RootFS).
- **Globalne bezpieczeństwo**: Mutexy zapewniające bezpieczeństwo jednoczesnych operacji migracji/kopii zapasowych oraz automatyczne czyszczenie pakietów Appx przy usuwaniu.
- **Ekstremalnie niskie zużycie pamięci**: Wysoce zoptymalizowana efektywność zasobów. Cichy start (taca systemowa) zużywa tylko około **10MB** pamięci. W trybie okienkowym zużycie wynosi około **18MB** (standardowe języki, np. angielski, niemiecki) do **38MB** (duże zestawy znaków, np. CJK) w zależności od złożoności czcionki.
- **Zaawansowane zarządzanie siecią**: Bezproblemowe zarządzanie przekierowaniem portów (automatyczne tworzenie reguł zapory sieciowej) oraz globalną konfiguracją proxy HTTP dla zunifikowanego doświadczenia połączenia.
- **Zarządzanie urządzeniami USB**: Głęboka integracja z `usbipd-win`, umożliwiająca łatwe wiązanie, dołączanie i zarządzanie lokalnymi urządzeniami USB dla wszystkich instancji WSL bezpośrednio z interfejsu pulpitu.


## ⚙️ Konfiguracja i logi

Wszystkimi konfiguracjami zarządza się z widoku "Ustawienia":

- Wybór domyślnego katalogu instalacji dla nowych instancji WSL.
- Konfiguracja katalogu logów i poziomu logowania (Error / Warn / Info / Debug / Trace).
- Wybór języka interfejsu lub podążanie za językiem systemowym.
- Przełączanie trybu ciemnego oraz opcji automatycznego wyłączania WSL po operacjach.
- Konfiguracja częstotliwości sprawdzania aktualizacji (codziennie, co tydzień, co dwa tygodnie, co miesiąc).
- Włączenie autostartu (z funkcją automatycznej naprawy ścieżki).
- Ustawienie minimalizacji do tacy przy starcie.
- Konfiguracja zachowania przycisku zamykania (minimalizacja do tacy zamiast zamykania programu).
- Dostosowanie paska bocznego przez przełączanie widoczności poszczególnych kart funkcji.

Pliki logów są zapisywane w skonfigurowanym katalogu logów i można je dołączać przy zgłaszaniu problemów.


## 🖼️ Zrzuty ekranu

### Ekran główny (tryb ciemny & jasny)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB & Zwinięte menu
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### Zarządzanie siecią
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### Dodawanie instancji & Ustawienia
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>
<p align="center">
  <img src="../assets/screenshot/settings-advanced.png" width="48%" />
  <img src="../assets/screenshot/settings-interface.png" width="48%" />
</p>

### O programie & Darowizny
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/donate.png" width="48%" />
</p>

## 🎬 Demonstracja

[Pomóż nam się ulepszyć! Obejrzyj nasz filmik wprowadzający i podziel się swoją opinią.](https://github.com/owu/wsl-dashboard/discussions/9)



## 💻 Wymagania systemowe

- Windows 10 lub Windows 11 z włączonym WSL (zalecane WSL 2).
- Przynajmniej jedna zainstalowana dystrybucja WSL lub uprawnienia do instalacji nowej.
- Procesor 64-bitowy; zalecane 4 GB RAM lub więcej dla płynnego użytkowania wielu dystrybucji.

## 📦 Instalacja

### Metoda 1: Pobranie gotowego pliku binarnego

Najprostszym sposobem na rozpoczęcie jest użycie gotowej wersji:

1. Przejdź do strony [GitHub Releases](https://github.com/owu/wsl-dashboard/releases).
2. Pobierz najnowszy plik wykonywalny `wsldashboard` dla Windows.
3. Rozpakuj (jeśli to archiwum) i uruchom `wsldashboard.exe`.

Instalacja nie jest wymagana — aplikacja jest przenośnym programem jednoplikowym.

### Metoda 2: Kompilacja ze źródeł

Upewnij się, że masz zainstalowany Rust toolchain (Rust 1.92+ lub nowszy).

1. Sklonuj repozytorium:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. Skompiluj i uruchom:

   - Dla dewelopów i debugowania:

     ```powershell
     cargo run
     ```
   - Zoptymalizowana kompilacja wersji wydania za pomocą skryptu build:

     > Skrypt build wymaga toolchain `x86_64-pc-windows-msvc`.

     ```powershell
     .\build\portable\build.ps1
     ```


## 🛠️ Stos technologiczny i wydajność

- **Rdzeń**: Zaimplementowany w Rust, zapewniający bezpieczeństwo pamięci i zero-cost abstraction.
- **Framework UI**: Slint z wydajnym backendem renderowania **Skia**.
- **Async runtime**: Tokio, do nieblokujących poleceń systemowych i operacji I/O.
- **Wyróżniki wydajności**:
  - **Szybkość reakcji**: Niemal natychmiastowy start, monitorowanie statusu WSL w czasie rzeczywistym.
  - **Efektywność zasobów**: Ekstremalnie niskie zużycie zasobów (patrz [Kluczowe funkcje](#-kluczowe-funkcje-i-użytkowanie)).
  - **Przenośność**: Zoptymalizowana wersja wydania generuje jeden kompaktowy plik wykonywalny.



## 🤝 Wsparcie społeczności

Serdeczne podziękowania dla następujących społeczności za ich wsparcie:

- [Rust Programming Language](https://www.rust-lang.org) - Potężny i bezpieczny język programowania
- [Slint | Declarative GUI for Rust, C++, JavaScript & Python](https://slint.dev) - Nowoczesny framework UI
- [WSL: Windows Subsystem for Linux](https://github.com/microsoft/WSL) - Doskonały Windows Subsystem for Linux
- [Tokio - An asynchronous Rust runtime](https://tokio.rs) - Wydajne async runtime
- [Windows Developer Community](https://developer.microsoft.com/en-us/windows/community) - Ciągłe udoskonalanie platformy
- [Reddit](https://www.reddit.com) - Globalne dyskusje i wsparcie społeczności
- [Hacker News](https://news.ycombinator.com) - Globalne dyskusje i wsparcie społeczności
- [Linux.do](https://linux.do) - Popularna społeczność specjalistów IT
- [V2EX](https://www.v2ex.com) - Dyskusje chińskiej społeczności technologicznej

Twój wkład i opinie sprawiają, że ten projekt jest możliwy!


## ❤️ Wsparcie projektu

- Ten projekt jest na licencji GPL-3.0 i jest dostępny bezpłatnie dla wszystkich użytkowników.
- Od rozwoju funkcji, codziennych testów po poprawki błędów — cała praca wykonywana jest w wolnym czasie. Droga open source nie jest łatwa do przejścia w pojedynkę. Twoje uznanie i wsparcie daje projektowi siłę do dalszego rozwoju.
- Jeśli to narzędzie naprawdę Ci pomogło, rozważ wsparcie. Wszystkie darowizny przeznaczane są na koszty serwera, rozwój wersji i udoskonalanie funkcji, aby projekt był na bieżąco aktualizowany i stabilnie się rozwijał.
- Każda mała życzliwość jest promieniem gwiazdy. Jeszcze raz dziękujemy za Twoje zrozumienie i hojność!

Odwiedź naszą stronę darowizn: [https://www.wslui.com/donate/](https://www.wslui.com/donate/)


## ⭐️ Gwiazdka wsparcia

Jeśli uważasz, że ten projekt jest pomocny, będę wdzięczny za gwiazdkę na GitHubie. Twoje wsparcie pomoże projektowi dotrzeć do szerszego grona użytkowników i jest bardzo doceniane. To właśnie ta motywacja napędza mnie do ciągłego rozwoju.


## 📄 Licencja

Ten projekt jest udostępniony na licencji GPL-3.0 – szczegóły w pliku [LICENSE](../LICENSE).


---

Built with ❤️ for the WSL Community.
