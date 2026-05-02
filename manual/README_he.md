# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="לוגו WSL Dashboard" />
</p>

לוח בקרה מודרני, בעל ביצועים גבוהים וקליל לניהול מופעי WSL (Windows Subsystem for Linux). נבנה עם Rust ו-Slint לחוויית שימוש טבעית (native) מובחרת.

---

```diff
הודעה:

- תוכנה זו אינה מופצת דרך Microsoft Store.
- כל יישום המפורט שם באותו שם אינו מורשה ועלול להיות מזויף.
- אנא אל תורידו אותו כדי להימנע מהונאה פוטנציאלית.
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

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردו](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | עברית | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md)

---

## 📑 תוכן עניינים
- [🌍 שפות נתמכות](#-שפות-נתמכות)
- [🚀 תכונות עיקריות ושימוש](#-תכונות-עיקריות-ושימוש)
- [⚙️ הגדרות ויומנים](#️-הגדרות-ויומנים)
- [🖼️ צילומי מסך](#️-צילומי-מסך)
- [🎬 הדגמת פעולה](#-הדגמת-פעולה)
- [💻 דרישות מערכת](#-דרישות-מערכת)
- [📦 מדריך התקנה](#-מדריך-התקנה)
- [🛠️ טכנולוגיה וביצועים](#️-טכנולוגיה-וביצועים)
  - [📄 רישיון](#-רישיון)
- [⭐️ עבודה מאהבה](#️-עבודה-מאהבה)

---

## 🌍 שפות נתמכות

אנגלית, סינית (מפושטת ומסורתית), הינדי, ספרדית, צרפתית, ערבית, בנגלית, פורטוגזית, רוסית, אורדו, אינדונזית, גרמנית, יפנית, טורקית, קוריאנית, איטלקית, הולנדית, שוודית, צ'כית, יוונית, הונגרית, עברית, נורווגית, דנית, פינית, סלובקית, סלובנית, איסלנדית.

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="אנגלית" alt="אנגלית" />
  <img src="../assets/flags/cn.svg" width="32" title="סינית (פשוטה)" alt="סינית (פשוטה)" />
  <img src="../assets/flags/tw.svg" width="32" title="סינית (מסורתית)" alt="סינית (מסורתית)" />
  <img src="../assets/flags/in.svg" width="32" title="הינדי" alt="הינדי" />
  <img src="../assets/flags/es.svg" width="32" title="ספרדית" alt="ספרדית" />
  <img src="../assets/flags/fr.svg" width="32" title="צרפתית" alt="צרפתית" />
  <img src="../assets/flags/sa.svg" width="32" title="ערבית" alt="ערבית" />
  <img src="../assets/flags/bd.svg" width="32" title="בנגלית" alt="בנגלית" />
  <img src="../assets/flags/pt.svg" width="32" title="פורטוגזית" alt="פורטוגזית" />
  <img src="../assets/flags/ru.svg" width="32" title="רוסית" alt="רוסית" />
  <img src="../assets/flags/pk.svg" width="32" title="אורדו" alt="אורדו" />
  <img src="../assets/flags/id.svg" width="32" title="אינדונזית" alt="אינדונזית" />
  <img src="../assets/flags/de.svg" width="32" title="גרמנית" alt="גרמנית" />
  <img src="../assets/flags/jp.svg" width="32" title="יפנית" alt="יפנית" />
  <img src="../assets/flags/tr.svg" width="32" title="טורקית" alt="טורקית" />
  <img src="../assets/flags/kr.svg" width="32" title="קוריאנית" alt="קוריאנית" />
  <img src="../assets/flags/it.svg" width="32" title="איטלקית" alt="איטלקית" />
  <img src="../assets/flags/nl.svg" width="32" title="הולנדית" alt="הולנדית" />
  <img src="../assets/flags/se.svg" width="32" title="שוודית" alt="שוודית" />
  <img src="../assets/flags/cz.svg" width="32" title="צ׳כית" alt="צ׳כית" />
  <img src="../assets/flags/gr.svg" width="32" title="יוונית" alt="יוונית" />
  <img src="../assets/flags/hu.svg" width="32" title="הונגרית" alt="הונגרית" />
  <img src="../assets/flags/il.svg" width="32" title="עברית" alt="עברית" />
  <img src="../assets/flags/no.svg" width="32" title="נורווגית" alt="נורווגית" />
  <img src="../assets/flags/dk.svg" width="32" title="דנית" alt="דנית" />
  <img src="../assets/flags/fi.svg" width="32" title="פינית" alt="פינית" />
  <img src="../assets/flags/sk.svg" width="32" title="סלובקית" alt="סלובקית" />
  <img src="../assets/flags/si.svg" width="32" title="סלובנית" alt="סלובנית" />
  <img src="../assets/flags/is.svg" width="32" title="איסלנדית" alt="איסלנדית" />
</p>


## 🚀 תכונות עיקריות ושימוש

- **ממשק משתמש מודרני**: ממשק גרפי אינטואיטיבי עם תמיכה במצב כהה/בהיר, אנימציות חלקות ורינדור בביצועים גבוהים המופעל על ידי **Skia**.
- **שילוב במגש המערכת (Tray)**: תמיכה מלאה במזעור למגש המערכת (שימוש ב-RAM של כ-10MB), לחיצה כפולה למעבר ותפריט כפתור ימני פונקציונלי.
- **הפעלה חכמה**: הגדר את לוח הבקרה להפעלה עם עליית Windows, מזעור למגש המערכת (מצב שקט עם `/silent`), וכיבוי אוטומטי של הפצות ביציאה.
- **שליטה מקיפה במופעים**: הפעלה, הפסקה, סיום וביטול רישום בלחיצה אחת. ניטור מצב בזמן אמת ותובנות מפורטות לגבי שימוש בדיסק ומיקומי קבצים.
- **ניהול הפצות**: הגדרה כברירת מחדל, הגירה (העברת VHDX לכוננים אחרים), וייצוא/שיבוט לארכייוני `.tar` או `.tar.gz`.
- **אינטגרציה מהירה**: הפעלה מיידית לתוך Terminal, VS Code או סייר הקבצים עם ספריות עבודה ניתנות להתאמה אישית וווקים (hooks) לסקריפטים של הפעלה.
- **התקנה חכמה**: התקנה מחנות Microsoft ה-GitHub, או קבצים מקומיים (RootFS/VHDX). כולל עוזר מובנה להורדת RootFS.
- **בטיחות גלובלית**: נעילות Mutex לפעולות הגירה/גיבוי מקביליות בטוחות וניקוי אוטומטי של Appx בעת הסרה.
- **טביעת רגל זיכרון נמוכה במיוחד**: מותאם במיוחד ליעילות. הפעלה שקטה (במגש המערכת) צורכת רק כ-**10MB** RAM. השימוש במצב חלון משתנה בהתאם למורכבות הגופנים: כ-**18MB** לשפות סטנדרטיות וכ-**38MB** לשפות עם ערכת תווים גדולה (סינית, יפנית, קוריאנית).
- **רשתות מתקדמות**: ניהול חלק של העברת פורטים (עם יצירה אוטומטית של חוקי חומת אש) ותצורת פרוקסי HTTP גלובלית לקישוריות מאוחדת.
- **ניהול התקני USB**: שילוב מלא עם `usbipd-win` לקישור, צירוף וניהול ללא מאמץ של התקני USB מקומיים על פני מופעי ה-WSL שלך ישירות ממשק המשתמש של לוח המחוונים.


## ⚙️ הגדרות ויומנים

כל ההגדרות מנוהלות דרך תצוגת ההגדרות:

- בחירת ספריית התקנה כברירת מחדל עבור מופעי WSL חדשים.
- הגדרת ספריית יומנים ורמת יומן (Error / Warn / Info / Debug / Trace).
- בחירת שפת הממשק או מעקב אחר שפת המערכת.
- החלפה למצב כהה והגדרת כיבוי אוטומטי של WSL לאחר פעולות.
- הגדרת תדירות בדיקת עדכונים (יומית, שבועית, דו-שבועית, חודשית).
- הפעלת הפעלה אוטומטית עם עליית המערכת (עם תיקון נתיב אוטומטי).
- הגדרת האפליקציה למזעור למגש המערכת בזמן ההפעלה.
- הגדרת כפתור הסגירה למזעור למגש המערכת במקום יציאה.
- התאם אישית את סרגל הצד על ידי החלפת נראות של כרטיסיות תכונות מסוימות.

קבצי יומן נכתבים לספריית היומנים שהוגדרה וניתן לצרף אותם בעת דיווח על בעיות.


## 🖼️ צילומי מסך

### דף הבית (מצב בהיר וכהה)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB & תפריט פשוט
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### רֶשֶׁת
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### הוספת מופע והגדרות
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>

### אוֹדוֹת
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
</p>

## 🎬 הדגמת פעולה

להלן הדגמה של WSL Dashboard בפעולה:

![הדגמת WSL Dashboard](../assets/screenshot/demo.gif)



## 💻 דרישות מערכת

- Windows 10 או Windows 11 עם WSL מופעל (מומלץ WSL 2).
- לפחות הפצת WSL אחת מותקנת, או הרשאה להתקנת חדשות.
- מעבד 64 סיביות; מומלץ 4 GB RAM או יותר לשימוש חלק בכמה הפצות.

## 📦 מדריך התקנה

### אפשרות 1: הורדת קובץ בינארי מוכן

הדרך הקלה ביותר להתחיל היא להשתמש בגרסה המקומפלת מראש:

1. עבור לדף ה-[GitHub Releases](https://github.com/owu/wsl-dashboard/releases).
2. הורד את קובץ ההרצה `wsldashboard` העדכני ביותר עבור Windows.
3. חלץ (אם זה בארכיון) והפעל את `wsldashboard.exe`.

אין צורך במתקין; האפליקציה היא קובץ בינארי נייד יחיד.

### אפשרות 2: בנייה מקוד המקור

ודא שמותקנת אצלך ערכת הכלים של Rust (גרסה 1.92 ומעלה).

1. שכפל את המאגר:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. בנה והפעל:

   - לצורכי פיתוח:

     ```powershell
     cargo run
     ```
   - בניית גרסה מותאמת (Release) באמצעות סקריפט הבנייה:

     > סקריפט הבנייה דורש את ערכת הכלים `x86_64-pc-windows-msvc`.

     ```powershell
     .\build\scripts\build.ps1
     ```


## 🛠️ טכנולוגיה וביצועים

- **ליבה**: מיושמת ב-Rust לבטיחות זיכרון והפשטות ללא עלות ביצועים.
- **תשתית ממשק משתמש**: Slint עם מנוע רינדור **Skia** בעל ביצועים גבוהים.
- **זמן ריצה אסינכרוני**: Tokio לפקודות מערכת וקלט/פלט לא חוסמים.
- **דגשי ביצועים**:
  - **תגובתיות**: הפעלה כמעט מיידית וניטור מצב WSL בזמן אמת.
  - **יעילות**: שימוש נמוך במיוחד במשאבים (ראה [תכונות עיקריות](#-תכונות-עיקריות-ושימוש) לפרטים).
  - **ניידות**: בניית ה-Release מפיקה קובץ הרצה קומפקטי יחיד.



## ⭐️ עבודה מאהבה

אם מצאת פרויקט זה שימושי, אודה לך אם תוכל להשאיר כוכב ב-GitHub. התמיכה שלך עוזרת לו להגיע לקהל רחב יותר ומוערכת מאוד. עידוד זה הוא מה שמניע אותי להמשיך לבנות.

## 📄 רישיון

פרויקט זה מופץ תחת רישיון GPL-3.0 – לפרטים נוספים עיין בקובץ ה-[LICENSE](../LICENSE).

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
