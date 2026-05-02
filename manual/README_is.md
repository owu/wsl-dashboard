# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logó" />
</p>

Nútímalegt, afkastamikið og létt stjórnborð til að sýsla með WSL (Windows Subsystem for Linux) tilvik. Smíðað með Rust og Slint fyrir úrvals upplifun.

---

```diff
Athugið:

- Þessi hugbúnaður er ekki dreift í gegnum Microsoft Store.
- Hvaða forrit sem skráð er þar undir sama nafni er óheimilt og gæti verið falskt.
- Vinsamlegast ekki hlaða því niður til að forðast möguleg svindl.
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

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | Íslenska

---

## 📑 Efnisyfirlit
- [🌍 Tungumálastuðningur](#-tungumálastuðningur)
- [🚀 Helstu eiginleikar og notkun](#-helstu-eiginleikar-og-notkun)
- [⚙️ Stillingar og annálar](#️-stillingar-og-annálar)
- [🖼️ Skjámyndir](#️-skjámyndir)
- [🎬 Sýnikennsla](#-sýnikennsla)
- [💻 Kerfiskröfur](#-kerfiskröfur)
- [📦 Uppsetningarleiðbeiningar](#-uppsetningarleiðbeiningar)
- [🛠️ Tæknistafla og afköst](#️-tæknistafla-og-afköst)
- [⭐️ Gert af ást](#️-gert-af-ást)
- [📄 Leyfi](#-leyfi)

---

## 🌍 Tungumálastuðningur

Enska, einfölduð kínverska, hefðbundin kínverska, hindí, spænska, franska, arabíska, bengalska, portúgalska, rússneska, úrdú, indónesíska, þýska, japanska, tyrkneska, kóreska, ítalska, hollenska, sænska, tékkneska, gríska, ungverska, hebreska, norska, danska, finnska, slóvakíska, slóvenska, íslenska.

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="Enska" alt="Enska" />
  <img src="../assets/flags/cn.svg" width="32" title="Kínverska (Einfölduð)" alt="Kínverska (Einfölduð)" />
  <img src="../assets/flags/tw.svg" width="32" title="Kínverska (Hefðbundin)" alt="Kínverska (Hefðbundin)" />
  <img src="../assets/flags/in.svg" width="32" title="Hindí" alt="Hindí" />
  <img src="../assets/flags/es.svg" width="32" title="Spænska" alt="Spænska" />
  <img src="../assets/flags/fr.svg" width="32" title="Franska" alt="Franska" />
  <img src="../assets/flags/sa.svg" width="32" title="Arabíska" alt="Arabíska" />
  <img src="../assets/flags/bd.svg" width="32" title="Bengalska" alt="Bengalska" />
  <img src="../assets/flags/pt.svg" width="32" title="Portúgalska" alt="Portúgalska" />
  <img src="../assets/flags/ru.svg" width="32" title="Rússneska" alt="Rússneska" />
  <img src="../assets/flags/pk.svg" width="32" title="Úrdú" alt="Úrdú" />
  <img src="../assets/flags/id.svg" width="32" title="Indónesíska" alt="Indónesíska" />
  <img src="../assets/flags/de.svg" width="32" title="Þýska" alt="Þýska" />
  <img src="../assets/flags/jp.svg" width="32" title="Japanska" alt="Japanska" />
  <img src="../assets/flags/tr.svg" width="32" title="Tyrkneska" alt="Tyrkneska" />
  <img src="../assets/flags/kr.svg" width="32" title="Kóreska" alt="Kóreska" />
  <img src="../assets/flags/it.svg" width="32" title="Ítalska" alt="Ítalska" />
  <img src="../assets/flags/nl.svg" width="32" title="Hollenska" alt="Hollenska" />
  <img src="../assets/flags/se.svg" width="32" title="Sænska" alt="Sænska" />
  <img src="../assets/flags/cz.svg" width="32" title="Tékkneska" alt="Tékkneska" />
  <img src="../assets/flags/gr.svg" width="32" title="Gríska" alt="Gríska" />
  <img src="../assets/flags/hu.svg" width="32" title="Ungverska" alt="Ungverska" />
  <img src="../assets/flags/il.svg" width="32" title="Hebreska" alt="Hebreska" />
  <img src="../assets/flags/no.svg" width="32" title="Norska" alt="Norska" />
  <img src="../assets/flags/dk.svg" width="32" title="Danska" alt="Danska" />
  <img src="../assets/flags/fi.svg" width="32" title="Finnska" alt="Finnska" />
  <img src="../assets/flags/sk.svg" width="32" title="Slóvakíska" alt="Slóvakíska" />
  <img src="../assets/flags/si.svg" width="32" title="Slóvenska" alt="Slóvenska" />
  <img src="../assets/flags/is.svg" width="32" title="Íslenska" alt="Íslenska" />
</p>


## 🚀 Helstu eiginleikar og notkun

- **Nútímalegt viðmót**: Innsæi GUI með stuðningi fyrir dökka og ljósa stillingu, mjúkar hreyfingar og afkastamikil teiknun knúin af **Skia**.
- **Samþætting við kerfisbakka (Tray)**: Fullur stuðningur við að fela forritið í kerfisbakkanum (~10MB vinnsluminni í notkun), tvísmella til að sýna/fela og hagnýtur hægri-smellismatseðill.
- **Snjöll ræsistýring**: Stilla stjórnborðið til að ræsast með Windows, lágmarkast í bakka (hljóðlaus ræsing með `/silent`) og slökkva sjálfkrafa á Linux-kerfum við lokun.
- **Víðtæk tilvikastýring**: Ræsa, stöðva, loka og afskrá með einum smelli. Rauntímavöktun á stöðu og nákvæmar upplýsingar um disknotkun og staðsetningu skráa.
- **Umsjón með kerfum**: Setja sem sjálfgefið, flutningur (færa VHDX á aðra drif) og útflutningur/afritun á `.tar` eða `.tar.gz` sniði.
- **Hröð samþætting**: Ræsa beint í Terminal, VS Code eða skráakönnuð með sérsniðnum vinnuefnum og ræsiforritaskriftum.
- **Snjöll uppsetning**: Setja upp frá Microsoft Store, GitHub eða staðbundnum skrám (RootFS/VHDX). Innbyggt RootFS niðurhalshjálpartæki fylgir.
- **Alþjóðlegt öryggi**: Mutex-lásar fyrir örugga samtímaflutninga/afritun og sjálfvirk Appx-hreinsun við eyðingu.
- **Mjög lítil minnisnotkun**: Mjög fínstillt fyrir skilvirkni. Hljóðlaus ræsing (í bakka) notar aðeins um **~10MB** vinnsluminni. Notkun í gluggaham fer eftir leturgerðum: **~18MB** fyrir stöðluð tungumál og **~38MB** fyrir tungumál með stóra táknasett (kínverska, japanska, kóreska).
- **Ítarleg netkerfi**: Óaðfinnanleg stjórnun á áframsendingu gátta (með sjálfvirkri gerð eldveggsreglna) og alþjóðleg uppsetning HTTP-umboðsþjóns fyrir sameinaða tengingu.
- **Stjórnun USB-tækja**: Full samþætting við `usbipd-win` til að binda, tengja og stjórna staðbundnum USB-tækjum áreynslulaust í öllum WSL-tilvikum þínum beint af mælaborðinu.


## ⚙️ Stillingar og annálar

Öllum stillingum er stjórnað í gegnum stillingavalmyndina:

- Velja sjálfgefna uppsetningarmöppu fyrir ný WSL tilvik.
- Stilla möppu fyrir annála og skráningarstig (Error / Warn / Info / Debug / Trace).
- Velja tungumál viðmóts eða láta það fylgja kerfistungumáli.
- Skipta yfir í dökka stillingu og velja hvort forritið megi slökkva sjálfkrafa á WSL eftir aðgerðir.
- Stilla hversu oft forritið leitar að uppfærslum (daglega, vikulega, hálfsmánaðarlega, mánaðarlega).
- Virkja sjálfvirka ræsingu með kerfinu (með sjálfvirkri slóðaviðgerð).
- Stilltu forritið á að lágmarkast í kerfisbakka við ræsingu.
- Stilla lokunarhnappinn á að lágmarka forritið í bakka í stað þess að hætta.
- Sérsníddu hliðarstikuna með því að skipta um sýnileika tiltekinna eiginleikaflipa.

Annálaskrár eru skrifaðar í stillta möppu og má láta þær fylgja með þegar tilkynnt er um vandamál.


## 🖼️ Skjámyndir

### Heimaskjár (Ljós og dökk stilling)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB og saman dreginn matseðill
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### net
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### Bæta við tilviki og Stillingar
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>

### Um forritið 
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
</p>

## 🎬 Sýnikennsla

Hér að neðan má sjá WSL Dashboard í verki:

![WSL Dashboard Demo](../assets/screenshot/demo.gif)



## 💻 Kerfiskröfur

- Windows 10 eða Windows 11 með WSL virkt (mælt er með WSL 2).
- Minnst eitt WSL kerfi uppsett, eða leyfi til að setja upp ný.
- 64-bita örgjörvi; mælt er með 4 GB vinnsluminni eða meira fyrir snurðulausa notkun.

## 📦 Uppsetningarleiðbeiningar

### Valkostur 1: Sækja tilbúna keyrsluskrá

Auðveldasta leiðin til að byrja er að nota tilbúna útgáfu:

1. Farðu á [GitHub Releases](https://github.com/owu/wsl-dashboard/releases) síðuna.
2. Sæktu nýjustu `wsldashboard` keyrsluskrána fyrir Windows.
3. Taktu úr þjöppun (ef við á) og keyrðu `wsldashboard.exe`.

Engin uppsetning er nauðsynleg; forritið er ein sjálfstæð keyrsluskrá.

### Valkostur 2: Smíða úr frumkóða

Gakktu úr skugga um að þú hafir Rust tækjapakkann (Rust 1.92+ eða nýrri) uppsettan.

1. Afritaðu geymsluna:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. Smíða og keyra:

   - Fyrir þróun:

     ```powershell
     cargo run
     ```
   - Fínstillt útgáfusmíð með því að nota smíðaskriftuna:

     > Smíðaskriftan krefst `x86_64-pc-windows-msvc` tækjapakkans.

     ```powershell
     .\build\scripts\build.ps1
     ```


## 🛠️ Tæknistafla og afköst

- **Kjarni**: Útfærður í Rust fyrir minnisöryggi og hámarksafköst.
- **Viðmótskerfi**: Slint með hágæða **Skia** teiknivél.
- **Ósamhæfð keyrsla**: Tokio fyrir kerfisskipanir og inntak/úttak án tafa.
- **Helstu kostir**:
  - **Viðbragðsflýtir**: Ræstist nánast samstundis og fylgist með stöðu WSL í rauntíma.
  - **Skilvirkni**: Einstaklega lítil auðlindanotkun (sjá [Helstu eiginleikar](#-helstu-eiginleikar-og-notkun) fyrir nánari upplýsingar).
  - **Sveigjanleiki**: Fínstillt útgáfusmíð skilar einni þéttri keyrsluskrá.



## ⭐️ Gert af ást

Ef þér finnst þetta verkefni gagnlegt væri ég þakklátur ef þú gætir gefið því stjörnu á GitHub. Stuðningur þinn hjálpar því að ná til breiðari hóps og er mikils metinn. Það er þessi hvatning sem dregur mig áfram í þróuninni.

## 📄 Leyfi

Þetta verkefni er gefið út undir GPL-3.0 leyfinu – sjá [LICENSE](../LICENSE) skrána fyrir nánari upplýsingar.

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

