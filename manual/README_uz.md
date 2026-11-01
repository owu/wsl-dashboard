# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

Zamonaviy, yuqori samarali, yengil va past xotira iste'moli bilan WSL (Windows Subsystem for Linux) instansiyalarini boshqarish paneli. Rust va Slint asosida qurilgan, ajoyib nativ tajriba taqdim etadi.

```diff
E'lon:

- WSL Dashboard Microsoft Store orqali tarqatilmaydi.
- U yerda "WSL Dashboard" nomi bilan roʻyxatga olingan har qanday dastur ruxsatsiz va soxta boʻlishi mumkin.
- Iltimos, mumkin boʻlgan firibgarliklardan qochish uchun uni yuklab olmang.
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

README I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | Oʻzbek | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 Mundarija
[🌍 Til qo'llab-quvvatlash](#-til-qollab-quvvatlash) | [🚀 Asosiy xususiyatlar va foydalanish](#-asosiy-xususiyatlar-va-foydalanish) | [🖼️ Dastur skrinshotlari](#️-dastur-skrinshotlari) | [📦 O'rnatish bo'yicha qo'llanma](#-ornatish-boyicha-qollanma) | [📢 Rasmiy jamoa guruhlari](#-rasmiy-jamoa-guruhlari) | [❤️ Loyihani qo'llab-quvvatlang](#️-loyihani-qollab-quvvatlang) | [⭐️ Sevgi bilan ishlaydigan loyiha](#️-sevgi-bilan-ishlaydigan-loyiha) | [📄 Ochiq manba litsenziyasi](#-ochiq-manba-litsenziyasi)

---

## 🌍 Til qo'llab-quvvatlash

Ingliz, Sadalashtirilgan Xitoy, An'anaviy Xitoy, Hindi, Ispan, Fransuz, Arab, Bengal, Portugiz, Rus, Urdu, Indoneziya, Nemis, Yapon, Turkiy, Koreya, Italiya, Golland, Shvetsiya, Chex, Gretsiya, Vengriya, Ibroniya, Norvegiya, Daniya, Finlyandiya, Slovak, Sloven, Island, Vetnam, Telugu, Yava, Tailand, Tamil, Filippin, Panjob, Malay, Polyak, Ukraina, Fors, Kannada, Marathi, Hausa, Myanma, O'zbek, Ozarbayjon, Sebuan, Malayalam, Sindhi, Amxar

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="Ingliz" alt="Ingliz" />
  <img src="../assets/flags/cn.svg" width="32" title="Sadalashtirilgan Xitoy" alt="Sadalashtirilgan Xitoy" />
  <img src="../assets/flags/tw.svg" width="32" title="An'anaviy Xitoy" alt="An'anaviy Xitoy" />
  <img src="../assets/flags/in.svg" width="32" title="Hindi" alt="Hindi" />
  <img src="../assets/flags/es.svg" width="32" title="Ispan" alt="Ispan" />
  <img src="../assets/flags/fr.svg" width="32" title="Fransuz" alt="Fransuz" />
  <img src="../assets/flags/sa.svg" width="32" title="Arab" alt="Arab" />
  <img src="../assets/flags/bd.svg" width="32" title="Bengal" alt="Bengal" />
  <img src="../assets/flags/pt.svg" width="32" title="Portugiz" alt="Portugiz" />
  <img src="../assets/flags/ru.svg" width="32" title="Rus" alt="Rus" />
  <img src="../assets/flags/pk.svg" width="32" title="Urdu" alt="Urdu" />
  <img src="../assets/flags/id.svg" width="32" title="Indoneziya" alt="Indoneziya" />
  <img src="../assets/flags/de.svg" width="32" title="Nemis" alt="Nemis" />
  <img src="../assets/flags/jp.svg" width="32" title="Yapon" alt="Yapon" />
  <img src="../assets/flags/tr.svg" width="32" title="Turkiy" alt="Turkiy" />
  <img src="../assets/flags/kr.svg" width="32" title="Koreya" alt="Koreya" />
  <img src="../assets/flags/it.svg" width="32" title="Italiya" alt="Italiya" />
  <img src="../assets/flags/nl.svg" width="32" title="Golland" alt="Golland" />
  <img src="../assets/flags/se.svg" width="32" title="Shvetsiya" alt="Shvetsiya" />
  <img src="../assets/flags/cz.svg" width="32" title="Chex" alt="Chex" />
  <img src="../assets/flags/gr.svg" width="32" title="Gretsiya" alt="Gretsiya" />
  <img src="../assets/flags/hu.svg" width="32" title="Vengriya" alt="Vengriya" />
  <img src="../assets/flags/il.svg" width="32" title="Ibroniya" alt="Ibroniya" />
  <img src="../assets/flags/no.svg" width="32" title="Norvegiya" alt="Norvegiya" />
  <img src="../assets/flags/dk.svg" width="32" title="Daniya" alt="Daniya" />
  <img src="../assets/flags/fi.svg" width="32" title="Finlyandiya" alt="Finlyandiya" />
  <img src="../assets/flags/sk.svg" width="32" title="Slovak" alt="Slovak" />
  <img src="../assets/flags/si.svg" width="32" title="Sloven" alt="Sloven" />
  <img src="../assets/flags/is.svg" width="32" title="Island" alt="Island" />
  <img src="../assets/flags/vn.svg" width="32" title="Vetnam" alt="Vetnam" />
  <img src="../assets/flags/in.svg" width="32" title="Telugu" alt="Telugu" />
  <img src="../assets/flags/id.svg" width="32" title="Yava" alt="Yava" />
  <img src="../assets/flags/th.svg" width="32" title="Tailand" alt="Tailand" />
  <img src="../assets/flags/in.svg" width="32" title="Tamil" alt="Tamil" />
  <img src="../assets/flags/ph.svg" width="32" title="Filippin" alt="Filippin" />
  <img src="../assets/flags/pk.svg" width="32" title="Panjob" alt="Panjob" />
  <img src="../assets/flags/my.svg" width="32" title="Malay" alt="Malay" />
  <img src="../assets/flags/pl.svg" width="32" title="Polyak" alt="Polyak" />
  <img src="../assets/flags/ua.svg" width="32" title="Ukraina" alt="Ukraina" />
  <img src="../assets/flags/ir.svg" width="32" title="Fors" alt="Fors" />
  <img src="../assets/flags/in.svg" width="32" title="Kannada" alt="Kannada" />
  <img src="../assets/flags/in.svg" width="32" title="Marathi" alt="Marathi" />
  <img src="../assets/flags/ng.svg" width="32" title="Hausa" alt="Hausa" />
  <img src="../assets/flags/mm.svg" width="32" title="Myanma" alt="Myanma" />
  <img src="../assets/flags/uz.svg" width="32" title="O'zbek" alt="O'zbek" />
  <img src="../assets/flags/az.svg" width="32" title="Ozarbayjon" alt="Ozarbayjon" />
  <img src="../assets/flags/ph.svg" width="32" title="Sebuan" alt="Sebuan" />
  <img src="../assets/flags/in.svg" width="32" title="Malayalam" alt="Malayalam" />
  <img src="../assets/flags/pk.svg" width="32" title="Sindhi" alt="Sindhi" />
  <img src="../assets/flags/et.svg" width="32" title="Amxar" alt="Amxar" />
</p>

## 🚀 Asosiy xususiyatlar va foydalanish

- **Zamonaviy nativ UI**: Qorong'i/yorug' rejimni qo'llab-quvvatlaydigan, silliq animatsiyalar va **Skia** asosidagi yuqori unumdorlikdagi renderlashga ega intuitiv GUI.
- **Tizim treyi integratsiyasi**: Tizim treyiga yig'ishni to'liq qo'llab-quvvatlash (~10MB RAM iste'moli), ikki marta bosish bilan oynani almashtirish va ishlaydigan o'ng tugma menyusi.
- **Aqlli ishga tushirish**: Paneli Windows bilan birga ishga tushirish, treyga yig'ish (`/silent` bilan jim rejim) va chiqishda distributsiyalarni avtomatik to'xtatish.
- **Vazifalarni rejalashtirish**: Moslashuvchan vazifalarni rejalashtirish boshqaruvi, avtomatlashtirilgan operatsiyalar uchun WSL instansiyalarida belgilangan vaqtda yoki oraliqlarda skript va buyruqlarni avtomatik bajarishni qo'llab-quvvatlaydi.
- **To'liq instansiya boshqaruvi**: Bir bosishda ishga tushirish, to'xtatish, yakunlash va ro'yxatdan chiqarish. Real vaqtda holat monitoringi, disk hajmi va fayl joylashuvi haqida batafsil ma'lumot.
- **Distro boshqaruvi**: Standart sifatida belgilash, migratsiya (VHDXni boshqa diskka ko'chirish) va `.tar` yoki `.tar.gz` arxivlariga eksport/klonlash.
- **Diskni ulash**: WSL instansiyasi VHDX virtual disklarini to'g'ridan-to'g'ri ulash va boshqarish, mount, unmount va disk boshqaruvi operatsiyalarini qo'llab-quvvatlaydi, host va WSL o'rtasida fayllarni uzatishni osonlashtiradi.
- **Tezkor integratsiya**: Terminal, VS Code yoki Fayl Exploreriga bir zumda ochish, ish papkasi va ishga tushirish skripti ilgaklarini sozlash imkoniyati bilan.
- **Distro o'rnatish**: Linux distributsiyalarini Microsoft Store, GitHub, lokal fayllar (RootFS/VHDX) yoki onlayn mirrorlar orqali o'rnatish (eng tez mirrorni avtomatik aniqlash va o'rnatilgan RootFS yuklab olish yordamchisi bilan).
- **Global xavfsizlik**: Bir vaqtda migratsiya/zaxiralash operatsiyalari uchun Mutex qulflari va o'chirishda Appxni avtomatik tozalash.
- **Ultra past xotira iste'moli**: Samaradorlik uchun yuqori darajada optimallashtirilgan. Jim ishga tushish (tizim treyi) faqat **~10MB** RAM iste'mol qiladi. Oynali rejim shrift murakkabligiga qarab farq qiladi: standart tillar (ingliz, nemis, ispan va boshqalar) uchun **~18MB**, katta shriftli tillar (xitoy, yapon, koreys va boshqalar) uchun **~38MB**.
- **Kengaytirilgan tarmoq**: Port forwarding boshqaruvi (fayl devor qoidalarini avtomatik yaratish bilan) va yagona ulanish uchun global HTTP proxy sozlamasi.
- **USB qurilmalarini boshqarish**: `usbipd-win` bilan to'liq integratsiya — lokal USB qurilmalarini WSL instansiyalari bo'ylab dashboard UI orqali osongina bog'lash, ulash va boshqarish.
- **Konfiguratsiya va loglar**: Standart o'rnatish papkasi, log papkasi va darajasi, interfeys tili, qorong'u rejim, yangilanishlarni tekshirish davriyligi, avtomatik ishga tushish va yopilish xatti-harakati, hamda yon panelni Sozlamalardan boshqaring.
- **Distro faol turishi**: «Ishga tushirish» tugmasini bir marta bosish orqali distro ~60 soniya bo'sh turgandan so'ng avtomatik yopilish o'rniga ishlashda davom etadi; «Windows ishga tushganda ushbu distroni ishga tushirish»ni ham yoqishingiz mumkin.
- **Global tezkor tugmalar**: Asosiy oynani ko'rsatish/yashirish yoki tanlangan distro terminalini darhol ochish uchun global tezkor tugmalarni belgilang.


## 🖼️ Dastur skrinshotlari
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

Boshqaruv panelini ish jarayonida ko'rishni xohlaysizmi? To'liq skrinshotlar galereyasi uchun [rasmiy veb-saytga](https://www.wslui.com/) tashrif buyuring.

## 📦 O'rnatish bo'yicha qo'llanma

### 1-usul: Loyiha veb-saytiga tashrif buyuring (Tavsiya etiladi)

Rasmiy veb-sayt orqali yuklab olishni tavsiya qilamiz, chunki u silliq tajriba uchun bir nechta mirror havolalarini taqdim etadi:

[Yuklab olish sahifasi](https://www.wslui.com/download/) ga o'ting va mintaqangizga mos mirrorni tanlang.

### 2-usul: winget orqali o'rnatish

Siz WSLDashboard'ni Windows Package Manager (winget) orqali to'g'ridan-to'g'ri o'rnatishingiz mumkin, moniker yoki to'liq paket identifikatoridan foydalanib:

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

> winget paket identifikatori `Owu.WSLDashboard` va moniker `wsl-dashboard` (case-insensitive). Ikkalasi ham ishlaydi.

Qo'shimcha ma'lumot uchun [WinGet hamjamiyat ombori](https://github.com/microsoft/winget-pkgs/tree/master/manifests/o/Owu/WSLDashboard) ga tashrif buyuring.

### 3-usul: Manba koddan qurish

Rust toolchain (Rust 1.92+ yoki yangiroq) o'rnatilganligiga ishonch hosil qiling.

1. Repozitoriyni klonlash:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. Qurish va ishga tushirish:

   - Ishlab chiqish uchun:

     ```powershell
     cargo run
     ```
   - Optimallashtirilgan release build, build skripti orqali:

     > Build skripti `x86_64-pc-windows-msvc` toolchain talab qiladi.

     ```powershell
     .\build\portable\build.ps1 -Arch x64
     ```


## 📢 Rasmiy jamoa guruhlari

WSL UI rasmiy jamoa guruhlariga xush kelibsiz! Foydalanish maslahatlarini muhokama qilish, muammolarni xabar qilish, so'nggi yangiliklarni olish va dasturchilar bilan boshqa foydalanuvchilar bilan bog'lanish uchun kanallarimizga qo'shiling.

<div align="center">

| Global Guruh (Telegram) | Xitoy Guruh (WeChat) |
| :---: | :---: |
| <img src="https://cdn1.wslui.com/static/images/group/group-telegram.png" width="240" alt="Telegram Group QR Code" /> | <img src="https://cdn1.wslui.com/static/images/group/group-wechat.png" width="240" alt="WeChat Group QR Code" /> |
| Qo'shilish uchun skanerlang | Qo'shilish uchun skanerlang |

</div>


## ❤️ Loyihani qo'llab-quvvatlang

- Bu loyiha GPL-3.0 litsenziyasi ostida va barcha foydalanuvchilar uchun bepul.
- Xususiyatlar ishlab chiqish va kundalik testlardan tortib xatoliklarni tuzatishgacha — barcha ishlar bo'sh vaqtda amalga oshiriladi. Ochiq manba yo'li yolg'iz oson emas — sizning e'tirofingiz va qo'llab-quvvatlashingiz loyihaga davom etish uchun ishonch beradi.
- Agar bu vosita sizga haqiqatan foyda bergan bo'lsa, qo'l uzatishni o'ylab ko'ring. Barcha xayriyalar server xarajatlari, versiya iteratsiyalari va funksiyalarni yaxshilashga yo'naltiriladi, loyiha doimiy yangilanib, barqaror rivojlanadi.
- Har bir samimiy yaxshilik — bir nur yulduz. Tushunishingiz va saxovatlaringiz uchun yana bir bor rahmat! [Donat sahifamizga tashrif buyuring](https://www.wslui.com/donate/).


## ⭐️ Sevgi bilan ishlaydigan loyiha

Agar bu loyiha sizga foydali bo'lgan bo'lsa, GitHubda yulduzcha qoldirsangiz juda minnatdor bo'lardim. Sizning e'tirofingiz loyihani kengroq auditoriyaga yetib borishiga yordam beradi va juda qadrlanadi. Aynan shu dalda meni qurishda davom etishga undaydi.


## 📄 Ochiq manba litsenziyasi

Bu loyiha GPL-3.0 litsenziyasi ostida — tafsilotlar uchun [LICENSE](../LICENSE) fayliga qarang.
