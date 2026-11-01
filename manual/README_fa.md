# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

داشبورد مدیریت نمونه‌های WSL (Windows Subsystem for Linux) مدرن، با کارایی بالا، سبک و با مصرف حافظه پایین. ساخته شده با Rust و Slint برای ارائه تجربه بومی عالی.

```diff
اعلامیه:

- WSL Dashboard از طریق Microsoft Store توزیع نمی‌شود.
- هر برنامه‌ای که در آنجا با نام "WSL Dashboard" فهرست شده است غیرمجاز است و ممکن است جعلی باشد.
- لطفاً آن را دانلود نکنید تا از کلاهبرداری‌های احتمالی جلوگیری شود.
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

README I18N : [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | فارسی | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 فهرست
[🌍 پشتیبانی زبان](#-پشتیبانی-زبان) | [🚀 ویژگی‌های اصلی و استفاده](#-ویژگیهای-اصلی-و-استفاده) | [🖼️ تصاویر صفحه](#️-تصاویر-صفحه) | [📦 راهنمای نصب](#-راهنمای-نصب) | [📢 گروه‌های رسمی جامعه](#-گروههای-رسمی-جامعه) | [❤️ حمایت از پروژه](#️-حمایت-از-پروژه) | [⭐️ ستاره حمایت](#️-ستاره-حمایت) | [📄 مجوز متن‌باز](#-مجوز-متنباز)

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
- **زمان‌بندی وظایف**: مدیریت زمان‌بندی وظایف انعطاف‌پذیر، با پشتیبانی از اجرای خودکار اسکریپت‌ها و دستورات در نمونه‌های WSL در زمان‌ها یا بازه‌های مشخص برای عملیات خودکار.
- **کنترل جامع نمونه‌ها**: شروع، توقف، توقف اجباری و لغو ثبت با یک کلیک. نظارت بر وضعیت در زمان واقعی، جزئیات استفاده از دیسک و مکان فایل.
- **مدیریت توزیع**: تنظیم به عنوان پیش‌فرض، مهاجرت فیزیکی (انتقال VHDX به دیسک دیگر) و خروجی/کلون به `.tar` یا `.tar.gz`.
- **نصب دیسک**: نصب و مدیریت مستقیم دیسک‌های مجازی VHDX نمونه‌های WSL، با پشتیبانی از عملیات نصب، جدا کردن و مدیریت دیسک، برای انتقال آسان فایل‌ها بین میزبان و WSL.
- **ادغام سریع**: باز کردن ترمینال، VS Code یا مدیر فایل با یک کلیک، با پشتیبانی از دایرکتوری کاری سفارشی و هوک‌های اسکریپت راه‌اندازی.
- **نصب توزیع**: نصب توزیع‌های لینوکس از طریق Microsoft Store، GitHub، فایل‌های محلی (RootFS/VHDX) یا آینه‌های آنلاین (با تست سرعت خودکار برای انتخاب سریع‌ترین آینه و دستیار دانلود RootFS داخلی).
- **امنیت سراسری**: قفل‌های mutex برای امنیت عملیات همزمان مهاجرت/پشتیبان‌گیری و پاکسازی خودکار بسته‌های Appx هنگام حذف.
- **مصرف حافظه بسیار پایین**: بهینه‌سازی بالای منابع. راه‌اندازی بی‌صدا (سینی سیستم) تنها حدود **10 مگابایت** حافظه مصرف می‌کند. در حالت پنجره، بسته به پیچیدگی فونت، حدود **18 مگابایت** (زبان‌های استاندارد مانند انگلیسی، آلمانی و غیره) تا **38 مگابایت** (مجموعه کاراکترهای بزرگ مانند CJK) مصرف می‌شود.
- **مدیریت شبکه پیشرفته**: مدیریت یکپارچه ارسال پورت (ایجاد خودکار قوانین فایروال) و پیکربندی پراکسی HTTP سراسری برای تجربه اتصال یکپارچه.
- **مدیریت دستگاه USB**: ادغام عمیق با `usbipd-win`، که امکان اتصال، پیوست و مدیریت آسان دستگاه‌های USB محلی را برای همه نمونه‌های WSL مستقیماً از رابط داشبورد فراهم می‌کند.
- **پیکربندی و گزارش‌ها**: از بخش تنظیمات، پوشه نصب پیش‌فرض، پوشه و سطح گزارش‌ها، زبان رابط، حالت تیره، بازه بررسی به‌روزرسانی، اجرای خودکار و رفتار بستن، و همچنین نوار کناری را مدیریت کنید.
- **زنده نگه داشتن توزیع**: با یک کلیک روی «راه‌اندازی»، توزیع به‌جای خاموش‌شدن پس از حدود ۶۰ ثانیه بیکاری، به کار خود ادامه می‌دهد؛ می‌توانید گزینه «اجرای این توزیع هنگام راه‌اندازی Windows» را نیز فعال کنید.
- **میانبرهای سراسری**: میانبرهای سراسری برای نمایش/پنهان‌کردن پنجره اصلی یا باز کردن فوری ترمینال یک توزیع تعیین کنید.


## 🖼️ تصاویر صفحه
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

می‌خواهید داشبورد را در حال کار ببینید؟ برای دیدن گالری کامل تصاویر صفحه، از [وب‌سایت رسمی](https://www.wslui.com/) بازدید کنید.

## 📦 راهنمای نصب

### گزینه 1: بازدید از وب‌سایت پروژه (توصیه می‌شود)

توصیه می‌کنیم برای دانلود از وب‌سایت رسمی بازدید کنید، زیرا چندین لینک آینه برای تجربه‌ای روان‌تر ارائه می‌دهد:

به [صفحه دانلود](https://www.wslui.com/download/) بروید و آینه مناسب منطقه خود را انتخاب کنید.

### گزینه 2: نصب از طریق winget

می‌توانید WSLDashboard را مستقیماً از Windows Package Manager (winget) نصب کنید، با استفاده از نام مستعار یا شناسه کامل بسته:

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

> The winget package identifier is `Owu.WSLDashboard` and the moniker is `wsl-dashboard` (case-insensitive). Either works.

For more information, visit the [WinGet community repository](https://github.com/microsoft/winget-pkgs/tree/master/manifests/o/Owu/WSLDashboard).

### گزینه 3: ساخت از کد منبع

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
     .\build\portable\build.ps1 -Arch x64
     ```


