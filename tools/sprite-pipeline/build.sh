#!/bin/sh
# Compile the Swift tools next to their sources (binaries are git-ignored).
set -eu
cd "$(dirname "$0")"
for t in frames grid motion process tint; do swiftc -O "$t.swift" -o "$t" 2>/dev/null; done
echo "built: frames grid motion process tint"
