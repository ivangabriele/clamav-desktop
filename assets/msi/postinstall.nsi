Outfile "InstallService.exe"
RequestExecutionLevel admin

Section
    SetOutPath $INSTDIR

    ; Copy the binary to the installation directory
    File "clamav-desktop-daemon.exe"

    ; Create the service
    nsExec::ExecToLog 'sc create clamav-desktop-daemon binPath= "$INSTDIR\clamav-desktop-daemon.exe" start= auto'
    Pop $0 ; Get the exit code
    StrCmp $0 0 +3
        MessageBox MB_ICONEXCLAMATION|MB_OK "Failed to create the service. Error code $0."
        Quit

    ; Start the service
    nsExec::ExecToLog 'sc start clamav-desktop-daemon'
    Pop $0
    StrCmp $0 0 +3
        MessageBox MB_ICONEXCLAMATION|MB_OK "Failed to start the service. Error code $0."
        Quit

    MessageBox MB_ICONINFORMATION|MB_OK "Service installed and started successfully."
SectionEnd

Section "Uninstall"
    ; Stop the service before uninstalling
    nsExec::ExecToLog 'sc stop clamav-desktop-daemon'
    ; TODO Find a solid way to wait for the service to stop.
    Sleep 3000 ; Wait for the service to stop

    ; Delete the service
    nsExec::ExecToLog 'sc delete clamav-desktop-daemon'

    ; Delete the binary
    Delete "$INSTDIR\clamav-desktop-daemon.exe"
SectionEnd
