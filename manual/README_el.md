# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="Λογότυπο WSL Dashboard" />
</p>

Ένας σύγχρονος, υψηλής απόδοσης και ελαφρύς πίνακας ελέγχου για τη διαχείριση περιβαλλόντων WSL (Windows Subsystem for Linux). Κατασκευασμένος με Rust και Slint για μια κορυφαία εμπειρία χρήσης.

---

```diff
Ειδοποίηση:

- Αυτό το λογισμικό δεν διανέμεται μέσω του Microsoft Store.
- Κάθε εφαρμογή που αναφέρεται εκεί με το ίδιο όνομα δεν είναι εξουσιοδοτημένη και μπορεί να είναι πλαστή.
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

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | Ελληνικά | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md)

---

## 📑 Πίνακας Περιεχομένων
- [🌍 Υποστηριζόμενες Γλώσσες](#-υποστηριζόμενες-γλώσσες)
- [🚀 Κύρια Χαρακτηριστικά & Χρήση](#-κύρια-χαρακτηριστικά--χρήση)
- [⚙️ Ρυθμίσεις & Logs](#️-ρυθμίσεις--logs)
- [🖼️ Στιγμιότυπα Οθόνης](#️-στιγμιότυπα-οθόνης)
- [🎬 Επίδειξη Λειτουργίας](#-επίδειξη-λειτουργίας)
- [💻 Απαιτήσεις Συστήματος](#-απαιτήσεις-συστήματος)
- [📦 Οδηγός Εγκατάστασης](#-οδηγός-εγκατάστασης)
- [🛠️ Τεχνολογίες & Απόδοση](#️-τεχνολογίες--απόδοση)
- [⭐️ Έργο αγάπης](#️-έργο-αγάπης)
- [📄 Άδεια Χρήσης](#-άδεια-χρήσης)

---

## 🌍 Υποστηριζόμενες Γλώσσες

Αγγλικά, Απλοποιημένα Κινέζικα, Παραδοσιακά Κινέζικα, Χίντι, Ισπανικά, Γαλλικά, Αραβικά, Μπενγκάλι, Πορτογαλικά, Ρωσικά, Ουρντού, Ινδονησιακά, Γερμανικά, Ιαπωνικά, Τουρκικά, Κορεάτικα, Ιταλικά, Ολλανδικά, Σουηδικά, Τσεχικά, Ελληνικά, Ουγγρικά, Εβραϊκά, Νορβηγικά, Δανικά, Φινλανδικά, Σλοβακικά, Σλοβενικά, Ισλανδικά.

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
</p>


## 🚀 Κύρια Χαρακτηριστικά & Χρήση

- **Σύγχρονο Γραφικό Περιβάλλον**: Διαισθητικό GUI με υποστήριξη Dark/Light mode, ομαλά εφέ κίνησης και απόδοση υψηλής ταχύτητας μέσω της μηχανής **Skia**.
- **Ενσωμάτωση στο System Tray**: Πλήρης υποστήριξη για ελαχιστοποίηση στην περιοχή ειδοποιήσεων (χρήση RAM ~10MB), διπλό κλικ για επαναφορά και λειτουργικό μενού δεξιού κλικ.
- **Έξυπνη Εκκίνηση**: Ρυθμίστε το dashboard να ξεκινά μαζί με τα Windows, να ελαχιστοποιείται στο tray (αθόρυβη λειτουργία με την παράμετρο `/silent`) και να κλείνει αυτόματα τα περιβάλλοντα WSL κατά την έξοδο.
- **Πλήρης Έλεγχος Περιβαλλόντων**: Εκκίνηση, διακοπή, τερματισμός και απεγκατάσταση με ένα κλικ. Παρακολούθηση κατάστασης σε πραγματικό χρόνο και λεπτομερείς πληροφορίες για τη χρήση δίσκου και τις τοποθεσίες αρχείων.
- **Διαχείριση Διανομών**: Ορισμός ως προεπιλογή, μεταφορά (μετακίνηση VHDX σε άλλους δίσκους) και εξαγωγή/κλωνοποίηση σε αρχεία `.tar` ή `.tar.gz`.
- **Γρήγορη Ενοποίηση**: Άμεση εκκίνηση σε Terminal, VS Code ή File Explorer με προσαρμόσιμους φακέλους εργασίας και υποστήριξη startup scripts.
- **Έξυπνη Εγκατάσταση**: Εγκατάσταση από Microsoft Store, GitHub ή τοπικά αρχεία (RootFS/VHDX). Περιλαμβάνει ενσωματωμένο βοηθό λήψης RootFS.
- **Ασφάλεια**: Κλειδώματα Mutex για ασφαλείς ταυτόχρονες λειτουργίες μεταφοράς/αντιγράφων ασφαλείας και αυτόματος καθαρισμός Appx κατά την αφαίρεση.
- **Ελάχιστο Αποτύπωμα Μνήμης**: Εξαιρετικά βελτιστοποιημένο για αποδοτικότητα. Η εκκίνηση στο system tray χρησιμοποιεί μόνο **~10MB** RAM. Η χρήση σε προβολή παραθύρου ποικίλλει ανάλογα με τη γλώσσα: **~18MB** για τυπικές γλώσσες και **~38MB** για γλώσσες με μεγάλα σύνολα χαρακτήρων (Κινέζικα, Ιαπωνικά, Κορεάτικα).
- **Προηγμένα δίκτυα**: Απρόσκοπτη διαχείριση προώθησης θυρών (με αυτόματη δημιουργία κανόνων τείχους προστασίας) και καθολική διαμόρφωση διακομιστή μεσολάβησης HTTP για ενοποιημένη συνδεσιμότητα.
- **Διαχείριση συσκευών USB**: Πλήρης ενσωμάτωση με το `usbipd-win` για εύκολη δέσμευση, προσάρτηση και διαχείριση τοπικών συσκευών USB σε όλες τις παρουσίες WSL απευθείας από τη διεπαφή χρήστη του πίνακα ελέγχου.


## ⚙️ Ρυθμίσεις & Logs

Η διαχείριση όλων των ρυθμίσεων γίνεται μέσω της καρτέλας "Ρυθμίσεις":

- Επιλογή προεπιλεγμένου φακέλου εγκατάστασης για νέα WSL instances.
- Ρύθμιση φακέλου καταγραφής και επιπέδου logs (Error / Warn / Info / Debug / Trace).
- Επιλογή γλώσσας διεπαφής ή αυτόματη ακολουθία της γλώσσας του συστήματος.
- Εναλλαγή Dark mode και επιλογή για αυτόματο κλείσιμο του WSL μετά από λειτουργίες.
- Ρύθμιση συχνότητας ελέγχου ενημερώσεων (καθημερινά, εβδομαδιαία, ανά δύο εβδομάδες, μηνιαία).
- Ενεργοποίηση αυτόματης εκκίνησης με το σύστημα (με αυτόματη διόρθωση διαδρομής).
- Ρύθμιση της εφαρμογής για ελαχιστοποίηση στο tray κατά την εκκίνηση.
- Ρύθμιση του κουμπιού κλεισίματος για ελαχιστοποίηση στο tray αντί για έξοδο.
- Προσαρμόστε την πλευρική γραμμή εναλλάσσοντας την ορατότητα συγκεκριμένων καρτελών δυνατοτήτων.

Τα αρχεία καταγραφής εγγράφονται στον ρυθμισμένο φάκελο και μπορούν να επισυναφθούν κατά την αναφορά προβλημάτων.


## 🖼️ Στιγμιότυπα Οθόνης

### Αρχική (Light & Dark Mode)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB & Συμπτυγμένο Μενού
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### Δίκτυο
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### Προσθήκη Περιβάλλοντος & Ρυθμίσεις
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>

### Σχετικά
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
</p>

## 🎬 Επίδειξη Λειτουργίας

Παρακάτω ακολουθεί μια επίδειξη του WSL Dashboard σε λειτουργία:

![WSL Dashboard Demo](../assets/screenshot/demo.gif)



## 💻 Απαιτήσεις Συστήματος

- Windows 10 ή Windows 11 με ενεργοποιημένο το WSL (συστήνεται το WSL 2).
- Τουλάχιστον μία εγκατεστημένη διανομή WSL ή δικαιώματα για την εγκατάσταση νέων.
- Επεξεργαστής 64-bit. Προτείνονται 4 GB RAM ή περισσότερο για ομαλή χρήση πολλών διανομών ταυτόχρονα.

## 📦 Οδηγός Εγκατάστασης

### Επιλογή 1: Λήψη έτοιμου εκτελέσιμου αρχείου

Ο ευκολότερος τρόπος για να ξεκινήσετε είναι η χρήση μιας προ-μεταγλωττισμένης έκδοσης:

1. Μεταβείτε στη σελίδα [GitHub Releases](https://github.com/owu/wsl-dashboard/releases).
2. Κατεβάστε το πιο πρόσφατο εκτελέσιμο αρχείο `wsldashboard` για Windows.
3. Αποσυμπιέστε (αν χρειάζεται) και εκτελέστε το `wsldashboard.exe`.

Δεν απαιτείται εγκατάσταση. Η εφαρμογή είναι ένα φορητό (portable) εκτελέσιμο αρχείο.

### Επιλογή 2: Μεταγλώττιση από τον κώδικα

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
     .\build\scripts\build.ps1
     ```