## 📢 گروه‌های رسمی جامعه

به گروه‌های رسمی جامعه WSL UI خوش آمدید! به کانال‌های ما بپیوندید تا نکات استفاده را بحث کنید، مشکلات را گزارش دهید، آخرین به‌روزرسانی‌ها را دریافت کنید و با توسعه‌دهندگان و کاربران دیگر ارتباط برقرار کنید.

<div align="center">

| گروه جهانی (Telegram) | گروه چینی (WeChat) |
| :---: | :---: |
| <img src="https://cdn1.wslui.com/static/images/group/group-telegram.png" width="240" alt="Telegram Group QR Code" /> | <img src="https://cdn1.wslui.com/static/images/group/group-wechat.png" width="240" alt="WeChat Group QR Code" /> |
| برای عضویت اسکن کنید | برای عضویت اسکن کنید |

</div>


## ❤️ حمایت از پروژه

- این پروژه تحت مجوز GPL-3.0 منتشر شده و برای همه کاربران رایگان است.
- از توسعه ویژگی، تست روزانه تا رفع باگ — تمام کارها در اوقات فراغت انجام می‌شود. مسیر متن‌باز به تنهایی آسان نیست. تأیید و حمایت شما به پروژه انگیزه ادامه می‌دهد.
- اگر این ابزار واقعاً به شما کمک کرده، لطفاً حمایت را در نظر بگیرید. تمام کمک‌ها صرف هزینه سرور، توسعه نسخه و بهبود ویژگی‌ها می‌شود تا پروژه به‌روز و پایدار بماند.
- هر مهربانی کوچکی پرتویی از ستاره است. یک بار دیگر از درک و سخاوت شما متشکریم! [از صفحه اهدای ما دیدن کنید](https://www.wslui.com/donate/).


## ⭐️ ستاره حمایت

اگر این پروژه را مفید می‌دانید، بسیار سپاسگزار خواهم بود اگر بتوانید یک ستاره در GitHub بدهید. حمایت شما به پروژه کمک می‌کند تا به کاربران بیشتری دسترسی پیدا کند و من آن را بسیار ارج می‌نهم. این تشویق همان چیزی است که مرا به ادامه توسعه ترغیب می‌کند.


## 📄 مجوز متن‌باز

این پروژه تحت مجوز GPL-3.0 منتشر شده است – برای جزئیات بیشتر فایل [LICENSE](../LICENSE) را ببینید.
