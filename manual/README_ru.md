# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

Современная, высокопроизводительная, легкая и экономящая память панель управления экземплярами WSL (подсистема Windows для Linux). Создана на Rust и Slint для обеспечения первоклассного нативного опыта.

---

```diff
Внимание:

- WSL Dashboard не распространяется через Microsoft Store.
- Любое приложение, указанное там под названием "WSL Dashboard", не авторизовано и может быть поддельным.
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

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | Русский | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 Оглавление
- [🌍 Поддерживаемые языки](#-поддерживаемые-языки)
- [🚀 Ключевые возможности и использование](#-ключевые-возможности-и-использование)
- [⚙️ Настройки и логи](#️-настройки-и-логи)
- [🖼️ Скриншоты](#️-скриншоты)
- [🎬 Демонстрация работы](#-демонстрация-работы)
- [💻 Системные требования](#-системные-требования)
- [📦 Руководство по установке](#-руководство-по-установке)
- [🛠️ Технологии и производительность](#️-технологии-и-производительность)
- [🤝 Поддержка сообщества](#-поддержка-сообщества)
- [❤️ Поддержать этот проект](#️-поддержать-этот-проект)
- [⭐️ Труд любви](#️-труд-любви)
- [📄 Лицензия](#-лицензия)

---

## 🌍 Поддерживаемые языки

Английский, Китайский, Китайский, Хинди, Испанский, Французский, Arabic, Бенгальский, Португальский, Русский, Urdu, Индонезийский, Немецкий, Японский, Турецкий, Korean, Итальянский, Dutch, Swedish, Czech, Greek, Hungarian, Hebrew, Norwegian, Danish, Finnish, Slovak, Slovenian, Icelandic, Вьетнамский, Телугу, Яванский, Тайский, Тамильский, Филиппинский, Панджаби, Малайский, Польский, Украинский, Персидский, Каннада, Маратхи, Хауса, Бирманский, Узбекский, Азербайджанский, Себуано, Малаялам, Синдхи, Амхарский

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
  <img src="../assets/flags/vn.svg" width="32" title="Вьетнамский" alt="Вьетнамский" />
  <img src="../assets/flags/in.svg" width="32" title="Телугу" alt="Телугу" />
  <img src="../assets/flags/id.svg" width="32" title="Яванский" alt="Яванский" />
  <img src="../assets/flags/th.svg" width="32" title="Тайский" alt="Тайский" />
  <img src="../assets/flags/in.svg" width="32" title="Тамильский" alt="Тамильский" />
  <img src="../assets/flags/ph.svg" width="32" title="Филиппинский" alt="Филиппинский" />
  <img src="../assets/flags/pk.svg" width="32" title="Панджаби" alt="Панджаби" />
  <img src="../assets/flags/my.svg" width="32" title="Малайский" alt="Малайский" />
  <img src="../assets/flags/pl.svg" width="32" title="Польский" alt="Польский" />
  <img src="../assets/flags/ua.svg" width="32" title="Украинский" alt="Украинский" />
  <img src="../assets/flags/ir.svg" width="32" title="Персидский" alt="Персидский" />
  <img src="../assets/flags/in.svg" width="32" title="Каннада" alt="Каннада" />
  <img src="../assets/flags/in.svg" width="32" title="Маратхи" alt="Маратхи" />
  <img src="../assets/flags/ng.svg" width="32" title="Хауса" alt="Хауса" />
  <img src="../assets/flags/mm.svg" width="32" title="Бирманский" alt="Бирманский" />
  <img src="../assets/flags/uz.svg" width="32" title="Узбекский" alt="Узбекский" />
  <img src="../assets/flags/az.svg" width="32" title="Азербайджанский" alt="Азербайджанский" />
  <img src="../assets/flags/ph.svg" width="32" title="Себуано" alt="Себуано" />
  <img src="../assets/flags/in.svg" width="32" title="Малаялам" alt="Малаялам" />
  <img src="../assets/flags/pk.svg" width="32" title="Синдхи" alt="Синдхи" />
  <img src="../assets/flags/et.svg" width="32" title="Амхарский" alt="Амхарский" />
</p>


## 🚀 Ключевые возможности и использование

- **Современный нативный интерфейс**: Интуитивный GUI с поддержкой темной/светлой тем, плавной анимацией и высокопроизводительным рендерингом на базе **Skia**.
- **Интеграция с системным треем**: Полная поддержка сворачивания в трей (потребление ОЗУ ~10 МБ), поддержка двойного клика для переключения и функциональное контекстное меню.
- **Интеллектуальный автозапуск**: Настройка запуска вместе с Windows, запуск в свернутом виде (бесшумный режим с параметром `/silent`) и автоматическое завершение работы дистрибутивов при выходе.
- **Полный контроль экземпляров**: Запуск, остановка, завершение и удаление регистрации в один клик. Мониторинг статуса в реальном времени, детальная информация об использовании диска и расположении файлов.
- **Управление дистрибутивами**: Установка по умолчанию, миграция (перемещение VHDX на другие диски), экспорт и клонирование в форматы `.tar` или `.tar.gz`.
- **Быстрая интеграция**: Мгновенный запуск Терминала, VS Code или Проводника с настраиваемыми рабочими каталогами и хуками сценариев запуска.
- **Установка дистрибутива**: Установка дистрибутивов Linux через Microsoft Store, GitHub, локальные файлы (RootFS/VHDX) или онлайн-зеркала (с автоматическим тестированием скорости для выбора самого быстрого зеркала и встроенным помощником загрузки RootFS).
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
<p align="center">
  <img src="../assets/screenshot/settings-advanced.png" width="48%" />
  <img src="../assets/screenshot/settings-interface.png" width="48%" />
</p>

### О программе & Пожертвования
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/donate.png" width="48%" />
</p>

## 🎬 Демонстрация работы

[Помогите нам стать лучше! Посмотрите наше вступительное видео и поделитесь мнением.](https://github.com/owu/wsl-dashboard/discussions/9)



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
     .\build\portable\build.ps1
     ```


