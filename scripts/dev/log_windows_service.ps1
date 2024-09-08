# Check if the script is running as administrator
if (-not ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    Write-Host "Script is not running as administrator. Relaunching with elevated privileges..."
    Start-Process pwsh "-ExecutionPolicy Bypass -File $PSCommandPath" -Verb RunAs
    exit
}

# Service name
$serviceName = "clamav-desktop-daemon"

# Display the status of the service
Write-Host "Fetching status of service '$serviceName'..."
$service = Get-Service -Name $serviceName -ErrorAction SilentlyContinue

if ($null -ne $service) {
    $serviceStatus = Get-Service -Name $serviceName
    Write-Host "Service '$serviceName' is currently $($serviceStatus.Status)."
} else {
    Write-Host "Service '$serviceName' does not exist."
    exit
}

# Fetch logs from Windows Event Viewer and continuously monitor new ones
Write-Host "Tailing logs for service '$serviceName'... (Press Ctrl+C to stop)"
$lastTime = (Get-Date).AddMinutes(-5)  # Start by fetching logs from the last 5 minutes

while ($true) {
    # Fetch the most recent logs
    $eventLogs = Get-WinEvent -LogName System -FilterXPath "*[System[Provider[@Name='$serviceName'] and TimeCreated[timediff(@SystemTime) <= 300000]]]" -ErrorAction SilentlyContinue

    # Display new logs if any
    if ($eventLogs) {
        $eventLogs | Format-Table TimeCreated, Message -AutoSize
    } else {
        Write-Host "No new logs..."
    }

    # Update the time to only fetch new logs in the next loop
    $lastTime = (Get-Date)
    Start-Sleep -Seconds 5  # Poll every 5 seconds
}
