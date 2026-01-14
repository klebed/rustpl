#!/bin/bash
set -e

RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-musl
echo "Release build complete: target/x86_64-unknown-linux-musl/release/rustpl"


# RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target aarch64-unknown-linux-musl