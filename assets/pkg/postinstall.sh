#!/bin/bash

cp clamav-desktop-daemon /usr/local/bin/
chmod +x /usr/local/bin/clamav-desktop-daemon

launchctl load /Library/LaunchDaemons/com.clamav-desktop.service.plist

exit 0
