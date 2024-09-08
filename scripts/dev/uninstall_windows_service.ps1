# Check if the script is running as administrator
if (-not ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    Write-Host "Script is not running as administrator. Relaunching with elevated privileges..."
    Start-Process pwsh "-ExecutionPolicy Bypass -File $PSCommandPath" -Verb RunAs
    exit
}

$serviceName = "clamav-desktop-daemon"
$daemonBinaryPath = "C:\Program Files\ClamAV Desktop\clamav-desktop-daemon.exe"
$daemonFolderPath = "C:\Program Files\ClamAV Desktop"

$service = Get-Service -Name $serviceName -ErrorAction SilentlyContinue
if ($null -ne $service) {
    if ($service.Status -eq 'Running') {
        Stop-Service -Name $serviceName
        Write-Host "[uninstall_windows_service] Service '$serviceName' stopped successfully."
    }

    sc.exe delete $serviceName
    Write-Host "[uninstall_windows_service] Service '$serviceName' uninstalled successfully."
} else {
    Write-Host "[uninstall_windows_service] Service '$serviceName' does not exist."
}

if (Test-Path $daemonFolderPath) {
    Remove-Item -Path $daemonFolderPath -Recurse -Force
    Write-Host "[uninstall_windows_service] Daemon binary and folder removed successfully."
} else {
    Write-Host "[uninstall_windows_service] Daemon folder '$daemonFolderPath' does not exist."
}
