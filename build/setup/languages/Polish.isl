; *** Inno Setup version 6.1.0+ Polish messages ***

[LangOptions]
LanguageName=Polish
LanguageID=$0415
LanguageCodePage=0

[Messages]

; *** Application titles
SetupAppTitle=Instalator
SetupWindowTitle=Instalator - %1
UninstallAppTitle=Deinstalator
UninstallAppFullTitle=Deinstalacja %1

; *** Misc. common
InformationTitle=Informacja
ConfirmTitle=Potwierdzenie
ErrorTitle=Błąd

; *** SetupLdr messages
SetupLdrStartupMessage=Zostanie zainstalowany program %1. Czy chcesz kontynuować?
LdrCannotCreateTemp=Instalator nie może utworzyć pliku tymczasowego. Instalacja przerwana
LdrCannotExecTemp=Instalator nie może uruchomić pliku w katalogu tymczasowym. Instalacja przerwana
HelpTextNote=

; *** Startup error messages
LastErrorMessage=%1.%n%nBłąd %2: %3
SetupFileMissing=Brak pliku %1 w katalogu instalacyjnym. Proszę naprawić problem lub uzyskać nową kopię programu.
SetupFileCorrupt=Pliki instalatora są uszkodzone. Proszę uzyskać nową kopię programu.
SetupFileCorruptOrWrongVer=Pliki instalatora są uszkodzone lub niezgodne z tą wersją Instalatora. Proszę naprawić problem lub uzyskać nową kopię programu.
InvalidParameter=Przekazano nieprawidłowy parametr w wierszu polecenia:%n%n%1
SetupAlreadyRunning=Instalator jest już uruchomiony.
WindowsVersionNotSupported=Ten program nie obsługuje wersji Windows uruchomionej na Twoim komputerze.
WindowsServicePackRequired=Ten program wymaga %1 Service Pack %2 lub nowszego.
NotOnThisPlatform=Ten program nie może być uruchomiony na %1.
OnlyOnThisPlatform=Ten program musi być uruchomiony na %1.
OnlyOnTheseArchitectures=Ten program może być zainstalowany tylko na wersjach Windows przeznaczonych dla następujących architektur procesorów:%n%n%1
WinVersionTooLowError=Ten program wymaga %1 wersji %2 lub nowszej.
WinVersionTooHighError=Ten program nie może być zainstalowany na %1 wersji %2 lub nowszej.
AdminPrivilegesRequired=Musisz być zalogowany jako administrator podczas instalacji tego programu.
PowerUserPrivilegesRequired=Musisz być zalogowany jako administrator lub jako członek grupy Użytkownicy zaawansowani podczas instalacji tego programu.
SetupAppRunningError=Instalator wykrył, że %1 jest aktualnie uruchomiony.%n%nProszę zamknąć wszystkie instancje, a następnie kliknąć OK, aby kontynuować, lub Anuluj, aby zakończyć.
UninstallAppRunningError=Deinstalator wykrył, że %1 jest aktualnie uruchomiony.%n%nProszę zamknąć wszystkie instancje, a następnie kliknąć OK, aby kontynuować, lub Anuluj, aby zakończyć.

; *** Startup questions
PrivilegesRequiredOverrideTitle=Wybierz tryb instalacji
PrivilegesRequiredOverrideInstruction=Wybierz tryb instalacji
PrivilegesRequiredOverrideText1=%1 może być zainstalowany dla wszystkich użytkowników (wymaga uprawnień administratora) lub tylko dla Ciebie.
PrivilegesRequiredOverrideText2=%1 może być zainstalowany tylko dla Ciebie lub dla wszystkich użytkowników (wymaga uprawnień administratora).
PrivilegesRequiredOverrideAllUsers=Instaluj dla &wszystkich użytkowników
PrivilegesRequiredOverrideAllUsersRecommended=Instaluj dla &wszystkich użytkowników (zalecane)
PrivilegesRequiredOverrideCurrentUser=Instaluj tylko dla &mnie
PrivilegesRequiredOverrideCurrentUserRecommended=Instaluj tylko dla &mnie (zalecane)

