# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="شعار WSL Dashboard" />
</p>

لوحة تحكم حديثة وعالية الأداء وخفيفة الوزن ومنخفضة الذاكرة لإدارة مثيلات WSL (النظام الفرعي لـ Windows لنظام Linux). تم إنشاؤه باستخدام Rust و Slint للحصول على تجربة أصلية مميزة.

```diff
تنبيه:

- WSL Dashboard لا يتم توزيعه عبر متجر Microsoft.
- أي تطبيق مدرج هناك باسم "WSL Dashboard" غير مصرح به وقد يكون مزيفاً.
- يرجى عدم تنزيله لتجنب الاحتيال المحتمل.
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

README I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | العربية | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 جدول المحتويات
[🌍 اللغات المدعومة](#-اللغات-المدعومة) | [🚀 الميزات الرئيسية والاستخدام](#-الميزات-الرئيسية-والاستخدام) | [🖼️ لقطات الشاشة](#️-لقطات-الشاشة) | [📦 دليل التثبيت](#-دليل-التثبيت) | [📢 مجموعات المجتمع الرسمية](#-مجموعات-المجتمع-الرسمية) | [❤️ دعم هذا المشروع](#️-دعم-هذا-المشروع) | [⭐️ عمل نابع من الحب](#️-عمل-نابع-من-الحب) | [📄 الترخيص](#-الترخيص)

---

## 🌍 اللغات المدعومة

الإنجليزية، الصينية المبسطة، الصينية التقليدية، الهندية، الإسبانية، الفرنسية، العربية، البنغالية، البرتغالية، الروسية، الأردية، الإندونيسية، الألمانية، اليابانية، التركية، الكورية، الإيطالية، الهولندية، السويدية، التشيكية، اليونانية، المجرية، العبرية، النرويجية، الدنماركية، الفنلندية، السلوفاكية، السلوفينية، الأيسلندية، الفيتنامية، التيلوغوية، الجاوية، التايلاندية، التاميلية، الفلبينية، البنجابية، الملاوية، البولندية، الأوكرانية، الفارسية، الكانادية، الماراثية، الهوسا، البورمية، الأوزبكية، الأذربيجانية، السيبوانية، المالايالامية، السندية، الأمهرية.

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="الإنجليزية" alt="الإنجليزية" />
  <img src="../assets/flags/cn.svg" width="32" title="الصينية المبسطة" alt="الصينية المبسطة" />
  <img src="../assets/flags/tw.svg" width="32" title="الصينية التقليدية" alt="الصينية التقليدية" />
  <img src="../assets/flags/in.svg" width="32" title="الهندية" alt="الهندية" />
  <img src="../assets/flags/es.svg" width="32" title="الإسبانية" alt="الإسبانية" />
  <img src="../assets/flags/fr.svg" width="32" title="الفرنسية" alt="الفرنسية" />
  <img src="../assets/flags/sa.svg" width="32" title="العربية" alt="العربية" />
  <img src="../assets/flags/bd.svg" width="32" title="البنغالية" alt="البنغالية" />
  <img src="../assets/flags/pt.svg" width="32" title="البرتغالية" alt="البرتغالية" />
  <img src="../assets/flags/ru.svg" width="32" title="الروسية" alt="الروسية" />
  <img src="../assets/flags/pk.svg" width="32" title="الأردية" alt="الأردية" />
  <img src="../assets/flags/id.svg" width="32" title="الإندونيسية" alt="الإندونيسية" />
  <img src="../assets/flags/de.svg" width="32" title="الألمانية" alt="الألمانية" />
  <img src="../assets/flags/jp.svg" width="32" title="اليابانية" alt="اليابانية" />
  <img src="../assets/flags/tr.svg" width="32" title="التركية" alt="التركية" />
  <img src="../assets/flags/kr.svg" width="32" title="الكورية" alt="الكورية" />
  <img src="../assets/flags/it.svg" width="32" title="الإيطالية" alt="الإيطالية" />
  <img src="../assets/flags/nl.svg" width="32" title="الهولندية" alt="الهولندية" />
  <img src="../assets/flags/se.svg" width="32" title="السويدية" alt="السويدية" />
  <img src="../assets/flags/cz.svg" width="32" title="التشيكية" alt="التشيكية" />
  <img src="../assets/flags/gr.svg" width="32" title="اليونانية" alt="اليونانية" />
  <img src="../assets/flags/hu.svg" width="32" title="المجرية" alt="المجرية" />
  <img src="../assets/flags/il.svg" width="32" title="العبرية" alt="العبرية" />
  <img src="../assets/flags/no.svg" width="32" title="النرويجية" alt="النرويجية" />
  <img src="../assets/flags/dk.svg" width="32" title="الدنماركية" alt="الدنماركية" />
  <img src="../assets/flags/fi.svg" width="32" title="الفنلندية" alt="الفنلندية" />
  <img src="../assets/flags/sk.svg" width="32" title="السلوفاكية" alt="السلوفاكية" />
  <img src="../assets/flags/si.svg" width="32" title="السلوفينية" alt="السلوفينية" />
  <img src="../assets/flags/is.svg" width="32" title="الأيسلندية" alt="الأيسلندية" />
  <img src="../assets/flags/vn.svg" width="32" title="الفيتنامية" alt="الفيتنامية" />
  <img src="../assets/flags/in.svg" width="32" title="التيلوغوية" alt="التيلوغوية" />
  <img src="../assets/flags/id.svg" width="32" title="الجاوية" alt="الجاوية" />
  <img src="../assets/flags/th.svg" width="32" title="التايلاندية" alt="التايلاندية" />
  <img src="../assets/flags/in.svg" width="32" title="التاميلية" alt="التاميلية" />
  <img src="../assets/flags/ph.svg" width="32" title="الفلبينية" alt="الفلبينية" />
  <img src="../assets/flags/pk.svg" width="32" title="البنجابية" alt="البنجابية" />
  <img src="../assets/flags/my.svg" width="32" title="الملاوية" alt="الملاوية" />
  <img src="../assets/flags/pl.svg" width="32" title="البولندية" alt="البولندية" />
  <img src="../assets/flags/ua.svg" width="32" title="الأوكرانية" alt="الأوكرانية" />
  <img src="../assets/flags/ir.svg" width="32" title="الفارسية" alt="الفارسية" />
  <img src="../assets/flags/in.svg" width="32" title="الكانادية" alt="الكانادية" />
  <img src="../assets/flags/in.svg" width="32" title="الماراثية" alt="الماراثية" />
  <img src="../assets/flags/ng.svg" width="32" title="الهوسا" alt="الهوسا" />
  <img src="../assets/flags/mm.svg" width="32" title="البورمية" alt="البورمية" />
  <img src="../assets/flags/uz.svg" width="32" title="الأوزبكية" alt="الأوزبكية" />
  <img src="../assets/flags/az.svg" width="32" title="الأذربيجانية" alt="الأذربيجانية" />
  <img src="../assets/flags/ph.svg" width="32" title="السيبوانية" alt="السيبوانية" />
  <img src="../assets/flags/in.svg" width="32" title="المالايالامية" alt="المالايالامية" />
  <img src="../assets/flags/pk.svg" width="32" title="السندية" alt="السندية" />
  <img src="../assets/flags/et.svg" width="32" title="الأمهرية" alt="الأمهرية" />
</p>

## 🚀 الميزات الرئيسية والاستخدام

- **واجهة مستخدم عصرية**: واجهة رسومية بديهية مع دعم الوضعين الداكن والفاتح، ورسوم متحركة سلسة، ورندرة عالية الأداء مدعومة بمحرك **Skia**.
- **تكامل مع منطقة الإشعارات (Tray)**: دعم كامل للتصغير إلى منطقة الإشعارات (استهلاك ذاكرة ~10 ميجابايت)، والنقر المزدوج للتبديل، وقائمة سياق وظيفية.
- **بدء تشغيل ذكي**: قم بتهيئة لوحة التحكم لتبدأ مع ويندوز، والتصغير إلى منطقة الإشعارات (الوضع الصامت باستخدام `/silent`)، والإغلاق التلقائي للتوزيعات عند الخروج.
- **جدولة المهام**: إدارة مرنة لجدولة المهام، تدعم التنفيذ التلقائي للبرامج النصية والأوامر في مثيلات WSL في أوقات أو فترات زمنية محددة للعمليات الآلية.
- **تحكم شامل في المثيلات**: بدء، إيقاف، إنهاء، وإلغاء تسجيل بنقرة واحدة. مراقبة الحالة في الوقت الفعلي ومعلومات مفصلة حول استخدام القرص ومواقع الملفات.
- **إدارة التوزيعات**: تعيين كافتراضي، ترحيل (نقل ملف VHDX إلى أقراص أخرى)، وتصدير/نسخ إلى أرشيفات `.tar` أو `.tar.gz`.
- **تثبيت القرص**: قم بتثبيت وإدارة أقراص VHDX الافتراضية لمثيلات WSL مباشرة، مع دعم عمليات التثبيت والفك وإدارة الأقراص، مما يسهل نقل الملفات بين المضيف وWSL.
- **تكامل سريع**: تشغيل فوري في Terminal أو VS Code أو مستكشف الملفات مع أدلة عمل قابلة للتخصيص وخطافات لنصوص بدء التشغيل.
- **تثبيت التوزيعة**: قم بتثبيت توزيعات Linux عبر Microsoft Store أو GitHub أو الملفات المحلية (RootFS/VHDX) أو المرايا عبر الإنترنت (مع اختبار السرعة التلقائي لاختيار أسرع مرآة ومساعد تحميل RootFS مدمج).
- **أمان شامل**: أقفال Mutex لعمليات الترحيل/النسخ الاحتياطي المتزامنة الآمنة وتنظيف تلقائي لـ Appx عند الإزالة.
- **بصمة ذاكرة منخفضة للغاية**: محسن للغاية لتحقيق الكفاءة. يستهلك بدء التشغيل الصامت (في منطقة الإشعارات) حوالي **10 ميجابايت** فقط من الذاكرة. يختلف الاستهلاك في وضع النافذة حسب تعقيد الخط: **~18 ميجابايت** للغات القياسية و **~38 ميجابايت** للغات ذات مجموعات المحارف الكبيرة (الصينية، اليابانية، الكورية).
- **الشبكات المتقدمة**: إدارة سلسة لإعادة توجيه المنافذ (مع إنشاء قواعد جدار حماية تلقائياً) وتكوين وكيل HTTP عالمي لتحقيق اتصال موحد.
- **إدارة أجهزة USB**: تكامل كامل مع `usbipd-win` لربط وإرفاق وإدارة أجهزة USB المحلية بسهولة عبر مثيلات WSL الحالية مباشرة من واجهة المستخدم.
- **الإعدادات والسجلات**: إدارة المجلد الافتراضي للتثبيت، ومجلد السجلات ومستواها، ولغة الواجهة، والوضع الداكن، وتكرار فحص التحديثات، والتشغيل التلقائي وسلوك الإغلاق، وتخصيص الشريط الجانبي من صفحة الإعدادات.
- **إبقاء التوزيعة نشطة**: نقرة واحدة على «تشغيل» تُبقي التوزيعة قيد التشغيل بدلاً من إيقافها بعد نحو 60 ثانية من الخمول؛ ويمكنك تمكين «تشغيل هذه التوزيعة عند بدء Windows».
- **اختصارات عالمية**: حدّد اختصارات عالمية لإظهار/إخفاء النافذة الرئيسية أو فتح طرفية توزيعة محددة فورًا.


## 🖼️ لقطات الشاشة
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

هل تريد رؤية لوحة التحكم أثناء العمل؟ تفضل بزيارة [الموقع الرسمي](https://www.wslui.com/) للاطلاع على معرض لقطات الشاشة الكامل.

## 📦 دليل التثبيت

### الخيار 1: زيارة موقع المشروع (موصى به)

نوصي بزيارة الموقع الرسمي للتنزيل، حيث يوفر روابط مرايا متعددة لتجربة أكثر سلاسة:

تفضل بزيارة [صفحة التحميل](https://www.wslui.com/download/) واختر المرآة المناسبة لمنطقتك.

### الخيار 2: التثبيت عبر winget

يمكنك تثبيت WSLDashboard مباشرة من Windows Package Manager (winget)، باستخدام الاسم المختصر أو معرف الحزمة الكامل:

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

> معرف حزمة winget هو `Owu.WSLDashboard` والاسم المختصر هو `wsl-dashboard` (غير حساس لحالة الأحرف). أي منهما يعمل.

لمزيد من المعلومات، تفضل بزيارة [مستودع WinGet المجتمعي](https://github.com/microsoft/winget-pkgs/tree/master/manifests/o/Owu/WSLDashboard).

### الخيار 3: البناء من المصدر

تأكد من تثبيت أدوات Rust (Rust 1.92+ أو أحدث).

1. استنساخ المستودع:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. البناء والتشغيل:

   - للتطوير:

     ```powershell
     cargo run
     ```
   - بناء إصدار محسن باستخدام سكريبت البناء:

     > يتطلب سكريبت البناء توفر أدوات `x86_64-pc-windows-msvc`.

     ```powershell
     .\build\portable\build.ps1 -Arch x64
     ```


## 📢 مجموعات المجتمع الرسمية

مرحبًا بكم في مجموعات مجتمع WSL UI الرسمية! انضم إلى قنواتنا لمناقشة نصائح الاستخدام والإبلاغ عن المشكلات والحصول على آخر التحديثات والتواصل مع المطورين والمستخدمين الآخرين.

<div align="center">

| المجموعة العالمية (Telegram) | المجموعة الصينية (WeChat) |
| :---: | :---: |
| <img src="https://cdn1.wslui.com/static/images/group/group-telegram.png" width="240" alt="Telegram Group QR Code" /> | <img src="https://cdn1.wslui.com/static/images/group/group-wechat.png" width="240" alt="WeChat Group QR Code" /> |
| امسح للانضمام | امسح للانضمام |

</div>


## ❤️ دعم هذا المشروع

- هذا المشروع مرخص بموجب GPL-3.0 ومجاني لجميع المستخدمين.
- من تطوير الميزات والاختبار اليومي إلى إصلاح الأخطاء، يتم كل العمل في وقت الفراغ. طريق المصادر المفتوحة ليس سهلاً بمفردك — تقديرك和支持ك يمنح المشروع الثقة للاستمرار.
- إذا ساعدك هذا الأداة حقاً، فكّر في مد يد العون. جميع التبرعات تذهب نحو تكاليف الخادم وتحديثات الإصدارات وتحسين الميزات، مما يحافظ على تحديث المشروع باستمرار وتقدمه المستقر.
- كل لطف صغير هو شعاع من ضوء النجوم. شكراً لك مرة أخرى على تفهمك وكرمك！ [قم بزيارة صفحة التبرع](https://www.wslui.com/donate/).


## ⭐️ عمل نابع من الحب

إذا وجدت هذا المشروع مفيدًا، سأكون ممتنًا لو تركت نجمة على GitHub. يساعد دعمك في وصوله إلى جمهور أوسع وهو محل تقدير كبير. هذا التشجيع هو ما يحفزني على مواصلة البناء.


## 📄 الترخيص

هذا المشروع مرخص بموجب رخصة GPL-3.0 - انظر ملف [LICENSE](../LICENSE) لمزيد من التفاصيل.
