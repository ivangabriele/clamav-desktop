#!/bin/bash

cp clamav-desktop-daemon /usr/local/bin/
chmod +x /usr/local/bin/clamav-desktop-daemon

systemctl enable clamav-desktop-daemon
systemctl start clamav-desktop-daemon

exit 0
