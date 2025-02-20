#!/usr/bin/env bash

cargo install --path .

if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    sudo cp daemon/com.larpi.srm.plist /Library/LaunchDaemons/
    sudo launchctl load -w /Library/LaunchDaemons/com.larpi.srm.plist
else
    # Linux
    sudo cp daemon/srm.service /etc/systemd/system/
    sudo systemctl daemon-reload
    sudo systemctl enable srm
    sudo systemctl start srm
fi
