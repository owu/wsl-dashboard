# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

Современная, высокопроизводительная и легкая панель управления экземплярами WSL (Windows Subsystem for Linux). Создана с использованием Rust и Slint для премиального нативного опыта.

---

```diff
Внимание:

- Данное программное обеспечение не распространяется через Microsoft Store.
- Любое приложение, указанное там под тем же названием, является несанкционированным и может быть поддельным.
- Пожалуйста, не загружайте его, чтобы избежать возможного мошенничества.
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

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | Русский | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md)

---

## 📑 Оглавление
- [🌍 Поддерживаемые языки](#-поддерживаемые-языки)
- [🚀 Ключевые возможности и использование](#-ключевые-возможности-и-использование)
- [⚙️ Настройки и логи](#️-настройки-и-логи)
- [🖼️ Скриншоты](#️-скриншоты)
- [🎬 Демонстрация работы](#-демонстрация-работы)
- [💻 Системные требования](#-системные-требования)
- [📦 Руководство по установке](#-руководство-по-установке)
- [⭐️ Труд любви](#️-труд-любви)
  - [🛠️ Технологии и производительность](#️-технологии-и-производительность)
  - [📄 Лицензия](#-лицензия)

---

## 🌍 Поддерживаемые языки

Английский, Китайский, Китайский, Хинди, Испанский, Французский, Arabic, Бенгальский, Португальский, Русский, Urdu, Индонезийский, Немецкий, Японский, Турецкий, Korean, Итальянский, Dutch, Swedish, Czech, Greek, Hungarian, Hebrew, Norwegian, Danish, Finnish, Slovak, Slovenian, Icelandic

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="Английский" alt="Английский" />
  <img src="../assets/flags/cn.svg" width="32" title="Китайский (упрощенный)" alt="Китайский (упрощенный)" />
  <img src="../assets/flags/tw.svg" width="32" title="Китайский (традиционный)" alt="Китайский (традиционный)" />
  <img src="../assets/flags/in.svg" width="32" title="Хинди" alt="Хинди" />
  <img src="../assets/flags/es.svg" width="32" title="Испанский" alt="Испанский" />
  <img src="../assets/flags/fr.svg" width="32" title="Французский" alt="Французский" />
  <img src="../assets/flags/sa.svg" width="32" title="Арабский" alt="Арабский" />
  <img src="../assets/flags/bd.svg" width="32" title="Бенгальский" alt="Бенгальский" />
  <img src="../assets/flags/pt.svg" width="32" title="Португальский" alt="Португальский" />
  <img src="../assets/flags/ru.svg" width="32" title="Русский" alt="Русский" />
  <img src="../assets/flags/pk.svg" width="32" title="Урду" alt="Урду" />
  <img src="../assets/flags/id.svg" width="32" title="Индонезийский" alt="Индонезийский" />
  <img src="../assets/flags/de.svg" width="32" title="Немецкий" alt="Немецкий" />
  <img src="../assets/flags/jp.svg" width="32" title="Японский" alt="Японский" />
  <img src="../assets/flags/tr.svg" width="32" title="Турецкий" alt="Турецкий" />
  <img src="../assets/flags/kr.svg" width="32" title="Корейский" alt="Корейский" />
  <img src="../assets/flags/it.svg" width="32" title="Итальянский" alt="Итальянский" />
  <img src="../assets/flags/nl.svg" width="32" title="Нидерландский" alt="Нидерландский" />
  <img src="../assets/flags/se.svg" width="32" title="Шведский" alt="Шведский" />
  <img src="../assets/flags/cz.svg" width="32" title="Чешский" alt="Чешский" />
  <img src="../assets/flags/gr.svg" width="32" title="Греческий" alt="Греческий" />
  <img src="../assets/flags/hu.svg" width="32" title="Венгерский" alt="Венгерский" />
  <img src="../assets/flags/il.svg" width="32" title="Иврит" alt="Иврит" />
  <img src="../assets/flags/no.svg" width="32" title="Норвежский" alt="Норвежский" />
  <img src="../assets/flags/dk.svg" width="32" title="Датский" alt="Датский" />
  <img src="../assets/flags/fi.svg" width="32" title="Финский" alt="Финский" />
  <img src="../assets/flags/sk.svg" width="32" title="Словацкий" alt="Словацкий" />
  <img src="../assets/flags/si.svg" width="32" title="Словенский" alt="Словенский" />
  <img src="../assets/flags/is.svg" width="32" title="Исландский" alt="Исландский" />
</p>


## 🚀 Ключевые возможности и использование

- **Современный нативный интерфейс**: Интуитивный GUI с поддержкой темной/светлой тем, плавной анимацией и высокопроизводительным рендерингом на базе **Skia**.
- **Интеграция с системным треем**: Полная поддержка сворачивания в трей (потребление ОЗУ ~10 МБ), поддержка двойного клика для переключения и функциональное контекстное меню.
- **Интеллектуальный автозапуск**: Настройка запуска вместе с Windows, запуск в свернутом виде (бесшумный режим с параметром `/silent`) и автоматическое завершение работы дистрибутивов при выходе.
- **Полный контроль экземпляров**: Запуск, остановка, завершение и удаление регистрации в один клик. Мониторинг статуса в реальном времени, детальная информация об использовании диска и расположении файлов.
- **Управление дистрибутивами**: Установка по умолчанию, миграция (перемещение VHDX на другие диски), экспорт и клонирование в форматы `.tar` или `.tar.gz`.
- **Быстрая интеграция**: Мгновенный запуск Терминала, VS Code или Проводника с настраиваемыми рабочими каталогами и хуками сценариев запуска.
- **Умная установка**: Установка из Microsoft Store, GitHub или локальных файлов (RootFS/VHDX). Встроенный помощник по загрузке RootFS.
- **Глобальная безопасность**: Мьютекс-блокировки для безопасных параллельных операций миграции/копирования и автоматическая очистка Appx при удалении.
- **Ультра-низкое потребление памяти**: Максимальная оптимизация. В режиме ожидания в трее используется всего **~10 МБ** ОЗУ. В оконном режиме потребление зависит от сложности шрифтов: **~18 МБ** для стандартных языков (английский, немецкий и т. д.) и **~38 МБ** для языков с большими наборами символов (китайский, японский, корейский).
- **Расширенные сети**: Бесшовное управление переадресацией портов (с автоматическим созданием правил брандмауэра) и глобальная настройка HTTP-прокси для унифицированного подключения.
- **Управление USB-устройствами**: Полная интеграция с `usbipd-win` для легкого подключения, привязки и управления локальными USB-устройствами в ваших экземплярах WSL непосредственно из интерфейса панели управления.


## ⚙️ Настройки и логи

Все настройки управляются через окно «Настройки»:

- Выбор пути установки по умолчанию для новых экземпляров WSL.
- Настройка папки для логов и уровня логирования (Error / Warn / Info / Debug / Trace).
- Выбор языка интерфейса или использование системных настроек.
- Переключение темной темы и автоматическое завершение работы WSL после операций.
- Настройка частоты проверки обновлений (ежедневно, еженедельно, раз в две недели, ежемесячно).
- Включение автозапуска при старте системы (с автоматическим исправлением пути).
- Настройка сворачивания приложения в системный трей при запуске.
- Настройка кнопки закрытия для сворачивания в трей вместо выхода из программы.
- Настройте боковую панель, переключая видимость определенных вкладок функций.

Лог-файлы записываются в настроенную папку и могут быть приложены к сообщениям об ошибках.


## 🖼️ Скриншоты

### Главная (Светлая и Темная темы)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB и сворачиваемое меню
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### сеть
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### Добавление экземпляра и Настройки
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>

### О программе 
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
</p>

## 🎬 Демонстрация работы

Ниже представлена демонстрация WSL Dashboard в действии:

![WSL Dashboard Demo](../assets/screenshot/demo.gif)



## 💻 Системные требования

- Windows 10 или Windows 11 с включенным WSL (рекомендуется WSL 2).
- Минимум один установленный дистрибутив WSL или права на установку новых.
- 64-битный процессор; рекомендуется 4 ГБ ОЗУ или более для плавной работы с несколькими дистрибутивами.

## 📦 Руководство по установке

### Вариант 1: Загрузка готового бинарного файла

Самый простой способ — использовать предварительно скомпилированную версию:

1. Перейдите на страницу [GitHub Releases](https://github.com/owu/wsl-dashboard/releases).
2. Загрузите последнюю версию исполняемого файла `wsldashboard` для Windows.
3. Распакуйте (если файл в архиве) и запустите `wsldashboard.exe`.

Установка не требуется; приложение представляет собой один переносной бинарный файл.

### Вариант 2: Сборка из исходного кода

Убедитесь, что у вас установлена цепочка инструментов Rust (версия 1.92+ или новее).

1. Клонируйте репозиторий:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. Соберите и запустите:

   - Для разработки:

     ```powershell
     cargo run
     ```
   - Сборка оптимизированного релиза с помощью скрипта:

     > Скрипт сборки требует цепочку инструментов `x86_64-pc-windows-msvc`.

     ```powershell
     .\build\scripts\build.ps1
     ```


## 🛠️ Технологии и производительность

- **Ядро**: Реализовано на Rust для безопасности памяти и абстракций с нулевой стоимостью.
- **UI-фреймворк**: Slint с высокопроизводительным бэкендом рендеринга **Skia**.
- **Асинхронный runtime**: Tokio для неблокирующих системных команд и ввода-вывода.
- **Ключевые показатели**:
  - **Отклик**: Практически мгновенный запуск и мониторинг статуса WSL в реальном времени.
  - **Эффективность**: Ультра-низкое потребление ресурсов (подробнее см. в разделе [Ключевые возможности](#-ключевые-возможности-и-использование)).
  - **Портативность**: Оптимизированная сборка генерирует один компактный исполняемый файл.



## ⭐️ Труд любви

Если вы нашли этот проект полезным, я был бы признателен, если бы вы оставили звезду на GitHub. Ваша поддержка помогает охватить более широкую аудиторию и очень ценится. Именно это поощрение мотивирует меня продолжать создание.

## 📄 Лицензия

Проект распространяется по лицензии GPL-3.0 – подробности см. в файле [LICENSE](../LICENSE).

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
