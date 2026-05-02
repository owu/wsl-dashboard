# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

WSL (Windows Subsystem for Linux) örneklerini yönetmek için modern, yüksek performanslı ve hafif bir pano. Birinci sınıf bir yerel deneyim için Rust ve Slint ile oluşturulmuştur.

---

```diff
Bilgi:

- Bu yazılım Microsoft Store üzerinden dağıtılmamaktadır.
- Aynı ad altında orada listelenen tüm uygulamalar yetkisizdir ve sahte olabilir.
- Dolandırıcılıktan kaçınmak için lütfen indirmeyin.
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

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | Türkçe | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md)

---

## 📑 İçindekiler
- [🌍 Dil Desteği](#-dil-desteği)
- [🚀 Temel Özellikler ve Kullanım](#-temel-özellikler-ve-kullanım)
- [⚙️ Yapılandırma ve Günlükler](#️-yapılandırma-ve-günlükler)
- [🖼️ Ekran Görüntüleri](#️-ekran-görüntüleri)
- [🎬 Çalışma Gösterimi](#-çalışma-gösterimi)
- [💻 Sistem Gereksinimleri](#-sistem-gereksinimleri)
- [📦 Kurulum Kılavuzu](#-kurulum-kılavuzu)
- [🛠️ Teknoloji Yığını ve Performans](#️-teknoloji-yığını-ve-performans)
- [⭐️ Emek ve sevgi işi](#️-emek-ve-sevgi-işi)
  - [📄 Lisans](#-lisans)

---

## 🌍 Dil Desteği

İngilizce, Çince, Çince, Hintçe, İspanyolca, Fransızca, Arabic, Bengalce, Portekizce, Rusça, Urdu, Endonezce, Almanca, Japonca, Türkçe, Korean, İtalyanca, Dutch, Swedish, Czech, Greek, Hungarian, Hebrew, Norwegian, Danish, Finnish, Slovak, Slovenian, Icelandic

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="İngilizce" alt="İngilizce" />
  <img src="../assets/flags/cn.svg" width="32" title="Çince (Basitleştirilmiş)" alt="Çince (Basitleştirilmiş)" />
  <img src="../assets/flags/tw.svg" width="32" title="Çince (Geleneksel)" alt="Çince (Geleneksel)" />
  <img src="../assets/flags/in.svg" width="32" title="Hintçe" alt="Hintçe" />
  <img src="../assets/flags/es.svg" width="32" title="İspanyolca" alt="İspanyolca" />
  <img src="../assets/flags/fr.svg" width="32" title="Fransızca" alt="Fransızca" />
  <img src="../assets/flags/sa.svg" width="32" title="Arapça" alt="Arapça" />
  <img src="../assets/flags/bd.svg" width="32" title="Bengalce" alt="Bengalce" />
  <img src="../assets/flags/pt.svg" width="32" title="Portekizce" alt="Portekizce" />
  <img src="../assets/flags/ru.svg" width="32" title="Rusça" alt="Rusça" />
  <img src="../assets/flags/pk.svg" width="32" title="Urduca" alt="Urduca" />
  <img src="../assets/flags/id.svg" width="32" title="Endonezce" alt="Endonezce" />
  <img src="../assets/flags/de.svg" width="32" title="Almanca" alt="Almanca" />
  <img src="../assets/flags/jp.svg" width="32" title="Japonca" alt="Japonca" />
  <img src="../assets/flags/tr.svg" width="32" title="Türkçe" alt="Türkçe" />
  <img src="../assets/flags/kr.svg" width="32" title="Korece" alt="Korece" />
  <img src="../assets/flags/it.svg" width="32" title="İtalyanca" alt="İtalyanca" />
  <img src="../assets/flags/nl.svg" width="32" title="Felemenkçe" alt="Felemenkçe" />
  <img src="../assets/flags/se.svg" width="32" title="İsveççe" alt="İsveççe" />
  <img src="../assets/flags/cz.svg" width="32" title="Çekçe" alt="Çekçe" />
  <img src="../assets/flags/gr.svg" width="32" title="Yunanca" alt="Yunanca" />
  <img src="../assets/flags/hu.svg" width="32" title="Macarca" alt="Macarca" />
  <img src="../assets/flags/il.svg" width="32" title="İbranice" alt="İbranice" />
  <img src="../assets/flags/no.svg" width="32" title="Norveççe" alt="Norveççe" />
  <img src="../assets/flags/dk.svg" width="32" title="Danca" alt="Danca" />
  <img src="../assets/flags/fi.svg" width="32" title="Fince" alt="Fince" />
  <img src="../assets/flags/sk.svg" width="32" title="Slovakça" alt="Slovakça" />
  <img src="../assets/flags/si.svg" width="32" title="Slovence" alt="Slovence" />
  <img src="../assets/flags/is.svg" width="32" title="İzlandaca" alt="İzlandaca" />
</p>


## 🚀 Temel Özellikler ve Kullanım

- **Modern Yerel UI**: Sezgisel GUI, koyu/açık mod desteği, akıcı animasyonlar ve **Skia** ile güçlendirilmiş yüksek performanslı işleme.
- **Sistem Tepsisi Entegrasyonu**: Tepsi simgesi haline gelme desteği (~10MB RAM kullanımı), gizle/göster için çift tıklama ve işlevsel sağ tık menüsü.
- **Akıllı Başlangıç**: Panoyu Windows ile başlayacak, tepside başlayacak (`/silent` seçeneği ile sessiz mod) ve çıkışta dağıtımları otomatik kapatacak şekilde yapılandırın.
- **Kapsamlı Örnek Kontrolü**: Tek tıkla başlatma, durdurma, sonlandırma ve kaydını silme. Gerçek zamanlı durum izleme, disk kullanımı ve dosya konumu hakkında detaylı bilgiler.
- **Dağıtım Yönetimi**: Varsayılan olarak ayarla, taşıma (VHDX'i diğer sürücülere taşıma) ve `.tar` veya `.tar.gz` formatında dışa aktarma/kopyalama.
- **Hızlı Entegrasyon**: Özelleştirilebilir çalışma dizinleri ve başlangıç betiği kancaları ile Terminal, VS Code veya Dosya Gezgini'ni anında başlatın.
- **Akıllı Kurulum**: Microsoft Store, GitHub veya yerel dosyalardan (RootFS/VHDX) yükleyin. Yerleşik RootFS indirme yardımcısı dahildir.
- **Küresel Güvenlik**: Güvenli eşzamanlı taşıma/yedekleme işlemleri için mutex kilitleri ve kaldırma sırasında otomatik Appx temizliği.
- **Süper Düşük Bellek Kullanımı**: Verimlilik için yüksek düzeyde optimize edilmiştir. Sessiz başlangıç (tepsi) yalnızca **~10MB** RAM kullanır. Pencere modu kullanımı yazı tipi karmaşıklığına göre değişir: Standart diller için **~18MB** ve büyük karakter setli diller (Çince, Japonca, Korece) için **~38MB**.
- **Gelişmiş Ağ**: Kesintisiz bağlantı noktası yönlendirme yönetimi (otomatik güvenlik duvarı kuralı oluşturma ile) ve birleşik bağlantı için küresel HTTP proxy yapılandırması.
- **USB Cihaz Yönetimi**: Tüm WSL örneklerinizde yerel USB cihazlarını doğrudan kontrol panelinden zahmetsizce bağlamak, eklemek ve yönetmek için `usbipd-win` ile tam entegrasyon.


## ⚙️ Yapılandırma ve Günlükler

Tüm yapılandırmalar Ayarlar görünümü üzerinden yönetilir:

- Yeni WSL örnekleri için varsayılan kurulum dizinini seçin.
- Günlük dizinini ve günlük seviyesini (Error / Warn / Info / Debug / Trace) yapılandırın.
- Kullanıcı arayüzü dilini seçin veya sistem dilini takip etmesini sağlayın.
- Koyu modu ve işlemlerden sonra WSL'yi otomatik kapatmayı yapılandırın.
- Güncelleme kontrol sıklığını ayarlayın (günlük, haftalık, iki haftalık, aylık).
- Sistem başlangıcında otomatik başlamayı etkinleştirin (otomatik yol onarımı ile).
- Uygulamayı başlangıçta sistem tepsisine küçülecek şekilde ayarlayın.
- Kapat düğmesini uygulamadan çıkmak yerine tepsiye küçülecek şekilde yapılandırın.
- Belirli özellik sekmelerinin görünürlüğünü değiştirerek kenar çubuğunu özelleştirin.

Günlük dosyaları yapılandırılan günlük dizinine yazılır ve sorunları bildirirken eklenebilir.


## 🖼️ Ekran Görüntüleri

### Ana Sayfa (Koyu ve Açık Mod)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB ve Menü daraltma
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### ağ
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### Örnek Ekle ve Ayarlar
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>

### Hakkında 
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
</p>

## 🎬 Çalışma Gösterimi

Aşağıda WSL Dashboard'un çalışırken bir gösterimi yer almaktadır:

![WSL Dashboard Gösterimi](../assets/screenshot/demo.gif)



## 💻 Sistem Gereksinimleri

- WSL etkinleştirilmiş Windows 10 veya Windows 11 (WSL 2 önerilir).
- En az bir WSL dağıtımı yüklü veya yenilerini yükleme izni olmalı.
- 64 bit CPU; sorunsuz çoklu dağıtım kullanımı için 4 GB RAM veya daha fazlası önerilir.

## 📦 Kurulum Kılavuzu

### Seçenek 1: Önceden derlenmiş ikiliyi indirin

Başlamanın en kolay yolu önceden derlenmiş sürümü kullanmaktır:

1. [GitHub Releases](https://github.com/owu/wsl-dashboard/releases) sayfasına gidin.
2. Windows için en son `wsldashboard` yürütülebilir dosyasını indirin.
3. Paket dosyası ise çıkartın ve `wsldashboard.exe` dosyasını çalıştırın.

Yükleyici gerekmez; uygulama tek bir taşınabilir ikili dosyadır.

### Seçenek 2: Kaynaktan derleyin

Rust araç zincirinin (Rust 1.92+ veya daha yeni) kurulu olduğundan emin olun.

1. Depoyu klonlayın:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. Derleyin ve çalıştırın:

   - Geliştirme için:

     ```powershell
     cargo run
     ```
   - Betik aracılığıyla optimize edilmiş yayın derlemesi oluşturun:

     > Derleme betiği `x86_64-pc-windows-msvc` araç zincirini gerektirir.

     ```powershell
     .\build\scripts\build.ps1
     ```


## 🛠️ Teknoloji Yığını ve Performans

- **Çekirdek**: Bellek güvenliği ve sıfır maliyetli soyutlamalar için Rust ile uygulanmıştır.
- **UI Çerçevesi**: Yüksek performanslı **Skia** işleme arka ucuna sahip Slint.
- **Asenkron Çalışma Zamanı**: Engelleyici olmayan sistem komutları ve G/Ç için Tokio.
- **Performans Önemli Noktaları**:
  - **Tepkisellik**: Neredeyse anında başlatma ve gerçek zamanlı WSL durum izleme.
  - **Verimlilik**: Süper düşük kaynak kullanımı (ayrıntılar için [Temel Özellikler](#-temel-özellikler-ve-kullanım) bölümüne bakın).
  - **Taşınabilirlik**: Optimize edilmiş yayın derlemesi tek bir kompakt yürütülebilir dosya üretir.



## ⭐️ Emek ve sevgi işi

Eğer bu projeyi faydalı bulduysanız, GitHub'da bir yıldız bırakırsanız minnettar olurum. Desteğiniz projenin daha geniş bir kitleye ulaşmasına yardımcı olur ve derinden takdir edilir. Beni yeni özellikler eklemeye motive eden şey bu teşviktir.

## 📄 Lisans

Bu proje GPL-3.0 altında lisanslanmıştır – detaylar için [LICENSE](../LICENSE) dosyasına bakın.

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

