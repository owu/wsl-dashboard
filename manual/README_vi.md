# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

Một bảng điều khiển quản lý phiên bản WSL (Windows Subsystem for Linux) hiện đại, hiệu suất cao, nhẹ và ít tiêu tốn bộ nhớ. Được xây dựng bằng Rust và Slint để mang lại trải nghiệm gốc tuyệt vời.

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

README I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | Tiếng Việt | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 Mục lục
[🌍 Hỗ trợ ngôn ngữ](#-hỗ-trợ-ngôn-ngữ) | [🚀 Tính năng cốt lõi & Sử dụng](#-tính-năng-cốt-lõi--sử-dụng) | [🖼️ Ảnh chụp phần mềm](#️-ảnh-chụp-phần-mềm) | [📦 Hướng dẫn cài đặt](#-hướng-dẫn-cài-đặt) | [📢 Nhóm Cộng đồng Chính thức](#-nhóm-cộng-đồng-chính-thức) | [❤️ Hỗ trợ dự án](#️-hỗ-trợ-dự-án) | [⭐️Ủng hộ từ trái tim](#️ủng-hộ-từ-trái-tim) | [📄 Giấy phép mã nguồn mở](#-giấy-phép-mã-nguồn-mở)

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
- **Lập lịch tác vụ**: Quản lý lập lịch tác vụ linh hoạt, hỗ trợ tự động thực thi các tập lệnh và lệnh trong các phiên bản WSL vào thời gian hoặc khoảng thời gian xác định cho các hoạt động tự động hóa.
- **Kiểm soát phiên bản toàn diện**: Khởi động, dừng, buộc dừng và gỡ đăng ký chỉ bằng một nhấp. Giám sát trạng thái thời gian thực, xem chi tiết dung lượng ổ đĩa và vị trí tệp.
- **Quản lý bản phân phối**: Đặt làm mặc định, di chuyển vật lý (chuyển VHDX sang ổ đĩa khác), và xuất/sao lưu dưới dạng `.tar` hoặc `.tar.gz`.
- **Gắn đĩa**: Trực tiếp gắn và quản lý các đĩa ảo VHDX của phiên bản WSL, hỗ trợ các thao tác gắn, tháo và quản lý đĩa, giúp dễ dàng truyền tệp giữa máy chủ và WSL.
- **Tích hợp nhanh**: Mở terminal, VS Code hoặc trình quản lý tệp chỉ bằng một nhấp, hỗ trợ thư mục làm việc tùy chỉnh và script hook khởi chạy.
- **Cài đặt bản phân phối**: Cài đặt các bản phân phối Linux qua Microsoft Store, GitHub, tệp cục bộ (RootFS/VHDX) hoặc mirror trực tuyến (với tự động kiểm tra tốc độ để chọn mirror nhanh nhất và trợ giúp tải RootFS tích hợp).
- **Bảo mật toàn cục**: Sử dụng mutex để đảm bảo an toàn cho các thao tác di chuyển/sao lưu đồng thời, và tự động dọn dẹp gói Appx khi xóa.
- **Chiếm cực ít bộ nhớ**: Tối ưu hóa tài nguyên cao. Khởi động im lặng (khay hệ thống) chỉ chiếm khoảng **10MB** bộ nhớ. Ở chế độ cửa sổ, chiếm khoảng **18MB** (ngôn ngữ tiêu chuẩn như tiếng Anh, tiếng Đức...) đến **38MB** (bộ ký tự lớn như CJK) tùy thuộc vào độ phức tạp phông chữ.
- **Quản lý mạng nâng cao**: Quản lý liền mạch chuyển tiếp cổng (tự động tạo quy tắc tường lửa) và cấu hình proxy HTTP toàn cục, mang lại trải nghiệm kết nối thống nhất.
- **Quản lý thiết bị USB**: Tích hợp sâu với `usbipd-win`, cho phép ràng buộc, gắn và quản lý thiết bị USB cục bộ dễ dàng trên tất cả các phiên bản WSL ngay từ giao diện bảng điều khiển.
- **Cấu hình và nhật ký**: Quản lý trong Cài đặt: thư mục cài đặt mặc định, thư mục và mức nhật ký, ngôn ngữ giao diện, chế độ tối, tần suất kiểm tra cập nhật, tự khởi động và hành vi khi đóng, cùng thanh bên.
- **Giữ bản phân phối luôn chạy**: Một cú nhấp «Khởi chạy» giúp bản phân phối tiếp tục chạy thay vì bị tắt sau ~60 giây nhàn rỗi; bạn cũng có thể bật «Khởi chạy bản phân phối này khi Windows khởi động».
- **Phím tắt toàn cục**: Đặt phím tắt toàn cục để hiện/ẩn cửa sổ chính hoặc mở ngay terminal của một bản phân phối.


## 🖼️ Ảnh chụp phần mềm
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

Muốn xem bảng điều khiển hoạt động thực tế? Truy cập [trang web chính thức](https://www.wslui.com/) để xem thư viện ảnh chụp màn hình đầy đủ.

## 📦 Hướng dẫn cài đặt

### Cách 1: Truy cập trang web dự án (Khuyến nghị)

Chúng tôi khuyên bạn nên truy cập trang web chính thức để tải xuống, vì trang này cung cấp nhiều đường dẫn mirror để có trải nghiệm mượt mà hơn:

Truy cập [Trang tải xuống](https://www.wslui.com/download/) và chọn mirror phù hợp với khu vực của bạn.

### Cách 2: Cài đặt qua winget

Bạn có thể cài đặt WSLDashboard trực tiếp từ Windows Package Manager (winget), sử dụng tên gọi (moniker) hoặc mã định danh gói đầy đủ:

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

> Mã định danh gói winget là `Owu.WSLDashboard` và tên gọi là `wsl-dashboard` (không phân biệt chữ hoa/thường). Cả hai đều hoạt động.

Để biết thêm thông tin, hãy truy cập [kho lưu trữ cộng đồng WinGet](https://github.com/microsoft/winget-pkgs/tree/master/manifests/o/Owu/WSLDashboard).

### Cách 3: Biên dịch từ mã nguồn

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
     .\build\portable\build.ps1 -Arch x64
     ```


## 📢 Nhóm Cộng đồng Chính thức

Chào mừng bạn đến với các nhóm cộng đồng chính thức của WSL UI! Tham gia kênh của chúng tôi để thảo luận mẹo sử dụng, báo cáo vấn đề, nhận cập nhật mới nhất và kết nối với nhà phát triển và người dùng khác.

<div align="center">

| Nhóm Toàn cầu (Telegram) | Nhóm Tiếng Trung (WeChat) |
| :---: | :---: |
| <img src="https://cdn1.wslui.com/static/images/group/group-telegram.png" width="240" alt="Telegram Group QR Code" /> | <img src="https://cdn1.wslui.com/static/images/group/group-wechat.png" width="240" alt="WeChat Group QR Code" /> |
| Quét để tham gia | Quét để tham gia |

</div>


## ❤️ Hỗ trợ dự án

- Dự án này sử dụng giấy phép GPL-3.0, miễn phí cho tất cả người dùng.
- Từ phát triển tính năng, kiểm thử hàng ngày đến sửa lỗi, tất cả công việc đều được thực hiện trong thời gian rảnh. Con đường mã nguồn mở không dễ đi một mình, sự công nhận và hỗ trợ của bạn là động lực để dự án tiếp tục phát triển.
- Nếu công cụ này thực sự hữu ích với bạn, hãy cân nhắc giúp đỡ. Tất cả quyên góp sẽ được dùng cho chi phí máy chủ, phát triển phiên bản và tối ưu hóa tính năng, giúp dự án cập nhật liên tục và phát triển ổn định.
- Mỗi lòng tốt nhỏ đều là ánh sao. Một lần nữa cảm ơn sự thấu hiểu và sự hào phóng của bạn! [Truy cập trang quyên góp của chúng tôi](https://www.wslui.com/donate/).


## ⭐️Ủng hộ từ trái tim

Nếu bạn thấy dự án này hữu ích, tôi rất biết ơn nếu bạn có thể dành cho nó một ngôi sao trên GitHub. Sự ủng hộ của bạn sẽ giúp dự án tiếp cận được nhiều người dùng hơn và tôi rất trân trọng điều đó. Chính sự động viên này thúc đẩy tôi không ngừng tiến bộ.


## 📄 Giấy phép mã nguồn mở

Dự án này được phát hành theo giấy phép GPL-3.0 – xem tệp [LICENSE](../LICENSE) để biết thêm chi tiết.
