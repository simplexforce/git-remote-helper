#!/bin/bash

cargo build -p git-remote-demo
TARGET_BIN=target/debug/git-remote-demo
chmod +x $TARGET_BIN

echo "Installing git-remote-demo to /usr/bin"
sudo cp -f $TARGET_BIN /usr/bin
