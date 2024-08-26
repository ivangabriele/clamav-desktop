$ROOT_PATH = Join-Path -Path $PSScriptRoot -ChildPath "..\.."

function Test-IsAdmin {
    $currentUser = New-Object Security.Principal.WindowsPrincipal([Security.Principal.WindowsIdentity]::GetCurrent())

    return $currentUser.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

# if (-not (Test-IsAdmin)) {
#     Write-Host "[install_msi_bundle.ps1] Elevating script to run as administrator..."
#     Start-Process powershell.exe "-ExecutionPolicy Bypass -File '$($MyInvocation.MyCommand.Path)'" -Verb RunAs

#     exit
# }

$msiBundlePath = Join-Path -Path $ROOT_PATH -ChildPath "src-tauri\target\x86_64-pc-windows-msvc\release\bundle\msi\ClamAV Desktop_0.3.24_x64_en-US.msi"
if (-not (Test-Path $msiBundlePath)) {
    Write-Host "[install_msi_bundle.ps1] Error: MSI bundle not found at ``$msiBundlePath``." -ForegroundColor Red

    exit
}

# TODO Using `msiexec.exe` doesn't work for some reason (error: "The installation package could not be opened. [...]").
# Write-Host "[install_msi_bundle.ps1] Installing MSI bundle..." -ForegroundColor Blue
# Start-Process msiexec.exe -ArgumentList "/i `"$msiBundlePath`"" -Wait

Write-Host "[install_msi_bundle.ps1] Running MSI bundle..." -ForegroundColor Blue
Start-Process -FilePath $msiBundlePath -Wait

Write-Host "[install_msi_bundle.ps1] Looking for ClamAV Dektop in installed programs..."
$program = Get-CimInstance -ClassName Win32_Product | Where-Object { $_.Name -eq "ClamAV Desktop" }
if ($program) {
    Write-Host "[install_msi_bundle.ps1] ClamAV Dektop successfully installed from MSI bundle." -ForegroundColor Green
} else {
    Write-Host "[install_msi_bundle.ps1] Error: ClamAV Dektop installation from MSI bundle failed." -ForegroundColor Red
}
