# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

Современная, высокопроизводительная, легкая и экономящая память панель управления экземплярами WSL (подсистема Windows для Linux). Создана на Rust и Slint для обеспечения первоклассного нативного опыта.

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

README I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | Русский | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 Оглавление
[🌍 Поддерживаемые языки](#-поддерживаемые-языки) | [🚀 Ключевые возможности и использование](#-ключевые-возможности-и-использование) | [🖼️ Скриншоты](#️-скриншоты) | [📦 Руководство по установке](#-руководство-по-установке) | [📢 Официальные сообщества](#-официальные-сообщества) | [❤️ Поддержать этот проект](#️-поддержать-этот-проект) | [⭐️ Труд любви](#️-труд-любви) | [📄 Лицензия](#-лицензия)

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
- **Планирование задач**: Гибкое управление планированием задач, поддерживающее автоматическое выполнение скриптов и команд в экземплярах WSL в заданное время или с заданными интервалами для автоматизированных операций.
- **Полный контроль экземпляров**: Запуск, остановка, завершение и удаление регистрации в один клик. Мониторинг статуса в реальном времени, детальная информация об использовании диска и расположении файлов.
- **Управление дистрибутивами**: Установка по умолчанию, миграция (перемещение VHDX на другие диски), экспорт и клонирование в форматы `.tar` или `.tar.gz`.
- **Монтирование дисков**: Прямое монтирование и управление VHDX-виртуальными дисками экземпляров WSL, поддержка операций монтирования, размонтирования и управления дисками, упрощающее передачу файлов между хостом и WSL.
- **Быстрая интеграция**: Мгновенный запуск Терминала, VS Code или Проводника с настраиваемыми рабочими каталогами и хуками сценариев запуска.
- **Установка дистрибутива**: Установка дистрибутивов Linux через Microsoft Store, GitHub, локальные файлы (RootFS/VHDX) или онлайн-зеркала (с автоматическим тестированием скорости для выбора самого быстрого зеркала и встроенным помощником загрузки RootFS).
- **Глобальная безопасность**: Мьютекс-блокировки для безопасных параллельных операций миграции/копирования и автоматическая очистка Appx при удалении.
- **Ультра-низкое потребление памяти**: Максимальная оптимизация. В режиме ожидания в трее используется всего **~10 МБ** ОЗУ. В оконном режиме потребление зависит от сложности шрифтов: **~18 МБ** для стандартных языков (английский, немецкий и т. д.) и **~38 МБ** для языков с большими наборами символов (китайский, японский, корейский).
- **Расширенные сети**: Бесшовное управление переадресацией портов (с автоматическим созданием правил брандмауэра) и глобальная настройка HTTP-прокси для унифицированного подключения.
- **Управление USB-устройствами**: Полная интеграция с `usbipd-win` для легкого подключения, привязки и управления локальными USB-устройствами в ваших экземплярах WSL непосредственно из интерфейса панели управления.
- **Конфигурация и журналы**: В «Настройках» управляйте каталогом установки, каталогом и уровнем журналов, языком интерфейса, тёмной темой, частотой проверки обновлений, автозапуском и поведением при закрытии, а также боковой панелью.
- **Поддержание дистрибутива активным**: Одно нажатие «Запустить» оставляет дистрибутив работающим вместо завершения через ~60 с простоя; также можно включить «Запускать дистрибутив при старте Windows».
- **Глобальные горячие клавиши**: Назначьте глобальные сочетания для показа/скрытия главного окна или мгновенного открытия терминала нужного дистрибутива.


## 🖼️ Скриншоты
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

Хотите увидеть панель в работе? Посетите [официальный сайт](https://www.wslui.com/), чтобы посмотреть полную галерею скриншотов.

## 📦 Руководство по установке

### Вариант 1: Посетить веб-сайт проекта (Рекомендуется)

Рекомендуем посетить официальный сайт для загрузки, так как он предлагает несколько зеркальных ссылок для более удобного использования:

Перейдите на [страницу загрузки](https://www.wslui.com/download/) и выберите зеркало, подходящее для вашего региона.

### Вариант 2: Установка через winget

Вы можете установить WSLDashboard напрямую из Windows Package Manager (winget), используя моникер или полный идентификатор пакета:

```powershell
# Search (case-insensitive)
winget search wsl-dashboard
# or
winget search WSLDashboard

# Install (pick one)
winget install wsl-dashboard
# or
winget install Owu.WSLDashboard
```

> Идентификатор пакета winget — `Owu.WSLDashboard`, а моникер — `wsl-dashboard` (без учета регистра). Подойдет любой из них.

Для получения дополнительной информации посетите [сообщество WinGet](https://github.com/microsoft/winget-pkgs/tree/master/manifests/o/Owu/WSLDashboard).

### Вариант 3: Сборка из исходного кода

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
     .\build\portable\build.ps1 -Arch x64
     ```


## 📢 Официальные сообщества

Добро пожаловать в официальные сообщества WSL UI! Присоединяйтесь к нашим каналам, чтобы обсуждать советы по использованию, сообщать о проблемах, получать последние обновления и общаться с разработчиками и другими пользователями.

<div align="center">

| Глобальная группа (Telegram) | Китайская группа (WeChat) |
| :---: | :---: |
| <img src="https://cdn1.wslui.com/static/images/group/group-telegram.png" width="240" alt="Telegram Group QR Code" /> | <img src="https://cdn1.wslui.com/static/images/group/group-wechat.png" width="240" alt="WeChat Group QR Code" /> |
| Отсканируйте для вступления | Отсканируйте для вступления |

</div>


## ❤️ Поддержать этот проект

- Этот проект распространяется по лицензии GPL-3.0 и бесплатен для всех пользователей.
- От разработки функций и ежедневного тестирования до исправления ошибок — вся работа выполняется в свободное время. Путь открытого источника нелегок в одиночку — ваше признание и поддержка дают проекту уверенность в продолжении.
- Если этот инструмент действительно помог вам, рассмотрите возможность поддержки. Все пожертвования идут на расходы сервера, обновления версий и улучшение функций, обеспечивая постоянное обновление и стабильное развитие проекта.
- Каждый маленький добрый жест — это луч света звёзд. Ещё раз спасибо за ваше понимание и щедрость！ [Посетите нашу страницу пожертвований](https://www.wslui.com/donate/).


## ⭐️ Труд любви

Если вы нашли этот проект полезным, я был бы признателен, если бы вы оставили звезду на GitHub. Ваша поддержка помогает охватить более широкую аудиторию и очень ценится. Именно это поощрение мотивирует меня продолжать создание.


## 📄 Лицензия

Проект распространяется по лицензии GPL-3.0 – подробности см. в файле [LICENSE](../LICENSE).
