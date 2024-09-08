# Check if the script is running as administrator
if (-not ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    Write-Host "Script is not running as administrator. Relaunching with elevated privileges..."
    # Relaunch script with elevated privileges using 'RunAs'
    Start-Process pwsh "-ExecutionPolicy Bypass -File $PSCommandPath" -Verb RunAs
    exit
}

$daemonBinaryPath = "C:\Program Files\ClamAV Desktop\clamav-desktop-daemon.exe"
$serviceName = "clamav-desktop-daemon"

# Ensure binary exists before proceeding
if (-Not (Test-Path ".\daemon\target\debug\clamav-desktop-daemon.exe")) {
    throw "[install_windows_service] Error: Daemon binary not found. Make sure it is built and available in ./daemon/target/debug/."
}

# Create the directory for the daemon binary if it doesn't exist
if (-Not (Test-Path "C:\Program Files\ClamAV Desktop")) {
    New-Item -Path "C:\Program Files\ClamAV Desktop" -ItemType Directory
}

# Copy the daemon binary to Program Files
Copy-Item -Path ".\daemon\target\debug\clamav-desktop-daemon.exe" -Destination $daemonBinaryPath -Force

# Check if the service already exists
$service = Get-Service -Name $serviceName -ErrorAction SilentlyContinue
if ($null -ne $service) {
    throw "[install_windows_service] Error: Service '$serviceName' already exists. Installation aborted."
}

# Install the service
sc.exe create $serviceName binPath= $daemonBinaryPath start= auto
Write-Host "[install_windows_service] Service '$serviceName' installed successfully."

# Start the service
Start-Service -Name $serviceName
if ($?) {
    Write-Host "[install_windows_service] Service '$serviceName' started successfully."
} else {
    Write-Host "[install_windows_service] Failed to start the service '$serviceName'."
}
