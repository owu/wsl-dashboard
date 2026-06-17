# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

একটি আধুনিক, উচ্চ-কর্মক্ষমতাসম্পন্ন, হালকা এবং কম-মেমরি যুক্ত WSL (উইন্ডোজ সাবসিস্টেম ফর লিনাক্স) ইনস্ট্যান্স ম্যানেজমেন্ট ড্যাশবোর্ড। প্রিমিয়াম নেটিভ অভিজ্ঞতার জন্য Rust এবং Slint দিয়ে তৈরি।

---

```diff
বিজ্ঞপ্তি:

- WSL Dashboard Microsoft Store এর মাধ্যমে বিতরণ করা হয় না।
- সেখানে "WSL Dashboard" নামে তালিকাভুক্ত যেকোনো অ্যাপ্লিকেশন অনুমোদিত নয় এবং জাল হতে পারে।
- সম্ভাব্য প্রতারণা এড়াতে দয়া করে এটি ডাউনলোড করবেন না।
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

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | বাংলা | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 সূচিপত্র
- [🌍 ভাষা সমর্থন](#-ভাষা-সমর্থন)
- [🚀 মূল বৈশিষ্ট্য এবং ব্যবহার](#-মূল-বৈশিষ্ট্য-এবং-ব্যবহার)
- [⚙️ কনফিগারেশন এবং লগ](#️-কনফিগারেশন-এবং-লগ)
- [🖼️ স্ক্রিনশট](#️-স্ক্রিনশট)
- [🎬 অপারেশন ডেমো](#-অপারেশন-ডেমো)
- [💻 সিস্টেমের প্রয়োজনীয়তা](#-সিস্টেমের-প্রয়োজনীয়তা)
- [📦 ইনস্টলেশন গাইড](#-ইনস্টলেশন-গাইড)
- [🛠️ টেক স্ট্যাক এবং পারফরম্যান্স](#️-টেক-স্ট্যাক-এবং-পারফরম্যান্স)
- [🤝 কমিউনিটি সাপোর্ট](#-কমিউনিটি-সাপোর্ট)
- [❤️ এই প্রকল্পকে সমর্থন করুন](#️-এই-প্রকল্পকে-সমর্থন-করুন)
- [⭐️ ভালবাসার শ্রম](#️-ভালবাসার-শ্রম)
- [📄 লাইসেন্স](#-লাইসেন্স)

---

## 🌍 ভাষা সমর্থন

ইংরেজি, চীনা (সরলীকৃত), চীনা (ঐতিহ্যবাহী), হিন্দি, স্প্যানিশ, ফ্রেঞ্চ, আরবি, বাংলা, পর্তুগিজ, রুশ, উর্দু, ইন্দোনেশীয়, জার্মান, জাপানি, তুর্কি, কোরীয়, ইতালীয়, ডাচ, সুইডিশ, চেক, গ্রিক, হাঙ্গেরীয়, হিব্রু, নরওয়েজীয়, ডেনিশ, ফিনিশ, স্লোভাক, স্লোভেনীয়, আইসল্যান্ডীয়, ভিয়েতনামী, তেলুগু, জাভানি, থাই, তামিল, ফিলিপিনো, পাঞ্জাবি, মালয়, পোলিশ, ইউক্রেনীয়, ফার্সি, কান্নাড়া, মারাঠি, হাউসা, বর্মী, উজবেক, আজারবাইজানি, সেবুয়ানো, মালায়ালাম, সিন্ধি, আমহারিক

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="ইংরেজি" alt="ইংরেজি" />
  <img src="../assets/flags/cn.svg" width="32" title="চীনা (সরলীকৃত)" alt="চীনা (সরলীকৃত)" />
  <img src="../assets/flags/tw.svg" width="32" title="চীনা (ঐতিহ্যবাহী)" alt="চীনা (ঐতিহ্যবাহী)" />
  <img src="../assets/flags/in.svg" width="32" title="হিন্দি" alt="হিন্দি" />
  <img src="../assets/flags/es.svg" width="32" title="স্প্যানিশ" alt="স্প্যানিশ" />
  <img src="../assets/flags/fr.svg" width="32" title="ফ্রেঞ্চ" alt="ফ্রেঞ্চ" />
  <img src="../assets/flags/sa.svg" width="32" title="আরবি" alt="আরবি" />
  <img src="../assets/flags/bd.svg" width="32" title="বাংলা" alt="বাংলা" />
  <img src="../assets/flags/pt.svg" width="32" title="পর্তুগিজ" alt="পর্তুগিজ" />
  <img src="../assets/flags/ru.svg" width="32" title="রুশ" alt="রুশ" />
  <img src="../assets/flags/pk.svg" width="32" title="উর্দু" alt="উর্দু" />
  <img src="../assets/flags/id.svg" width="32" title="ইন্দোনেশীয়" alt="ইন্দোনেশীয়" />
  <img src="../assets/flags/de.svg" width="32" title="জার্মান" alt="জার্মান" />
  <img src="../assets/flags/jp.svg" width="32" title="জাপানি" alt="জাপানি" />
  <img src="../assets/flags/tr.svg" width="32" title="তুর্কি" alt="তুর্কি" />
  <img src="../assets/flags/kr.svg" width="32" title="কোরীয়" alt="কোরীয়" />
  <img src="../assets/flags/it.svg" width="32" title="ইতালীয়" alt="ইতালীয়" />
  <img src="../assets/flags/nl.svg" width="32" title="ডাচ" alt="ডাচ" />
  <img src="../assets/flags/se.svg" width="32" title="সুইডিশ" alt="সুইডিশ" />
  <img src="../assets/flags/cz.svg" width="32" title="চেক" alt="চেক" />
  <img src="../assets/flags/gr.svg" width="32" title="গ্রিক" alt="গ্রিক" />
  <img src="../assets/flags/hu.svg" width="32" title="হাঙ্গেরীয়" alt="হাঙ্গেরীয়" />
  <img src="../assets/flags/il.svg" width="32" title="হিব্রু" alt="হিব্রু" />
  <img src="../assets/flags/no.svg" width="32" title="নরওয়েজীয়" alt="নরওয়েজীয়" />
  <img src="../assets/flags/dk.svg" width="32" title="ডেনিশ" alt="ডেনিশ" />
  <img src="../assets/flags/fi.svg" width="32" title="ফিনিশ" alt="ফিনিশ" />
  <img src="../assets/flags/sk.svg" width="32" title="স্লোভাক" alt="স্লোভাক" />
  <img src="../assets/flags/si.svg" width="32" title="স্লোভেনীয়" alt="স্লোভেনীয়" />
  <img src="../assets/flags/is.svg" width="32" title="আইসল্যান্ডীয়" alt="আইসল্যান্ডীয়" />
  <img src="../assets/flags/vn.svg" width="32" title="ভিয়েতনামী" alt="ভিয়েতনামী" />
  <img src="../assets/flags/in.svg" width="32" title="তেলুগু" alt="তেলুগু" />
  <img src="../assets/flags/id.svg" width="32" title="জাভানি" alt="জাভানি" />
  <img src="../assets/flags/th.svg" width="32" title="থাই" alt="থাই" />
  <img src="../assets/flags/in.svg" width="32" title="তামিল" alt="তামিল" />
  <img src="../assets/flags/ph.svg" width="32" title="ফিলিপিনো" alt="ফিলিপিনো" />
  <img src="../assets/flags/pk.svg" width="32" title="পাঞ্জাবি" alt="পাঞ্জাবি" />
  <img src="../assets/flags/my.svg" width="32" title="মালয়" alt="মালয়" />
  <img src="../assets/flags/pl.svg" width="32" title="পোলিশ" alt="পোলিশ" />
  <img src="../assets/flags/ua.svg" width="32" title="ইউক্রেনীয়" alt="ইউক্রেনীয়" />
  <img src="../assets/flags/ir.svg" width="32" title="ফার্সি" alt="ফার্সি" />
  <img src="../assets/flags/in.svg" width="32" title="কান্নাড়া" alt="কান্নাড়া" />
  <img src="../assets/flags/in.svg" width="32" title="মারাঠি" alt="মারাঠি" />
  <img src="../assets/flags/ng.svg" width="32" title="হাউসা" alt="হাউসা" />
  <img src="../assets/flags/mm.svg" width="32" title="বর্মী" alt="বর্মী" />
  <img src="../assets/flags/uz.svg" width="32" title="উজবেক" alt="উজবেক" />
  <img src="../assets/flags/az.svg" width="32" title="আজারবাইজানি" alt="আজারবাইজানি" />
  <img src="../assets/flags/ph.svg" width="32" title="সেবুয়ানো" alt="সেবুয়ানো" />
  <img src="../assets/flags/in.svg" width="32" title="মালায়ালাম" alt="মালায়ালাম" />
  <img src="../assets/flags/pk.svg" width="32" title="সিন্ধি" alt="সিন্ধি" />
  <img src="../assets/flags/et.svg" width="32" title="আমহারিক" alt="আমহারিক" />
</p>


## 🚀 মূল বৈশিষ্ট্য এবং ব্যবহার

- **আধুনিক নেটিভ ইউজার ইন্টারফেস**: স্বজ্ঞাত GUI, লাইট/ডার্ক মোড সাপোর্ট, মসৃণ অ্যানিমেশন এবং **Skia** চালিত উচ্চ-ক্ষমতাসম্পন্ন রেন্ডারিং।
- **সিস্টেম ট্রে ইন্টিগ্রেশন**: নোটিফিকেশন এরিয়াতে মিনিমাইজ করার পূর্ণ সুবিধা (~১০ এমবি র‌্যাম ব্যবহার), টগল করতে ডাবল-ক্লিক এবং একটি কার্যকর রাইট-ক্লিক মেনু।
- **স্মার্ট স্টার্টআপ**: ড্যাশবোর্ডটি উইন্ডোজের সাথে শুরু হওয়ার জন্য কনফিগার করুন, ট্রেতে মিনিমাইজ করে রাখা (সাইলেন্ট মোড `/silent` সহ) এবং প্রস্থান করার সময় স্বয়ংক্রিয়ভাবে ডিস্ট্রিবিউশন শাটডাউন।
- **পূর্ণাঙ্গ ইনস্ট্যান্স নিয়ন্ত্রণ**: এক ক্লিকেই স্টার্ট, স্টপ, টার্মিনেট এবং আনরেজিস্টার। রিয়েল-টাইম স্ট্যাটাস মনিটরিং এবং ডিস্ক ব্যবহার ও ফাইলের অবস্থান সম্পর্কে বিস্তারিত তথ্য।
- **ডিস্ট্রো ম্যানেজমেন্ট**: ডিফল্ট সেট করা, মাইগ্রেশন (VHDX অন্য ড্রাইভে সরানো) এবং `.tar` বা `.tar.gz` ফরম্যাটে এক্সপোর্ট/ক্লোন।
- **দ্রুত ইন্টিগ্রেশন**: কাস্টম ওয়ার্কিং ডিরেক্টরি এবং স্টার্টআপ স্ক্রিপ্ট হুকসহ তাৎক্ষণিক টার্মিনাল, VS Code বা ফাইল এক্সপ্লোরার লঞ্চ।
- **ডিস্ট্রো ইনস্টলেশন**: Microsoft Store, GitHub, লোকাল ফাইল (RootFS/VHDX), বা অনলাইন মিররের মাধ্যমে Linux ডিস্ট্রিবিউশন ইনস্টল করুন (স্বয়ংক্রিয় গতি পরীক্ষার মাধ্যমে দ্রুততম মিরর নির্বাচন এবং বিল্ট-ইন RootFS ডাউনলোড সহকারী)।
- **গ্লোবাল সিকিউরিটি**: নিরাপদ সমবর্তী মাইগ্রেশন/ব্যাকআপ অপারেশনের জন্য Mutex লক এবং ডিলিট করার সময় স্বয়ংক্রিয় Appx ক্লিনআপ।
- **অত্যন্ত কম মেমরি ব্যবহার**: দক্ষতার জন্য বিশেষভাবে অপ্টিমাইজ করা হয়েছে। সাইলেন্ট স্টার্টআপ (সিস্টেম ট্রে) মাত্র **~১০ এমবি** র‌্যাম ব্যবহার করে। উইন্ডো মোড ব্যবহার ফন্টের জটিলতার ওপর নির্ভর করে: স্ট্যান্ডার্ড ভাষার জন্য **~১৮ এমবি** এবং বড় ক্যারেক্টার সেট যুক্ত ভাষার জন্য (চীনা, জাপানি, কোরিয়ান) **~৩৫ এমবি**।
- **উন্নত নেটওয়ার্কিং**: নির্বিঘ্ন পোর্ট ফরওয়ার্ডিং পরিচালনা (স্বয়ংক্রিয় ফায়ারওয়াল নিয়ম তৈরি সহ) এবং একীভূত সংযোগের জন্য গ্লোবাল HTTP প্রক্সি কনফিগারেশন।
- **USB ডিভাইস ম্যানেজমেন্ট**: ড্যাশবোর্ড UI-এর মাধ্যমে সরাসরি আপনার WSL ইনস্ট্যান্সে লোকাল USB ডিভাইসগুলো সংযুক্ত, ব্যবহার এবং পরিচালনা করার জন্য `usbipd-win`-এর সাথে পূর্ণ ইন্টিগ্রেশন।


## ⚙️ কনফিগারেশন এবং লগ

সেটিংস ভিউয়ের মাধ্যমে সমস্ত কনফিগারেশন পরিচালনা করা হয়:

- নতুন WSL ইনস্ট্যান্সের জন্য ডিফল্ট ইনস্টলেশন ডিরেক্টরি বেছে নিন।
- লগ ডিরেক্টরি এবং লগ লেভেল (Error / Warn / Info / Debug / Trace) কনফিগার করুন।
- UI ভাষা বেছে নিন অথবা এটিকে সিস্টেম ল্যাঙ্গুয়েজ অনুসরণ করতে দিন।
- ডার্ক মোড টগল করুন এবং অপারেশনের পর WSL অটো-শাটডাউন এনাবল করুন।
- অ্যাপটি কতবার আপডেটের জন্য চেক করবে তা কনফিগার করুন (প্রতিদিন, প্রতি সপ্তাহে, প্রতি দুই সপ্তাহে, প্রতি মাসে)।
- সিস্টেম বুট হওয়ার সময় অটো-স্টার্ট এনাবল করুন (অটোমেটিক পাথ রিপেয়ারসহ)।
- স্টার্টআপের সময় অ্যাপটিকে সিস্টেমে ট্রেতে মিনিমাইজ করার জন্য সেট করুন।
- ক্লোজ বোতামটি প্রোগ্রাম থেকে বের হওয়ার বদলে ট্রেতে মিনিমাইজ করার জন্য কনফিগার করুন।
- নির্দিষ্ট বৈশিষ্ট্য ট্যাবের দৃশ্যমানতা টগল করে সাইডবার കাস্টমাইজ করুন।

কনফিগার করা লগ ডিরেক্টরিতে লগ ফাইলগুলো লেখা হয় এবং সমস্যার রিপোর্ট করার সময় এগুলো যুক্ত করা যেতে পারে।


## 🖼️ স্ক্রিনশট

### হোম (ডার্ক এবং লাইট মোড)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB & মেনু সংকুচিত করুন
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### নেটওয়ার্ক
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### ইনস্ট্যান্স যোগ করুন এবং সেটিংস
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>
<p align="center">
  <img src="../assets/screenshot/settings-advanced.png" width="48%" />
  <img src="../assets/screenshot/settings-interface.png" width="48%" />
</p>

### ভূমিকা ও দান
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/donate.png" width="48%" />
</p>

## 🎬 অপারেশন ডেমো

[আমাদের উন্নত করতে সাহায্য করুন! আমাদের পরিচয়মূলক ভিডিও দেখুন এবং আপনার মতামত শেয়ার করুন।](https://github.com/owu/wsl-dashboard/discussions/9)



## 💻 সিস্টেমের প্রয়োজনীয়তা

- WSL এনাবলসহ Windows 10 বা Windows 11 (WSL 2 সুপারিশকৃত)।
- অন্তত একটি WSL ডিস্ট্রিবিউশন ইনস্টল করা থাকতে হবে, অথবা নতুন ইনস্টল করার অনুমতি থাকতে হবে।
- ৬৪-বিট CPU; মসৃণ ব্যবহারের জন্য ৪ জিবি র‌্যাম বা তার বেশি সুপারিশকৃত।

## 📦 ইনস্টলেশন গাইড

### অপশন ১: প্রি-বিল্ট বাইনারি ডাউনলোড করুন

শুরু করার সবচেয়ে সহজ উপায় হলো আগে থেকে কম্পাইল করা রিলিজ ব্যবহার করা:

1. [GitHub Releases](https://github.com/owu/wsl-dashboard/releases) পেজে যান।
2. Windows-এর জন্য সর্বশেষ `wsldashboard` এক্সিকিউটেবল ফাইলটি ডাউনলোড করুন।
3. এক্সট্রাক্ট করুন (যদি সংকুচিত থাকে) এবং `wsldashboard.exe` চালান।

কোনো ইনস্টলারের প্রয়োজন নেই; অ্যাপটি একটি একক পোর্টেবল বাইনারি।

### অপশন ২: সোর্স থেকে বিল্ড করুন

আপনার কাছে Rust টুলচেইন (Rust 1.92+ বা তার নতুন) ইনস্টল করা আছে তা নিশ্চিত করুন।

1. রিপোজিটরি ক্লোন করুন:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. বিল্ড এবং রান করুন:

   - ডেভেলপমেন্টের জন্য:

     ```powershell
     cargo run
     ```
   - স্ক্রিপ্টের মাধ্যমে অপ্টিমাইজড রিলিজ বিল্ড তৈরি করুন:

     > বিল্ড স্ক্রিপ্টের জন্য `x86_64-pc-windows-msvc` টুলচেইন প্রয়োজন।

     ```powershell
     .\build\portable\build.ps1
     ```


## 🛠️ টেক স্ট্যাক এবং পারফরম্যান্স

- **কোর**: মেমরি সুরক্ষা এবং জিরো-কস্ট অ্যাবস্ট্রাকশনের জন্য Rust-এ বাস্তবায়ন করা হয়েছে।
- **UI ফ্রেমওয়ার্ক**: উচ্চ-ক্ষমতাসম্পন্ন **Skia** রেন্ডারিং ব্যাকএন্ড যুক্ত Slint।
- **অ্যাসিঙ্ক রানটাইম**: নন-ব্লকিং সিস্টেম কমান্ড এবং I/O-এর জন্য Tokio।
- **পারফরম্যান্স হাইলাইট**:
  - **রেসপন্সিভনেস**: প্রায় তাৎক্ষণিক স্টার্টআপ এবং রিয়েল-টাইম WSL স্ট্যাটাস মনিটরিং।
  - **দক্ষতা**: অত্যন্ত কম রিসোর্স ব্যবহার (বিস্তারিত জানতে [মূল বৈশিষ্ট্য](#-মূল-বৈশিষ্ট্য-এবং-ব্যবহার) দেখুন)।
  - **পোর্টেবিলিটি**: অপ্টিমাইজড রিলিজ বিল্ড একটি একক কমপ্যাক্ট এক্সিকিউটেবল তৈরি করে।



## 🤝 কমিউনিটি সাপোর্ট

নিম্নলিখিত কমিউনিটিগুলোর সমর্থনের জন্য অনেক ধন্যবাদ:

- [Rust Programming Language](https://www.rust-lang.org) - শক্তিশালী এবং নিরাপদ প্রোগ্রামিং ভাষার জন্য
- [Slint | Declarative GUI for Rust, C++, JavaScript & Python](https://slint.dev) - আধুনিক UI ফ্রেমওয়ার্কের জন্য
- [WSL: Windows Subsystem for Linux](https://github.com/microsoft/WSL) - চমৎকার Windows Subsystem for Linux এর জন্য
- [Tokio - An asynchronous Rust runtime](https://tokio.rs) - দক্ষ অ্যাসিঙ্ক রানটাইমের জন্য
- [Windows Developer Community](https://developer.microsoft.com/en-us/windows/community) - প্ল্যাটফর্মের ক্রমাগত উন্নতির জন্য
- [Reddit](https://www.reddit.com) - বিশ্বব্যাপী কমিউনিটি আলোচনা এবং সমর্থনের জন্য
- [Hacker News](https://news.ycombinator.com) - বিশ্বব্যাপী কমিউনিটি আলোচনা এবং সমর্থনের জন্য
- [Linux.do](https://linux.do) - IT পেশাদারদের জন্য জনপ্রিয় কমিউনিটি
- [V2EX](https://www.v2ex.com) - চীনা প্রযুক্তি কমিউনিটি আলোচনার জন্য

আপনার অবদান এবং প্রতিক্রিয়া এই প্রজেক্টটি সম্ভব করে！


## ❤️ এই প্রকল্পকে সমর্থন করুন

- এই প্রকল্পটি GPL-3.0 ওপেন সোর্স লাইসেন্সের অধীনে এবং সকল ব্যবহারকারীর জন্য বিনামূল্যে।
- ফিচার ডেভেলপমেন্ট, দৈনিক পরীক্ষা থেকে বগ ফিক্স পর্যন্ত, সমস্ত কাজ অবসর সময়ে করা হয়। ওপেন সোর্সের পথে একা চলা সহজ নয় — আপনার স্বীকৃতি ও সমর্থন প্রকল্পকে এগিয়ে যাওয়ার আত্মবিশ্বাস দেয়।
- এই টুলটি যদি সত্যিই আপনাকে সাহায্য করে থাকে, তাহলে সহায়তা করুন। সমস্ত দান সার্ভার খরচ, সংস্করণ আপডেট এবং ফিচার উন্নতিতে ব্যবহৃত হয়, যাতে প্রকল্প ক্রমাগত আপডেট ও স্থিতিশীল অগ্রগতি বজায় রাখে।
- প্রতিটি ছোট দয়ালুতা তারাআলোর একটি রশ্মি। আপনার বোঝাপড়া ও উদারতার জন্য আবারও ধন্যবাদ！

আমাদের ডোনেশন পেজ দেখুন：[https://www.wslui.com/donate/](https://www.wslui.com/donate/)


## ⭐️ ভালবাসার শ্রম

এই প্রজেক্টটি আপনার কাছে কার্যকর মনে হলে, GitHub-এ একটি স্টার দিলে আমি কৃতজ্ঞ হব। আপনার সমর্থন এটিকে আরও ব্যাপক দর্শকের কাছে পৌঁছাতে সাহায্য করে এবং গভীরভাবে প্রশংসিত হয়। এই উৎসাহই আমাকে তৈরি করতে থাকতে অনুপ্রাণিত করে।


## 📄 লাইসেন্স

এই প্রজেক্টটি GPL-3.0 এর অধীনে লাইসেন্সকৃত - বিস্তারিত জানতে [LICENSE](../LICENSE) ফাইলটি দেখুন।


---

Built with ❤️ for the WSL Community.