; *** Misc. errors
ErrorCreatingDir=Instalator nie może utworzyć katalogu "%1"
ErrorTooManyFilesInDir=Nie można utworzyć pliku w katalogu "%1", ponieważ zawiera on zbyt wiele plików

; *** Setup common messages
ExitSetupTitle=Zakończ Instalatora
ExitSetupMessage=Instalacja nie została zakończona. Jeżeli teraz zakończysz, program nie zostanie zainstalowany.%n%nMożesz uruchomić Instalator ponownie w innym czasie, aby dokończyć instalację.%n%nZakończyć Instalatora?
AboutSetupMenuItem=
AboutSetupTitle=O Instalatorze
AboutSetupMessage=%1 wersja %2%n%3%n%n%1 strona domowa:%n%4
AboutSetupNote=

; *** Buttons
ButtonBack=< &Wstecz
ButtonNext=&Dalej >
ButtonInstall=&Instaluj
ButtonOK=OK
ButtonCancel=Anuluj
ButtonYes=&Tak
ButtonYesToAll=Tak dla &wszystkich
ButtonNo=&Nie
ButtonNoToAll=N&ie dla wszystkich
ButtonFinish=&Zakończ
ButtonBrowse=&Przeglądaj...
ButtonWizardBrowse=&Podgląd...
ButtonNewFolder=&Utwórz nowy folder

; *** "Select Language" dialog messages
SelectLanguageTitle=Wybierz język instalacji
SelectLanguageLabel=Wybierz język używany podczas instalacji.

; *** Common wizard text
ClickNext=Kliknij Dalej, aby kontynuować, lub Anuluj, aby zakończyć Instalatora.
BeveledLabel=
BrowseDialogTitle=Przeglądaj w poszukiwaniu folderu
BrowseDialogLabel=Wybierz folder z listy poniżej, a następnie kliknij OK.
NewFolderName=Nowy folder

; *** "Welcome" wizard page
WelcomeLabel1=Witamy w Kreatorze instalacji [name]
WelcomeLabel2=Zostanie zainstalowany program [name/ver] na Twoim komputerze.%n%nZaleca się zamknięcie wszystkich innych aplikacji przed kontynuowaniem.

; *** "Password" wizard page
WizardPassword=Hasło
PasswordLabel1=Ta instalacja jest chroniona hasłem.
PasswordLabel3=Proszę przeczytać następujące ważne informacje przed kontynuowaniem.
PasswordEditLabel=&Hasło:
IncorrectPassword=Wprowadzone hasło jest nieprawidłowe. Proszę spróbować ponownie.

; *** "License Agreement" wizard page
WizardLicense=Umowa licencyjna
LicenseLabel=Proszę przeczytać następujące ważne informacje przed kontynuowaniem.
LicenseLabel3=Proszę przeczytać następującą Umowę licencyjną. Musisz zaakceptować warunki tej umowy przed kontynuowaniem instalacji.
LicenseAccepted=&Akceptuję umowę
LicenseNotAccepted=&Nie akceptuję umowy

; *** "Information" wizard pages
WizardInfoBefore=Informacja
InfoBeforeLabel=Proszę przeczytać następujące ważne informacje przed kontynuowaniem.
InfoBeforeClickLabel=Gdy będziesz gotowy do kontynuowania, kliknij Dalej.
WizardInfoAfter=Informacja
InfoAfterLabel=Proszę przeczytać następujące ważne informacje przed kontynuowaniem.
InfoAfterClickLabel=Gdy będziesz gotowy do kontynuowania, kliknij Dalej.

; *** "User Information" wizard page
WizardUserInfo=Informacje o użytkowniku
UserInfoDesc=Proszę wprowadzić swoje dane.
UserInfoName=Nazwa &użytkownika:
UserInfoOrg=&Organizacja:
UserInfoSerial=Numer &seryjny:
UserInfoNameRequired=Musisz wprowadzić nazwę.

