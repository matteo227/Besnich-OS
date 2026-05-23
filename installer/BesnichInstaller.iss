#define MyAppName "Besnich OS"
#define MyAppVersion "1.0.0"
#define MyAppPublisher "Besnich Labs"
#define MyAppURL "https://github.com/matteo227/BesnichOS"

[Setup]
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
DefaultDirName={pf}\BesnichOS
DefaultGroupName={#MyAppName}
OutputDir=.\output
OutputBaseFilename=BesnichOS_Installer
Compression=lzma2/max
SolidCompression=yes
PrivilegesRequired=admin
SetupIconFile=resources\besnich.ico
UninstallDisplayIcon=resources\besnich.ico

[Languages]
Name: "italian"; MessagesFile: "compiler:Languages\Italian.isl"
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "usb"; Description: "Crea chiavetta USB avviabile Besnich"; GroupDescription: "Modalità installazione:"
Name: "baremetal"; Description: "Installa su disco fisico (bare metal)"; GroupDescription: "Modalità installazione:"
Name: "vm"; Description: "Installa su Virtual Machine"; GroupDescription: "Modalità installazione:"

[Files]
Source: "..\iso\BesnichOS.iso"; DestDir: "{tmp}"; Flags: deleteafterinstall
Source: "..\bootloader\build\bootloader.bin"; DestDir: "{app}\boot"; Flags: ignoreversion
Source: "..\kernel\build\besnich_kernel.bin"; DestDir: "{app}\kernel"; Flags: ignoreversion
Source: "..\crypto\build\besnich_loader.efi"; DestDir: "{app}\efi"; Flags: ignoreversion

[Run]
Filename: "{app}\tools\rufus.exe"; Parameters: "{tmp}\BesnichOS.iso"; Tasks: usb; Flags: runascurrentuser
Filename: "{app}\tools\rawrite.exe"; Parameters: "{tmp}\BesnichOS.iso"; Tasks: baremetal; Flags: runascurrentuser
Filename: "{tmp}\BesnichOS.iso"; Tasks: vm; Flags: shellexec

[Code]
function VerifyBesnichIntegrity: Boolean;
var
  StoredHash: String;
  ComputedHash: String;
begin
  StoredHash := 'e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855';
  ComputedHash := GetSHA256OfFile(ExpandConstant('{tmp}\BesnichOS.iso'));
  Result := (ComputedHash = StoredHash);
  
  if not Result then
    WizardForm.PageNameLabel.Caption := 'ALERT SICUREZZA BESNICH';
    MsgBox('ATTENZIONE: L''immagine BesnichOS è stata manomessa!' + #13#10 +
           'Download interrotto. Scarica nuovamente dal repository ufficiale.',
           mbCriticalError, MB_OK);
  end;
end;

function CheckTPM: Boolean;
var
  TPMVersion: string;
begin
  if RegQueryStringValue(HKEY_LOCAL_MACHINE, 
     'SYSTEM\CurrentControlSet\Services\TPM\State', 
     'TPMVersion', TPMVersion) then
  begin
    Result := (TPMVersion >= '2.0');
    if not Result then
      MsgBox('Besnich OS richiede TPM 2.0 per la massima sicurezza.', 
             mbWarning, MB_OK);
  end
  else
    Result := False;
end;

function InitializeSetup: Boolean;
begin
  Result := IsAdminLoggedOn;
  if not Result then
    MsgBox('Besnich OS richiede privilegi di amministratore per installarsi.', 
           mbError, MB_OK);
end;

procedure CurStepChanged(CurStep: TSetupStep);
begin
  if CurStep = ssPostInstall then
  begin
    VerifyBesnichIntegrity;
  end;
end;
