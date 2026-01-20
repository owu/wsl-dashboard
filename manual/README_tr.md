# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" />
</p>

WSL (Windows Subsystem for Linux) örneklerini yönetmek için modern, yüksek performanslı ve hafif bir pano. Birinci sınıf bir yerel deneyim için Rust ve Slint ile oluşturulmuştur.

---

[![Rust](https://img.shields.io/badge/rust-v1.92+-orange.svg)](https://www.rust-lang.org)
[![Slint](https://img.shields.io/badge/UI-Slint-blue.svg)](https://slint.dev)
[![License](https://img.shields.io/badge/license-GPL--3.0-blue.svg)](LICENSE)

I18N: [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [English](../README.md) | [日本語](./README_ja.md) | [Français](./README_fr.md) | [Español](./README_es.md) | [Русский](./README_ru.md) | [Português](./README_pt.md) | [Deutsch](./README_de.md) | [Italiano](./README_it.md) | Türkçe | [Bahasa Indonesia](./README_id.md) | [हिन्दी](./README_hi.md) | [বাংলা](./README_bn.md)

---

## 🖼️ Ekran Görüntüleri

### Ana Sayfa (Açık ve Koyu Mod)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

### Örnek Ekle ve Ayarlar
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>

### Hakkında ve Menü daraltma
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

## 🎬 Çalışma Gösterimi

Aşağıda WSL Dashboard'un çalışırken bir gösterimi yer almaktadır:

![WSL Dashboard Gösterimi](../assets/screenshot/demo.gif)

## 🚀 Temel Özellikler

- Koyu mod desteği ve akıcı animasyonlarla sezgisel GUI.
- Tüm WSL dağıtımlarınız için tek tıkla yönetim (Başlat, Durdur, Sonlandır, Kaydını Sil).
- Dağıtım terminallerine, VS Code'a ve Dosya Gezgini'ne hızlı erişim.
- Gerçek zamanlı WSL örneği durumu izleme ve görüntüleme.
- `.tar` veya sıkıştırılmış `.tar.gz` arşivlerine dışa aktarma ve yedekleme.
- Yedeklerden veya mevcut dağıtımlardan örnekleri içe aktarma ve kopyalama.
- C: sürücüsünde yer kazanmak için büyük WSL örneklerini (VHDX taşıma) diğer disklere taşıma.
- Microsoft Store veya GitHub'dan akıllı dağıtım kurulumu.
- Manuel kurulumlar için yerleşik RootFS indirme yardımcısı.
- VHDX dosya konumu, sanal disk boyutu ve gerçek disk kullanımı hakkında detaylı bilgiler.

## Sistem Gereksinimleri

- WSL etkinleştirilmiş Windows 10 veya Windows 11 (WSL 2 önerilir).
- En az bir WSL dağıtımı yüklü veya yenilerini yükleme izni olmalı.
- 64 bit CPU; sorunsuz çoklu dağıtım kullanımı için 4 GB RAM veya daha fazlası önerilir.

## 📦 Kurulum

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

   - Optimize edilmiş yayın derlemesi:

     ```powershell
     cargo run --release
     ```

   - Derleme betiğini kullanma (yayın ikili dosyaları üretmek için önerilir):

     > Derleme betiği `x86_64-pc-windows-gnu` araç zincirini gerektirir.

     ```powershell
     .\build\scripts\build.ps1
     ```

## 🧭 Kullanım Özeti

- **Mevcut dağıtımları yönetin**: Ana görünümden herhangi bir WSL dağıtımını başlatın, durdurun, sonlandırın veya kaydını silin.
- **Araçları hızlıca açın**: Bir dağıtımı terminalinizde, VS Code'da veya Dosya Gezgini'nde tek bir tıklamayla başlatın.
- **Yeni örnekler oluşturun**: Microsoft Store'dan yüklemek, RootFS imajlarını indirmek veya mevcut dağıtımları kopyalamak için Örnek Ekle görünümünü kullanın.
- **Yedekleme ve geri yükleme**: Dağıtımları `.tar` / `.tar.gz` arşivlerine dışa aktarın ve daha sonra veya başka bir makinede içe aktarın.
- **Depolama konumunu taşıyın**: C: sürücüsünde yer kazanmak için büyük VHDX dosyalarını farklı bir sürücüye taşıyın.
- **Durumu izleyin**: WSL Dashboard çalışırken gerçek zamanlı dağıtım durumunu ve depolama kullanımını takip edin.

## ⚙️ Yapılandırma ve Günlükler

Tüm yapılandırmalar Ayarlar görünümü üzerinden yönetilir:

- Yeni WSL örnekleri için varsayılan kurulum dizinini seçin.
- Günlük dizinini ve günlük seviyesini (Error / Warn / Info / Debug / Trace) yapılandırın.
- Kullanıcı arayüzü dilini seçin veya sistem dilini takip etmesini sağlayın.
- Koyu modu ve uygulamanın işlemlerden sonra WSL'yi otomatik olarak kapatıp kapatamayacağını yapılandırın.
- Uygulamanın güncellemeleri ne sıklıkla kontrol edeceğini (günlük, haftalık, iki haftalık, aylık) yapılandırın.

Günlük dosyaları yapılandırılan günlük dizinine yazılır ve sorunları bildirirken eklenebilir.

## 🛠️ Teknoloji Yığını ve Performans

- **Çekirdek**: Bellek güvenliği ve sıfır maliyetli soyutlamalar için Rust ile uygulanmıştır.
- **UI çerçevesi**: Slint, modern bir GPU hızlandırmalı UI araç seti (arka uç: `winit`).
- **Asenkron çalışma zamanı**: Yüksek eşzamanlı, engelleyici olmayan sistem komutları ve G/Ç için Tokio.
- **Performans**:
  - **Bellek kullanımı**: Genellikle 60–80 MB RAM civarındadır.
  - **Tepkisellik**: Akış kullanarak neredeyse anında başlatma ve gerçek zamanlı WSL durum güncellemeleri.
  - **İkili boyutu**: Optimize edilmiş yayın derlemesi tek bir kompakt yürütülebilir dosya üretir.

## 🌍 Dil Desteği

Aşağıdaki diller için tam uluslararasılaştırma desteği sağlanmaktadır:

| Dil | Kod | Emoji |
| :--- | :---: | :---: |
| Basitleştirilmiş Çince | `zh-CN` | 🇨🇳 |
| Geleneksel Çince | `zh-TW` | 🇭🇰 / 🇹🇼 |
| İngilizce | `en` | 🇺🇸 |
| Japonca | `ja` | 🇯🇵 |
| Fransızca | `fr` | 🇫🇷 |
| İspanyolca | `es` | 🇪🇸 |
| Rusça | `ru` | 🇷🇺 |
| Portekizce | `pt` | 🇵🇹 |
| Almanca | `de` | 🇩🇪 |
| İtalyanca | `it` | 🇮🇹 |
| Türkçe | `tr` | 🇹🇷 |
| Endonezce | `id` | 🇮🇩 |
| Hintçe | `hi` | 🇮🇳 |
| Bengalce | `bn` | 🇧🇩 |

## 📄 Lisans

Bu proje GPL-3.0 altında lisanslanmıştır – detaylar için [LICENSE](LICENSE) dosyasına bakın.

---

Built with ❤️ for the WSL Community.
