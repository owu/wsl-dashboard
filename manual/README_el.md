# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="Λογότυπο WSL Dashboard" />
</p>

Ένας σύγχρονος, υψηλής απόδοσης, ελαφρύς και με χαμηλή μνήμη πίνακας ελέγχου διαχείρισης στιγμιότυπων WSL (Windows Subsystem for Linux). Δημιουργημένος με Rust και Slint για μια κορυφαία εγγενή εμπειρία.

```diff
Ειδοποίηση:

- Το WSL Dashboard δεν διανέμεται μέσω του Microsoft Store.
- Οποιαδήποτε εφαρμογή που αναφέρεται εκεί με το όνομα "WSL Dashboard" δεν είναι εξουσιοδοτημένη και μπορεί να είναι πλαστή.
- Παρακαλώ μην το κατεβάσετε για να αποφύγετε πιθανές απάτες.
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

README I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | Ελληνικά | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 Πίνακας Περιεχομένων
[🌍 Υποστηριζόμενες Γλώσσες](#-υποστηριζόμενες-γλώσσες) | [🚀 Κύρια Χαρακτηριστικά & Χρήση](#-κύρια-χαρακτηριστικά--χρήση) | [🖼️ Στιγμιότυπα Οθόνης](#️-στιγμιότυπα-οθόνης) | [📦 Οδηγός Εγκατάστασης](#-οδηγός-εγκατάστασης) | [📢 Επίσημες Ομάδες Κοινότητας](#-επίσημες-ομάδες-κοινότητας) | [❤️ Υποστήριξη αυτού του έργου](#️-υποστήριξη-αυτού-του-έργου) | [⭐️ Έργο αγάπης](#️-έργο-αγάπης) | [📄 Άδεια Χρήσης](#-άδεια-χρήσης)

---

## 🌍 Υποστηριζόμενες Γλώσσες

Αγγλικά, Απλοποιημένα Κινέζικα, Παραδοσιακά Κινέζικα, Χίντι, Ισπανικά, Γαλλικά, Αραβικά, Μπενγκάλι, Πορτογαλικά, Ρωσικά, Ουρντού, Ινδονησιακά, Γερμανικά, Ιαπωνικά, Τουρκικά, Κορεάτικα, Ιταλικά, Ολλανδικά, Σουηδικά, Τσεχικά, Ελληνικά, Ουγγρικά, Εβραϊκά, Νορβηγικά, Δανικά, Φινλανδικά, Σλοβακικά, Σλοβενικά, Ισλανδικά, Βιετναμέζικα, Τελούγκου, Ιαβανέζικα, Ταϊλανδέζικα, Ταμίλ, Φιλιππινέζικα, Παντζάμπι, Μαλαϊκά, Πολωνικά, Ουκρανικά, Περσικά, Κανάντα, Μαράθι, Χάουσα, Βιρμανέζικα, Ουζμπεκικά, Αζερμπαϊτζανικά, Σεμπουάνο, Μαγιαλάμ, Σίντι, Αμχαρικά.

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="Αγγλικά" alt="Αγγλικά" />
  <img src="../assets/flags/cn.svg" width="32" title="Κινεζικά (Απλοποιημένα)" alt="Κινεζικά (Απλοποιημένα)" />
  <img src="../assets/flags/tw.svg" width="32" title="Κινεζικά (Παραδοσιακά)" alt="Κινεζικά (Παραδοσιακά)" />
  <img src="../assets/flags/in.svg" width="32" title="Χίντι" alt="Χίντι" />
  <img src="../assets/flags/es.svg" width="32" title="Ισπανικά" alt="Ισπανικά" />
  <img src="../assets/flags/fr.svg" width="32" title="Γαλλικά" alt="Γαλλικά" />
  <img src="../assets/flags/sa.svg" width="32" title="Αραβικά" alt="Αραβικά" />
  <img src="../assets/flags/bd.svg" width="32" title="Βεγγαλικά" alt="Βεγγαλικά" />
  <img src="../assets/flags/pt.svg" width="32" title="Πορτογαλικά" alt="Πορτογαλικά" />
  <img src="../assets/flags/ru.svg" width="32" title="Ρωσικά" alt="Ρωσικά" />
  <img src="../assets/flags/pk.svg" width="32" title="Ουρντού" alt="Ουρντού" />
  <img src="../assets/flags/id.svg" width="32" title="Ινδονησιακά" alt="Ινδονησιακά" />
  <img src="../assets/flags/de.svg" width="32" title="Γερμανικά" alt="Γερμανικά" />
  <img src="../assets/flags/jp.svg" width="32" title="Ιαπωνικά" alt="Ιαπωνικά" />
  <img src="../assets/flags/tr.svg" width="32" title="Τουρκικά" alt="Τουρκικά" />
  <img src="../assets/flags/kr.svg" width="32" title="Κορεατικά" alt="Κορεατικά" />
  <img src="../assets/flags/it.svg" width="32" title="Ιταλικά" alt="Ιταλικά" />
  <img src="../assets/flags/nl.svg" width="32" title="Ολλανδικά" alt="Ολλανδικά" />
  <img src="../assets/flags/se.svg" width="32" title="Σουηδικά" alt="Σουηδικά" />
  <img src="../assets/flags/cz.svg" width="32" title="Τσεχικά" alt="Τσεχικά" />
  <img src="../assets/flags/gr.svg" width="32" title="Ελληνικά" alt="Ελληνικά" />
  <img src="../assets/flags/hu.svg" width="32" title="Ουγγρικά" alt="Ουγγρικά" />
  <img src="../assets/flags/il.svg" width="32" title="Εβραϊκά" alt="Εβραϊκά" />
  <img src="../assets/flags/no.svg" width="32" title="Νορβηγικά" alt="Νορβηγικά" />
  <img src="../assets/flags/dk.svg" width="32" title="Δανικά" alt="Δανικά" />
  <img src="../assets/flags/fi.svg" width="32" title="Φινλανδικά" alt="Φινλανδικά" />
  <img src="../assets/flags/sk.svg" width="32" title="Σλοβακικά" alt="Σλοβακικά" />
  <img src="../assets/flags/si.svg" width="32" title="Σλοβενικά" alt="Σλοβενικά" />
  <img src="../assets/flags/is.svg" width="32" title="Ισλανδικά" alt="Ισλανδικά" />
  <img src="../assets/flags/vn.svg" width="32" title="Βιετναμέζικα" alt="Βιετναμέζικα" />
  <img src="../assets/flags/in.svg" width="32" title="Τελούγκου" alt="Τελούγκου" />
  <img src="../assets/flags/id.svg" width="32" title="Ιαβανέζικα" alt="Ιαβανέζικα" />
  <img src="../assets/flags/th.svg" width="32" title="Ταϊλανδέζικα" alt="Ταϊλανδέζικα" />
  <img src="../assets/flags/in.svg" width="32" title="Ταμίλ" alt="Ταμίλ" />
  <img src="../assets/flags/ph.svg" width="32" title="Φιλιππινέζικα" alt="Φιλιππινέζικα" />
  <img src="../assets/flags/pk.svg" width="32" title="Παντζάμπι" alt="Παντζάμπι" />
  <img src="../assets/flags/my.svg" width="32" title="Μαλαϊκά" alt="Μαλαϊκά" />
  <img src="../assets/flags/pl.svg" width="32" title="Πολωνικά" alt="Πολωνικά" />
  <img src="../assets/flags/ua.svg" width="32" title="Ουκρανικά" alt="Ουκρανικά" />
  <img src="../assets/flags/ir.svg" width="32" title="Περσικά" alt="Περσικά" />
  <img src="../assets/flags/in.svg" width="32" title="Κανάντα" alt="Κανάντα" />
  <img src="../assets/flags/in.svg" width="32" title="Μαράθι" alt="Μαράθι" />
  <img src="../assets/flags/ng.svg" width="32" title="Χάουσα" alt="Χάουσα" />
  <img src="../assets/flags/mm.svg" width="32" title="Βιρμανέζικα" alt="Βιρμανέζικα" />
  <img src="../assets/flags/uz.svg" width="32" title="Ουζμπεκικά" alt="Ουζμπεκικά" />
  <img src="../assets/flags/az.svg" width="32" title="Αζερμπαϊτζανικά" alt="Αζερμπαϊτζανικά" />
  <img src="../assets/flags/ph.svg" width="32" title="Σεμπουάνο" alt="Σεμπουάνο" />
  <img src="../assets/flags/in.svg" width="32" title="Μαγιαλάμ" alt="Μαγιαλάμ" />
  <img src="../assets/flags/pk.svg" width="32" title="Σίντι" alt="Σίντι" />
  <img src="../assets/flags/et.svg" width="32" title="Αμχαρικά" alt="Αμχαρικά" />
</p>

## 🚀 Κύρια Χαρακτηριστικά & Χρήση

- **Σύγχρονο Γραφικό Περιβάλλον**: Διαισθητικό GUI με υποστήριξη Dark/Light mode, ομαλά εφέ κίνησης και απόδοση υψηλής ταχύτητας μέσω της μηχανής **Skia**.
- **Ενσωμάτωση στο System Tray**: Πλήρης υποστήριξη για ελαχιστοποίηση στην περιοχή ειδοποιήσεων (χρήση RAM ~10MB), διπλό κλικ για επαναφορά και λειτουργικό μενού δεξιού κλικ.
- **Έξυπνη Εκκίνηση**: Ρυθμίστε το dashboard να ξεκινά μαζί με τα Windows, να ελαχιστοποιείται στο tray (αθόρυβη λειτουργία με την παράμετρο `/silent`) και να κλείνει αυτόματα τα περιβάλλοντα WSL κατά την έξοδο.
- **Προγραμματισμός Εργασιών**: Ευέλικτη διαχείριση προγραμματισμού εργασιών, που υποστηρίζει την αυτόματη εκτέλεση σεναρίων και εντολών σε instances WSL σε καθορισμένες ώρες ή διαστήματα για αυτοματοποιημένες λειτουργίες.
- **Πλήρης Έλεγχος Περιβαλλόντων**: Εκκίνηση, διακοπή, τερματισμός και απεγκατάσταση με ένα κλικ. Παρακολούθηση κατάστασης σε πραγματικό χρόνο και λεπτομερείς πληροφορίες για τη χρήση δίσκου και τις τοποθεσίες αρχείων.
- **Διαχείριση Διανομών**: Ορισμός ως προεπιλογή, μεταφορά (μετακίνηση VHDX σε άλλους δίσκους) και εξαγωγή/κλωνοποίηση σε αρχεία `.tar` ή `.tar.gz`.
- **Προσάρτηση Δίσκου**: Απευθείας προσάρτηση και διαχείριση VHDX εικονικών δίσκων instances WSL, υποστηρίζοντας λειτουργίες προσάρτησης, αποπροσάρτησης και διαχείρισης δίσκων, διευκολύνοντας τη μεταφορά αρχείων μεταξύ του κεντρικού υπολογιστή και του WSL.
- **Γρήγορη Ενοποίηση**: Άμεση εκκίνηση σε Terminal, VS Code ή File Explorer με προσαρμόσιμους φακέλους εργασίας και υποστήριξη startup scripts.
- **Εγκατάσταση Διανομής**: Εγκαταστήστε διανομές Linux μέσω Microsoft Store, GitHub, τοπικών αρχείων (RootFS/VHDX) ή διαδικτυακών καθρεπτών (με αυτόματο τεστ ταχύτητας για επιλογή του γρηγορότερου καθρέπτη και ενσωματωμένο βοηθό λήψης RootFS).
- **Ασφάλεια**: Κλειδώματα Mutex για ασφαλείς ταυτόχρονες λειτουργίες μεταφοράς/αντιγράφων ασφαλείας και αυτόματος καθαρισμός Appx κατά την αφαίρεση.
- **Ελάχιστο Αποτύπωμα Μνήμης**: Εξαιρετικά βελτιστοποιημένο για αποδοτικότητα. Η εκκίνηση στο system tray χρησιμοποιεί μόνο **~10MB** RAM. Η χρήση σε προβολή παραθύρου ποικίλλει ανάλογα με τη γλώσσα: **~18MB** για τυπικές γλώσσες και **~38MB** για γλώσσες με μεγάλα σύνολα χαρακτήρων (Κινέζικα, Ιαπωνικά, Κορεάτικα).
- **Προηγμένα δίκτυα**: Απρόσκοπτη διαχείριση προώθησης θυρών (με αυτόματη δημιουργία κανόνων τείχους προστασίας) και καθολική διαμόρφωση διακομιστή μεσολάβησης HTTP για ενοποιημένη συνδεσιμότητα.
- **Διαχείριση συσκευών USB**: Πλήρης ενσωμάτωση με το `usbipd-win` για εύκολη δέσμευση, προσάρτηση και διαχείριση τοπικών συσκευών USB σε όλες τις παρουσίες WSL απευθείας από τη διεπαφή χρήστη του πίνακα ελέγχου.
- **Ρυθμίσεις και αρχεία καταγραφής**: Από τις Ρυθμίσεις διαχειριστείτε τον προεπιλεγμένο φάκελο εγκατάστασης, τον φάκελο και το επίπεδο καταγραφής, τη γλώσσα, τη σκοτεινή λειτουργία, τη συχνότητα ελέγχου ενημερώσεων, την αυτόματη εκκίνηση και τη συμπεριφορά κλεισίματος, καθώς και την πλαϊνή γραμμή.
- **Διατήρηση διανομών ενεργών**: Ένα κλικ στο «Εκκίνηση» κρατά τη διανομή σε λειτουργία αντί να τερματιστεί μετά από ~60 s αδράνειας· μπορείτε επίσης να ενεργοποιήσετε το «Εκκίνηση αυτής της διανομής κατά την εκκίνηση των Windows».
- **Καθολικές συντομεύσεις**: Ορίστε καθολικές συντομεύσεις για εμφάνιση/απόκρυψη του κύριου παραθύρου ή άνοιγμα του τερματικού μιας διανομής.


## 🖼️ Στιγμιότυπα Οθόνης
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

Θέλετε να δείτε τον πίνακα ελέγχου σε δράση; Επισκεφθείτε την [επίσημη ιστοσελίδα](https://www.wslui.com/) για την πλήρη συλλογή στιγμιοτύπων.

## 📦 Οδηγός Εγκατάστασης

### Επιλογή 1: Επισκεφθείτε τον ιστότοπο του έργου (Προτείνεται)

Συνιστούμε να επισκεφθείτε τον επίσημο ιστότοπο για λήψη, καθώς προσφέρει πολλαπλούς συνδέσμους κατοπτρισμού (mirror) για μια πιο ομαλή εμπειρία:

Μεταβείτε στη [σελίδα Λήψης](https://www.wslui.com/download/) και επιλέξτε τον καθρέπτη που ταιριάζει στην περιοχή σας.

### Επιλογή 2: Εγκατάσταση μέσω winget

Μπορείτε να εγκαταστήσετε το WSLDashboard απευθείας από τον Windows Package Manager (winget), χρησιμοποιώντας είτε το ψευδώνυμο (moniker) είτε το πλήρες αναγνωριστικό πακέτου:

```powershell
# Αναζήτηση (χωρίς διάκριση πεζών-κεφαλαίων)
winget search wsl-dashboard
# ή
winget search WSLDashboard

