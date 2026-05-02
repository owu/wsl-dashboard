# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

Sodobna, visoko zmogljiva in lahka nadzorna plošča za upravljanje instanc WSL (Windows Subsystem for Linux). Zgrajena z Rust in Slint za vrhunsko izvorno izkušnjo.

---

```diff
Obvestilo:

- Ta programska oprema se ne distribuira prek Microsoft Store.
- Katera koli aplikacija, navedena tam pod istim imenom, je nepooblaščena in je lahko ponarejena.
- Prosimo, je ne prenašajte, da se izognete potencialni prevari.
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

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | Slovenščina | [Íslenska](./README_is.md)

---

## 📑 Kazalo vsebine
- [🌍 Podprti jeziki](#-podprti-jeziki)
- [🚀 Ključne lastnosti in uporaba](#-ključne-lastnosti-in-uporaba)
- [⚙️ Konfiguracija in dnevniki](#️-konfiguracija-in-dnevniki)
- [🖼️ Posnetki zaslona](#️-posnetki-zaslona)
- [🎬 Prikaz delovanja](#-prikaz-delovanja)
- [💻 Sistemske zahteve](#-sistemske-zahteve)
- [📦 Vodnik za namestitev](#-vodnik-za-namestitev)
- [🛠️ Tehnologije in zmogljivost](#️-tehnologije-in-zmogljivost)
- [⭐️ Delo iz ljubezni](#️-delo-iz-ljubezni)
  - [📄 Licenca](#-licenca)

---

## 🌍 Podprti jeziki

Angleščina, poenostavljena kitajščina, tradicionalna kitajščina, hindijščina, španščina, francoščina, arabščina, bengalščina, portugalščina, ruščina, urdujščina, indonezijščina, nemščina, japonščina, turščina, korejščina, italijanščina, nizozemščina, švedščina, češčina, grščina, madžarščina, hebrejščina, norveščina, danščina, finščina, slovaščina, slovenščina, islandščina.

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="Angleščina" alt="Angleščina" />
  <img src="../assets/flags/cn.svg" width="32" title="Kitajščina (Poenostavljena)" alt="Kitajščina (Poenostavljena)" />
  <img src="../assets/flags/tw.svg" width="32" title="Kitajščina (Tradicionalna)" alt="Kitajščina (Tradicionalna)" />
  <img src="../assets/flags/in.svg" width="32" title="Hindi" alt="Hindi" />
  <img src="../assets/flags/es.svg" width="32" title="Španščina" alt="Španščina" />
  <img src="../assets/flags/fr.svg" width="32" title="Francoščina" alt="Francoščina" />
  <img src="../assets/flags/sa.svg" width="32" title="Arabščina" alt="Arabščina" />
  <img src="../assets/flags/bd.svg" width="32" title="Bengalščina" alt="Bengalščina" />
  <img src="../assets/flags/pt.svg" width="32" title="Portugalščina" alt="Portugalščina" />
  <img src="../assets/flags/ru.svg" width="32" title="Ruščina" alt="Ruščina" />
  <img src="../assets/flags/pk.svg" width="32" title="Urdu" alt="Urdu" />
  <img src="../assets/flags/id.svg" width="32" title="Indonezijščina" alt="Indonezijščina" />
  <img src="../assets/flags/de.svg" width="32" title="Nemščina" alt="Nemščina" />
  <img src="../assets/flags/jp.svg" width="32" title="Japonščina" alt="Japonščina" />
  <img src="../assets/flags/tr.svg" width="32" title="Turščina" alt="Turščina" />
  <img src="../assets/flags/kr.svg" width="32" title="Korejščina" alt="Korejščina" />
  <img src="../assets/flags/it.svg" width="32" title="Italijanščina" alt="Italijanščina" />
  <img src="../assets/flags/nl.svg" width="32" title="Nizozemščina" alt="Nizozemščina" />
  <img src="../assets/flags/se.svg" width="32" title="Švedščina" alt="Švedščina" />
  <img src="../assets/flags/cz.svg" width="32" title="Češčina" alt="Češčina" />
  <img src="../assets/flags/gr.svg" width="32" title="Grščina" alt="Grščina" />
  <img src="../assets/flags/hu.svg" width="32" title="Madžarščina" alt="Madžarščina" />
  <img src="../assets/flags/il.svg" width="32" title="Hebrejščina" alt="Hebrejščina" />
  <img src="../assets/flags/no.svg" width="32" title="Norveščina" alt="Norveščina" />
  <img src="../assets/flags/dk.svg" width="32" title="Danščina" alt="Danščina" />
  <img src="../assets/flags/fi.svg" width="32" title="Finščina" alt="Finščina" />
  <img src="../assets/flags/sk.svg" width="32" title="Slovaščina" alt="Slovaščina" />
  <img src="../assets/flags/si.svg" width="32" title="Slovenščina" alt="Slovenščina" />
  <img src="../assets/flags/is.svg" width="32" title="Islandščina" alt="Islandščina" />
</p>


## 🚀 Ključne lastnosti in uporaba

- **Sodoben izvorni uporabniški vmesnik**: Intuitiven GUI s podporo za temni/svetli način, gladkimi animacijami in visoko zmogljivim upodabljanjem, ki ga poganja **Skia**.
- **Integracija v sistemsko vrstico (Tray)**: Popolna podpora za zmanjšanje v sistemsko vrstico (poraba RAM-a ~10 MB), dvojni klik za preklop in funkcionalen meni na desni klik.
- **Inteligentni zagon**: Konfigurirajte nadzorno ploščo za zagon z Windows, zmanjšanje v sistemsko vrstico (tihi način s parametrom `/silent`) in samodejni izklop distribucij ob izhodu.
- **Celovit nadzor instanc**: Zagon, ustavitev, prekinitev in preklic registracije z enim klikom. Spremljanje stanja v realnem času in podrobni vpogledi v porabo diska in lokacije datotek.
- **Upravljanje distribucij**: Nastavitev kot privzeto, migracija (premik VHDX na druge pogone) in izvoz/kloniranje v `.tar` ali `.tar.gz` arhive.
- **Hitra integracija**: Takojšen zagon v Terminal, VS Code ali Raziskovalec s prilagodljivimi delovnimi imeniki in kavlji za zagonske skripte.
- **Pametna namestitev**: Namestitev iz Microsoft Store, GitHub ali lokalnih datotek (RootFS/VHDX). Vključuje vgrajenega pomočnika za prenos RootFS.
- **Globalna varnost**: Zaklepanje Mutex za varne sočasne migracije/varnostne kopije in samodejno čiščenje Appx ob odstranitvi.
- **Izjemno nizka poraba pomnilnika**: Visoko optimizirano za učinkovitost. Tihi zagon (v sistemski vrstici) porabi le okoli **10 MB** RAM-a. Poraba v okenskem načinu se razlikuje glede na kompleksnost pisav: **~18 MB** za standardne jezike in **~38 MB** za jezike z velikimi nabori znakov (kitajščina, japonščina, korejščina).
- **Napredna omrežja**: Brezhibno upravljanje posredovanja vrat (s samodejnim ustvarjanjem pravil požarnega zidu) in globalna konfiguracija proxyja HTTP za poenoteno povezljivost.
- **Upravljanje naprav USB**: Popolna integracija z orodjem `usbipd-win` za enostavno vezavo, pripenjanje in upravljanje lokalnih naprav USB v vseh primerih WSL neposredno z nadzorne plošče.


## ⚙️ Konfiguracija in dnevniki

Vse nastavitve se upravljajo prek pogleda Nastavitve:

- Izberite privzeti imenik za namestitev novih instanc WSL.
- Konfigurirajte imenik za dnevnike in raven beleženja (Error / Warn / Info / Debug / Trace).
- Izberite jezik uporabniškega vmesnika ali pustite, da sledi sistemskemu jeziku.
- Preklopite temni način in izberite, ali lahko aplikacija samodejno izklopi WSL po operacijah.
- Konfigurirajte pogostost preverjanja posodobitev (dnevno, tedensko, dvotedensko, mesečno).
- Omogočite samodejni zagon ob zagonu sistema (s samodejnim popravilom poti).
- Nastavite aplikacijo, da se ob zagonu zmanjša v sistemsko vrstico.
- Konfigurirajte gumb za zapiranje, da se aplikacija zmanjša v sistemsko vrstico namesto izhoda.
- Prilagodite stransko vrstico s preklopom vidnosti določenih zavihkov funkcij.

Dnevniške datoteke se zapisujejo v konfiguriran imenik in jih je mogoče priložiti ob prijavi težav.


## 🖼️ Posnetki zaslona

### Domov (Svetli in temni način)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB in strnjeni meni
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### omrežje
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### Dodajanje instance in Nastavitve
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>

### O aplikaciji 
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
</p>

## 🎬 Prikaz delovanja

Spodaj je prikaz delovanja WSL Dashboarda:

![WSL Dashboard Demo](../assets/screenshot/demo.gif)



## 💻 Sistemske zahteve

- Windows 10 ali Windows 11 z omogočenim WSL (priporočljiv WSL 2).
- Vsaj ena nameščena distribucija WSL ali dovoljenje za namestitev novih.
- 64-bitni procesor; priporočljivo 4 GB RAM-a ali več za nemoteno uporabo več distribucij.

## 📦 Vodnik za namestitev

### Možnost 1: Prenos že zgrajene binarne datoteke

Najlažji način za začetek je uporaba vnaprej prevedene različice:

1. Obiščite stran [GitHub Releases](https://github.com/owu/wsl-dashboard/releases).
2. Prenesite najnovejšo izvršljivo datoteko `wsldashboard` za Windows.
3. Razširite (če je pakirano) in zaženite `wsldashboard.exe`.

Namestitev ni potrebna; aplikacija je ena sama prenosna binarna datoteka.

### Možnost 2: Gradnja iz izvorne kode

Prepričajte se, da imate nameščena orodja Rust (Rust 1.92+ ali novejša).

1. Klonirajte repozitorij:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. Gradnja in zagon:

   - Za razvoj:

     ```powershell
     cargo run
     ```
   - Optimizirana izdaja z uporabo gradbene skripte:

     > Gradbena skripta zahteva orodja `x86_64-pc-windows-msvc`.

     ```powershell
     .\build\scripts\build.ps1
     ```


## 🛠️ Tehnologije in zmogljivost

- **Jedro**: Implementirano v jeziku Rust za varnost pomnilnika in brezplačne abstrakcije.
- **UI ogrodje**: Slint z visoko zmogljivim **Skia** zaledjem za upodabljanje.
- **Asinhroni čas izvajanja**: Tokio za neblokirajoče sistemske ukaze in I/O.
- **Poudarki delovanja**:
  - **Odzivnost**: Skoraj takojšen zagon in spremljanje stanja WSL v realnem času.
  - **Učinkovitost**: Izjemno nizka poraba virov (za podrobnosti glejte [Ključne lastnosti](#-ključne-lastnosti-in-uporaba)).
  - **Prenosljivost**: Optimizirana izdaja ustvari eno samo kompaktno izvršljivo datoteko.



## ⭐️ Delo iz ljubezni

Če se vam zdi ta projekt uporaben, bi vam bil hvaležen, če bi pustili zvezdico na GitHubu. Vaša podpora mu pomaga doseči širše občinstvo in jo zelo cenim. Prav ta spodbuda me motivira, da nadaljujem z gradnjo.

## 📄 Licenca

Ta projekt je licenciran pod GPL-3.0 – podrobnosti najdete v datoteki [LICENSE](../LICENSE).

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