; *** "Select Destination Location" wizard page
WizardSelectDir=Wybierz lokalizację docelową
SelectDesc=Gdzie powinien zostać zainstalowany [name]?
SelectDirLabel3=Instalator zainstaluje [name] w następującym folderze.
SelectDirBrowseLabel=Aby kontynuować, kliknij Dalej. Jeżeli chcesz wybrać inny folder, kliknij Przeglądaj.
DiskSpaceGBLabel=Wymagane jest co najmniej [gb] GB wolnego miejsca na dysku.
DiskSpaceMBLabel=Wymagane jest co najmniej [mb] MB wolnego miejsca na dysku.
CannotInstallToNetworkDrive=Instalator nie może instalować na dysku sieciowym.
CannotInstallToUNCPath=Instalator nie może instalować do ścieżki UNC.
InvalidPath=Musisz wprowadzić pełną ścieżkę z literą dysku; na przykład:%n%nC:\APP%n%nlub ścieżkę UNC w formie:%n%n\\server\share
InvalidDrive=Wybrany dysk lub udział UNC nie istnieje lub jest niedostępny. Proszę wybrać inny.
DiskSpaceWarningTitle=Brak wystarczającej ilości miejsca na dysku
DiskSpaceWarning=Instalator wymaga co najmniej %1 KB wolnego miejsca do instalacji, ale wybrany dysk ma dostępne tylko %2 KB.%n%nCzy chcesz mimo to kontynuować?
DirNameTooLong=Nazwa folderu lub ścieżka jest zbyt długa.
InvalidDirName=Nazwa folderu jest nieprawidłowa.
BadDirName32=Nazwa folderu nie może zawierać żadnego z następujących znaków:%n%n%1
DirExistsTitle=Folder istnieje
DirExists=Folder:%n%n%1%n%njuż istnieje. Czy chcesz mimo to zainstalować w tym folderze?
DirDoesntExistTitle=Folder nie istnieje
DirDoesntExist=Folder:%n%n%1%n%nnie istnieje. Czy chcesz, żeby został utworzony?

; *** "Select Components" wizard page
WizardSelectComponents=Wybierz komponenty
SelectComponentsDesc=Które komponenty powinny zostać zainstalowane?
SelectComponentsLabel2=Wybierz komponenty, które chcesz zainstalować; odznacz komponenty, których nie chcesz instalować. Kliknij Dalej, gdy będziesz gotowy do kontynuowania.
FullInstallation=Instalacja pełna
CompactInstallation=Instalacja kompaktowa
CustomInstallation=Instalacja niestandardowa
NoUninstallWarningTitle=Komponent istnieje
NoUninstallWarning=Instalator wykrył, że następujące komponenty są już zainstalowane na Twoim komputerze:%n%n%1%n%nOdznaczenie tych komponentów nie spowoduje ich deinstalacji.%n%nCzy chcesz mimo to kontynuować?
ComponentSize1=%1 KB
ComponentSize2=%1 MB
ComponentsDiskSpaceGBLabel=Bieżący wybór wymaga co najmniej [gb] GB miejsca na dysku.
ComponentsDiskSpaceMBLabel=Bieżący wybór wymaga co najmniej [mb] MB miejsca na dysku.

; *** "Select Additional Tasks" wizard page
WizardSelectTasks=Wybierz dodatkowe zadania
SelectTasksDesc=Które dodatkowe zadania powinny zostać wykonane?
SelectTasksLabel2=Wybierz dodatkowe zadania, które mają zostać wykonane podczas instalacji [name], a następnie kliknij Dalej.

; *** "Select Start Menu Folder" wizard page
WizardSelectProgramGroup=Wybierz folder Menu Start
SelectStartMenuFolderDesc=Gdzie Instalator powinien umieścić skróty programu?
SelectStartMenuFolderLabel3=Instalator utworzy skróty programu w następującym folderze Menu Start.
SelectStartMenuFolderBrowseLabel=Aby kontynuować, kliknij Dalej. Jeżeli chcesz wybrać inny folder, kliknij Przeglądaj.
MustEnterGroupName=Musisz wprowadzić nazwę folderu.
GroupNameTooLong=Nazwa folderu lub ścieżka jest zbyt długa.
InvalidGroupName=Nazwa folderu jest nieprawidłowa.
BadGroupName=Nazwa folderu nie może zawierać żadnego z następujących znaków:%n%n%1
NoProgramGroupCheck2=&Nie twórz folderu Menu Start