# Εγκατάσταση (επιλέξτε ένα)
winget install wsl-dashboard
# ή
winget install Owu.WSLDashboard
```

> Το αναγνωριστικό πακέτου winget είναι `Owu.WSLDashboard` και το ψευδώνυμο είναι `wsl-dashboard` (χωρίς διάκριση πεζών-κεφαλαίων). Και τα δύο λειτουργούν.

Για περισσότερες πληροφορίες, επισκεφθείτε το [αποθετήριο κοινότητας WinGet](https://github.com/microsoft/winget-pkgs/tree/master/manifests/o/Owu/WSLDashboard).

### Επιλογή 3: Μεταγλώττιση από τον κώδικα

Βεβαιωθείτε ότι έχετε εγκατεστημένη την εργαλειοθήκη Rust (Rust 1.92+ ή νεότερη).

1. Κλωνοποιήστε το αποθετήριο:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. Μεταγλώττιση και εκτέλεση:

   - Για ανάπτυξη:

     ```powershell
     cargo run
     ```
   - Βελτιστοποιημένη έκδοση παραγωγής μέσω του script:

     > Το script απαιτεί την εργαλειοθήκη `x86_64-pc-windows-msvc`.

     ```powershell
     .\build\portable\build.ps1 -Arch x64
     ```


## 📢 Επίσημες Ομάδες Κοινότητας

Καλώς ήρθατε στις επίσημες ομάδες κοινότητας του WSL UI! Γίνετε μέλη των καναλιών μας για να συζητήσετε συμβουλές χρήσης, να αναφέρετε προβλήματα, να λάβετε τις τελευταίες ενημερώσεις και να επικοινωνήσετε με τους προγραμματιστές και άλλους χρήστες.

<div align="center">

| Παγκόσμια Ομάδα (Telegram) | Ομάδα Κινεζικής (WeChat) |
| :---: | :---: |
| <img src="https://cdn1.wslui.com/static/images/group/group-telegram.png" width="240" alt="Telegram Group QR Code" /> | <img src="https://cdn1.wslui.com/static/images/group/group-wechat.png" width="240" alt="WeChat Group QR Code" /> |
| Σαρώστε για εγγραφή | Σαρώστε για εγγραφή |

</div>


## ❤️ Υποστήριξη αυτού του έργου

- Αυτό το έργο είναι υπό άδεια GPL-3.0 και είναι δωρεάν για όλους τους χρήστες.
- Από την ανάπτυξη λειτουργιών και τις καθημερινές δοκιμές μέχρι τη διόρθωση σφαλμάτων, όλη η εργασία γίνεται στον ελεύθερο χρόνο. Ο δρόμος του ανοιχτού κώδικα δεν είναι εύκολος μόνος — η αναγνώριση και η υποστήριξή σας δίνουν στο έργο την εμπιστοσύνη να συνεχίσει.
- Αν αυτό το εργαλείο σας βοήθησε πραγματικά, σκεφτείτε να βοηθήσετε. Όλες οι δωρεές πηγαίνουν στα έξοψα διακομιστή, τις ενημερώσεις εκδόσεων και τις βελτιώσεις λειτουργιών, διατηρώντας το έργο συνεχώς ενημερωμένο και σταθερά προοδευτικό.
- Κάθε μικρή καλοσύνη είναι μια ακτίνα αστρικού φωτός. Ευχαριστούμε και πάλι για την κατανόηση και τη γενναιοδωρία σας！ [Επισκεφτείτε τη σελίδα δωρεάς μας](https://www.wslui.com/donate/).


## ⭐️ Έργο αγάπης

Εάν βρήκατε αυτό το έργο χρήσιμο, θα σας ήμουν ευγνώμων εάν μπορούσατε να αφήσετε ένα αστέρι στο GitHub. Η υποστήριξή σας το βοηθά να προσεγγίσει ένα ευρύτερο κοινό και εκτιμάται βαθύτατα. Αυτή η ενθάρρυνση είναι που με ωθεί να συνεχίσω να δημιουργώ.


## 📄 Άδεια Χρήσης

Αυτό το έργο διατίθεται υπό την άδεια GPL-3.0 – δείτε το αρχείο [LICENSE](../LICENSE) για λεπτομέρειες.
