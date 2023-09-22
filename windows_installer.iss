[Setup]
AppName=Thriftshop
AppVersion=1.0
DefaultDirName={pf}\Thriftshop
PrivilegesRequired=admin  ; Request administrative privileges

[Files]
; Include your Flutter-compiled app
Source: "build\windows\runner\Release\*"; DestDir: "{app}"; Flags: recursesubdirs createallsubdirs

; Include your PowerShell scripts for Hugo installation
Source: ".\powershell_scripts\hugo_install.ps1"; DestDir: "{tmp}"; Flags: deleteafterinstall

[Code]
function InitializeSetup(): Boolean;
var
  cmd: string;
  ResultCode: Integer;
  hugoPath, appPath: string;
begin
  ; Remove old Hugo installation if it exists
  hugoPath := ExpandConstant('{userappdata}\.local\bin\hugo');
  if DirExists(hugoPath) then
  begin
    DelTree(hugoPath, True, True, True);
  end;

  ; Remove old app if it exists
  appPath := ExpandConstant('{pf}\Thriftshop');
  if DirExists(appPath) then
  begin
    DelTree(appPath, True, True, True);
  end;

  ; Run Hugo-install PowerShell script
  cmd := '"powershell.exe" -ExecutionPolicy Bypass -File "' + ExpandConstant('{tmp}\hugo_install.ps1') + '" 0.118.2 --extended';
  Exec(cmd, '', '', SW_HIDE, ewWaitUntilTerminated, ResultCode);

  ; Check if Hugo installation was successful
  if ResultCode <> 0 then
  begin
    MsgBox('Hugo installation failed. The installer will restart.', mbError, MB_OK);
    Exec(ExpandConstant('{srcexe}'), '', '', SW_SHOWNORMAL, ewNoWait, ResultCode);
    Result := False;
    exit;
  end;

  ; Cleanup: Your code for removing temporary or downloaded files, if any

  Result := True;
end;
