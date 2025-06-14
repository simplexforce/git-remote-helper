#!/bin/bash

cargo build --example git-remote-demo
TARGET_BIN=target/debug/examples/git-remote-demo
chmod +x $TARGET_BIN
sudo cp -f $TARGET_BIN /usr/bin
