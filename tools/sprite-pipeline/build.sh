#!/bin/sh
# Compile the Swift tools next to their sources (binaries are git-ignored).
set -eu
cd "$(dirname "$0")"
for t in frames grid motion process tint; do
  swiftc -O "$t.swift" -o "$t" || { echo "build.sh: failed to compile $t.swift" >&2; rm -f "$t"; exit 1; }
done
echo "built: frames grid motion process tint"
