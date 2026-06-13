#!/bin/bash
set -e
echo "=== Building Release Binary ==="
cargo build --release
echo "=== Copying Binary to dist/binaries/ ==="
mkdir -p dist/binaries
cp target/release/flame dist/binaries/
echo "=== Build Successful ==="
