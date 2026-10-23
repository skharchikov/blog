#!/bin/sh
# One command per scene: Gemini video (pixel art on flat magenta) -> transparent sprite sheet.
#   sprite-pipeline.sh <video.mp4> <scene-name> <out-dir> [fps=8] [cols=10]
# Steps: extract frames (frames), detect the art-pixel grid (grid), clean + pack (process),
# then verify no magenta-tinted pixels are left (tint).
set -eu
TOOLS=$(cd "$(dirname "$0")" && pwd)
VIDEO=$1; NAME=$2; OUT=$3; FPS=${4:-8}; COLS=${5:-10}
WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT

"$TOOLS/frames" "$VIDEO" "$WORK/frames" "$FPS"

# best period/phase per axis = first entry of each line printed by `grid`
GRID=$("$TOOLS/grid" "$WORK/frames/f000.png")
PX=$(echo "$GRID" | sed -n 's/^x: \["score [0-9.]* period \([0-9.]*\) phase \([0-9.]*\)".*/\1 \2/p')
PY=$(echo "$GRID" | sed -n 's/^y: \["score [0-9.]* period \([0-9.]*\) phase \([0-9.]*\)".*/\1 \2/p')
PERIOD_X=${PX% *}; PHASE_X=${PX#* }; PERIOD_Y=${PY% *}; PHASE_Y=${PY#* }
# one square grid: average the two axis estimates (override with PERIOD, PHASE_X_OVERRIDE, PHASE_Y_OVERRIDE env vars)
PERIOD=${PERIOD:-$(echo "$PERIOD_X $PERIOD_Y" | awk '{printf "%.3f", ($1 + $2) / 2}')}
PHASE_X=${PHASE_X_OVERRIDE:-$PHASE_X}; PHASE_Y=${PHASE_Y_OVERRIDE:-$PHASE_Y}
echo "grid: period $PERIOD (x $PERIOD_X, y $PERIOD_Y), phase $PHASE_X,$PHASE_Y"

"$TOOLS/process" "$WORK/frames" "$OUT" "$PERIOD" "$PHASE_X" "$PHASE_Y" "$FPS" "$COLS" "$NAME"
"$TOOLS/tint" "$OUT/$NAME-still.png"
rm -f "$OUT/$NAME.apng.png"
ls -la "$OUT"
