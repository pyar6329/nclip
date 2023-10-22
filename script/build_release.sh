#!/bin/bash

set -e

CURRENT_DIR=$(echo $(cd $(dirname $0) && pwd))

PROJECT_ROOT="${CURRENT_DIR}/.."

cd $PROJECT_ROOT

if ! $(rustup component list | grep "clippy" | grep "installed" > /dev/null); then
  rustup component add clippy
fi

if ! $(rustup component list | grep "rustfmt" | grep "installed" > /dev/null); then
  rustup component add rustfmt
fi

rustup target add x86_64-unknown-linux-musl
rustup target add aarch64-apple-darwin

cargo build --locked --release --target x86_64-unknown-linux-musl
tar -I "pzstd -19" -cvf nclip-Linux-x86_64.tar.zst target/x86_64-unknown-linux-musl/release/nclip

cargo build --locked --release --target aarch64-apple-darwin
tar -I "pzstd -19" -cvf nclip-Darwin-arm64.tar.zst target/aarch64-apple-darwin/release/nclip
