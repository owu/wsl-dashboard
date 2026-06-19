; WSL Dashboard Inno Setup Script
; Version: 0.8.0

#define AppName "WSL Dashboard"
#ifndef AppVersion
  #define AppVersion "0.8.0"
#endif
#define AppPublisher "https://github.com/owu"
#define AppURL "https://www.wslui.com"
#define AppExeName "wsldashboard.exe"
#define AppId "{{5CCAB770-FE6B-4A69-9486-74C5D24D3860}}"

[Setup]
; NOTE: The value of AppId uniquely identifies this application.
; Do not use the same AppId value in installers for other applications.
AppId={#AppId}
AppName={#AppName}
AppVersion={#AppVersion}
AppVerName={#AppName}
AppPublisher={#AppPublisher}
AppSupportURL={#AppURL}
DefaultDirName={autopf}\{#AppName}
VersionInfoCompany=WSL Dashboard
VersionInfoCopyright=2026 WSL Dashboard. All rights reserved.
VersionInfoDescription=https://github.com/owu/wsl-dashboard
VersionInfoProductName=WSL Dashboard Setup
VersionInfoProductVersion={#AppVersion}.0
VersionInfoVersion={#AppVersion}.0
DefaultGroupName={#AppName}
DisableProgramGroupPage=yes
LicenseFile=..\..\LICENSE
; Set to 64-bit installation mode to install to C:\Program Files by default instead of (x86)
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
; Request admin privileges to install for all users (default installation to C:\Program Files)
PrivilegesRequired=admin
PrivilegesRequiredOverridesAllowed=commandline
OutputDir=..\..\build\releases
OutputBaseFilename=WSLDashboard.{#AppVersion}.Setup.x64
SetupIconFile=..\..\assets\logo\logo.ico
UninstallDisplayIcon={app}\{#AppExeName}
UninstallDisplayName={#AppName}
Compression=lzma
SolidCompression=yes
WizardStyle=modern
; Custom wizard image: large image on the left (welcome/finish page), recommended size 164x314
WizardImageFile=..\..\assets\setup\wizard_image.png
; Allow overwriting existing installation
DirExistsWarning=no

[Languages]
Name: "english"; MessagesFile: "languages\English.isl"
Name: "chinesesimplified"; MessagesFile: "languages\ChineseSimplified.isl"
Name: "spanish"; MessagesFile: "languages\Spanish.isl"
Name: "hindi"; MessagesFile: "languages\Hindi.isl"
Name: "french"; MessagesFile: "languages\French.isl"
Name: "arabic"; MessagesFile: "languages\Arabic.isl"
Name: "bengali"; MessagesFile: "languages\Bengali.isl"
Name: "portuguese"; MessagesFile: "languages\Portuguese.isl"
Name: "russian"; MessagesFile: "languages\Russian.isl"
Name: "urdu"; MessagesFile: "languages\Urdu.isl"
Name: "indonesian"; MessagesFile: "languages\Indonesian.isl"
Name: "german"; MessagesFile: "languages\German.isl"
Name: "japanese"; MessagesFile: "languages\Japanese.isl"
Name: "chinesetraditional"; MessagesFile: "languages\ChineseTraditional.isl"
Name: "turkish"; MessagesFile: "languages\Turkish.isl"
Name: "korean"; MessagesFile: "languages\Korean.isl"
Name: "italian"; MessagesFile: "languages\Italian.isl"
Name: "dutch"; MessagesFile: "languages\Dutch.isl"
Name: "swedish"; MessagesFile: "languages\Swedish.isl"
Name: "czech"; MessagesFile: "languages\Czech.isl"
Name: "greek"; MessagesFile: "languages\Greek.isl"
Name: "hungarian"; MessagesFile: "languages\Hungarian.isl"
Name: "norwegian"; MessagesFile: "languages\Norwegian.isl"
Name: "danish"; MessagesFile: "languages\Danish.isl"
Name: "finnish"; MessagesFile: "languages\Finnish.isl"
Name: "slovak"; MessagesFile: "languages\Slovak.isl"
Name: "slovenian"; MessagesFile: "languages\Slovenian.isl"
Name: "hebrew"; MessagesFile: "languages\Hebrew.isl"
Name: "icelandic"; MessagesFile: "languages\Icelandic.isl"
Name: "vietnamese"; MessagesFile: "languages\Vietnamese.isl"
Name: "telugu"; MessagesFile: "languages\Telugu.isl"
Name: "javanese"; MessagesFile: "languages\Javanese.isl"
Name: "thai"; MessagesFile: "languages\Thai.isl"
Name: "tamil"; MessagesFile: "languages\Tamil.isl"
Name: "filipino"; MessagesFile: "languages\Filipino.isl"
Name: "punjabi"; MessagesFile: "languages\Punjabi.isl"
Name: "malay"; MessagesFile: "languages\Malay.isl"
Name: "polish"; MessagesFile: "languages\Polish.isl"
Name: "ukrainian"; MessagesFile: "languages\Ukrainian.isl"
Name: "persian"; MessagesFile: "languages\Persian.isl"
Name: "kannada"; MessagesFile: "languages\Kannada.isl"
Name: "marathi"; MessagesFile: "languages\Marathi.isl"
Name: "hausa"; MessagesFile: "languages\Hausa.isl"
Name: "burmese"; MessagesFile: "languages\Burmese.isl"
Name: "uzbek"; MessagesFile: "languages\Uzbek.isl"
Name: "azerbaijani"; MessagesFile: "languages\Azerbaijani.isl"
Name: "cebuano"; MessagesFile: "languages\Cebuano.isl"
Name: "malayalam"; MessagesFile: "languages\Malayalam.isl"
Name: "sindhi"; MessagesFile: "languages\Sindhi.isl"
Name: "amharic"; MessagesFile: "languages\Amharic.isl"

[Messages]
SetupWindowTitle={#AppName} v{#AppVersion}
SelectLanguageTitle={#AppName} v{#AppVersion}
PrivilegesRequiredOverrideTitle={#AppName} v{#AppVersion}


[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"
Name: "startmenuicon"; Description: "{cm:StartMenuIcon}"; Flags: checkedonce
Name: "createscheduler"; Description: "{cm:CreateSchedulerTask}"; Check: IsAdminInstallMode; Flags: unchecked

[Files]
Source: "..\..\target\release\{#AppExeName}"; DestDir: "{app}"; Flags: ignoreversion
; Add other non-embedded resources here if needed
; Source: "assets\*"; DestDir: "{app}\assets"; Flags: ignoreversion recursesubdirs createallsubdirs

[Run]
Filename: "{app}\{#AppExeName}"; Description: "{cm:LaunchProgram,{#StringChange(AppName, '&', '&&')}}"; Flags: nowait postinstall skipifsilent

[Dirs]
; Explicitly create start menu directory
Name: "{group}"; Tasks: startmenuicon

[Icons]
; Program shortcut in start menu directory (created based on task)
Name: "{group}\{#AppName}"; Filename: "{app}\{#AppExeName}"; Tasks: startmenuicon
; Visit website (ensures folder is not automatically merged in Windows 11)
Name: "{group}\{cm:VisitWebsite}"; Filename: "{#AppURL}"; Tasks: startmenuicon
; Uninstall shortcut in start menu directory (created with program shortcut)
Name: "{group}\{cm:UninstallerName,{#AppName}}"; Filename: "{uninstallexe}"; Parameters: "/SILENT"; IconFilename: "{sys}\shell32.dll"; IconIndex: 31; Tasks: startmenuicon
; Desktop shortcut (optional task)
Name: "{autodesktop}\{#AppName}"; Filename: "{app}\{#AppExeName}"; Tasks: desktopicon


[Code]
const
  GWL_EXSTYLE = -20;
  WS_EX_APPWINDOW = $00040000;
  GWL_HWNDPARENT = -8;
  APP_URL = '{#AppURL}';

function GetWindowLong(hWnd: HWND; nIndex: Integer): Longint;
  external 'GetWindowLongW@user32.dll stdcall';
function SetWindowLong(hWnd: HWND; nIndex: Integer; dwNewLong: Longint): Longint;
  external 'SetWindowLongW@user32.dll stdcall';
function SetWindowLongPtr(hWnd: HWND; nIndex: Integer; dwNewLong: Longint): Longint;
  external 'SetWindowLongW@user32.dll stdcall';
function SetForegroundWindow(hWnd: HWND): Boolean;
  external 'SetForegroundWindow@user32.dll stdcall';
function BringWindowToTop(hWnd: HWND): Boolean;
  external 'BringWindowToTop@user32.dll stdcall';
function GetSystemMenu(hWnd: HWND; bRevert: Boolean): LongWord;
  external 'GetSystemMenu@user32.dll stdcall';
function DeleteMenu(hMenu: LongWord; uPosition: UINT; uFlags: UINT): Boolean;
  external 'DeleteMenu@user32.dll stdcall';
function GetMenuItemCount(hMenu: LongWord): Integer;
  external 'GetMenuItemCount@user32.dll stdcall';
function SendMessage(hWnd: HWND; Msg: UINT; wParam: LongInt; lParam: LongInt): LongInt;
  external 'SendMessageW@user32.dll stdcall';

const
  PRIVACY_POLICY_URL = APP_URL + '/privacy/';
  TERMS_OF_SERVICE_URL = APP_URL + '/terms/';

var
  ShouldCleanup: Boolean;
  CleanupCheckBox: TNewCheckBox;
  SchedulerNote1, SchedulerNote2, SchedulerNote3: TNewStaticText;
  AgreementCheckBox: TNewCheckBox;
  AgreementPrefixLabel, AgreementMiddleLabel, PrivacyPolicyLabel, TermsLabel: TLabel;

function IsSilentMode(): Boolean;
var
  I: Integer;
begin
  Result := False;
  for I := 1 to ParamCount do
  begin
    if (CompareText(ParamStr(I), '/SILENT') = 0) or (CompareText(ParamStr(I), '/VERYSILENT') = 0) then
    begin
      Result := True;
      Exit;
    end;
  end;
end;

procedure CleanupLabelClick(Sender: TObject);
begin
  CleanupCheckBox.Checked := not CleanupCheckBox.Checked;
end;

// Toggle agreement checkbox when prefix text is clicked
procedure AgreementPrefixClick(Sender: TObject);
begin
  AgreementCheckBox.Checked := not AgreementCheckBox.Checked;
end;

// Open privacy policy link in default browser
procedure PrivacyPolicyClick(Sender: TObject);
var
  ErrorCode: Integer;
begin
  ShellExec('open', PRIVACY_POLICY_URL, '', '', SW_SHOWNORMAL, ewNoWait, ErrorCode);
end;

// Open terms of service link in default browser
procedure TermsClick(Sender: TObject);
var
  ErrorCode: Integer;
begin
  ShellExec('open', TERMS_OF_SERVICE_URL, '', '', SW_SHOWNORMAL, ewNoWait, ErrorCode);
end;

// Open official website in default browser when URL label is clicked
procedure URLLabelClick(Sender: TObject);
var
  ErrorCode: Integer;
begin
  ShellExec('open', APP_URL, '', '', SW_SHOWNORMAL, ewNoWait, ErrorCode);
end;

// Insert clickable website link at the bottom of all installation pages
procedure InitializeWizard();
var
  URLLabel: TNewStaticText;
begin
  // Force window to foreground to resolve potential focus loss after UAC elevation
  WizardForm.BringToFront;
  SetForegroundWindow(WizardForm.Handle);

  // Insert link label in the area below the wizard bottom bevel (separator)
  URLLabel := TNewStaticText.Create(WizardForm);
  URLLabel.Parent := WizardForm;
  // Align horizontally to the left button area
  URLLabel.Left := WizardForm.ClientWidth div 20;
  // Vertical position: align with bottom button row (CancelButton.Top is button row Top)
  URLLabel.Top := WizardForm.CancelButton.Top + (WizardForm.CancelButton.Height - URLLabel.Height) div 2;
  URLLabel.Caption := APP_URL;
  URLLabel.Font.Color := $C06020;   // Orange-brown color, noticeable but not harsh
  URLLabel.Font.Style := [fsUnderline];
  URLLabel.Cursor := crHand;
  URLLabel.OnClick := @URLLabelClick;

  // Create supplementary note labels for scheduler task (only shown on select tasks page)
  SchedulerNote1 := TNewStaticText.Create(WizardForm);
  SchedulerNote1.Parent := WizardForm.SelectTasksPage;
  SchedulerNote1.Caption := CustomMessage('SchedulerTaskNote1');
  SchedulerNote1.Left := ScaleX(24); // Continue aligning to the left
  SchedulerNote1.Top := ScaleY(95);  // Fine-tune position again, reduce spacing
  SchedulerNote1.Font.Color := clGray;

  SchedulerNote2 := TNewStaticText.Create(WizardForm);
  SchedulerNote2.Parent := WizardForm.SelectTasksPage;
  SchedulerNote2.Caption := CustomMessage('SchedulerTaskNote2');
  SchedulerNote2.Left := SchedulerNote1.Left;
  SchedulerNote2.Top := SchedulerNote1.Top + ScaleY(18);
  SchedulerNote2.Font.Color := clGray;

  SchedulerNote3 := TNewStaticText.Create(WizardForm);
  SchedulerNote3.Parent := WizardForm.SelectTasksPage;
  SchedulerNote3.Caption := CustomMessage('SchedulerTaskNote3');
  SchedulerNote3.Left := SchedulerNote1.Left;
  SchedulerNote3.Top := SchedulerNote2.Top + ScaleY(18);
  SchedulerNote3.Width := WizardForm.SelectTasksPage.Width - SchedulerNote3.Left - ScaleX(20);
  SchedulerNote3.WordWrap := True;
  SchedulerNote3.Font.Color := clGray;

  // Reduce ReadyMemo height to make room for agreement checkbox
  WizardForm.ReadyMemo.Height := WizardForm.ReadyMemo.Height - ScaleY(30);

  // Create agreement checkbox on Ready to Install page
  AgreementCheckBox := TNewCheckBox.Create(WizardForm);
  AgreementCheckBox.Parent := WizardForm.ReadyPage;
  AgreementCheckBox.Left := WizardForm.ReadyMemo.Left;
  AgreementCheckBox.Top := WizardForm.ReadyMemo.Top + WizardForm.ReadyMemo.Height + ScaleY(16);
  AgreementCheckBox.Width := ScaleX(20);
  AgreementCheckBox.Height := ScaleY(20);
  AgreementCheckBox.Caption := '';
  AgreementCheckBox.Checked := False;

  // Create agreement text label with inline links
  // Prefix text: "I have read and agree to the"
  AgreementPrefixLabel := TLabel.Create(WizardForm);
  AgreementPrefixLabel.Parent := WizardForm.ReadyPage;
  AgreementPrefixLabel.Left := AgreementCheckBox.Left + ScaleX(16);
  AgreementPrefixLabel.Top := AgreementCheckBox.Top + ScaleY(2);
  AgreementPrefixLabel.Caption := CustomMessage('AgreementPrefix') + ' ';
  AgreementPrefixLabel.Cursor := crHand;
  AgreementPrefixLabel.OnClick := @AgreementPrefixClick;

  // Privacy Policy link
  PrivacyPolicyLabel := TLabel.Create(WizardForm);
  PrivacyPolicyLabel.Parent := WizardForm.ReadyPage;
  PrivacyPolicyLabel.Left := AgreementPrefixLabel.Left + AgreementPrefixLabel.Width;
  PrivacyPolicyLabel.Top := AgreementPrefixLabel.Top;
  PrivacyPolicyLabel.Caption := CustomMessage('PrivacyPolicy');
  PrivacyPolicyLabel.Font.Color := $C06020;
  PrivacyPolicyLabel.Font.Style := [fsUnderline];
  PrivacyPolicyLabel.Cursor := crHand;
  PrivacyPolicyLabel.OnClick := @PrivacyPolicyClick;

  // Middle text: "and"
  AgreementMiddleLabel := TLabel.Create(WizardForm);
  AgreementMiddleLabel.Parent := WizardForm.ReadyPage;
  AgreementMiddleLabel.Left := PrivacyPolicyLabel.Left + PrivacyPolicyLabel.Width;
  AgreementMiddleLabel.Top := AgreementPrefixLabel.Top;
  AgreementMiddleLabel.Caption := ' ' + CustomMessage('AgreementMiddle') + ' ';

  // Terms of Service link
  TermsLabel := TLabel.Create(WizardForm);
  TermsLabel.Parent := WizardForm.ReadyPage;
  TermsLabel.Left := AgreementMiddleLabel.Left + AgreementMiddleLabel.Width;
  TermsLabel.Top := AgreementPrefixLabel.Top;
  TermsLabel.Caption := CustomMessage('TermsOfService');
  TermsLabel.Font.Color := $C06020;
  TermsLabel.Font.Style := [fsUnderline];
  TermsLabel.Cursor := crHand;
  TermsLabel.OnClick := @TermsClick;
end;

// Remove redundant items from system menu
procedure CleanSystemMenu();
var
  hMenu: LongWord;
  nCount: Integer;
begin
  hMenu := GetSystemMenu(WizardForm.Handle, False);
  if hMenu <> 0 then
  begin
    // Remove standard system menu items (using MF_BYCOMMAND = $0)
    DeleteMenu(hMenu, $F120, $0); // SC_RESTORE
    DeleteMenu(hMenu, $F000, $0); // SC_SIZE
    DeleteMenu(hMenu, $F030, $0); // SC_MAXIMIZE

    // Remove Inno Setup injected "About" item
    nCount := GetMenuItemCount(hMenu);
    if nCount > 0 then
    begin
      // $400 is MF_BYPOSITION
      // Remove last item (About) and second-to-last item (Separator)
      DeleteMenu(hMenu, nCount - 1, $400);
      DeleteMenu(hMenu, nCount - 2, $400);
    end;
  end;
end;

procedure CurPageChanged(CurPageID: Integer);
var
  ItemHeight: Integer;
  TaskItemCount: Integer;
  NoteTop: Integer;
begin
  if CurPageID = wpSelectTasks then
  begin
    // Dynamically position SchedulerNote labels below the tasks list items
    // LB_GETITEMHEIGHT = $01A1, wParam=0 returns default item height
    ItemHeight := SendMessage(WizardForm.TasksList.Handle, $01A1, 0, 0);
    TaskItemCount := WizardForm.TasksList.Items.Count;
    NoteTop := WizardForm.TasksList.Top + TaskItemCount * ItemHeight + ScaleY(4);

    SchedulerNote1.Top := NoteTop;
    SchedulerNote2.Top := SchedulerNote1.Top + ScaleY(18);
    SchedulerNote3.Top := SchedulerNote2.Top + ScaleY(18);
  end;

  if CurPageID = wpWelcome then
  begin
    CleanSystemMenu();
  end;
end;

// Directory safety check: auto-append "\WSL Dashboard" if the last segment is not the app name
function NextButtonClick(CurPageID: Integer): Boolean;
var
  SelectedDir: String;
  LastSegment: String;
  I: Integer;
  LastSep: Integer;
begin
  Result := True; // Allow proceeding by default

  if CurPageID = wpSelectDir then
  begin
    SelectedDir := WizardForm.DirEdit.Text;

    // Remove trailing backslash if present
    if (Length(SelectedDir) > 0) and (SelectedDir[Length(SelectedDir)] = '\') then
      SetLength(SelectedDir, Length(SelectedDir) - 1);

    // Find the last backslash position
    LastSep := 0;
    for I := 1 to Length(SelectedDir) do
    begin
      if SelectedDir[I] = '\' then
        LastSep := I;
    end;

    if LastSep > 0 then
      LastSegment := Copy(SelectedDir, LastSep + 1, MaxInt)
    else
      LastSegment := SelectedDir;

    // If the last directory segment is not "WSL Dashboard", append it
    if CompareText(LastSegment, '{#AppName}') <> 0 then
    begin
      SelectedDir := SelectedDir + '\{#AppName}';
      WizardForm.DirEdit.Text := SelectedDir;
    end;
  end;

  // Agreement checkbox validation on Ready to Install page
  // Skip in silent mode (/SILENT, /VERYSILENT) since UI pages are not shown
  if (CurPageID = wpReady) and (not IsSilentMode()) then
  begin
    if not AgreementCheckBox.Checked then
    begin
      MsgBox(CustomMessage('AgreementRequired'), mbError, MB_OK);
      Result := False;
    end;
  end;
end;

// Modify registry after installation to add /SILENT parameter to uninstall command and skip default popup
procedure CurStepChanged(CurStep: TSetupStep);
var
  UninstallKey, UninstallString: String;
  ResultCode: Integer;
begin
  if CurStep = ssPostInstall then
  begin
    // Execute initialization if installing in admin mode and user checked create scheduler task
    if IsAdminInstallMode and WizardIsTaskSelected('createscheduler') then
    begin
      if not Exec(ExpandConstant('{app}\{#AppExeName}'), '/initialize', '', SW_HIDE, ewWaitUntilTerminated, ResultCode) then
      begin
        Log('Failed to execute initialization: ' + IntToStr(ResultCode));
      end;
    end;

    UninstallKey := 'Software\Microsoft\Windows\CurrentVersion\Uninstall\' + '{#AppId}' + '_is1';
    if IsAdminInstallMode then
    begin
      if RegQueryStringValue(HKEY_LOCAL_MACHINE, UninstallKey, 'UninstallString', UninstallString) then
      begin
        if Pos('/SILENT', UpperCase(UninstallString)) = 0 then
        begin
          RegWriteStringValue(HKEY_LOCAL_MACHINE, UninstallKey, 'UninstallString', UninstallString + ' /SILENT');
          RegWriteStringValue(HKEY_LOCAL_MACHINE, UninstallKey, 'QuietUninstallString', UninstallString + ' /SILENT');
        end;
      end;
    end
    else
    begin
      if RegQueryStringValue(HKEY_CURRENT_USER, UninstallKey, 'UninstallString', UninstallString) then
      begin
        if Pos('/SILENT', UpperCase(UninstallString)) = 0 then
        begin
          RegWriteStringValue(HKEY_CURRENT_USER, UninstallKey, 'UninstallString', UninstallString + ' /SILENT');
          RegWriteStringValue(HKEY_CURRENT_USER, UninstallKey, 'QuietUninstallString', UninstallString + ' /SILENT');
        end;
      end;
    end;
  end;
end;

function HasCmdLineParam(Param: String): Boolean;
var
  I: Integer;
begin
  Result := False;
  for I := 1 to ParamCount do
  begin
    if CompareText(ParamStr(I), Param) = 0 then
    begin
      Result := True;
      Break;
    end;
  end;
end;

function GetCmdLineParamValue(Param: String): String;
var
  I: Integer;
  Arg: String;
begin
  Result := '';
  for I := 1 to ParamCount do
  begin
    Arg := UpperCase(ParamStr(I));
    if Pos(UpperCase(Param) + '=', Arg) = 1 then
    begin
      Result := Copy(ParamStr(I), Length(Param) + 2, Length(ParamStr(I)));
      Break;
    end;
  end;
end;

// Check if the application is currently running
function IsAppRunning(): Boolean;
var
  ResultCode: Integer;
begin
  Result := Exec('cmd.exe', '/c tasklist /FI "IMAGENAME eq {#AppExeName}" | find /I "{#AppExeName}"', '', SW_HIDE, ewWaitUntilTerminated, ResultCode) and (ResultCode = 0);
end;

// Terminate the running process (polling loop, wait up to MaxWaitMs milliseconds)
function KillAppProcess(): Boolean;
var
  ResultCode: Integer;
  Waited: Integer;
  MaxWaitMs: Integer;
  CheckIntervalMs: Integer;
begin
  Result := False;
  MaxWaitMs := 5000;       // Maximum wait time: 5 seconds
  CheckIntervalMs := 250;  // Check every 250 milliseconds

  // /F = Force termination, /IM = Match by image name
  if not Exec('cmd.exe', '/c taskkill /F /IM {#AppExeName}', '', SW_HIDE, ewWaitUntilTerminated, ResultCode) then
    Exit;

  // Poll to check if process has exited
  Waited := 0;
  while Waited < MaxWaitMs do
  begin
    Sleep(CheckIntervalMs);
    Waited := Waited + CheckIntervalMs;

    // Process has exited, return immediately
    if not IsAppRunning() then
    begin
      Result := True;
      Exit;
    end;
  end;

  // Check again after timeout
  Result := not IsAppRunning();
end;

// Uninstall logic: take over all entry points to ensure only one custom popup appears
function InitializeUninstall(): Boolean;
var
  UninstallForm: TForm;
  ConfirmLabel, CleanupLabel: TLabel;
  YesButton, NoButton: TNewButton;
  ResultCode: Integer;
  ExecParams: String;
begin
  Result := False;

  // Check if application is running, auto-terminate if found
  if IsAppRunning() then
  begin
    if not KillAppProcess() then
    begin
      MsgBox(CustomMessage('FailedToCloseApp'), mbError, MB_OK);
      Result := False;
      Exit;
    end;
    // Wait for process to fully exit, then check again
    if IsAppRunning() then
    begin
      MsgBox(CustomMessage('FailedToCloseApp'), mbError, MB_OK);
      Result := False;
      Exit;
    end;
  end;

  // If this is a silent uninstall pass-through process initiated by us, directly read parameters and allow
  if HasCmdLineParam('/PASSTHROUGH') then
  begin
    ShouldCleanup := (GetCmdLineParamValue('/CLEANUP') = '1');
    Result := True;
    Exit;
  end;

  // Silent uninstall (winget / silent mode): skip custom dialog, proceed without cleanup
  if IsSilentMode() then
  begin
    ShouldCleanup := False;
    Result := True;
    Exit;
  end;

  // Create custom window
  UninstallForm := TForm.Create(nil);
  // Adjust to a smaller size
  UninstallForm.ClientWidth := ScaleX(400);
  UninstallForm.ClientHeight := ScaleY(180);
  UninstallForm.Caption := FmtMessage(CustomMessage('UninstallerName'), ['{#AppName}']);
  UninstallForm.Position := poScreenCenter;
  
  // Set taskbar visibility
  SetWindowLong(UninstallForm.Handle, GWL_EXSTYLE, GetWindowLong(UninstallForm.Handle, GWL_EXSTYLE) or WS_EX_APPWINDOW);
  SetWindowLongPtr(UninstallForm.Handle, GWL_HWNDPARENT, 0);
  
  ConfirmLabel := TLabel.Create(UninstallForm);
  ConfirmLabel.Parent := UninstallForm;
  ConfirmLabel.AutoSize := False;
  ConfirmLabel.Left := ScaleX(20);
  ConfirmLabel.Top := ScaleY(15);
  ConfirmLabel.Width := UninstallForm.ClientWidth - ScaleX(40);
  ConfirmLabel.Height := ScaleY(45);
  ConfirmLabel.WordWrap := True;
  ConfirmLabel.Font.Size := 10;
  ConfirmLabel.Caption := FmtMessage(SetupMessage(msgConfirmUninstall), ['{#AppName}']);
  
  CleanupCheckBox := TNewCheckBox.Create(UninstallForm);
  CleanupCheckBox.Parent := UninstallForm;
  CleanupCheckBox.Left := ConfirmLabel.Left;
  CleanupCheckBox.Top := ConfirmLabel.Top + ConfirmLabel.Height + ScaleY(5);
  CleanupCheckBox.Width := ScaleX(16);
  CleanupCheckBox.Height := ScaleY(16);
  CleanupCheckBox.Caption := '';
  CleanupCheckBox.Checked := False;

  CleanupLabel := TLabel.Create(UninstallForm);
  CleanupLabel.Parent := UninstallForm;
  CleanupLabel.AutoSize := False;
  CleanupLabel.WordWrap := True;
  CleanupLabel.Left := CleanupCheckBox.Left + ScaleX(20);
  CleanupLabel.Top := CleanupCheckBox.Top + ScaleY(2);
  CleanupLabel.Width := UninstallForm.ClientWidth - CleanupLabel.Left - ScaleX(20);
  CleanupLabel.Height := ScaleY(45);
  CleanupLabel.Caption := CustomMessage('CleanupData');
  CleanupLabel.OnClick := @CleanupLabelClick;
  
  YesButton := TNewButton.Create(UninstallForm);
  YesButton.Parent := UninstallForm;
  YesButton.Caption := SetupMessage(msgButtonYes);
  YesButton.ModalResult := mrYes;
  YesButton.Width := ScaleX(80);
  YesButton.Height := ScaleY(30);
  YesButton.Left := UninstallForm.ClientWidth - ScaleX(180);
  YesButton.Top := UninstallForm.ClientHeight - ScaleY(45);
  
  NoButton := TNewButton.Create(UninstallForm);
  NoButton.Parent := UninstallForm;
  NoButton.Caption := SetupMessage(msgButtonNo);
  NoButton.ModalResult := mrNo;
  NoButton.Width := ScaleX(80);
  NoButton.Height := ScaleY(30);
  NoButton.Left := UninstallForm.ClientWidth - ScaleX(90);
  NoButton.Top := YesButton.Top;
  NoButton.Default := True;

  if UninstallForm.ShowModal = mrYes then
  begin
    ShouldCleanup := CleanupCheckBox.Checked;
    
    // If already in silent mode (e.g., launched from Control Panel or Start Menu shortcut), allow directly
    if HasCmdLineParam('/SILENT') or HasCmdLineParam('/VERYSILENT') then
    begin
      Result := True;
    end
    else
    begin
      // Otherwise (e.g., user double-clicks unins000.exe directly), to skip Inno's built-in confirmation popup,
      // we relaunch the uninstaller with /SILENT and our pass-through parameters
      if ShouldCleanup then
        ExecParams := '/SILENT /PASSTHROUGH /CLEANUP=1'
      else
        ExecParams := '/SILENT /PASSTHROUGH /CLEANUP=0';
        
      Exec(ExpandConstant('{uninstallexe}'), ExecParams, '', SW_SHOW, ewNoWait, ResultCode);
      Result := False; // Abort current non-silent uninstall
    end;
  end;
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
var
  AppPath: String;
  Params: String;
  ResultCode: Integer;
begin
  // Execute cleanup logic at the start of uninstall (before file deletion)
  if CurUninstallStep = usUninstall then
  begin
    // System-level cleanup (scheduler tasks, registry, etc.) should always execute regardless of user's cleanup choice
    Params := '/clean';
    if ShouldCleanup then
      Params := Params + ' /all';

    // Call the program's own cleanup logic
    if not Exec(ExpandConstant('{app}\{#AppExeName}'), Params, '', SW_HIDE, ewWaitUntilTerminated, ResultCode) then
    begin
      Log('Failed to execute cleanup command: ' + Params);
    end;
  end;

  if CurUninstallStep = usPostUninstall then
  begin
    // Force cleanup of installation directory and any logs/temp files that may have been created
    // Note: Config directory .wsldashboard is already handled by the /clean /all logic in the previous step, no need to repeat
    AppPath := ExpandConstant('{app}');
    if DirExists(AppPath) then
    begin
      Log('Force cleaning up app directory: ' + AppPath);
      DelTree(AppPath, True, True, True);
    end;
  end;
end;

// Check if process is running before installation
function InitializeSetup(): Boolean;
begin
  Result := True;

  // Check if application is running, auto-terminate if found
  if IsAppRunning() then
  begin
    if not KillAppProcess() then
    begin
      MsgBox(CustomMessage('FailedToCloseApp'), mbError, MB_OK);
      Result := False;
      Exit;
    end;
    // Wait for process to fully exit, then check again
    if IsAppRunning() then
    begin
      MsgBox(CustomMessage('FailedToCloseApp'), mbError, MB_OK);
      Result := False;
      Exit;
    end;
  end;
end;
