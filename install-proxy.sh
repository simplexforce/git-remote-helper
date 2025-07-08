#!/bin/bash

cargo build -p git-remote-proxy
TARGET_BIN=target/debug/git-remote-proxy
chmod +x $TARGET_BIN

echo "Installing git-remote-proxy to /usr/bin"
sudo cp -f $TARGET_BIN /usr/bin