; *** "Ready to Install" wizard page
WizardReady=Gotowy do instalacji
ReadyLabel1=Instalator jest gotowy do rozpoczęcia instalacji [name] na Twoim komputerze.
ReadyLabel2a=Kliknij Instaluj, aby kontynuować instalację, lub kliknij Wstecz, jeżeli chcesz przejrzeć lub zmienić ustawienia.
ReadyLabel2b=Kliknij Instaluj, aby kontynuować instalację.
ReadyMemoUserInfo=Informacje o użytkowniku:
ReadyMemoDir=Lokalizacja docelowa:
ReadyMemoType=Typ instalacji:
ReadyMemoComponents=Wybrane komponenty:
ReadyMemoGroup=Folder Menu Start:
ReadyMemoTasks=Dodatkowe zadania:

; *** TDownloadWizardPage wizard page and DownloadTemporaryFile
DownloadingLabel2=Pobieranie dodatkowych plików...
ButtonStopDownload=&Zatrzymaj
StopDownload=Czy na pewno chcesz zatrzymać pobieranie?
ErrorDownloadAborted=Pobieranie przerwane
ErrorDownloadFailed=Pobieranie nie powiodło się: %1 %2
ErrorDownloadSizeFailed=Sprawdzenie rozmiaru nie powiodło się: %1 %2
ErrorProgress=Nieprawidłowy postęp: %1 / %2
ErrorFileSize=Nieprawidłowy rozmiar pliku: oczekiwano %1, otrzymano %2

; *** TExtractionWizardPage wizard page and ExtractArchive
ExtractingLabel=Wypakowywanie dodatkowych plików...
ButtonStopExtraction=&Zatrzymaj
StopExtraction=Czy na pewno chcesz zatrzymać wypakowywanie?
ErrorExtractionAborted=Wypakowywanie przerwane
ErrorExtractionFailed=Wypakowywanie nie powiodło się: %1

; *** Archive extraction failure details
ArchiveIncorrectPassword=Podane hasło jest nieprawidłowe
ArchiveIsCorrupted=Plik jest uszkodzony
ArchiveUnsupportedFormat=Format archiwum nie jest obsługiwany

; *** "Preparing to Install" wizard page
WizardPreparing=Przygotowywanie do instalacji
PreparingDesc=Instalator przygotowuje się do instalacji [name] na Twoim komputerze.
PreviousInstallNotCompleted=Instalacja/usunięcie poprzedniego programu nie zostało zakończone. Będziesz musiał uruchomić ponownie komputer, aby dokończyć tę instalację.%n%nPo ponownym uruchomieniu komputera uruchom ponownie Instalatora, aby dokończyć instalację [name].
CannotContinue=Instalator nie może kontynuować. Proszę kliknąć Anuluj, aby zakończyć.
ApplicationsFound=Następujące aplikacje używają plików, które muszą zostać zaktualizowane przez Instalatora. Zaleca się umożliwienie Instalatorowi automatycznego zamknięcia tych aplikacji.
ApplicationsFound2=Następujące aplikacje używają plików, które muszą zostać zaktualizowane przez Instalatora. Zaleca się umożliwienie Instalatorowi automatycznego zamknięcia tych aplikacji. Po zakończeniu instalacji Instalator spróbuje uruchomić te aplikacje ponownie.
CloseApplications=&Automatycznie zamknij aplikacje
DontCloseApplications=&Nie zamykaj aplikacji
ErrorCloseApplications=Instalator nie mógł automatycznie zamknąć wszystkich aplikacji. Zaleca się zamknięcie wszystkich aplikacji używających plików, które muszą zostać zaktualizowane przez Instalatora, przed kontynuowaniem.
PrepareToInstallNeedsRestart=Instalator musi uruchomić ponownie Twój komputer. Po ponownym uruchomieniu komputera uruchom ponownie Instalatora, aby dokończyć instalację [name].%n%nCzy chcesz uruchomić ponownie teraz?

