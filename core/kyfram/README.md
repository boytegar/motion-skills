# Kyfram engine

One `.lgv` script per scene, written in the Logos DSL, rendered by the `kyfram` binary
(declare states at times, Go tweens every frame, ffmpeg encodes). This is the third
engine of the library, next to `remotion` (React) and `fframes` (Rust crate).

- Tool: https://kyfram-site-68056d.gitlab.io/ · `curl -fsSL https://kyfram-site-68056d.gitlab.io/install.sh | sh`
- Needs ffmpeg on PATH for mp4; `kyfram view script.lgv` previews with scrub and hot reload.
- The DSL reference lives in the kyfram skill: `skills/kyfram/SKILL.md` in this repo.

## Layout

```
core/kyfram/lgv/core.lgv            engine helpers, style-free (tags, plates, camera)
style/<style>/kyfram/PRELUDE.lgv    tokens + furniture for that style
core/tools/kyfram_header.py         the generated header builder
core/tools/render_kyfram.py         the render loop (cache per scene, concat, voice mux)
core/tools/kyfram_stubs.py          scene stub generator (also via core/tools/stubs.py)
core/tools/kyfram_qa.py             QA stills from cached segments (also via core/tools/qa.sh)
```

An episode project with `"engine": "kyfram"` looks like:

```
episodes/NN-slug/
  video/
    episode.json        {"engine": "kyfram", "style": ..., "width": 1080, "height": 1920, ...}
    src/plan.json       per-scene duration + word cues (core/tools/build_vo.py writes it)
    scenes/S01Hook.lgv  what a scene agent writes: shapes only
    media/fonts/*.ttf   copied once at scaffold time
```

## How a scene compiles

`render_kyfram.py` builds one script per scene, in this order:

1. `core/kyfram/lgv/core.lgv` — engine helpers. Every scene gets these.
2. `style/<style>/kyfram/PRELUDE.lgv` — tokens and furniture for the style.
3. A generated header — `SCENE_ID`, `BANNER` (this beat's taped headline), `DUR`,
   `FPSR`, `CAPS` (word cues `[{w, f, e}]`) and the `scene(DUR, FPSR)` call.
4. `scenes/<Scene>.lgv` — the shapes.

The header is generated from `episode.json` + `src/plan.json`, so a scene never
hard-codes its duration or caption cues, and retiming is a plan edit, not a rewrite.
The rendered frame count is checked against the plan; a mismatch fails the render
instead of shipping silent drift.

## Writing a scene

A scene file draws. It never calls `scene()`, never hard-codes `1080`, and never
re-declares a helper. Everything else in the DSL is yours: loops, `fn`, groups,
camera, arbitrary keyframe tables.

```lua
// scenes/S01Hook.lgv
drawRoom(C_BOARD, C_FLOOR)
drawHero(540, 1240, 120, C_HERO)
drawBanner(BANNER)
drawCaptions(CAPS)
```

Style furniture (papercut): `drawRoom(wall, floor)`, `drawHero(cx, cy, r, color)`,
`drawBanner(txt)`, `drawCaptions(CAPS)`. Engine helpers: `textW`, `pad`, `clamp`,
`popText`, `popPlate`, `drawTag`, `drawCaptionRow`, `pushIn`.

## Kyfram gotchas that the render enforces or the helpers avoid

- **Colour does not hold.** Omit `color` on a later keyframe and the shape draws
  white. Every keyframe table the helpers emit repeats `color` on both keys; do
  the same in your own tables.
- **`..` does not concat.** String concat is interpolation: `"tag_${name}"`.
- **`--format png` renders t=0 only.** QA stills come out of the cached mp4
  segments (`kyfram_qa.py`), not from `--format png`.
- **A scene's duration is the plan's.** Removing the `scene()` call is enforced by
  a render failure if you sneak one in.
- **Arity errors panic** rather than error cleanly: a wrong argument count to a
  helper fails the build with a Go panic. Count the args.

## Audio: voice yes, SFX and music not yet

`render_kyfram.py --voice` muxes exactly one track, `public/vo/voiceover.wav`, as AAC. That
covers narration only. The Remotion and fframes paths also mix a silent SFX layer
(`public/audio/fx.wav`) and a music bed (`public/audio/music.wav`) built by
`core/tools/cues.py` + `core/audio/render.mjs`, but a kyfram render has nowhere to put
them, so an episode with sound design needs a post-mix:

```
# once out/<name>.mp4 exists with the voice muxed
ffmpeg -i out/<name>.mp4 -i public/audio/fx.wav -i public/audio/music.wav \
  -filter_complex "[1:a][2:a]amix=inputs=2:duration=longest[add];[0:a][add]amix=inputs=2:duration=first:dropout_transition=0" \
  -c:v copy -c:a aac -b:a 320k out/<name>-mixed.mp4
```

That is a manual step today. The clean fix is a `--sfx` flag on `render_kyfram.py` doing the
same amix inside the existing voice mux; do it there, not in a wrapper script, so the cache
key and the QA path stay one code path.

## Pipeline wiring

`motion-build` and the other pipeline skills treat `kyfram` like any engine: the
brief names it, `new_episode.sh --engine kyfram` scaffolds the project,
`stubs.py` writes the scene stubs, `build_vo.py` writes `src/plan.json`, and
`core/tools/render.sh` renders (it reads `engine` from `episode.json`).
`kyfram view out/cache/kyfram/build/<Scene>.lgv` previews a scene with the exact
script the renderer compiled.