## 🛠️ Τεχνολογίες & Απόδοση

- **Πυρήνας**: Υλοποιημένος σε Rust για ασφάλεια μνήμης και μέγιστη ταχύτητα.
- **Περιβάλλον UI**: Slint με backend απόδοσης **Skia** υψηλής ταχύτητας.
- **Async Runtime**: Tokio για μη-ανασταλτικές εντολές συστήματος και I/O.
- **Κύρια Σημεία Απόδοσης**:
  - **Αμεσότητα**: Σχεδόν ακαριαία εκκίνηση και παρακολούθηση της κατάστασης του WSL σε πραγματικό χρόνο.
  - **Αποδοτικότητα**: Εξαιρετικά χαμηλή κατανάλωση πόρων (βλ. [Κύρια Χαρακτηριστικά](#-κύρια-χαρακτηριστικά--χρήση)).
  - **Φορητότητα**: Η βελτιστοποιημένη έκδοση παράγει ένα μοναδικό, συμπαγές εκτελέσιμο αρχείο.



## ⭐️ Έργο αγάπης

Εάν βρήκατε αυτό το έργο χρήσιμο, θα σας ήμουν ευγνώμων εάν μπορούσατε να αφήσετε ένα αστέρι στο GitHub. Η υποστήριξή σας το βοηθά να προσεγγίσει ένα ευρύτερο κοινό και εκτιμάται βαθύτατα. Αυτή η ενθάρρυνση είναι που με ωθεί να συνεχίσω να δημιουργώ.

## 📄 Άδεια Χρήσης

Αυτό το έργο διατίθεται υπό την άδεια GPL-3.0 – δείτε το αρχείο [LICENSE](../LICENSE) για λεπτομέρειες.

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

