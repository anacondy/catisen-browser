#!/bin/bash

# Install dependencies
sudo apt-get update
sudo apt-get install -y cmake pkg-config libgtk-4-dev libssl-dev

# Build with Servo
cd servo
./mach build --release

# Copy assets
cp -r ../assets .

# Run
./mach run