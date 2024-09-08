# Check if the script is run as administrator, otherwise relaunch it with elevated privileges
if (-not ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    Write-Host "[install_windows_service] Must be run as an administrator. Relaunching with elevated privileges..."

    # Relaunch script with elevated privileges using 'RunAs'
    Start-Process powershell "-ExecutionPolicy Bypass -File $PSCommandPath" -Verb RunAs

    exit
}

$daemonBinaryPath = "C:\Program Files\ClamAV Desktop\clamav-desktop-daemon.exe"
$serviceName = "clamav-desktop-daemon"

if (-Not (Test-Path ".\daemon\target\debug\clamav-desktop-daemon.exe")) {
    throw "[install_windows_service] Error: Daemon binary not found. Make sure it is built and available in ./daemon/target/debug/."
}

if (-Not (Test-Path "C:\Program Files\ClamAV Desktop")) {
    New-Item -Path "C:\Program Files\ClamAV Desktop" -ItemType Directory
}

Copy-Item -Path ".\daemon\target\debug\clamav-desktop-daemon.exe" -Destination $daemonBinaryPath -Force

$service = Get-Service -Name $serviceName -ErrorAction SilentlyContinue
if ($null -ne $service) {
    throw "[install_windows_service] Error: Service '$serviceName' already exists. Installation aborted."
}

sc.exe create $serviceName binPath= $daemonBinaryPath start= auto DisplayName= "ClamAV Desktop Daemon"
Write-Host "[install_windows_service] Service '$serviceName' installed successfully."

Start-Service -Name $serviceName
if ($?) {
    Write-Host "[install_windows_service] Service '$serviceName' started successfully."
} else {
    Write-Host "[install_windows_service] Failed to start the service '$serviceName'."
}
