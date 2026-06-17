# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

แดชบอร์ดจัดการอินสแตนซ์ WSL (Windows Subsystem for Linux) ที่ทันสมัย ประสิทธิภาพสูง น้ำหนักเบา และใช้หน่วยความจำต่ำ สร้างขึ้นด้วย Rust และ Slint เพื่อมอบประสบการณ์เนทีฟที่ยอดเยี่ยม

---

```diff
ประกาศ:

- WSL Dashboard ไม่ได้แจกจ่ายผ่าน Microsoft Store
- แอปพลิเคชันใดก็ตามที่ระบุไว้ที่นั่นในชื่อ "WSL Dashboard" ไม่ได้รับอนุญาตและอาจเป็นของปลอม
- กรุณาอย่าดาวน์โหลดเพื่อหลีกเลี่ยงการหลอกลวงที่อาจเกิดขึ้น
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

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md) | ภาษาไทย | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 สารบัญ
- [🌍 การรองรับภาษา](#-การรองรับภาษา)
- [🚀 ฟีเจอร์หลักและการใช้งาน](#-ฟีเจอร์หลักและการใช้งาน)
- [⚙️ การตั้งค่าและบันทึก](#️-การตั้งค่าและบันทึก)
- [🖼️ ภาพหน้าจอ](#️-ภาพหน้าจอ)
- [🎬 การสาธิต](#-การสาธิต)
- [💻 ข้อกำหนดของระบบ](#-ข้อกำหนดของระบบ)
- [📦 คู่มือการติดตั้ง](#-คู่มือการติดตั้ง)
- [🛠️ เทคโนโลยีและประสิทธิภาพ](#️-เทคโนโลยีและประสิทธิภาพ)
- [🤝 การสนับสนุนจากชุมชน](#-การสนับสนุนจากชุมชน)
- [❤️ สนับสนุนโครงการ](#️-สนับสนุนโครงการ)
- [⭐️ สนับสนุนด้วยใจ](#️-สนับสนุนด้วยใจ)
- [📄 สัญญาอนุญาตโอเพนซอร์ส](#-สัญญาอนุญาตโอเพนซอร์ส)

---

## 🌍 การรองรับภาษา

ภาษาอังกฤษ, ภาษาจีนตัวย่อ, ภาษาจีนตัวเต็ม, ภาษาฮินดี, ภาษาสเปน, ภาษาฝรั่งเศส, ภาษาอาหรับ, ภาษาเบงกาลี, ภาษาโปรตุเกส, ภาษารัสเซีย, ภาษาอูรดู, ภาษาอินโดนีเซีย, ภาษาเยอรมัน, ภาษาญี่ปุ่น, ภาษาตุรกี, ภาษาเกาหลี, ภาษาอิตาลี, ภาษาดัตช์, ภาษาสวีเดน, ภาษาเช็ก, ภาษากรีก, ภาษาฮังการี, ภาษาฮิบรู, ภาษานอร์เวย์, ภาษาเดนมาร์ก, ภาษาฟินแลนด์, ภาษาสโลวัก, ภาษาสโลวีเนีย, ภาษาไอซ์แลนด์, ภาษาเวียดนาม, ภาษาเตลูกู, ภาษาชวา, ภาษาไทย, ภาษาทมิฬ, ภาษาฟิลิปปินส์, ภาษาปัญจาบ, ภาษามาลายาลัม, ภาษามาเลย์, ภาษาโปแลนด์, ภาษายูเครน, ภาษาเปอร์เซีย, ภาษากันนาดา, ภาษามราฐี, ภาษาเฮาซา, ภาษาพม่า, ภาษาอุซเบก, ภาษาอาเซอร์ไบจาน, ภาษาเซบัวโน, ภาษามาเลย์, ภาษาสินธี, ภาษาอัมฮาริก

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="ภาษาอังกฤษ" alt="ภาษาอังกฤษ" />
  <img src="../assets/flags/cn.svg" width="32" title="ภาษาจีนตัวย่อ" alt="ภาษาจีนตัวย่อ" />
  <img src="../assets/flags/tw.svg" width="32" title="ภาษาจีนตัวเต็ม" alt="ภาษาจีนตัวเต็ม" />
  <img src="../assets/flags/in.svg" width="32" title="ภาษาฮินดี" alt="ภาษาฮินดี" />
  <img src="../assets/flags/es.svg" width="32" title="ภาษาสเปน" alt="ภาษาสเปน" />
  <img src="../assets/flags/fr.svg" width="32" title="ภาษาฝรั่งเศส" alt="ภาษาฝรั่งเศส" />
  <img src="../assets/flags/sa.svg" width="32" title="ภาษาอาหรับ" alt="ภาษาอาหรับ" />
  <img src="../assets/flags/bd.svg" width="32" title="ภาษาเบงกาลี" alt="ภาษาเบงกาลี" />
  <img src="../assets/flags/pt.svg" width="32" title="ภาษาโปรตุเกส" alt="ภาษาโปรตุเกส" />
  <img src="../assets/flags/ru.svg" width="32" title="ภาษารัสเซีย" alt="ภาษารัสเซีย" />
  <img src="../assets/flags/pk.svg" width="32" title="ภาษาอูรดู" alt="ภาษาอูรดู" />
  <img src="../assets/flags/id.svg" width="32" title="ภาษาอินโดนีเซีย" alt="ภาษาอินโดนีเซีย" />
  <img src="../assets/flags/de.svg" width="32" title="ภาษาเยอรมัน" alt="ภาษาเยอรมัน" />
  <img src="../assets/flags/jp.svg" width="32" title="ภาษาญี่ปุ่น" alt="ภาษาญี่ปุ่น" />
  <img src="../assets/flags/tr.svg" width="32" title="ภาษาตุรกี" alt="ภาษาตุรกี" />
  <img src="../assets/flags/kr.svg" width="32" title="ภาษาเกาหลี" alt="ภาษาเกาหลี" />
  <img src="../assets/flags/it.svg" width="32" title="ภาษาอิตาลี" alt="ภาษาอิตาลี" />
  <img src="../assets/flags/nl.svg" width="32" title="ภาษาดัตช์" alt="ภาษาดัตช์" />
  <img src="../assets/flags/se.svg" width="32" title="ภาษาสวีเดน" alt="ภาษาสวีเดน" />
  <img src="../assets/flags/cz.svg" width="32" title="ภาษาเช็ก" alt="ภาษาเช็ก" />
  <img src="../assets/flags/gr.svg" width="32" title="ภาษากรีก" alt="ภาษากรีก" />
  <img src="../assets/flags/hu.svg" width="32" title="ภาษาฮังการี" alt="ภาษาฮังการี" />
  <img src="../assets/flags/il.svg" width="32" title="ภาษาฮิบรู" alt="ภาษาฮิบรู" />
  <img src="../assets/flags/no.svg" width="32" title="ภาษานอร์เวย์" alt="ภาษานอร์เวย์" />
  <img src="../assets/flags/dk.svg" width="32" title="ภาษาเดนมาร์ก" alt="ภาษาเดนมาร์ก" />
  <img src="../assets/flags/fi.svg" width="32" title="ภาษาฟินแลนด์" alt="ภาษาฟินแลนด์" />
  <img src="../assets/flags/sk.svg" width="32" title="ภาษาสโลวัก" alt="ภาษาสโลวัก" />
  <img src="../assets/flags/si.svg" width="32" title="ภาษาสโลวีเนีย" alt="ภาษาสโลวีเนีย" />
  <img src="../assets/flags/is.svg" width="32" title="ภาษาไอซ์แลนด์" alt="ภาษาไอซ์แลนด์" />
  <img src="../assets/flags/vn.svg" width="32" title="ภาษาเวียดนาม" alt="ภาษาเวียดนาม" />
  <img src="../assets/flags/in.svg" width="32" title="ภาษาเตลูกู" alt="ภาษาเตลูกู" />
  <img src="../assets/flags/id.svg" width="32" title="ภาษาชวา" alt="ภาษาชวา" />
  <img src="../assets/flags/th.svg" width="32" title="ภาษาไทย" alt="ภาษาไทย" />
  <img src="../assets/flags/in.svg" width="32" title="Tamil" alt="Tamil" />
  <img src="../assets/flags/ph.svg" width="32" title="Filipino" alt="Filipino" />
  <img src="../assets/flags/pk.svg" width="32" title="Punjabi" alt="Punjabi" />
  <img src="../assets/flags/my.svg" width="32" title="Malay" alt="Malay" />
  <img src="../assets/flags/pl.svg" width="32" title="Polish" alt="Polish" />
  <img src="../assets/flags/ua.svg" width="32" title="Ukrainian" alt="Ukrainian" />
  <img src="../assets/flags/ir.svg" width="32" title="Persian" alt="Persian" />
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


## 🚀 ฟีเจอร์หลักและการใช้งาน

- **อินเทอร์เฟซเนทีฟที่ทันสมัย**: GUI ที่ใช้งานง่าย รองรับโหมดมืด/สว่าง แอนิเมชันลื่นไหล การเรนเดอร์ประสิทธิภาพสูงที่ขับเคลื่อนโดย **Skia**
- **การรวมกับซิสเต็มเทรย์**: รองรับเทรย์เต็มรูปแบบ (ใช้หน่วยความจำประมาณ 10MB) ดับเบิลคลิกเพื่อแสดง/ซ่อน และเมนูคลิกขวาที่มีฟังก์ชันครบถ้วน
- **การเริ่มต้นอัจฉริยะ**: รองรับการเริ่มต้นพร้อม Windows ย่อเล็กสุดไปที่เทรย์ (เริ่มต้นแบบเงียบด้วยพารามิเตอร์ `/silent`) และปิดดิสทริบิวชันโดยอัตโนมัติเมื่อออกจากโปรแกรม
- **การควบคุมอินสแตนซ์อย่างครอบคลุม**: เริ่มต้น หยุด บังคับหยุด และยกเลิกการลงทะเบียนด้วยคลิกเดียว ตรวจสอบสถานะแบบเรียลไทม์ ดูรายละเอียดการใช้งานดิสก์และที่อยู่ไฟล์
- **การจัดการดิสทริบิวชัน**: ตั้งเป็นค่าเริ่มต้น ย้ายทางกายภาพ (ย้าย VHDX ไปยังดิสก์อื่น) และส่งออก/โคลนเป็น `.tar` หรือ `.tar.gz`
- **การรวมอย่างรวดเร็ว**: เปิด Terminal, VS Code หรือ File Explorer ด้วยคลิกเดียว รองรับไดเรกทอรีทำงานที่กำหนดเองและสคริปต์ฮุกสำหรับการเปิดใช้งาน
- **การติดตั้งดิสโทร**: ติดตั้ง Linux Distribution ผ่าน Microsoft Store, GitHub, ไฟล์ท้องถิ่น (RootFS/VHDX) หรือ Mirror ออนไลน์ (พร้อมทดสอบความเร็วอัตโนมัติเพื่อเลือก Mirror ที่เร็วที่สุดและเครื่องมือช่วยดาวน์โหลด RootFS ในตัว)
- **ความปลอดภัยทั่วไป**: ใช้ mutex เพื่อความปลอดภัยของปฏิบัติการย้าย/สำรองข้อมูลพร้อมกัน และทำความสะอาดแพ็กเกจ Appx อัตโนมัติเมื่อลบ
- **การใช้หน่วยความจำต่ำมาก**: ปรับทรัพยากรอย่างเหมาะสมที่สุด การเริ่มต้นแบบเงียบ (ซิสเต็มเทรย์) ใช้หน่วยความจำเพียงประมาณ **10MB** ในโหมดหน้าต่าง ใช้ประมาณ **18MB** (ภาษามาตรฐาน เช่น อังกฤษ เยอรมัน ฯลฯ) ถึง **38MB** (ชุดอักขระขนาดใหญ่ เช่น CJK) ขึ้นอยู่กับความซับซ้อนของแบบอักษร
- **การจัดการเครือข่ายขั้นสูง**: จัดการการส่งต่อพอร์ต (สร้างกฎไฟร์วอลล์โดยอัตโนมัติ) และการกำหนดค่า HTTP proxy ทั่วไปอย่างราบรื่น เพื่อประสบการณ์การเชื่อมต่อที่เป็นหนึ่งเดียว
- **การจัดการอุปกรณ์ USB**: ผสานรวมอย่างลึกซึ้งกับ `usbipd-win` สามารถผูก ต่อ และจัดการอุปกรณ์ USB ท้องถิ่นได้อย่างง่ายดายสำหรับอินสแตนซ์ WSL ทั้งหมดโดยตรงจากแดชบอร์ด UI


## ⚙️ การตั้งค่าและบันทึก

การตั้งค่าทั้งหมดจัดการผ่านมุมมอง "การตั้งค่า":

- เลือกไดเรกทอรีติดตั้งเริ่มต้นสำหรับอินสแตนซ์ WSL ใหม่
- กำหนดค่าไดเรกทอรีบันทึกและระดับบันทึก (Error / Warn / Info / Debug / Trace)
- เลือกภาษาอินเทอร์เฟซหรือให้ตามภาษาของระบบ
- สลับโหมดมืด และเลือกว่าแอปพลิเคชันจะปิด WSL โดยอัตโนมัติหลังปฏิบัติการหรือไม่
- กำหนดค่าความถี่ในการตรวจสอบอัปเดต (รายวัน รายสัปดาห์ ทุกสองสัปดาห์ รายเดือน)
- เปิดใช้งานการเริ่มต้นพร้อม Windows (มีฟีเจอร์ซ่อมแซมเส้นทางอัตโนมัติ)
- ตั้งค่าย่อเล็กสุดไปที่เทรย์เมื่อเริ่มต้น
- กำหนดค่าพฤติกรรมปุ่มปิด (ย่อเล็กสุดไปที่เทรย์แทนการออกจากโปรแกรม)
- ปรับแต่งแถบข้างโดยสลับการแสดงแท็บฟีเจอร์เฉพาะ

ไฟล์บันทึกจะถูกเขียนไปยังไดเรกทอรีบันทึกที่กำหนดค่าไว้ สามารถแนบมาพร้อมกับรายงานปัญหาได้


## 🖼️ ภาพหน้าจอ

### หน้าหลัก (โหมดมืด & สว่าง)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB & การพับเมนู
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### การจัดการเครือข่าย
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### เพิ่มอินสแตนซ์ & การตั้งค่า
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>
<p align="center">
  <img src="../assets/screenshot/settings-advanced.png" width="48%" />
  <img src="../assets/screenshot/settings-interface.png" width="48%" />
</p>

### เกี่ยวกับ & การบริจาค
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/donate.png" width="48%" />
</p>

## 🎬 การสาธิต

[ช่วยเราปรับปรุง! ชมวิดีโอแนะนำและแบ่งปันความคิดเห็นของคุณ](https://github.com/owu/wsl-dashboard/discussions/9)



## 💻 ข้อกำหนดของระบบ

- Windows 10 หรือ Windows 11 ที่เปิดใช้งาน WSL (แนะนำ WSL 2)
- มีดิสทริบิวชัน WSL ติดตั้งอยู่อย่างน้อยหนึ่งรายการ หรือมีสิทธิ์ในการติดตั้งดิสทริบิวชันใหม่
- CPU 64 บิต; แนะนำ RAM 4 GB ขึ้นไปเพื่อการใช้งานหลายดิสทริบิวชันอย่างราบรื่น

## 📦 คู่มือการติดตั้ง

### วิธีที่ 1: ดาวน์โหลดไบนารีที่คอมไพล์แล้ว

วิธีที่ง่ายที่สุดในการเริ่มต้นคือใช้เวอร์ชันที่คอมไพล์แล้ว:

1. ไปที่หน้า [GitHub Releases](https://github.com/owu/wsl-dashboard/releases)
2. ดาวน์โหลดไฟล์ปฏิบัติการ `wsldashboard` ล่าสุดสำหรับ Windows
3. แตกไฟล์ (ถ้าเป็นไฟล์บีบอัด) และเรียกใช้ `wsldashboard.exe`

ไม่จำเป็นต้องติดตั้ง แอปพลิเคชันเป็นโปรแกรมพกพาไฟล์เดี่ยว

### วิธีที่ 2: คอมไพล์จากซอร์สโค้ด

ตรวจสอบให้แน่ใจว่าคุณได้ติดตั้ง Rust toolchain (Rust 1.92+ หรือใหม่กว่า) แล้ว

1. Clone รีโพซิทอรี:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. คอมไพล์และเรียกใช้:

   - สำหรับการพัฒนาและดีบัก:

     ```powershell
     cargo run
     ```
   - คอมไพล์เวอร์ชันรีลีสที่ปรับปรุงแล้วด้วยสคริปต์บิลด์:

     > สคริปต์บิลด์ต้องใช้ toolchain `x86_64-pc-windows-msvc`

     ```powershell
     .\build\portable\build.ps1
     ```


## 🛠️ เทคโนโลยีและประสิทธิภาพ

- **แกนหลัก**: พัฒนาด้วย Rust เพื่อความปลอดภัยของหน่วยความจำและ zero-cost abstraction
- **กรอบงาน UI**: Slint ที่มีแบ็กเอนด์เรนเดอร์ **Skia** ประสิทธิภาพสูง
- **Async runtime**: Tokio สำหรับคำสั่งระบบและ I/O แบบ non-blocking
- **จุดเด่นด้านประสิทธิภาพ**:
  - **ความเร็วในการตอบสนอง**: การเริ่มต้นเกือบจะทันที ตรวจสอบสถานะ WSL แบบเรียลไทม์
  - **ประสิทธิภาพทรัพยากร**: การใช้ทรัพยากรต่ำมาก (ดู [ฟีเจอร์หลัก](#-ฟีเจอร์หลักและการใช้งาน))
  - **ความพกพาสะดวก**: เวอร์ชันรีลีสที่ปรับปรุงแล้วสร้างไฟล์ปฏิบัติการเดี่ยวที่กะทัดรัด



## 🤝 การสนับสนุนจากชุมชน

ขอขอบคุณอย่างจริงใจต่อชุมชนต่อไปนี้สำหรับการสนับสนุน:

- [Rust Programming Language](https://www.rust-lang.org) - ภาษาโปรแกรมที่ทรงพลังและปลอดภัย
- [Slint | Declarative GUI for Rust, C++, JavaScript & Python](https://slint.dev) - กรอบงาน UI ที่ทันสมัย
- [WSL: Windows Subsystem for Linux](https://github.com/microsoft/WSL) - Windows Subsystem for Linux ที่ยอดเยี่ยม
- [Tokio - An asynchronous Rust runtime](https://tokio.rs) - Async runtime ที่มีประสิทธิภาพ
- [Windows Developer Community](https://developer.microsoft.com/en-us/windows/community) - การปรับปรุงแพลตฟอร์มอย่างต่อเนื่อง
- [Reddit](https://www.reddit.com) - การสนทนาและสนับสนุนจากชุมชนทั่วโลก
- [Hacker News](https://news.ycombinator.com) - การสนทนาและสนับสนุนจากชุมชนทั่วโลก
- [Linux.do](https://linux.do) - ชุมชนผู้เชี่ยวชาญ IT ที่ได้รับความนิยม
- [V2EX](https://www.v2ex.com) - การสนทนาชุมชนเทคโนโลยีภาษาจีน

การมีส่วนร่วมและข้อเสนอแนะของคุณทำให้โครงการนี้เป็นไปได้!


## ❤️ สนับสนุนโครงการ

- โครงการนี้ใช้สัญญาอนุญาต GPL-3.0 ให้ใช้งานได้ฟรีสำหรับผู้ใช้ทุกคน
- ตั้งแต่การพัฒนาฟีเจอร์ การทดสอบประจำวัน ไปจนถึงการแก้ไขข้อบกพร่อง ทุกงานล้วนทำในเวลาว่าง เส้นทางโอเพนซอร์สไม่ง่ายที่จะเดินคนเดียว การยอมรับและการสนับสนุนของคุณเป็นพลังให้โครงการเดินต่อไป
- หากเครื่องมือนี้มีประโยชน์กับคุณจริงๆ โปรดพิจารณาให้การสนับสนุน การบริจาคทั้งหมดจะนำไปใช้สำหรับค่าใช้จ่ายเซิร์ฟเวอร์ การพัฒนาเวอร์ชัน และการปรับปรุงฟีเจอร์ เพื่อให้โครงการอัปเดตอย่างต่อเนื่องและก้าวไปอย่างมั่นคง
- น้ำใจเล็กน้อยคือแสงดาว ขอขอบคุณอีกครั้งสำหรับความเข้าใจและความใจดีของคุณ!

เยี่ยมชมหน้าบริจาคของเรา: [https://www.wslui.com/donate/](https://www.wslui.com/donate/)


## ⭐️ สนับสนุนด้วยใจ

หากคุณรู้สึกว่าโครงการนี้มีประโยชน์ ผมจะขอบคุณมากหากคุณสามารถให้ดาวบน GitHub การสนับสนุนของคุณจะช่วยให้โครงการเข้าถึงผู้ใช้ได้กว้างขึ้น และผมขอขอบคุณอย่างจริงใจ กำลังใจจากทุกท่านคือสิ่งที่ผลักดันให้ผมก้าวต่อไป


## 📄 สัญญาอนุญาตโอเพนซอร์ส

โครงการนี้เผยแพร่ภายใต้สัญญาอนุญาต GPL-3.0 – ดูไฟล์ [LICENSE](../LICENSE) สำหรับรายละเอียดเพิ่มเติม


---

Built with ❤️ for the WSL Community.