; *** "Installing" wizard page
WizardInstalling=Instalowanie
InstallingLabel=Proszę czekać, trwa instalacja [name] na Twoim komputerze.

; *** "Setup Completed" wizard page
FinishedHeadingLabel=Kończenie Kreatora instalacji [name]
FinishedLabelNoIcons=Instalator zakończył instalację [name] na Twoim komputerze.
FinishedLabel=Instalator zakończył instalację [name] na Twoim komputerze. Aplikację można uruchomić, wybierając zainstalowane skróty.
ClickFinish=Kliknij Zakończ, aby zamknąć Instalatora.
FinishedRestartLabel=Aby dokończyć instalację [name], Instalator musi uruchomić ponownie Twój komputer. Czy chcesz uruchomić ponownie teraz?
FinishedRestartMessage=Aby dokończyć instalację [name], Instalator musi uruchomić ponownie Twój komputer.%n%nCzy chcesz uruchomić ponownie teraz?
ShowReadmeCheck=Tak, chcę zobaczyć plik README
YesRadio=&Tak, uruchom ponownie komputer teraz
NoRadio=&Nie, uruchomię ponownie komputer później
RunEntryExec=&Uruchom %1
RunEntryShellExec=&Wyświetl %1

; *** "Setup Needs the Next Disk" stuff
ChangeDiskTitle=Instalator potrzebuje następnej dyskietki
SelectDiskLabel2=Proszę włożyć dyskietkę %1 i kliknąć OK.%n%nJeżeli pliki na tej dyskietce mogą być znalezione w folderze innym niż wyświetlony poniżej, wprowadz prawidłową ścieżkę lub kliknij Przeglądaj.
PathLabel=&Ścieżka:
FileNotInDir2=Plik "%1" nie został znaleziony w "%2". Proszę włożyć prawidłową dyskietkę lub wybrać inny folder.
SelectDirectoryLabel=Proszę określić lokalizację następnej dyskietki.

; *** Installation phase messages
SetupAborted=Instalacja nie została zakończona.%n%nProszę naprawić problem i uruchomić Instalatora ponownie.
AbortRetryIgnoreSelectAction=Wybierz działanie
AbortRetryIgnoreRetry=&Spróbuj ponownie
AbortRetryIgnoreIgnore=&Ignoruj błąd i kontynuuj
AbortRetryIgnoreCancel=Anuluj instalację
RetryCancelSelectAction=Wybierz działanie
RetryCancelRetry=&Spróbuj ponownie
RetryCancelCancel=Anuluj

; *** Installation status messages
StatusClosingApplications=Zamykanie aplikacji...
StatusCreateDirs=Tworzenie katalogów...
StatusExtractFiles=Wypakowywanie plików...
StatusDownloadFiles=Pobieranie plików...
StatusCreateIcons=Tworzenie skrótów...
StatusCreateIniEntries=Tworzenie wpisów INI...
StatusCreateRegistryEntries=Tworzenie wpisów rejestru...
StatusRegisterFiles=Rejestrowanie plików...
StatusSavingUninstall=Zapisywanie informacji o deinstalacji...
StatusRunProgram=Kończenie instalacji...
StatusRestartingApplications=Ponowne uruchamianie aplikacji...
StatusRollback=Cofanie zmian...

; *** Misc. errors
ErrorInternal2=Błąd wewnętrzny: %1
ErrorFunctionFailedNoCode=%1 nie powiodło się
ErrorFunctionFailed=%1 nie powiodło się; kod %2
ErrorFunctionFailedWithMessage=%1 nie powiodło się; kod %2.%n%3
ErrorExecutingProgram=Nie można uruchomić pliku:%n%1

; *** Registry errors
ErrorRegOpenKey=Błąd otwierania klucza rejestru:%n%1\%2
ErrorRegCreateKey=Błąd tworzenia klucza rejestru:%n%1\%2
ErrorRegWriteKey=Błąd zapisu do klucza rejestru:%n%1\%2

