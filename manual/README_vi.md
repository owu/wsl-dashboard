# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

Một bảng điều khiển quản lý phiên bản WSL (Windows Subsystem for Linux) hiện đại, hiệu suất cao và nhẹ. Được xây dựng bằng Rust và Slint cho trải nghiệm native cao cấp.

---

<p align="left">
  <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/rust-v1.92+-orange.svg" alt="Rust" /></a>
  <a href="https://slint.dev"><img src="https://img.shields.io/badge/UI-Slint-blue.svg" alt="Slint" /></a>
  <a href="https://tokio.rs"><img src="https://img.shields.io/badge/Async-Tokio-000000.svg" alt="Tokio" /></a>
  <a href="https://github.com/microsoft/windows-rs"><img src="https://img.shields.io/badge/OS-Windows-0078D6.svg" alt="Windows" /></a>
  <a href="../LICENSE"><img src="https://img.shields.io/badge/license-GPL--3.0-blue.svg" alt="License" /></a>
</p>

I18N : [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md)

---

## 📑 Mục lục

- [🌍 Hỗ trợ ngôn ngữ](#-hỗ-trợ-ngôn-ngữ)
- [🚀 Các tính năng chính & Cách sử dụng](#-các-tính-năng-chính--cách-sử-dụng)
- [⚙️ Cấu hình & Nhật ký](#️-cấu-hình--nhật-ký)
- [🖼️ Ảnh chụp màn hình](#-ảnh-chụp-màn-hình)
- [🎬 Demo hoạt động](#-demo-hoạt-động)
- [💻 Yêu cầu hệ thống](#-yêu-cầu-hệ-thống)
- [📦 Cài đặt](#-cài-đặt)
- [🛠️ Công nghệ & Hiệu suất](#️-công-nghệ--hiệu-suất)
- [📄 Giấy phép](#-giấy-phép)

---

## 🌍 Hỗ trợ ngôn ngữ

Tiếng Anh, Tiếng Trung Giản thể, Tiếng Trung Phồn thể, Tiếng Hindi, Tiếng Tây Ban Nha, Tiếng Pháp, Tiếng Ả Rập, Tiếng Bengali, Tiếng Bồ Đào Nha, Tiếng Nga, Tiếng Urdu, Tiếng Indonesia, Tiếng Đức, Tiếng Nhật, Tiếng Thổ Nhĩ Kỳ, Tiếng Hàn, Tiếng Ý, Tiếng Hà Lan, Tiếng Thụy Điển, Tiếng Séc, Tiếng Hy Lạp, Tiếng Hungary, Tiếng Do Thái, Tiếng Na Uy, Tiếng Đan Mạch, Tiếng Phần Lan, Tiếng Slovak, Tiếng Slovenia, Tiếng Iceland, Tiếng Việt

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="English" alt="English" />
  <img src="../assets/flags/cn.svg" width="32" title="Simplified Chinese" alt="Simplified Chinese" />
  <img src="../assets/flags/tw.svg" width="32" title="Traditional Chinese" alt="Traditional Chinese" />
  <img src="../assets/flags/in.svg" width="32" title="Hindi" alt="Hindi" />
  <img src="../assets/flags/es.svg" width="32" title="Spanish" alt="Spanish" />
  <img src="../assets/flags/fr.svg" width="32" title="French" alt="French" />
  <img src="../assets/flags/sa.svg" width="32" title="Arabic" alt="Arabic" />
  <img src="../assets/flags/bd.svg" width="32" title="Bengali" alt="Bengali" />
  <img src="../assets/flags/pt.svg" width="32" title="Portuguese" alt="Portuguese" />
  <img src="../assets/flags/ru.svg" width="32" title="Russian" alt="Russian" />
  <img src="../assets/flags/pk.svg" width="32" title="Urdu" alt="Urdu" />
  <img src="../assets/flags/id.svg" width="32" title="Indonesian" alt="Indonesian" />
  <img src="../assets/flags/de.svg" width="32" title="German" alt="German" />
  <img src="../assets/flags/jp.svg" width="32" title="Japanese" alt="Japanese" />
  <img src="../assets/flags/tr.svg" width="32" title="Turkish" alt="Turkish" />
  <img src="../assets/flags/kr.svg" width="32" title="Korean" alt="Korean" />
  <img src="../assets/flags/it.svg" width="32" title="Italian" alt="Italian" />
  <img src="../assets/flags/nl.svg" width="32" title="Dutch" alt="Dutch" />
  <img src="../assets/flags/se.svg" width="32" title="Swedish" alt="Swedish" />
  <img src="../assets/flags/cz.svg" width="32" title="Czech" alt="Czech" />
  <img src="../assets/flags/gr.svg" width="32" title="Greek" alt="Greek" />
  <img src="../assets/flags/hu.svg" width="32" title="Hungarian" alt="Hungarian" />
  <img src="../assets/flags/il.svg" width="32" title="Hebrew" alt="Hebrew" />
  <img src="../assets/flags/no.svg" width="32" title="Norwegian" alt="Norwegian" />
  <img src="../assets/flags/dk.svg" width="32" title="Danish" alt="Danish" />
  <img src="../assets/flags/fi.svg" width="32" title="Finnish" alt="Finnish" />
  <img src="../assets/flags/sk.svg" width="32" title="Slovak" alt="Slovak" />
  <img src="../assets/flags/si.svg" width="32" title="Slovenian" alt="Slovenian" />
  <img src="../assets/flags/is.svg" width="32" title="Icelandic" alt="Icelandic" />
  <img src="../assets/flags/vn.svg" width="32" title="Tiếng Việt" alt="Tiếng Việt" />
</p>

## 🚀 Các tính năng chính & Cách sử dụng

- **Giao diện Native hiện đại**: GUI trực quan hỗ trợ chế độ Sáng/Tối, hiệu ứng chuyển động mượt mà và dựng hình hiệu suất cao được cung cấp bởi **Skia**.
- **Tích hợp khay hệ thống**: Hỗ trợ đầy đủ việc thu nhỏ xuống khay hệ thống (sử dụng ~10MB RAM), nhấp đúp để chuyển đổi và menu chuột phải đầy đủ chức năng.
- **Khởi động thông minh**: Cấu hình bảng điều khiển để khởi động cùng Windows, thu nhỏ vào khay (chế độ ẩn với `/silent`) và tự động tắt các phân phối khi thoát.
- **Điều khiển phiên bản toàn diện**: Bắt đầu, Dừng, Chấm dứt và Hủy đăng ký chỉ bằng một cú nhấp chuột. Theo dõi trạng thái thời gian thực và thông tin chi tiết về việc sử dụng đĩa và vị trí tệp.
- **Quản lý phân phối**: Đặt làm mặc định, di chuyển (Di chuyển VHDX sang ổ đĩa khác), và xuất/sao chép sang kho lưu trữ `.tar` hoặc `.tar.gz`.
- **Tích hợp nhanh**: Khởi động tức thì vào Terminal, VS Code, hoặc File Explorer với các thư mục làm việc có thể tùy chỉnh và các móc nối tập lệnh khởi động.
- **Cài đặt thông minh**: Cài đặt từ Microsoft Store, GitHub, hoặc các tệp cục bộ (RootFS/VHDX). Bao gồm một trợ giúp tải xuống RootFS tích hợp sẵn.
- **An toàn toàn cầu**: Khóa Mutex cho các hoạt động di chuyển/sao lưu đồng thời an toàn và tự động dọn dẹp Appx khi gỡ bỏ.
- **Dấu chân bộ nhớ cực thấp**: Được tối ưu hóa cao cho hiệu quả. Khởi động ẩn (khay hệ thống) chỉ sử dụng **~10MB** RAM. Chế độ cửa sổ thay đổi tùy theo độ phức tạp của phông chữ: **~18MB** cho các ngôn ngữ tiêu chuẩn (tiếng Anh, tiếng Đức, tiếng Tây Ban Nha, v.v.) và **~38MB** cho các ngôn ngữ phông chữ lớn (tiếng Trung, tiếng Nhật, tiếng Hàn, v.v.).

## ⚙️ Cấu hình & Nhật ký

Tất cả cấu hình được quản lý thông qua chế độ xem Cài đặt:

- Chọn thư mục cài đặt mặc định cho các phiên bản WSL mới.
- Cấu hình thư mục nhật ký và mức độ nhật ký (Error / Warn / Info / Debug / Trace).
- Chọn ngôn ngữ giao diện hoặc để nó theo ngôn ngữ hệ thống.
- Chuyển đổi chế độ tối và liệu ứng dụng có thể tự động tắt WSL sau các hoạt động hay không.
- Cấu hình tần suất ứng dụng kiểm tra các bản cập nhật (hàng ngày, hàng tuần, hai tuần một lần, hàng tháng).
- Bật tự động khởi động khi khởi động hệ thống (với chức năng tự động sửa đường dẫn).
- Đặt ứng dụng thu nhỏ vào khay hệ thống khi khởi động để có trải nghiệm không gây xao nhãng.
- Cấu hình nút đóng để thu nhỏ vào khay hệ thống thay vì thoát.

Các tệp nhật ký được ghi vào thư mục nhật ký đã cấu hình và có thể được đính kèm khi báo cáo sự cố.

## 🖼️ Ảnh chụp màn hình

### Trang chủ (Chế độ Sáng & Tối)

<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

### Thêm phiên bản & Cài đặt

<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>

### Giới thiệu & Menu thu gọn

<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

## 🎬 Demo hoạt động

Dưới đây là bản trình diễn WSL Dashboard đang hoạt động:

![WSL Dashboard Demo](../assets/screenshot/demo.gif)

## 💻 Yêu cầu hệ thống

- Windows 10 hoặc Windows 11 đã bật WSL (khuyên dùng WSL 2).
- Ít nhất một phân phối WSL được cài đặt, hoặc có quyền cài đặt phân phối mới.
- CPU 64-bit; khuyên dùng RAM 4 GB trở lên để sử dụng nhiều phân phối mượt mà.

## 📦 Cài đặt

### Tùy chọn 1: Tải xuống bản dựng sẵn

Cách dễ nhất để bắt đầu là sử dụng bản phát hành đã được biên dịch sẵn:

1. Truy cập trang [GitHub Releases](https://github.com/owu/wsl-dashboard/releases).
2. Tải xuống tệp thực thi `wsldashboard` mới nhất cho Windows.
3. Giải nén (nếu được đóng gói) và chạy `wsldashboard.exe`.

Không cần bộ cài đặt; ứng dụng là một tệp thực thi di động duy nhất.

### Tùy chọn 2: Xây dựng từ nguồn

Đảm bảo bạn đã cài đặt Rust toolchain (Rust 1.92+ hoặc mới hơn).

1. Sao chép kho lưu trữ:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. Xây dựng và chạy:
   - Cho phát triển:

     ```powershell
     cargo run
     ```

   - Bản dựng phát hành được tối ưu hóa, sử dụng tập lệnh xây dựng:

     > Tập lệnh xây dựng yêu cầu toolchain `x86_64-pc-windows-msvc`.

     ```powershell
     .\build\scripts\build.ps1
     ```

## 🛠️ Công nghệ & Hiệu suất

- **Core**: Được triển khai bằng Rust cho an toàn bộ nhớ và trừu tượng chi phí bằng không.
- **UI Framework**: Slint với backend dựng hình **Skia** hiệu suất cao.
- **Async Runtime**: Tokio cho các lệnh hệ thống và I/O không chặn.
- **Điểm nổi bật về hiệu suất**:
  - **Khả năng phản hồi**: Khởi động gần như tức thì và giám sát trạng thái WSL thời gian thực.
  - **Hiệu quả**: Sử dụng tài nguyên cực thấp (xem [Các tính năng chính](#-các-tính-năng-chính--cách-sử-dụng) để biết chi tiết).
  - **Khả năng di động**: Bản dựng phát hành được tối ưu hóa tạo ra một tệp thực thi nhỏ gọn duy nhất.

## 📄 Giấy phép

Dự án này được cấp phép theo GPL-3.0 – xem tệp [LICENSE](../LICENSE) để biết chi tiết.

---

Được xây dựng với ❤️ cho Cộng đồng WSL.
