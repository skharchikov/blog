# Sprite pipeline (macOS)

Turns a Gemini-generated pixel-art video on a flat magenta background into a
transparent sprite sheet for `public/sprites/`.

```sh
tools/sprite-pipeline/build.sh                      # compile the Swift tools once
tools/sprite-pipeline/sprite-pipeline.sh <video.mp4> <scene> <out-dir> [fps=8] [cols=10]
```

Steps: extract frames (`frames`), detect the art-pixel grid (`grid`), clean and
pack (`process`), then check no magenta-tinted pixels remain (`tint`). `motion`
reports what moves between frames and whether the loop closes.

`process` only cleans up: it samples each grid cell, keys out the magenta
background (and floor shadow), recolours magenta-tinted edge pixels from their
outline neighbours, drops stray specks away from the subject and keeps static
areas stable between frames. It never repaints the art. If a video looks wrong,
regenerate it instead of editing frames.

If the detected grid is slightly off (noisy output), pass a hand-checked one:

```sh
PERIOD=5.7 PHASE_X_OVERRIDE=3.9 PHASE_Y_OVERRIDE=3.6 tools/sprite-pipeline/sprite-pipeline.sh video.mp4 home out/
```

After running, copy `<scene>.png` and `<scene>-still.png` to `public/sprites/`
and add the scene to `public/sprites/manifest.json`.
