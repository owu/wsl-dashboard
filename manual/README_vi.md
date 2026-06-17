# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

Một bảng điều khiển quản lý phiên bản WSL (Windows Subsystem for Linux) hiện đại, hiệu suất cao, nhẹ và ít tiêu tốn bộ nhớ. Được xây dựng bằng Rust và Slint để mang lại trải nghiệm gốc tuyệt vời.

---

```diff
Thông báo:

- WSL Dashboard không được phân phối thông qua Microsoft Store.
- Bất kỳ ứng dụng nào được liệt kê ở đó với tên "WSL Dashboard" đều không được ủy quyền và có thể là giả mạo.
- Vui lòng không tải xuống để tránh các trò lừa đảo tiềm ẩn.
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

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | Tiếng Việt | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 Mục lục
- [🌍 Hỗ trợ ngôn ngữ](#-hỗ-trợ-ngôn-ngữ)
- [🚀 Tính năng cốt lõi & Sử dụng](#-tính-năng-cốt-lõi--sử-dụng)
- [⚙️ Cấu hình & Nhật ký](#️-cấu-hình--nhật-ký)
- [🖼️ Ảnh chụp phần mềm](#️-ảnh-chụp-phần-mềm)
- [🎬 Demo thao tác](#-demo-thao-tác)
- [💻 Yêu cầu hệ thống](#-yêu-cầu-hệ-thống)
- [📦 Hướng dẫn cài đặt](#-hướng-dẫn-cài-đặt)
- [🛠️ Công nghệ & Hiệu suất](#️-công-nghệ--hiệu-suất)
- [🤝 Hỗ trợ cộng đồng](#-hỗ-trợ-cộng-đồng)
- [❤️ Hỗ trợ dự án](#️-hỗ-trợ-dự-án)
- [⭐️Ủng hộ từ trái tim](#️ủng-hộ-từ-trái-tim)
- [📄 Giấy phép mã nguồn mở](#-giấy-phép-mã-nguồn-mở)

---

## 🌍 Hỗ trợ ngôn ngữ

Tiếng Anh, Tiếng Trung (Giản thể), Tiếng Trung (Phồn thể), Tiếng Hindi, Tiếng Tây Ban Nha, Tiếng Pháp, Tiếng Ả Rập, Tiếng Bengali, Tiếng Bồ Đào Nha, Tiếng Nga, Tiếng Urdu, Tiếng Indonesia, Tiếng Đức, Tiếng Nhật, Tiếng Thổ Nhĩ Kỳ, Tiếng Hàn, Tiếng Ý, Tiếng Hà Lan, Tiếng Thụy Điển, Tiếng Séc, Tiếng Hy Lạp, Tiếng Hungary, Tiếng Do Thái, Tiếng Na Uy, Tiếng Đan Mạch, Tiếng Phần Lan, Tiếng Slovakia, Tiếng Slovenia, Tiếng Iceland, Tiếng Việt, Tiếng Telugu, Tiếng Java, Tiếng Thái, Tiếng Tamil, Tiếng Filipino, Tiếng Punjab, Tiếng Mã Lai, Tiếng Ba Lan, Tiếng Ukraina, Tiếng Ba Tư, Tiếng Kannada, Tiếng Marathi, Tiếng Hausa, Tiếng Miến Điện, Tiếng Uzbek, Tiếng Azerbaijan, Tiếng Cebuano, Tiếng Malayalam, Tiếng Sindhi, Tiếng Amhara

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="Tiếng Anh" alt="Tiếng Anh" />
  <img src="../assets/flags/cn.svg" width="32" title="Tiếng Trung (Giản thể)" alt="Tiếng Trung (Giản thể)" />
  <img src="../assets/flags/tw.svg" width="32" title="Tiếng Trung (Phồn thể)" alt="Tiếng Trung (Phồn thể)" />
  <img src="../assets/flags/in.svg" width="32" title="Tiếng Hindi" alt="Tiếng Hindi" />
  <img src="../assets/flags/es.svg" width="32" title="Tiếng Tây Ban Nha" alt="Tiếng Tây Ban Nha" />
  <img src="../assets/flags/fr.svg" width="32" title="Tiếng Pháp" alt="Tiếng Pháp" />
  <img src="../assets/flags/sa.svg" width="32" title="Tiếng Ả Rập" alt="Tiếng Ả Rập" />
  <img src="../assets/flags/bd.svg" width="32" title="Tiếng Bengali" alt="Tiếng Bengali" />
  <img src="../assets/flags/pt.svg" width="32" title="Tiếng Bồ Đào Nha" alt="Tiếng Bồ Đào Nha" />
  <img src="../assets/flags/ru.svg" width="32" title="Tiếng Nga" alt="Tiếng Nga" />
  <img src="../assets/flags/pk.svg" width="32" title="Tiếng Urdu" alt="Tiếng Urdu" />
  <img src="../assets/flags/id.svg" width="32" title="Tiếng Indonesia" alt="Tiếng Indonesia" />
  <img src="../assets/flags/de.svg" width="32" title="Tiếng Đức" alt="Tiếng Đức" />
  <img src="../assets/flags/jp.svg" width="32" title="Tiếng Nhật" alt="Tiếng Nhật" />
  <img src="../assets/flags/tr.svg" width="32" title="Tiếng Thổ Nhĩ Kỳ" alt="Tiếng Thổ Nhĩ Kỳ" />
  <img src="../assets/flags/kr.svg" width="32" title="Tiếng Hàn" alt="Tiếng Hàn" />
  <img src="../assets/flags/it.svg" width="32" title="Tiếng Ý" alt="Tiếng Ý" />
  <img src="../assets/flags/nl.svg" width="32" title="Tiếng Hà Lan" alt="Tiếng Hà Lan" />
  <img src="../assets/flags/se.svg" width="32" title="Tiếng Thụy Điển" alt="Tiếng Thụy Điển" />
  <img src="../assets/flags/cz.svg" width="32" title="Tiếng Séc" alt="Tiếng Séc" />
  <img src="../assets/flags/gr.svg" width="32" title="Tiếng Hy Lạp" alt="Tiếng Hy Lạp" />
  <img src="../assets/flags/hu.svg" width="32" title="Tiếng Hungary" alt="Tiếng Hungary" />
  <img src="../assets/flags/il.svg" width="32" title="Tiếng Do Thái" alt="Tiếng Do Thái" />
  <img src="../assets/flags/no.svg" width="32" title="Tiếng Na Uy" alt="Tiếng Na Uy" />
  <img src="../assets/flags/dk.svg" width="32" title="Tiếng Đan Mạch" alt="Tiếng Đan Mạch" />
  <img src="../assets/flags/fi.svg" width="32" title="Tiếng Phần Lan" alt="Tiếng Phần Lan" />
  <img src="../assets/flags/sk.svg" width="32" title="Tiếng Slovakia" alt="Tiếng Slovakia" />
  <img src="../assets/flags/si.svg" width="32" title="Tiếng Slovenia" alt="Tiếng Slovenia" />
  <img src="../assets/flags/is.svg" width="32" title="Tiếng Iceland" alt="Tiếng Iceland" />
  <img src="../assets/flags/vn.svg" width="32" title="Tiếng Việt" alt="Tiếng Việt" />
  <img src="../assets/flags/in.svg" width="32" title="Tiếng Telugu" alt="Tiếng Telugu" />
  <img src="../assets/flags/id.svg" width="32" title="Javanese" alt="Javanese" />
  <img src="../assets/flags/th.svg" width="32" title="Thai" alt="Thai" />
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


## 🚀 Tính năng cốt lõi & Sử dụng

- **Giao diện gốc hiện đại**: GUI trực quan, hỗ trợ chế độ tối/sáng, hoạt ảnh mượt mà, kết xuất hiệu suất cao được hỗ trợ bởi **Skia**.
- **Tích hợp khay hệ thống**: Hỗ trợ khay đầy đủ (chiếm khoảng 10MB bộ nhớ), nhấp đúp để ẩn/hiện và menu chuột phải đầy đủ tính năng.
- **Khởi động thông minh**: Hỗ trợ khởi động cùng Windows, thu nhỏ vào khay (khởi động im lặng với tham số `/silent`), và tự động tắt các bản phân phối khi thoát.
- **Kiểm soát phiên bản toàn diện**: Khởi động, dừng, buộc dừng và gỡ đăng ký chỉ bằng một nhấp. Giám sát trạng thái thời gian thực, xem chi tiết dung lượng ổ đĩa và vị trí tệp.
- **Quản lý bản phân phối**: Đặt làm mặc định, di chuyển vật lý (chuyển VHDX sang ổ đĩa khác), và xuất/sao lưu dưới dạng `.tar` hoặc `.tar.gz`.
- **Tích hợp nhanh**: Mở terminal, VS Code hoặc trình quản lý tệp chỉ bằng một nhấp, hỗ trợ thư mục làm việc tùy chỉnh và script hook khởi chạy.
- **Cài đặt bản phân phối**: Cài đặt các bản phân phối Linux qua Microsoft Store, GitHub, tệp cục bộ (RootFS/VHDX) hoặc mirror trực tuyến (với tự động kiểm tra tốc độ để chọn mirror nhanh nhất và trợ giúp tải RootFS tích hợp).
- **Bảo mật toàn cục**: Sử dụng mutex để đảm bảo an toàn cho các thao tác di chuyển/sao lưu đồng thời, và tự động dọn dẹp gói Appx khi xóa.
- **Chiếm cực ít bộ nhớ**: Tối ưu hóa tài nguyên cao. Khởi động im lặng (khay hệ thống) chỉ chiếm khoảng **10MB** bộ nhớ. Ở chế độ cửa sổ, chiếm khoảng **18MB** (ngôn ngữ tiêu chuẩn như tiếng Anh, tiếng Đức...) đến **38MB** (bộ ký tự lớn như CJK) tùy thuộc vào độ phức tạp phông chữ.
- **Quản lý mạng nâng cao**: Quản lý liền mạch chuyển tiếp cổng (tự động tạo quy tắc tường lửa) và cấu hình proxy HTTP toàn cục, mang lại trải nghiệm kết nối thống nhất.
- **Quản lý thiết bị USB**: Tích hợp sâu với `usbipd-win`, cho phép ràng buộc, gắn và quản lý thiết bị USB cục bộ dễ dàng trên tất cả các phiên bản WSL ngay từ giao diện bảng điều khiển.


## ⚙️ Cấu hình & Nhật ký

Tất cả cấu hình được quản lý qua chế độ xem "Cài đặt":

- Chọn thư mục cài đặt mặc định cho phiên bản WSL mới.
- Cấu hình thư mục nhật ký và mức nhật ký (Error / Warn / Info / Debug / Trace).
- Chọn ngôn ngữ giao diện hoặc theo ngôn ngữ hệ thống.
- Chuyển đổi chế độ tối, và chọn ứng dụng có tự động tắt WSL sau thao tác hay không.
- Cấu hình tần suất kiểm tra cập nhật (hàng ngày, hàng tuần, hai tuần một lần, hàng tháng).
- Bật khởi động cùng Windows (có tính năng tự sửa đường dẫn).
- Cấu hình thu nhỏ vào khay khi khởi động.
- Cấu hình hành vi nút đóng (thu nhỏ vào khay thay vì thoát chương trình).
- Tùy chỉnh thanh bên bằng cách chuyển đổi hiển thị của các tab tính năng cụ thể.

Tệp nhật ký sẽ được ghi vào thư mục nhật ký đã cấu hình, có thể đính kèm khi báo cáo sự cố.


## 🖼️ Ảnh chụp phần mềm

### Giao diện chính (Chế độ tối & sáng)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB & Thu gọn menu
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### Quản lý mạng
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### Thêm phiên bản & Cài đặt
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>
<p align="center">
  <img src="../assets/screenshot/settings-advanced.png" width="48%" />
  <img src="../assets/screenshot/settings-interface.png" width="48%" />
</p>

### Giới thiệu & Quyên góp
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/donate.png" width="48%" />
</p>

## 🎬 Demo thao tác

[Hãy giúp chúng tôi cải thiện! Xem video giới thiệu và chia sẻ suy nghĩ của bạn.](https://github.com/owu/wsl-dashboard/discussions/9)



## 💻 Yêu cầu hệ thống

- Windows 10 hoặc Windows 11 với WSL được bật (khuyến nghị WSL 2).
- Ít nhất một bản phân phối WSL đã được cài đặt, hoặc có quyền cài đặt bản phân phối mới.
- CPU 64-bit; khuyến nghị RAM 4 GB trở lên để sử dụng nhiều bản phân phối mượt mà.

## 📦 Hướng dẫn cài đặt

### Cách 1: Tải tệp nhị phân đã biên dịch sẵn

Cách dễ nhất để bắt đầu là sử dụng phiên bản đã biên dịch:

1. Truy cập trang [GitHub Releases](https://github.com/owu/wsl-dashboard/releases).
2. Tải tệp thực thi `wsldashboard` mới nhất cho Windows.
3. Giải nén (nếu là tệp nén) và chạy `wsldashboard.exe`.

Không cần cài đặt, ứng dụng là chương trình di động một tệp duy nhất.

### Cách 2: Biên dịch từ mã nguồn

Đảm bảo bạn đã cài đặt Rust toolchain (Rust 1.92+ hoặc mới hơn).

1. Clone kho lưu trữ:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. Biên dịch và chạy:

   - Phát triển gỡ lỗi:

     ```powershell
     cargo run
     ```
   - Biên dịch phiên bản phát hành tối ưu bằng script xây dựng:

     > Script xây dựng yêu cầu toolchain `x86_64-pc-windows-msvc`.

     ```powershell
     .\build\portable\build.ps1
     ```


## 🛠️ Công nghệ & Hiệu suất

- **Lõi**: Được triển khai bằng Rust, đảm bảo an toàn bộ nhớ và trừu tượng chi phí bằng không.
- **Giao diện**: Slint với backend kết xuất **Skia** hiệu suất cao.
- **Runtime bất đồng bộ**: Tokio, cho các lệnh hệ thống và I/O không chặn.
- **Điểm nổi bật về hiệu suất**:
  - **Tốc độ phản hồi**: Khởi động gần như tức thời, giám sát trạng thái WSL thời gian thực.
  - **Hiệu quả tài nguyên**: Chiếm dụng tài nguyên cực thấp (xem [Tính năng cốt lõi](#-tính-năng-cốt-lõi--sử-dụng)).
  - **Tính di động**: Phiên bản phát hành tối ưu tạo ra một tệp thực thi duy nhất gọn nhẹ.



## 🤝 Hỗ trợ cộng đồng

Xin chân thành cảm ơn sự hỗ trợ từ các cộng đồng sau:

- [Rust Programming Language](https://www.rust-lang.org) - Ngôn ngữ lập trình mạnh mẽ và an toàn
- [Slint | Declarative GUI for Rust, C++, JavaScript & Python](https://slint.dev) - Framework giao diện hiện đại
- [WSL: Windows Subsystem for Linux](https://github.com/microsoft/WSL) - Windows Subsystem for Linux tuyệt vời
- [Tokio - An asynchronous Rust runtime](https://tokio.rs) - Runtime bất đồng bộ hiệu quả
- [Windows Developer Community](https://developer.microsoft.com/en-us/windows/community) - Cải tiến nền tảng liên tục
- [Reddit](https://www.reddit.com) - Thảo luận và hỗ trợ cộng đồng toàn cầu
- [Hacker News](https://news.ycombinator.com) - Thảo luận và hỗ trợ cộng đồng toàn cầu
- [Linux.do](https://linux.do) - Cộng đồng chuyên gia CNTT phổ biến
- [V2EX](https://www.v2ex.com) - Thảo luận cộng đồng kỹ thuật tiếng Trung

Đóng góp và phản hồi của bạn đã giúp dự án này trở thành hiện thực!


## ❤️ Hỗ trợ dự án

- Dự án này sử dụng giấy phép GPL-3.0, miễn phí cho tất cả người dùng.
- Từ phát triển tính năng, kiểm thử hàng ngày đến sửa lỗi, tất cả công việc đều được thực hiện trong thời gian rảnh. Con đường mã nguồn mở không dễ đi một mình, sự công nhận và hỗ trợ của bạn là động lực để dự án tiếp tục phát triển.
- Nếu công cụ này thực sự hữu ích với bạn, hãy cân nhắc giúp đỡ. Tất cả quyên góp sẽ được dùng cho chi phí máy chủ, phát triển phiên bản và tối ưu hóa tính năng, giúp dự án cập nhật liên tục và phát triển ổn định.
- Mỗi lòng tốt nhỏ đều là ánh sao. Một lần nữa cảm ơn sự thấu hiểu và sự hào phóng của bạn!

Truy cập trang quyên góp của chúng tôi: [https://www.wslui.com/donate/](https://www.wslui.com/donate/)


## ⭐️Ủng hộ từ trái tim

Nếu bạn thấy dự án này hữu ích, tôi rất biết ơn nếu bạn có thể dành cho nó một ngôi sao trên GitHub. Sự ủng hộ của bạn sẽ giúp dự án tiếp cận được nhiều người dùng hơn và tôi rất trân trọng điều đó. Chính sự động viên này thúc đẩy tôi không ngừng tiến bộ.


## 📄 Giấy phép mã nguồn mở

Dự án này được phát hành theo giấy phép GPL-3.0 – xem tệp [LICENSE](../LICENSE) để biết thêm chi tiết.


---

Built with ❤️ for the WSL Community.
