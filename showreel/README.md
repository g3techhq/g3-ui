# The G3 Stack device reel

68 seconds, 1080p, 60 fps. One floating rounded device morphs through portrait
phone, landscape phone, tablet, and a larger tablet-style desktop display.
White background, real app screens, no captions, keyboard, laptop, or stand.

Current output: `build/the-g3-stack-v5.mp4`. Older exports remain in `build/`
for comparison. Generated assets are ignored by Git.

## Regenerate

From `g3-ui`:

```powershell
# If the existing playground browser dependency is missing:
npm ci --prefix playground

# Start/reuse local services, capture, generate geometry, render and encode:
just showreel --capture

# Reuse recordings; rebuild the model and video:
just showreel

# Reuse both recordings and geometry; render and encode:
just showreel --encode-only

# Refresh recordings only:
just showreel --capture --capture-only

# Inspect a frame at 12 seconds:
node showreel/render-stage.cjs --still 12
```

Requires Blender 4.4+, FFmpeg/FFprobe, Python 3.10+, Node, Playwright from the
existing playground tooling, and Chrome with WebGL2 and WebCodecs. Python
uses the standard library. Set `BLENDER` if its executable is not on PATH or
in the standard Windows installation directory.

The runner reuses the configured playground, Media Mancer (8081), and charts
(8095) servers, or starts missing `dx serve` processes itself. If the playground
port belongs to another app, it selects the next free port and saves that URL
in `reel.json`. It starts the local
Media Mancer database with Docker Compose if needed, then stops only the
services it started after capture. Data is not deleted or reset. Media
Mancer needs its usual `.env` and provider configuration; capture uses a
fresh guest account.

## Sequence

| Time | Size | Live source |
| --- | --- | --- |
| 0–18s | Portrait phone, 390×844 CSS pixels | Routes, then Library segment and body slides |
| 18–32s | Landscape phone, 844×390 | Navigation rail, components and themes |
| 32–46s | Tablet, 1024×768 | Media Mancer home, search, and detail route |
| 46–68s | Large tablet-style desktop, 1600×900 | Charts, toolbar segments, circular theme reveals |

The portrait showcase explicitly selects the built-in Light theme. Its page
is `#e9ebef`, with cards at `#f7f8fa`; it does not use a custom video-only theme.
The queue uses a standard `Stack` to leave 16px above Done.

Size changes take 1.4 seconds, centered at 18, 32 and 46 seconds. The same
mesh changes width and height with constant bezel thickness. Pitch, yaw,
roll and vertical bob run continuously across all four sizes. There are no
cuts between hardware models. The desktop is the same slab with larger
dimensions, without a stand.

During a morph, each source recording uses a uniform cover crop and a short
crossfade. Text does not undergo nonuniform stretching to fit intermediate
screen ratios. At each settled size, the whole recording matches its display.

## Files and pipeline

- `reel.json`: viewports, physical screen widths, framing, timings and music.
- `capture.cjs`: Chrome compositor recording at its native changing-frame
  cadence, normalized to 60 fps using microsecond timestamps. Larger physical
  capture surfaces plus document zoom preserve each app's intended CSS size.
  Outer playground chrome is hidden from View Transition snapshots.
- `scene.py`: creates one rounded Blender mesh with matching morph targets
  and a floating animation. Writes `build/floating-device-v5.blend` for model
  review and `build/device-v5.json` for the renderer. No external models or
  fonts are required.
- `stage.js`: renders that mesh on the GPU. Local screen coordinates map
  directly to the flat display; morphing cannot invalidate UV coordinates.
  The renderer advances through every output frame explicitly, seeks to the
  matching source frames, and encodes the canvas with WebCodecs at 60 fps.
  It does not record a real-time preview and hope frames are not dropped.
- `render-stage.cjs`: serves local assets, prepares intra-frame browser clips,
  converts full-range captures to explicitly tagged limited-range BT.709 for
  faithful WebGL video upload, runs the GPU renderer, writes H.264, and muxes
  the original stereo score.
- `audio.py`: deterministic synthesized music and accents; no external song
  or audio samples.
- `render.py`: coordinates capture, model generation, music, and video export.

Recordings are in `build/screens-v5`, raw capture frames in `build/capture-v5`,
browser-ready clips in `build/browser-v5`, and render/capture reports in
`build/*-v5.json`. Keep source recordings with a release if you need to
reproduce its exact network-backed app content later.

## Aspect and color checks

Pixel dimensions alone do not prove a video will display correctly. The
previous export inherited the portrait source's pixel-aspect ratio when it
was resized to a widescreen composition. Players therefore squeezed the
entire 1920×1080 canvas into a portrait display shape.

The current export explicitly writes **1:1 sample aspect ratio** and **16:9
display aspect ratio** into the H.264 stream and container. A browser then
loads the actual exported MP4 and checks that a 960px-wide video element is
540px tall, with 1920×1080 intrinsic dimensions. This catches the failure
that a resolution-only check missed.

```powershell
python -m py_compile showreel/render.py showreel/scene.py showreel/audio.py showreel/verify.py
node --check showreel/stage.js
node --check showreel/render-stage.cjs
node --check showreel/capture.cjs
node showreel/ui-check.cjs
node showreel/phone-check.cjs
python showreel/verify.py
```

Verification covers matching morph topology, source aspect ratios and cadence,
complete encoded frame counts, duration, square pixels, display ratio, actual
browser playback dimensions, source-versus-delivery surface colors, audio,
and a full decode. Review the motion and
composition visually as well.
