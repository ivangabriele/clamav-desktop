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

$programName = "ClamAV Desktop"
$directoriesToDelete = @(
    "C:\Program Files\ClamAV Desktop",
    "C:\Users\ivang\AppData\Local\com.clamav-desktop.app",
    "C:\Users\ivang\AppData\Roaming\com.clamav-desktop.app"
)

Write-Host "[uninstall_msi_bundle.ps1] Looking for ClamAV Dektop in installed programs..."
$program = Get-CimInstance -ClassName Win32_Product | Where-Object { $_.Name -eq $programName }
if ($program) {
    Write-Host "[uninstall_msi_bundle.ps1] Uninstalling ClamAV Dektop..." -ForegroundColor Blue
    Start-Process msiexec.exe -ArgumentList "/x `"$($program.IdentifyingNumber)`"" -Wait

    Write-Host "[uninstall_msi_bundle.ps1] ClamAV Dektop successfully uninstalled from installed programs." -ForegroundColor Blue
} else {
    Write-Host "[uninstall_msi_bundle.ps1] Error: ClamAV Dektop not found in installed programs." -ForegroundColor Red

    exit
}

Write-Host "[uninstall_msi_bundle.ps1] Cleaning up leftover directories..."
foreach ($directory in $directoriesToDelete) {
    if (Test-Path $directory) {
        Write-Host "[uninstall_msi_bundle.ps1] Deleting directory: ``$directory``..."
        Remove-Item -Recurse -Force $directory
    } else {
        Write-Host "[uninstall_msi_bundle.ps1] Directory already deleted: ``$directory``. Continuing..."
    }
}

Write-Host "[uninstall_msi_bundle.ps1] MSI bundle uninstallation complete." -ForegroundColor Green