## 🛠️ Технологии и производительность

- **Ядро**: Реализовано на Rust для безопасности памяти и абстракций с нулевой стоимостью.
- **UI-фреймворк**: Slint с высокопроизводительным бэкендом рендеринга **Skia**.
- **Асинхронный runtime**: Tokio для неблокирующих системных команд и ввода-вывода.
- **Ключевые показатели**:
  - **Отклик**: Практически мгновенный запуск и мониторинг статуса WSL в реальном времени.
  - **Эффективность**: Ультра-низкое потребление ресурсов (подробнее см. в разделе [Ключевые возможности](#-ключевые-возможности-и-использование)).
  - **Портативность**: Оптимизированная сборка генерирует один компактный исполняемый файл.



## 🤝 Поддержка сообщества

Большое спасибо следующим сообществам за их поддержку:

- [Rust Programming Language](https://www.rust-lang.org) - За мощный и безопасный язык программирования
- [Slint | Declarative GUI for Rust, C++, JavaScript & Python](https://slint.dev) - За современный UI-фреймворк
- [WSL: Windows Subsystem for Linux](https://github.com/microsoft/WSL) - За замечательную подсистему Windows для Linux
- [Tokio - An asynchronous Rust runtime](https://tokio.rs) - За эффективный асинхронный рантайм
- [Windows Developer Community](https://developer.microsoft.com/en-us/windows/community) - За постоянные улучшения платформы
- [Reddit](https://www.reddit.com) - За глобальные обсуждения и поддержку сообщества
- [Hacker News](https://news.ycombinator.com) - За глобальные обсуждения и поддержку сообщества
- [Linux.do](https://linux.do) - За популярное сообщество IT-специалистов
- [V2EX](https://www.v2ex.com) - За обсуждения в китайском технологическом сообществе

Ваш вклад и обратная связь делают этот проект возможным!


## ❤️ Поддержать этот проект

- Этот проект распространяется по лицензии GPL-3.0 и бесплатен для всех пользователей.
- От разработки функций и ежедневного тестирования до исправления ошибок — вся работа выполняется в свободное время. Путь открытого источника нелегок в одиночку — ваше признание и поддержка дают проекту уверенность в продолжении.
- Если этот инструмент действительно помог вам, рассмотрите возможность поддержки. Все пожертвования идут на расходы сервера, обновления версий и улучшение функций, обеспечивая постоянное обновление и стабильное развитие проекта.
- Каждый маленький добрый жест — это луч света звёзд. Ещё раз спасибо за ваше понимание и щедрость！

Посетите нашу страницу пожертвований：[https://www.wslui.com/donate/](https://www.wslui.com/donate/)


## ⭐️ Труд любви

Если вы нашли этот проект полезным, я был бы признателен, если бы вы оставили звезду на GitHub. Ваша поддержка помогает охватить более широкую аудиторию и очень ценится. Именно это поощрение мотивирует меня продолжать создание.


## 📄 Лицензия

Проект распространяется по лицензии GPL-3.0 – подробности см. в файле [LICENSE](../LICENSE).


---

Built with ❤️ for the WSL Community.
