#!/usr/bin/env bash

apt-get purge -y libpq-dev:armhf libpq-dev:amd64
apt-get autoremove -y
apt-get install -y libpq-dev:$1
cargo build --target=armv7-unknown-linux-gnueabihf --release