; *** INI errors
ErrorIniEntry=Błąd tworzenia wpisu INI w pliku "%1".

; *** File copying errors
FileAbortRetryIgnoreSkipNotRecommended=&Pomiń ten plik (niezalecane)
FileAbortRetryIgnoreIgnoreNotRecommended=&Ignoruj błąd i kontynuuj (niezalecane)
SourceIsCorrupted=Plik źródłowy jest uszkodzony
SourceDoesntExist=Plik źródłowy "%1" nie istnieje
SourceVerificationFailed=Weryfikacja pliku źródłowego nie powiodła się: %1
VerificationSignatureDoesntExist=Plik podpisu "%1" nie istnieje
VerificationSignatureInvalid=Plik podpisu "%1" jest nieprawidłowy
VerificationKeyNotFound=Plik podpisu "%1" użył nieznanego klucza
VerificationFileNameIncorrect=Nazwa pliku jest nieprawidłowa
VerificationFileTagIncorrect=Tag pliku jest nieprawidłowy
VerificationFileSizeIncorrect=Rozmiar pliku jest nieprawidłowy
VerificationFileHashIncorrect=Hash pliku jest nieprawidłowy
ExistingFileReadOnly2=Istniejący plik nie może zostać zastąpiony, ponieważ jest oznaczony jako tylko do odczytu.
ExistingFileReadOnlyRetry=Usuń atrybut tylko do odczytu i &spróbuj ponownie
ExistingFileReadOnlyKeepExisting=&Zachowaj istniejący plik
ErrorReadingExistingDest=Wystąpił błąd podczas próby odczytu istniejącego pliku:
FileExistsSelectAction=Wybierz działanie
FileExists2=Plik już istnieje.
FileExistsOverwriteExisting=&Nadpisz istniejący plik
FileExistsKeepExisting=&Zachowaj istniejący plik
FileExistsOverwriteOrKeepAll=&Zastosuj to do wszystkich konfliktów
ExistingFileNewerSelectAction=Wybierz działanie
ExistingFileNewer2=Istniejący plik jest nowszy niż ten, który Instalator próbuje zainstalować.
ExistingFileNewerOverwriteExisting=&Nadpisz istniejący plik
ExistingFileNewerKeepExisting=&Zachowaj istniejący plik (zalecane)
ExistingFileNewerOverwriteOrKeepAll=&Zastosuj to do wszystkich konfliktów
ErrorChangingAttr=Wystąpił błąd podczas próby zmiany atrybutów istniejącego pliku:
ErrorCreatingTemp=Wystąpił błąd podczas próby utworzenia pliku w katalogu docelowym:
ErrorReadingSource=Wystąpił błąd podczas próby odczytu pliku źródłowego:
ErrorCopying=Wystąpił błąd podczas próby skopiowania pliku:
ErrorDownloading=Wystąpił błąd podczas próby pobrania pliku:
ErrorExtracting=Wystąpił błąd podczas próby wypakowania pliku:
ErrorReplacingExistingFile=Wystąpił błąd podczas próby zastąpienia istniejącego pliku:
ErrorRestartReplace=RestartReplace nie powiodło się:
ErrorRenamingTemp=Wystąpił błąd podczas próby zmiany nazwy pliku w katalogu docelowym:
ErrorRegisterServer=Nie można zarejestrować DLL/OCX: %1
ErrorRegSvr32Failed=RegSvr32 nie powiodło się z kodem wyjścia %1
ErrorRegisterTypeLib=Nie można zarejestrować biblioteki typów: %1

; *** Uninstall display name markings
UninstallDisplayNameMark=%1 (%2)
UninstallDisplayNameMarks=%1 (%2, %3)
UninstallDisplayNameMark32Bit=32-bit
UninstallDisplayNameMark64Bit=64-bit
UninstallDisplayNameMarkAllUsers=Wszyscy użytkownicy
UninstallDisplayNameMarkCurrentUser=Bieżący użytkownik

