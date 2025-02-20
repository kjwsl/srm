#!/usr/bin/env bash

cargo install --path .
sudo cp daemon/srm.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable srm
sudo systemctl start srm
