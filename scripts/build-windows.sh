#!/usr/bin/env bash
#
# Cross-compile the Rust binary for Windows (x86_64) from macOS or Linux.
#
# Prerequisites:
#   macOS:  brew install mingw-w64
#   Linux:  apt install gcc-mingw-w64-x86-64
#
#   rustup target add x86_64-pc-windows-gnu

set -euo pipefail

TARGET="x86_64-pc-windows-gnu"

# Ensure the target is installed
rustup target list --installed | grep -q "$TARGET" || {
    echo "Installing Rust target: $TARGET ..."
    rustup target add "$TARGET"
}

echo "Building ms-sql-query.exe for $TARGET ..."
cargo build --target "$TARGET" --release

EXE="target.tmp/$TARGET/release/ms-sql-query.exe"
if [ -f "$EXE" ]; then
    echo ""
    echo "✅ Build successful!"
    file "$EXE"
    ls -lh "$EXE"
    echo ""
    echo "To run in a Windows container:"
    echo "  docker build -f Dockerfile.windows -t ms-sql-query ."
    echo "  docker run --rm ms-sql-query"
else
    echo "❌ Build failed — binary not found at $EXE"
    exit 1
fi
