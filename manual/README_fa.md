# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

داشبورد مدیریت نمونه‌های WSL (Windows Subsystem for Linux) مدرن، با کارایی بالا، سبک و با مصرف حافظه پایین. ساخته شده با Rust و Slint برای ارائه تجربه بومی عالی.

---

```diff
توجه:

- این نرم‌افزار از طریق Microsoft Store توزیع نمی‌شود.
- هر برنامه‌ای با همین نام که در آنجا فهرست شده باشد، نرم‌افزار غیرمجاز و دزدی است.
- لطفاً برای جلوگیری از کلاهبرداری، آن را دانلود نکنید.
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

I18N : [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | فارسی | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 فهرست
- [🌍 پشتیبانی زبان](#-پشتیبانی-زبان)
- [🚀 ویژگی‌های اصلی و استفاده](#-ویژگی‌های-اصلی-و-استفاده)
- [⚙️ پیکربندی و گزارش‌ها](#️-پیکربندی-و-گزارش‌ها)
- [🖼️ تصاویر صفحه](#️-تصاویر-صفحه)
- [🎬 نمایش عملیات](#-نمایش-عملیات)
- [💻 الزامات سیستم](#-الزامات-سیستم)
- [📦 راهنمای نصب](#-راهنمای-نصب)
- [🛠️ فناوری و عملکرد](#️-فناوری-و-عملکرد)
- [🤝 حمایت جامعه](#-حمایت-جامعه)
- [❤️ حمایت از پروژه](#️-حمایت-از-پروژه)
- [⭐️ ستاره حمایت](#️-ستاره-حمایت)
- [📄 مجوز متن‌باز](#-مجوز-متن‌باز)

---

## 🌍 پشتیبانی زبان

انگلیسی، چینی ساده‌شده، چینی سنتی، هندی، اسپانیایی، فرانسوی، عربی، بنگالی، پرتغالی، روسی، اردو، اندونزیایی، آلمانی، ژاپنی، ترکی، کره‌ای، ایتالیایی، هلندی، سوئدی، چکی، یونانی، مجارستانی، عبری، نروژی، دانمارکی، فنلاندی، اسلواکی، اسلوونیایی، ایسلندی، ویتنامی، تلوگو، جاوه‌ای، تایلندی، تامیلی، فیلیپینی، پنجابی، مالایی، لهستانی، اوکراینی، فارسی، کانادا، مراتی، هائوسا، برمه‌ای، ازبکی، آذربایجانی، سبوانو، مالایالام، سندی، امهری

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="انگلیسی" alt="انگلیسی" />
  <img src="../assets/flags/cn.svg" width="32" title="چینی ساده‌شده" alt="چینی ساده‌شده" />
  <img src="../assets/flags/tw.svg" width="32" title="چینی سنتی" alt="چینی سنتی" />
  <img src="../assets/flags/in.svg" width="32" title="هندی" alt="هندی" />
  <img src="../assets/flags/es.svg" width="32" title="اسپانیایی" alt="اسپانیایی" />
  <img src="../assets/flags/fr.svg" width="32" title="فرانسوی" alt="فرانسوی" />
  <img src="../assets/flags/sa.svg" width="32" title="عربی" alt="عربی" />
  <img src="../assets/flags/bd.svg" width="32" title="بنگالی" alt="بنگالی" />
  <img src="../assets/flags/pt.svg" width="32" title="پرتغالی" alt="پرتغالی" />
  <img src="../assets/flags/ru.svg" width="32" title="روسی" alt="روسی" />
  <img src="../assets/flags/pk.svg" width="32" title="اردو" alt="اردو" />
  <img src="../assets/flags/id.svg" width="32" title="اندونزیایی" alt="اندونزیایی" />
  <img src="../assets/flags/de.svg" width="32" title="آلمانی" alt="آلمانی" />
  <img src="../assets/flags/jp.svg" width="32" title="ژاپنی" alt="ژاپنی" />
  <img src="../assets/flags/tr.svg" width="32" title="ترکی" alt="ترکی" />
  <img src="../assets/flags/kr.svg" width="32" title="کره‌ای" alt="کره‌ای" />
  <img src="../assets/flags/it.svg" width="32" title="ایتالیایی" alt="ایتالیایی" />
  <img src="../assets/flags/nl.svg" width="32" title="هلندی" alt="هلندی" />
  <img src="../assets/flags/se.svg" width="32" title="سوئدی" alt="سوئدی" />
  <img src="../assets/flags/cz.svg" width="32" title="چکی" alt="چکی" />
  <img src="../assets/flags/gr.svg" width="32" title="یونانی" alt="یونانی" />
  <img src="../assets/flags/hu.svg" width="32" title="مجارستانی" alt="مجارستانی" />
  <img src="../assets/flags/il.svg" width="32" title="عبری" alt="عبری" />
  <img src="../assets/flags/no.svg" width="32" title="نروژی" alt="نروژی" />
  <img src="../assets/flags/dk.svg" width="32" title="دانمارکی" alt="دانمارکی" />
  <img src="../assets/flags/fi.svg" width="32" title="فنلاندی" alt="فنلاندی" />
  <img src="../assets/flags/sk.svg" width="32" title="اسلواکی" alt="اسلواکی" />
  <img src="../assets/flags/si.svg" width="32" title="اسلوونیایی" alt="اسلوونیایی" />
  <img src="../assets/flags/is.svg" width="32" title="ایسلندی" alt="ایسلندی" />
  <img src="../assets/flags/vn.svg" width="32" title="ویتنامی" alt="ویتنامی" />
  <img src="../assets/flags/in.svg" width="32" title="Telugu" alt="Telugu" />
  <img src="../assets/flags/id.svg" width="32" title="Javanese" alt="Javanese" />
  <img src="../assets/flags/th.svg" width="32" title="تایلندی" alt="تایلندی" />
  <img src="../assets/flags/in.svg" width="32" title="Tamil" alt="Tamil" />
  <img src="../assets/flags/ph.svg" width="32" title="فیلیپینی" alt="فیلیپینی" />
  <img src="../assets/flags/pk.svg" width="32" title="Punjabi" alt="Punjabi" />
  <img src="../assets/flags/my.svg" width="32" title="مالایی" alt="مالایی" />
  <img src="../assets/flags/pl.svg" width="32" title="لهستانی" alt="لهستانی" />
  <img src="../assets/flags/ua.svg" width="32" title="اوکراینی" alt="اوکراینی" />
  <img src="../assets/flags/ir.svg" width="32" title="فارسی" alt="فارسی" />
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


## 🚀 ویژگی‌های اصلی و استفاده

- **رابط بومی مدرن**: GUI بصری با پشتیبانی از حالت تاریک/روشن، انیمیشن‌های روان و رندرینگ با کارایی بالا توسط **Skia**.
- **ادغام با سینی سیستم**: پشتیبانی کامل از سینی (~10 مگابایت مصرف حافظه)، دابل‌کلیک برای نمایش/مخفی کردن و منوی راست‌کلیک با تمام قابلیت‌ها.
- **راه‌اندازی هوشمند**: پشتیبانی از راه‌اندازی خودکار، حداقل‌سازی به سینی (راه‌اندازی بی‌صدا با پارامتر `/silent`) و بستن خودکار توزیع‌ها هنگام خروج.
- **کنترل جامع نمونه‌ها**: شروع، توقف، توقف اجباری و لغو ثبت با یک کلیک. نظارت بر وضعیت در زمان واقعی، جزئیات استفاده از دیسک و مکان فایل.
- **مدیریت توزیع**: تنظیم به عنوان پیش‌فرض، مهاجرت فیزیکی (انتقال VHDX به دیسک دیگر) و خروجی/کلون به `.tar` یا `.tar.gz`.
- **ادغام سریع**: باز کردن ترمینال، VS Code یا مدیر فایل با یک کلیک، با پشتیبانی از دایرکتوری کاری سفارشی و هوک‌های اسکریپت راه‌اندازی.
- **نصب توزیع**: نصب توزیع‌های لینوکس از طریق Microsoft Store، GitHub، فایل‌های محلی (RootFS/VHDX) یا آینه‌های آنلاین (با تست سرعت خودکار برای انتخاب سریع‌ترین آینه و دستیار دانلود RootFS داخلی).
- **امنیت سراسری**: قفل‌های mutex برای امنیت عملیات همزمان مهاجرت/پشتیبان‌گیری و پاکسازی خودکار بسته‌های Appx هنگام حذف.
- **مصرف حافظه بسیار پایین**: بهینه‌سازی بالای منابع. راه‌اندازی بی‌صدا (سینی سیستم) تنها حدود **10 مگابایت** حافظه مصرف می‌کند. در حالت پنجره، بسته به پیچیدگی فونت، حدود **18 مگابایت** (زبان‌های استاندارد مانند انگلیسی، آلمانی و غیره) تا **38 مگابایت** (مجموعه کاراکترهای بزرگ مانند CJK) مصرف می‌شود.
- **مدیریت شبکه پیشرفته**: مدیریت یکپارچه ارسال پورت (ایجاد خودکار قوانین فایروال) و پیکربندی پراکسی HTTP سراسری برای تجربه اتصال یکپارچه.
- **مدیریت دستگاه USB**: ادغام عمیق با `usbipd-win`، که امکان اتصال، پیوست و مدیریت آسان دستگاه‌های USB محلی را برای همه نمونه‌های WSL مستقیماً از رابط داشبورد فراهم می‌کند.


## ⚙️ پیکربندی و گزارش‌ها

تمام پیکربندی‌ها از طریق نمای "تنظیمات" مدیریت می‌شوند:

- انتخاب دایرکتوری نصب پیش‌فرض برای نمونه‌های جدید WSL.
- پیکربندی دایرکتوری گزارش و سطح گزارش (Error / Warn / Info / Debug / Trace).
- انتخاب زبان رابط یا پیروی از زبان سیستم.
- تغییر حالت تاریک و انتخاب بسته خودکار WSL پس از عملیات.
- پیکربندی فرکانس بررسی به‌روزرسانی (روزانه، هفتگی، دو هفتگی، ماهانه).
- فعال‌سازی راه‌اندازی خودکار (با قابلیت تعمیر خودکار مسیر).
- تنظیم حداقل‌سازی به سینی هنگام راه‌اندازی.
- پیکربندی رفتار دکمه بستن (حداقل‌سازی به سینی به جای خروج از برنامه).
- سفارشی‌سازی نوار کناری با تغییر نمایش تب‌های ویژگی خاص.

فایل‌های گزارش در دایرکتوری گزارش پیکربندی شده نوشته می‌شوند و می‌توان هنگام گزارش مشکلات پیوست کرد.


## 🖼️ تصاویر صفحه

### صفحه اصلی (حالت تاریک و روشن)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB و منوی جمع‌شده
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### مدیریت شبکه
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### افزودن نمونه و تنظیمات
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>
<p align="center">
  <img src="../assets/screenshot/settings-advanced.png" width="48%" />
  <img src="../assets/screenshot/settings-interface.png" width="48%" />
</p>

### درباره و اهدا
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/donate.png" width="48%" />
</p>

## 🎬 نمایش عملیات

[به ما کمک کنید بهتر شویم! ویدیوی معرفی ما را تماشا کنید و نظرات خود را به اشتراک بگذارید.](https://github.com/owu/wsl-dashboard/discussions/9)



## 💻 الزامات سیستم

- Windows 10 یا Windows 11 با WSL فعال (WSL 2 توصیه می‌شود).
- حداقل یک توزیع WSL نصب شده، یا مجوز نصب توزیع جدید.
- CPU 64 بیتی؛ 4 گیگابایت رم یا بیشتر برای استفاده روان از چندین توزیع توصیه می‌شود.

## 📦 راهنمای نصب

### روش 1: دانلود باینری از پیش کامپایل شده

ساده‌ترین راه برای شروع استفاده از نسخه کامپایل شده است:

1. به صفحه [GitHub Releases](https://github.com/owu/wsl-dashboard/releases) بروید.
2. آخرین فایل اجرایی `wsldashboard` برای Windows را دانلود کنید.
3. استخراج کنید (اگر آرشیو است) و `wsldashboard.exe` را اجرا کنید.

نیازی به نصب نیست — برنامه یک نرم‌افزار قابل حمل تک‌فایلی است.

### روش 2: ساخت از کد منبع

مطمئن شوید که Rust toolchain (Rust 1.92+ یا جدیدتر) را نصب کرده‌اید.

1. مخزن را کلون کنید:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. بسازید و اجرا کنید:

   - برای توسعه و اشکال‌زدایی:

     ```powershell
     cargo run
     ```
   - ساخت نسخه انتشار بهینه‌شده با اسکریپت بیلد:

     > اسکریپت بیلد به toolchain `x86_64-pc-windows-msvc` نیاز دارد.

     ```powershell
     .\build\portable\build.ps1
      ```


## 🛠️ فناوری و عملکرد

- **هسته**: پیاده‌سازی شده با Rust برای امنیت حافظه و zero-cost abstraction.
- **فریمورک UI**: Slint با بک‌اند رندرینگ **Skia** با کارایی بالا.
- **Async runtime**: Tokio، برای دستورات سیستمی و I/O غیرمسدودکننده.
- **نقاط برجسته عملکرد**:
  - **سرعت پاسخ**: راه‌اندازی تقریباً آنی، نظارت بر وضعیت WSL در زمان واقعی.
  - **بهره‌وری منابع**: مصرف منابع بسیار پایین (به [ویژگی‌های اصلی](#-ویژگی‌های-اصلی-و-استفاده) مراجعه کنید).
  - **قابلیت حمل**: نسخه انتشار بهینه‌شده یک فایل اجرایی فشرده واحد تولید می‌کند.



## 🤝 حمایت جامعه

صمیمانه از جوامع زیر بابت حمایتشان تشکر می‌کنیم:

- [Rust Programming Language](https://www.rust-lang.org) - زبان برنامه‌نویسی قدرتمند و امن
- [Slint | Declarative GUI for Rust, C++, JavaScript & Python](https://slint.dev) - فریمورک UI مدرن
- [WSL: Windows Subsystem for Linux](https://github.com/microsoft/WSL) - Windows Subsystem for Linux عالی
- [Tokio - An asynchronous Rust runtime](https://tokio.rs) - Async runtime کارآمد
- [Windows Developer Community](https://developer.microsoft.com/en-us/windows/community) - بهبود مستمر پلتفرم
- [Reddit](https://www.reddit.com) - بحث و حمایت جامعه جهانی
- [Hacker News](https://news.ycombinator.com) - بحث و حمایت جامعه جهانی
- [Linux.do](https://linux.do) - جامعه محبوب متخصصان فناوری اطلاعات
- [V2EX](https://www.v2ex.com) - بحث جامعه فناوری چینی

مشارکت‌ها و بازخوردهای شما این پروژه را ممکن می‌سازد!


## ❤️ حمایت از پروژه

- این پروژه تحت مجوز GPL-3.0 منتشر شده و برای همه کاربران رایگان است.
- از توسعه ویژگی، تست روزانه تا رفع باگ — تمام کارها در اوقات فراغت انجام می‌شود. مسیر متن‌باز به تنهایی آسان نیست. تأیید و حمایت شما به پروژه انگیزه ادامه می‌دهد.
- اگر این ابزار واقعاً به شما کمک کرده، لطفاً حمایت را در نظر بگیرید. تمام کمک‌ها صرف هزینه سرور، توسعه نسخه و بهبود ویژگی‌ها می‌شود تا پروژه به‌روز و پایدار بماند.
- هر مهربانی کوچکی پرتویی از ستاره است. یک بار دیگر از درک و سخاوت شما متشکریم!

از صفحه اهدای ما دیدن کنید: [https://www.wslui.com/donate/](https://www.wslui.com/donate/)


## ⭐️ ستاره حمایت

اگر این پروژه را مفید می‌دانید، بسیار سپاسگزار خواهم بود اگر بتوانید یک ستاره در GitHub بدهید. حمایت شما به پروژه کمک می‌کند تا به کاربران بیشتری دسترسی پیدا کند و من آن را بسیار ارج می‌نهم. این تشویق همان چیزی است که مرا به ادامه توسعه ترغیب می‌کند.


## 📄 مجوز متن‌باز

این پروژه تحت مجوز GPL-3.0 منتشر شده است – برای جزئیات بیشتر فایل [LICENSE](../LICENSE) را ببینید.


---

Built with ❤️ for the WSL Community.
