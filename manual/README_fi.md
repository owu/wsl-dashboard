# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

Moderni, suorituskykyinen, kevyt ja vähän muistia vievä WSL-instanssien (Windows Subsystem for Linux) hallintapaneeli. Rakennettu Rustilla ja Slintillä ensiluokkaista natiivikokemusta varten.

---

```diff
Ilmoitus:

- WSL Dashboard:a ei jaeta Microsoft Storen kautta.
- Mikä tahansa sovellus, joka on listattu siellä nimellä "WSL Dashboard", on luvaton ja saattaa olla väärennös.
- Älä lataa sitä välttääksesi mahdolliset huijaukset.
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

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | Suomi | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 Sisällysluettelo
- [🌍 Kielituki](#-kielituki)
- [🚀 Tärkeimmät ominaisuudet ja käyttö](#-tärkeimmät-ominaisuudet-ja-käyttö)
- [⚙️ Asetukset ja lokit](#️-asetukset-ja-lokit)
- [🖼️ Kuvakaappaukset](#️-kuvakaappaukset)
- [🎬 Demonstraatio](#-demonstraatio)
- [💻 Järjestelmävaatimukset](#-järjestelmävaatimukset)
- [📦 Asennusopas](#-asennusopas)
- [🛠️ Teknologiapina ja suorituskyky](#️-teknologiapina-ja-suorituskyky)
- [🤝 Yhteisön tuki](#-yhteisön-tuki)
- [❤️ Tue tätä projektia](#️-tee-tätä-projektia)
- [⭐️ Sydämen asia](#️-sydämen-asia)
- [📄 Lisenssi](#-lisenssi)

---

## 🌍 Kielituki

Englanti, Kiina (Yksinkertaistettu), Kiina (Perinteinen), Hindi, Espanja, Ranska, Arabia, Bengali, Portugali, Venäjä, Urdu, Indonesia, Saksa, Japani, Turkki, Korea, Italia, Hollanti, Ruotsi, Tšekki, Kreikka, Unkari, Heprea, Norja, Tanska, Suomi, Slovakki, Sloveeni, Islanti, Vietnam, Telugu, Jaava, Thai, Tamili, Filipino, Pandžabi, Malesia, Puola, Ukraina, Persia, Kannada, Marathi, Hausa, Burma, Uzbeki, Azerbaidžani, Cebuano, Malajalam, Sindhi, Amhara

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="Englanti" alt="Englanti" />
  <img src="../assets/flags/cn.svg" width="32" title="Kiina (Yksinkertaistettu)" alt="Kiina (Yksinkertaistettu)" />
  <img src="../assets/flags/tw.svg" width="32" title="Kiina (Perinteinen)" alt="Kiina (Perinteinen)" />
  <img src="../assets/flags/in.svg" width="32" title="Hindi" alt="Hindi" />
  <img src="../assets/flags/es.svg" width="32" title="Espanja" alt="Espanja" />
  <img src="../assets/flags/fr.svg" width="32" title="Ranska" alt="Ranska" />
  <img src="../assets/flags/sa.svg" width="32" title="Arabia" alt="Arabia" />
  <img src="../assets/flags/bd.svg" width="32" title="Bengali" alt="Bengali" />
  <img src="../assets/flags/pt.svg" width="32" title="Portugali" alt="Portugali" />
  <img src="../assets/flags/ru.svg" width="32" title="Venäjä" alt="Venäjä" />
  <img src="../assets/flags/pk.svg" width="32" title="Urdu" alt="Urdu" />
  <img src="../assets/flags/id.svg" width="32" title="Indonesia" alt="Indonesia" />
  <img src="../assets/flags/de.svg" width="32" title="Saksa" alt="Saksa" />
  <img src="../assets/flags/jp.svg" width="32" title="Japani" alt="Japani" />
  <img src="../assets/flags/tr.svg" width="32" title="Turkki" alt="Turkki" />
  <img src="../assets/flags/kr.svg" width="32" title="Korea" alt="Korea" />
  <img src="../assets/flags/it.svg" width="32" title="Italia" alt="Italia" />
  <img src="../assets/flags/nl.svg" width="32" title="Hollanti" alt="Hollanti" />
  <img src="../assets/flags/se.svg" width="32" title="Ruotsi" alt="Ruotsi" />
  <img src="../assets/flags/cz.svg" width="32" title="Tšekki" alt="Tšekki" />
  <img src="../assets/flags/gr.svg" width="32" title="Kreikka" alt="Kreikka" />
  <img src="../assets/flags/hu.svg" width="32" title="Unkari" alt="Unkari" />
  <img src="../assets/flags/il.svg" width="32" title="Heprea" alt="Heprea" />
  <img src="../assets/flags/no.svg" width="32" title="Norja" alt="Norja" />
  <img src="../assets/flags/dk.svg" width="32" title="Tanska" alt="Tanska" />
  <img src="../assets/flags/fi.svg" width="32" title="Suomi" alt="Suomi" />
  <img src="../assets/flags/sk.svg" width="32" title="Slovakki" alt="Slovakki" />
  <img src="../assets/flags/si.svg" width="32" title="Sloveeni" alt="Sloveeni" />
  <img src="../assets/flags/is.svg" width="32" title="Islanti" alt="Islanti" />
  <img src="../assets/flags/vn.svg" width="32" title="Vietnam" alt="Vietnam" />
  <img src="../assets/flags/in.svg" width="32" title="Telugu" alt="Telugu" />
  <img src="../assets/flags/id.svg" width="32" title="Jaava" alt="Jaava" />
  <img src="../assets/flags/th.svg" width="32" title="Thai" alt="Thai" />
  <img src="../assets/flags/in.svg" width="32" title="Tamili" alt="Tamili" />
  <img src="../assets/flags/ph.svg" width="32" title="Filipino" alt="Filipino" />
  <img src="../assets/flags/pk.svg" width="32" title="Pandžabi" alt="Pandžabi" />
  <img src="../assets/flags/my.svg" width="32" title="Malesia" alt="Malesia" />
  <img src="../assets/flags/pl.svg" width="32" title="Puola" alt="Puola" />
  <img src="../assets/flags/ua.svg" width="32" title="Ukraina" alt="Ukraina" />
  <img src="../assets/flags/ir.svg" width="32" title="Persia" alt="Persia" />
  <img src="../assets/flags/in.svg" width="32" title="Kannada" alt="Kannada" />
  <img src="../assets/flags/in.svg" width="32" title="Marathi" alt="Marathi" />
  <img src="../assets/flags/ng.svg" width="32" title="Hausa" alt="Hausa" />
  <img src="../assets/flags/mm.svg" width="32" title="Burma" alt="Burma" />
  <img src="../assets/flags/uz.svg" width="32" title="Uzbeki" alt="Uzbeki" />
  <img src="../assets/flags/az.svg" width="32" title="Azerbaidžani" alt="Azerbaidžani" />
  <img src="../assets/flags/ph.svg" width="32" title="Cebuano" alt="Cebuano" />
  <img src="../assets/flags/in.svg" width="32" title="Malajalam" alt="Malajalam" />
  <img src="../assets/flags/pk.svg" width="32" title="Sindhi" alt="Sindhi" />
  <img src="../assets/flags/et.svg" width="32" title="Amhara" alt="Amhara" />
</p>


## 🚀 Tärkeimmät ominaisuudet ja käyttö

- **Moderni natiivi käyttöliittymä**: Intuitiivinen GUI, jossa on tumma/vaalea tila -tuki, sulavat animaatiot ja korkean suorituskyvyn rendering, jota tukee **Skia**.
- **Järjestelmäilmoitusalueen integraatio (Tray)**: Täysi tuki ilmoitusalueelle pienentämiselle (~10 MB RAM-muistin käyttö), kaksoisnapsautus tilan vaihtamiseksi ja toimiva oikean painikkeen valikko.
- **Älykäs käynnistys**: Määritä hallintapaneeli käynnistymään Windowsin mukana, pienentymään ilmoitusalueelle (hiljainen tila `/silent`-parametrilla) ja sammuttamaan distributiot automaattisesti poistuttaessa.
- **Kattava instanssien hallinta**: Käynnistä, pysäytä, lopeta ja poista rekisteröinti yhdellä napsautuksella. Reaaliaikainen tilan seuranta ja yksityiskohtaiset tiedot levynkäytöstä ja tiedostojen sijainneista.
- **Distributioiden hallinta**: Aseta oletukseksi, siirrä (VHDX siirto muille asemille) ja vie/kloonaa `.tar` tai `.tar.gz` -arkistoihin.
- **Nopea integraatio**: Välitön käynnistys terminaaliin, VS Codeen tai tiedostonhallintaan muokattavilla työhakemistoilla ja käynnistysskriptien tuella.
- **Jakelun asennus**: Asenna Linux-jakeluita Microsoft Storen, GitHubin, paikallisten tiedostojen (RootFS/VHDX) tai online-peilien kautta (automaattisella nopeustestillä nopeimman peilin valintaan ja sisäänrakennetulla RootFS-latausapulaisella).
- **Globaali turvallisuus**: Mutex-lukot turvallisia rinnakkaisia siirto-/varmuuskopiointioperaatioita varten ja automaattinen Appx-puhdistus poiston yhteydessä.
- **Erittäin pieni muistijälki**: Optimoitu tehokkuutta varten. Hiljainen käynnistys (ilmoitusalueelle) käyttää vain **n. 10 MB** RAM-muistia. Ikkunatilassa käyttö vaihtelee kirjasinten monimutkaisuuden mukaan: **n. 18 MB** peruskielille ja **n. 38 MB** kielille, joissa on laajat merkistöt (kiina, japani, korea).
- **Kehittynyt verkko**: Saumaton porttiohjauksen hallinta (automaattisella palomuurisääntöjen luonnilla) ja globaali HTTP-välityspalvelimen määritys yhtenäistä yhteyttä varten.
- **USB-laitteiden hallinta**: Täydellinen integraatio `usbipd-win`:n kanssa paikallisten USB-laitteiden vaivatonta sitomista, liittämistä ja hallintaa varten WSL-ilmentymissä suoraan käyttöliittymästä.


## ⚙️ Asetukset ja lokit

Kaikki asetukset hallitaan Asetukset-näkymän kautta:

- Valitse oletusasennushakemisto uusille WSL-instansseille.
- Määritä lokihakemisto ja lokitaso (Error / Warn / Info / Debug / Trace).
- Valitse käyttöliittymän kieli tai anna sen seurata järjestelmän kieltä.
- Vaihda tumma tila ja valitse, voiko sovellus sammuttaa WSL:n automaattisesti operaatioiden jälkeen.
- Määritä, kuinka usein sovellus tarkistaa päivitykset (päivittäin, viikoittain, kahden viikon välein, kuukausittain).
- Ota käyttöön automaattinen käynnistys järjestelmän käynnistyessä (automaattisella polun korjauksella).
- Aseta sovellus pienentymään ilmoitusalueelle käynnistettäessä.
- Määritä sulkemispainike pienentämään sovellus ilmoitusalueelle sulkemisen sijaan.
- Mukauta sivupalkkia vaihtamalla tiettyjen ominaisuusvälilehtien näkyvyyttä.

Lokitiedostot kirjoitetaan määritettyyn lokihakemistoon ja ne voidaan liittää mukaan ongelmista ilmoitettaessa.


## 🖼️ Kuvakaappaukset

### Koti (Vaalea ja tumma tila)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB  ja pienennetty valikko
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### Verkko
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### Lisää instanssi ja asetukset
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>
<p align="center">
  <img src="../assets/screenshot/settings-advanced.png" width="48%" />
  <img src="../assets/screenshot/settings-interface.png" width="48%" />
</p>

### Tietoja & Lahjoitus
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/donate.png" width="48%" />
</p>

## 🎬 Demonstraatio

[Auta meitä parantamaan! Katso esittelyvideomme ja jaa ajatuksesi.](https://github.com/owu/wsl-dashboard/discussions/9)



## 💻 Järjestelmävaatimukset

- Windows 10 tai Windows 11, jossa WSL on käytössä (WSL 2 suositeltu).
- Vähintään yksi asennettu WSL-distributio tai oikeus asentaa uusia.
- 64-bittinen CPU; suosittelemme vähintään 4 Gt RAM-muistia useiden distributioiden sujuvaan käyttöön.

## 📦 Asennusopas

### Vaihtoehto 1: Lataa valmiiksi koottu binääri

Helpoin tapa aloittaa on käyttää valmiiksi käännettyä julkaisua:

1. Siirry [GitHub Releases](https://github.com/owu/wsl-dashboard/releases) -sivulle.
2. Lataa uusin Windows-yhteensopiva `wsldashboard`-suoritustiedosto.
3. Pura (jos pakattu) ja suorita `wsldashboard.exe`.

Asennusohjelmaa ei tarvita; sovellus on yksi kannettava binääritiedosto.

### Vaihtoehto 2: Kokoa lähdekoodista

Varmista, että sinulla on Rust-työkalukehys (Rust 1.92+ tai uudempi) asennettuna.

1. Kloonaa repositorio:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. Kokoa ja suorita:

   - Kehityskäyttöön:

     ```powershell
     cargo run
     ```
   - Optimoitu julkaisuversio rakennusskriptin avulla:

     > Rakennusskripti vaatii `x86_64-pc-windows-msvc` -työkalukehyksen.

     ```powershell
     .\build\portable\build.ps1
     ```


## 🛠️ Teknologiapina ja suorituskyky

- **Ydin**: Toteutettu Rustilla muistiturvallisuuden ja nollakustannusabstraktioiden saavuttamiseksi.
- **UI-kehys**: Slint, jossa on korkean suorituskyvyn **Skia**-renderöintitausta.
- **Async Runtime**: Tokio ei-blokkaavia järjestelmäkomentoja ja I/O:ta varten.
- **Suorituskyvyn kohokohdat**:
  - **Vasteaika**: Lähes välitön käynnistys ja reaaliaikainen WSL-tilan seuranta.
  - **Tehokkuus**: Erittäin pieni resurssien käyttö (katso lisätietoja kohdasta [Tärkeimmät ominaisuudet](#-tärkeimmät-ominaisuudet-ja-käyttö)).
  - **Siirrettävyys**: Optimoitu julkaisuversio tuottaa yhden tiiviin suoritustiedoston.



## 🤝 Yhteisön tuki

Suuri kiitos seuraaville yhteisöille tuestaan:

- [Rust Programming Language](https://www.rust-lang.org) - Tehokkaasta ja turvallisesta ohjelmointikielestä
- [Slint | Declarative GUI for Rust, C++, JavaScript & Python](https://slint.dev) - Modernista UI-kehyksestä
- [WSL: Windows Subsystem for Linux](https://github.com/microsoft/WSL) - Mahtavasta Windows Subsystem for Linux:stä
- [Tokio - An asynchronous Rust runtime](https://tokio.rs) - Tehokkaasta asynkronisesta ajoympäristöstä
- [Windows Developer Community](https://developer.microsoft.com/en-us/windows/community) - Jatkuvista alustaparannuksista
- [Reddit](https://www.reddit.com) - Maailmanlaajuisista yhteisökeskusteluista ja tuesta
- [Hacker News](https://news.ycombinator.com) - Maailmanlaajuisista yhteisökeskusteluista ja tuesta
- [Linux.do](https://linux.do) - Suositusta IT-ammattilaisten yhteisöstä
- [V2EX](https://www.v2ex.com) - Kiinalaisen teknologiayhteisön keskusteluista

Sinun panoksesi ja palautteesi tekevät tästä projektista mahdollisen！


## ❤️ Tue tätä projektia

- Tämä projekti on lisensoitu GPL-3.0-lisenssillä ja on ilmainen kaikille käyttäjille.
- Ominaisuuskehityksestä ja päivittäisistä testeistä virheenkorjauksiin — kaikki työ tehdään vapaa-ajalla. Avoimen lähdekoodin tie ei ole helppo kävellä yksin. Tunnustuksesi ja tukesi antavat projektille luottamuksen jatkaa.
- Jos tämä työkalu on aidosti auttanut sinua, harkitse tukemista. Kaikki lahjoitukset menevät palvelinkustannuksiin, versiopäivityksiin ja ominaisuusparannuksiin, pitäen projektin jatkuvasti päivitettynä ja vakaasti etenevänä.
- Jokainen pieni ystävällisyys on tähtivalonsäde. Kiitos vielä kerran ymmärryksestäsi ja anteliaisuudestasi！

Vieraile lahjoitussivullamme：[https://www.wslui.com/donate/](https://www.wslui.com/donate/)


## ⭐️ Sydämen asia

Jos olet kokenut tämän projektin hyödylliseksi, olisin kiitollinen, jos voisit jättää tähden GitHubissa. Tukesi auttaa sitä saavuttamaan laajemman yleisön ja on syvästi arvostettu. Juuri tämä rohkaisu motivoi minua jatkamaan rakentamista.


## 📄 Lisenssi

Tämä projekti on lisensoitu GPL-3.0-lisenssillä – katso tarkemmat tiedot [LICENSE](../LICENSE)-tiedostosta.


---

Built with ❤️ for the WSL Community.

