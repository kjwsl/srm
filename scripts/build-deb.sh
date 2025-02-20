#!/bin/bash
set -e

# Install build dependencies
sudo apt-get update
sudo apt-get install -y build-essential debhelper cargo

# Build the package
dpkg-buildpackage -b -us -uc

# The .deb file will be created in the parent directory
echo "Package created successfully" 