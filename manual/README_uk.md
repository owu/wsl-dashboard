# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

Сучасна, високопродуктивна, легка та економна панель керування екземплярами WSL (Windows Subsystem for Linux). Побудована на Rust та Slint для забезпечення чудового нативного досвіду.

---

```diff
Увага:

- Це програмне забезпечення не розповсюджується через Microsoft Store.
- Будь-які додатки з такою ж назвою, розміщені там, є неавторизованим піратським програмним забезпеченням.
- Будь ласка, не завантажуйте їх, щоб уникнути шахрайства.
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

I18N : [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | Українська | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 Зміст
- [🌍 Підтримка мов](#-підтримка-мов)
- [🚀 Основні функції та використання](#-основні-функції-та-використання)
- [⚙️ Конфігурація та журнали](#️-конфігурація-та-журнали)
- [🖼️ Скріншоти](#️-скріншоти)
- [🎬 Демонстрація](#-демонстрація)
- [💻 Системні вимоги](#-системні-вимоги)
- [📦 Посібник з встановлення](#-посібник-з-встановлення)
- [🛠️ Технології та продуктивність](#️-технології-та-продуктивність)
- [🤝 Підтримка спільноти](#-підтримка-спільноти)
- [❤️ Підтримати проєкт](#️-підтримати-проєкт)
- [⭐️ Зірка підтримки](#️-зірка-підтримки)
- [📄 Ліцензія](#-ліцензія)

---

## 🌍 Підтримка мов

Англійська, Китайська (спрощена), Китайська (традиційна), Гінді, Іспанська, Французька, Арабська, Бенгальська, Португальська, Російська, Урду, Індонезійська, Німецька, Японська, Турецька, Корейська, Італійська, Нідерландська, Шведська, Чеська, Грецька, Угорська, Іврит, Норвезька, Данська, Фінська, Словацька, Словенська, Ісландська, В'єтнамська, Телугу, Яванська, Тайська, Тамільська, Філіппінська, Пенджабська, Малайська, Польська, Українська, Перська, Каннада, Маратхі, Хауса, Бірманська, Узбецька, Азербайджанська, Себуано, Малаялам, Сіндхі, Амхарська

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="Англійська" alt="Англійська" />
  <img src="../assets/flags/cn.svg" width="32" title="Китайська (спрощена)" alt="Китайська (спрощена)" />
  <img src="../assets/flags/tw.svg" width="32" title="Китайська (традиційна)" alt="Китайська (традиційна)" />
  <img src="../assets/flags/in.svg" width="32" title="Гінді" alt="Гінді" />
  <img src="../assets/flags/es.svg" width="32" title="Іспанська" alt="Іспанська" />
  <img src="../assets/flags/fr.svg" width="32" title="Французька" alt="Французька" />
  <img src="../assets/flags/sa.svg" width="32" title="Арабська" alt="Арабська" />
  <img src="../assets/flags/bd.svg" width="32" title="Бенгальська" alt="Бенгальська" />
  <img src="../assets/flags/pt.svg" width="32" title="Португальська" alt="Португальська" />
  <img src="../assets/flags/ru.svg" width="32" title="Російська" alt="Російська" />
  <img src="../assets/flags/pk.svg" width="32" title="Урду" alt="Урду" />
  <img src="../assets/flags/id.svg" width="32" title="Індонезійська" alt="Індонезійська" />
  <img src="../assets/flags/de.svg" width="32" title="Німецька" alt="Німецька" />
  <img src="../assets/flags/jp.svg" width="32" title="Японська" alt="Японська" />
  <img src="../assets/flags/tr.svg" width="32" title="Турецька" alt="Турецька" />
  <img src="../assets/flags/kr.svg" width="32" title="Корейська" alt="Корейська" />
  <img src="../assets/flags/it.svg" width="32" title="Італійська" alt="Італійська" />
  <img src="../assets/flags/nl.svg" width="32" title="Нідерландська" alt="Нідерландська" />
  <img src="../assets/flags/se.svg" width="32" title="Шведська" alt="Шведська" />
  <img src="../assets/flags/cz.svg" width="32" title="Чеська" alt="Чеська" />
  <img src="../assets/flags/gr.svg" width="32" title="Грецька" alt="Грецька" />
  <img src="../assets/flags/hu.svg" width="32" title="Угорська" alt="Угорська" />
  <img src="../assets/flags/il.svg" width="32" title="Іврит" alt="Іврит" />
  <img src="../assets/flags/no.svg" width="32" title="Норвезька" alt="Норвезька" />
  <img src="../assets/flags/dk.svg" width="32" title="Данська" alt="Данська" />
  <img src="../assets/flags/fi.svg" width="32" title="Фінська" alt="Фінська" />
  <img src="../assets/flags/sk.svg" width="32" title="Словацька" alt="Словацька" />
  <img src="../assets/flags/si.svg" width="32" title="Словенська" alt="Словенська" />
  <img src="../assets/flags/is.svg" width="32" title="Ісландська" alt="Ісландська" />
  <img src="../assets/flags/vn.svg" width="32" title="В'єтнамська" alt="В'єтнамська" />
  <img src="../assets/flags/in.svg" width="32" title="Telugu" alt="Telugu" />
  <img src="../assets/flags/id.svg" width="32" title="Javanese" alt="Javanese" />
  <img src="../assets/flags/th.svg" width="32" title="Тайська" alt="Тайська" />
  <img src="../assets/flags/in.svg" width="32" title="Tamil" alt="Tamil" />
  <img src="../assets/flags/ph.svg" width="32" title="Філіппінська" alt="Філіппінська" />
  <img src="../assets/flags/pk.svg" width="32" title="Punjabi" alt="Punjabi" />
  <img src="../assets/flags/my.svg" width="32" title="Малайська" alt="Малайська" />
  <img src="../assets/flags/pl.svg" width="32" title="Польська" alt="Польська" />
  <img src="../assets/flags/ua.svg" width="32" title="Українська" alt="Українська" />
  <img src="../assets/flags/ir.svg" width="32" title="Перська" alt="Перська" />
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


## 🚀 Основні функції та використання

- **Сучасний нативний інтерфейс**: Інтуїтивний GUI з підтримкою темної/світлої теми, плавні анімації та високопродуктивний рендеринг на базі **Skia**.
- **Інтеграція з системним треєм**: Повна підтримка трею (~10 МБ оперативної пам'яті), подвійне клацання для показу/приховування та повнофункціональне контекстне меню.
- **Розумний запуск**: Підтримка автозапуску, мінімізація в трей (тихий запуск з параметром `/silent`) та автоматичне вимкнення дистрибутивів при закритті.
- **Комплексне керування екземплярами**: Запуск, зупинка, примусове завершення та скасування реєстрації одним кліком. Моніторинг стану в реальному часі, детальна інформація про використання диску та розташування файлів.
- **Управління дистрибутивами**: Встановлення за замовчуванням, фізична міграція (переміщення VHDX на інший диск) та експорт/клонування у `.tar` або `.tar.gz`.
- **Швидка інтеграція**: Відкриття терміналу, VS Code або файлового менеджера одним кліком, з підтримкою користувацьких робочих каталогів та скриптів запуску.
- **Встановлення дистрибутива**: Встановлюйте дистрибутиви Linux через Microsoft Store, GitHub, локальні файли (RootFS/VHDX) або онлайн-дзеркала (з автоматичним тестуванням швидкості для вибору найшвидшого дзеркала та вбудованим помічником завантаження RootFS).
- **Глобальна безпека**: Використання м'ютексів для безпеки одночасних операцій міграції/резервного копіювання та автоматичне очищення пакетів Appx при видаленні.
- **Надзвичайно низьке споживання пам'яті**: Високооптимізована ефективність ресурсів. Тихий запуск (системний трей) споживає лише близько **10 МБ** пам'яті. У віконному режимі споживання складає близько **18 МБ** (стандартні мови, напр. англійська, німецька) до **38 МБ** (великі набори символів, напр. CJK) залежно від складності шрифту.
- **Розширене мережеве управління**: Безшовне керування переадресацією портів (автоматичне створення правил брандмауера) та глобальною конфігурацією HTTP-проксі для єдиного досвіду підключення.
- **Управління USB-пристроями**: Глибока інтеграція з `usbipd-win`, що дозволяє легко прив'язувати, підключати та керувати локальними USB-пристроями для всіх екземплярів WSL безпосередньо з панелі керування.


## ⚙️ Конфігурація та журнали

Усім конфігураціями керують через вигляд "Налаштування":

- Вибір каталогу встановлення за замовчуванням для нових екземплярів WSL.
- Налаштування каталогу журналів та рівня логування (Error / Warn / Info / Debug / Trace).
- Вибір мови інтерфейсу або відповідно до мови системи.
- Перемикання темної теми та опції автоматичного вимкнення WSL після операцій.
- Налаштування частоти перевірки оновлень (щодня, щотижня, раз на два тижні, щомісяця).
- Увімкнення автозапуску (з функцією автоматичного виправлення шляху).
- Налаштування мінімізації в трей при запуску.
- Налаштування поведінки кнопки закриття (мінімізація в трей замість виходу з програми).
- Налаштування бічної панелі шляхом перемикання видимості окремих вкладок функцій.

Файли журналів записуються до налаштованого каталогу, їх можна додавати до звітів про проблеми.


## 🖼️ Скріншоти

### Головний екран (темна та світла тема)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB та згорнуте меню
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### Управління мережею
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### Додавання екземпляра та Налаштування
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>
<p align="center">
  <img src="../assets/screenshot/settings-advanced.png" width="48%" />
  <img src="../assets/screenshot/settings-interface.png" width="48%" />
</p>

### Про програму та Донати
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/donate.png" width="48%" />
</p>

## 🎬 Демонстрація

[Допоможіть нам стати кращими! Перегляньте наше відео та поділіться своїми думками.](https://github.com/owu/wsl-dashboard/discussions/9)



## 💻 Системні вимоги

- Windows 10 або Windows 11 з увімкненим WSL (рекомендується WSL 2).
- Принаймні один встановлений дистрибутив WSL або дозвіл на встановлення нового.
- 64-бітний процесор; рекомендується 4 ГБ ОЗП або більше для комфортної роботи з кількома дистрибутивами.

## 📦 Посібник з встановлення

### Спосіб 1: Завантаження зібраного бінарного файлу

Найпростіший спосіб почати роботу — використати зібрану версію:

1. Перейдіть на сторінку [GitHub Releases](https://github.com/owu/wsl-dashboard/releases).
2. Завантажте останній виконуваний файл `wsldashboard` для Windows.
3. Розпакуйте (якщо це архів) та запустіть `wsldashboard.exe`.

Встановлення не потрібне — додаток є портативною програмою одним файлом.

### Спосіб 2: Збірка з вихідного коду

Переконайтеся, що у вас встановлений Rust toolchain (Rust 1.92+ або новіший).

1. Клонуйте репозиторій:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. Зберіть та запустіть:

   - Для розробки та налагодження:

     ```powershell
     cargo run
     ```
   - Оптимізована збірка релізної версії за допомогою скрипту:

     > Скрипт збірки потребує toolchain `x86_64-pc-windows-msvc`.

     ```powershell
     .\build\portable\build.ps1
     ```


## 🛠️ Технології та продуктивність

- **Ядро**: Реалізовано на Rust для безпеки пам'яті та zero-cost abstraction.
- **UI-фреймворк**: Slint з високопродуктивним рендеринг-бекендом **Skia**.
- **Async runtime**: Tokio для неблокуючих системних команд та операцій введення/виведення.
- **Переваги продуктивності**:
  - **Швидкість відгуку**: Миттєвий запуск, моніторинг стану WSL в реальному часі.
  - **Ефективність ресурсів**: Надзвичайно низьке споживання ресурсів (див. [Основні функції](#-основні-функції-та-використання)).
  - **Портативність**: Оптимізована релізна збірка генерує один компактний виконуваний файл.



## 🤝 Підтримка спільноти

Щиро дякуємо наступним спільнотам за їхню підтримку:

- [Rust Programming Language](https://www.rust-lang.org) - Потужна та безпечна мова програмування
- [Slint | Declarative GUI for Rust, C++, JavaScript & Python](https://slint.dev) - Сучасний UI-фреймворк
- [WSL: Windows Subsystem for Linux](https://github.com/microsoft/WSL) - Чудовий Windows Subsystem for Linux
- [Tokio - An asynchronous Rust runtime](https://tokio.rs) - Ефективний async runtime
- [Windows Developer Community](https://developer.microsoft.com/en-us/windows/community) - Постійне вдосконалення платформи
- [Reddit](https://www.reddit.com) - Глобальні дискусії та підтримка спільноти
- [Hacker News](https://news.ycombinator.com) - Глобальні дискусії та підтримка спільноти
- [Linux.do](https://linux.do) - Популярна спільнота ІТ-фахівців
- [V2EX](https://www.v2ex.com) - Дискусії китайської технічної спільноти

Ваш внесок та відгуки роблять цей проєкт можливим!


## ❤️ Підтримати проєкт

- Цей проєкт ліцензовано під GPL-3.0 та доступний безкоштовно для всіх користувачів.
- Від розробки функцій, щоденного тестування до виправлення помилок — уся робота виконується у вільний час. Шлях відкритого коду нелегко пройти наодинці. Ваше визнання та підтримка надають проєкту сили продовжувати розвиток.
- Якщо цей інструмент дійсно допоміг вам, будь ласка, розгляньте можливість підтримки. Усі пожертвування спрямовуються на витрати сервера, розробку версій та вдосконалення функцій, щоб проєкт постійно оновлювався та стабільно розвивався.
- Кожна маленька доброта — це промінь зірки. Ще раз дякуємо за ваше розуміння та щедрість!

Відвідайте нашу сторінку пожертвувань: [https://www.wslui.com/donate/](https://www.wslui.com/donate/)


## ⭐️ Зірка підтримки

Якщо ви вважаєте цей проєкт корисним, буду вдячний, якщо ви поставите йому зірку на GitHub. Ваша підтримка допоможе проєкту охопити ширше коло користувачів і я дуже ціную це. Саме це надихає мене продовжувати розвиток.


## 📄 Ліцензія

Цей проєкт випущено під ліцензією GPL-3.0 – дивіться файл [LICENSE](../LICENSE) для деталей.


---

Built with ❤️ for the WSL Community.