; *** Post-installation errors
ErrorOpeningReadme=Wystąpił błąd podczas próby otwarcia pliku README.
ErrorRestartingComputer=Instalator nie mógł uruchomić ponownie komputera. Proszę zrobić to ręcznie.

; *** Uninstaller messages
UninstallNotFound=Plik "%1" nie istnieje. Nie można odinstalować.
UninstallOpenError=Plik "%1" nie mógł zostać otwarty. Nie można odinstalować.
UninstallUnsupportedVer=Plik dziennika deinstalacji "%1" jest w formacie nierozpoznawanym przez tę wersję deinstalatora. Nie można odinstalować
UninstallUnknownEntry=Napotkano nieznany wpis (%1) w dzienniku deinstalacji
ConfirmUninstall=Czy na pewno chcesz całkowicie usunąć %1 i wszystkie jego komponenty?
UninstallOnlyOnWin64=Tę instalację można odinstalować tylko na 64-bitowym Windows.
OnlyAdminCanUninstall=Tę instalację może odinstalować tylko użytkownik z uprawnieniami administratora.
UninstallStatusLabel=Proszę czekać, trwa usuwanie %1 z Twojego komputera.
UninstalledAll=%1 został pomyślnie usunięty z Twojego komputera.
UninstalledMost=Deinstalacja %1 zakończona.%n%nNiektórych elementów nie można było usunąć. Można je usunąć ręcznie.
UninstalledAndNeedsRestart=Aby dokończyć deinstalację %1, komputer musi zostać uruchomiony ponownie.%n%nCzy chcesz uruchomić ponownie teraz?
UninstallDataCorrupted=Plik "%1" jest uszkodzony. Nie można odinstalować

; *** Uninstallation phase messages
ConfirmDeleteSharedFileTitle=Usunąć plik współdzielony?
ConfirmDeleteSharedFile2=System wskazuje, że następujący plik współdzielony nie jest już używany przez żadne programy. Czy chcesz, żeby Deinstalator usunął ten plik współdzielony?%n%nJeżeli istnieją programy, które nadal używają tego pliku i zostanie on usunięty, te programy mogą nie działać prawidłowo. Jeżeli nie jesteś pewien, wybierz Nie. Pozostawienie pliku w systemie nie spowoduje żadnych szkód.
SharedFileNameLabel=Nazwa pliku:
SharedFileLocationLabel=Lokalizacja:
WizardUninstalling=Status deinstalacji
StatusUninstalling=Deinstalacja %1...

; *** Shutdown block reasons
ShutdownBlockReasonInstallingApp=Instalowanie %1.
ShutdownBlockReasonUninstallingApp=Deinstalacja %1.

[CustomMessages]

CreateSchedulerTask=Utwórz zadanie Task Scheduler (dla automatycznego uruchamiania dystrybucji, automatycznego podłączania USB, przekierowania portów)
SchedulerTaskNote1=└ Opcjonalne podczas instalacji; pominięcie nie wpływa na przyszłe użytkowanie.
SchedulerTaskNote2=└ Może być wywołane ręcznie w każdym momencie po otwarciu oprogramowania.
SchedulerTaskNote3=└ Z powodu braku certyfikatu EV, ta akcja może wywołać fałszywe alarmy przez oprogramowanie AV; proszę na to pozwolić.
CreateDesktopIcon=Utwórz skrót na &pulpicie
LaunchProgram=Uruchom %1
CleanupData=Usuń konfigurację użytkownika i logi: ~\.wsldashboard
StartMenuIcon=Utwórz folder Menu Start (w tym skróty uruchamiania i deinstalacji)
VisitWebsite=Odwiedź oficjalną stronę
UninstallerName=Deinstalator %1
AgreementPrefix=Przeczytałem/Przeczytałam i zgadzam się z
AgreementMiddle=i
PrivacyPolicy=Polityka prywatności
TermsOfService=Warunki świadczenia usług
AgreementRequired=Proszę przeczytać i zaakceptować Politykę prywatności oraz Warunki świadczenia usług przed kontynuowaniem.
