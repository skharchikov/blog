#!/bin/sh
# Build the site and screenshot routes × themes × widths.
#   tools/screenshots/shoot.sh <out-dir> [route ...]      (default: every route)
# Writes <out-dir>/<route>__<theme>-<width>.png. Needs the rustup toolchain
# on PATH (trunk) and Google Chrome.
set -eu
ROOT=$(cd "$(dirname "$0")/../.." && pwd)
OUT=$1; shift
ROUTES=${*:-"/ /posts /posts/blind75-two-sum /projects /projects/blackjack /books /contacts /nope"}
THEMES="skh tokyo-night gruvbox paper"
WIDTHS="1280 768 390"
PORT=8137
CHROME="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"

mkdir -p "$OUT"
(cd "$ROOT" && trunk build >"$OUT/build.log" 2>&1) || { tail -20 "$OUT/build.log"; exit 1; }
python3 "$ROOT/tools/screenshots/serve.py" "$ROOT/dist" "$PORT" &
SERVER=$!
trap 'kill $SERVER 2>/dev/null' EXIT
sleep 1

for route in $ROUTES; do
  name=$(echo "$route" | sed 's|^/$|home|; s|^/||; s|/|_|g')
  for theme in $THEMES; do
    for w in $WIDTHS; do
      "$CHROME" --headless=new --disable-gpu --hide-scrollbars --user-data-dir="$OUT/.chrome" \
        --window-size=$((w + 40)),940 --virtual-time-budget=6000 \
        --screenshot="$OUT/${name}__${theme}-${w}.png" \
        "http://127.0.0.1:$PORT/__harness.html?theme=$theme&w=$w&route=$route" >/dev/null 2>&1
    done
  done
done
rm -rf "$OUT/.chrome"
ls "$OUT"/*.png | wc -l | xargs echo "screenshots:"
