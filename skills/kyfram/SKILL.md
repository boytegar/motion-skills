---
name: kyfram
description: Write and render kyfram Logos scripts (.lgv): declare states at times, Go tweens to mp4/png via ffmpeg. Use when rendering a kyfram episode scene, debugging a .lgv that renders wrong or blank, or when asked how a kyfram declaration, kind, ease or flag works.
---

Kyfram is a declarative renderer: you write a `.lgv` script in Logos that states what
each shape looks like at specific times, and Go interpolates every frame in between.
Mental model is CSS `@keyframes` for video: `Timeline -> Interpolate(t) -> canvas -> ffmpeg`.
Retiming is editing a number, not re-directing callbacks.

This repo drives kyfram as a third engine, with the timing generated. Read
`$MOTION_LIB/core/kyfram/README.md` first for how an episode project is wired; this
skill is the DSL itself. The full upstream reference (every declaration, kind, flag,
interpolation rule, and a Logos primer) is `references/upstream-SKILL.md`; read it when
a script needs something the summary below does not cover.

## The five declarations here

```
scene(duration, fps)                    opens the next scene; chained calls = chained scenes
shape(name, kind, keyframes [, group])  a drawable, keyframes = {"time": table{props}}
text(name, keyframes [, group])         sugar for shape(name, "text", ...)
group(name, keyframes)                  one flat level of children, local coordinates
camera(keyframes)                       one global pan/zoom, identity when absent
```

Optional but load-bearing: `audio(path, offset, loop)`, `tts(text, path [, opts])` (returns
`.duration`, `.audio`, `.words`), `transparent(true)` for png overlays, and `scale`/`rotate`/
`alpha`/`layer`/`blend` on every shape.

## Times, easing, interpolation

- Times are quoted strings sorted numerically: `"0"`, `"1.5"`. Declaration order is irrelevant.
- `ease` lives on the **earlier** key of a segment: `linear` (default), `easeIn`, `easeOut`,
  `easeInOut`, `cubicIn`, `cubicOut`, `cubicInOut`, `sine`.
- Numbers lerp with easing; hex and named colours lerp per channel; numeric arrays morph
  when lengths match; strings (`content`, `font`, `path`) snap to the earlier value.
- Past the last key, values hold. Before the first, the first value holds. Nothing extrapolates.

## Kinds

| kind | anchor | props |
|---|---|---|
| `circle` | centre | `x y r color` |
| `rect` | top-left | `x y w h color` |
| `roundRect` | top-left | `x y w h radius color` |
| `ellipse` | centre | `x y rx ry color` |
| `line` | midpoint | `x1 y1 x2 y2 width color` |
| `arc` | centre | `x y r start end width color` |
| `wave` | — | `x y width amplitude frequency phase thickness color` |
| `path` | bbox centre | `points width cap join closed filled color` |
| `polygon` | bbox centre | `points filled width color` |
| `star` | centre | `x y r points inner filled width color` |
| `image` | centre | `path x y` (ignores alpha, color, gradient) |
| `text` | centre | `content x y size font points color` |

Rotation's pivot on non-centre-anchored kinds (`rect`, `roundRect`) is unspecified.
When precision matters, draw a `polygon` with literal vertices instead.

## Rules that decide whether the frame is right

1. **Repeat `color` on every keyframe of a shape.** Measured on v0.4.1: a keyframe that
   omits `color` draws the shape **white**, on every kind. `x`, `y`, `r` hold across keys;
   `color` does not. This is the single most common silent defect.
2. **String concat is interpolation.** `"a" .. b` is a parse error; write `"a_${b}"`.
   `concat()` does not exist. `str()` and `toStr()` both work.
3. **`scene()` once, from the header.** In a repo episode the renderer emits it; a scene
   file that calls `scene()` is rejected on purpose. Standalone scripts call it themselves.
4. **Count helper args.** A wrong arity panics the interpreter with
   `index out of range`, not a clean error.
5. **`--format png` renders t=0 only.** For a still at time t, render the mp4 and extract
   the frame with ffmpeg, which is what `core/tools/kyfram_qa.py` does.
6. **Groups are one flat level, and children are local.** A child's `x`/`y` is relative to
   the group's centre. Nested groups are not supported.
7. **`alpha: 0` on the first key plus `alpha: 1` later is the pop.** Combined with rule 1 it
   is safe; without rule 1 the shape flashes white.
8. **`for i, v in arr`** for arrays, `for i in range(0, n)` for counts (`while` is dead).
   Arrays are `[...]`; `list{...}` is invalid. Hex colours in tables are quoted strings.
9. **`sin`/`cos` take radians, and computing the angle into a variable first avoids an
   interpreter crash.** `let a = 0.3 * i` then `sin(a)`.
10. **Flags come before the script path.** `kyfram render --out x.mp4 script.lgv`.

## Verify before calling a scene done

Three checks, in order, and say which ran:

1. `kyfram view out/cache/kyfram/build/<Scene>.lgv` — the exact compiled script, scrubbable.
2. `bash $MOTION_LIB/core/tools/render.sh` then `bash $MOTION_LIB/core/tools/qa.sh NAME 0-N 12 0.3 6`
   — a labelled contact sheet across the beat's frames.
3. Read the sheet against `$MOTION_LIB/style/$MOTION_STYLE/docs/style-guide.md` section 2
   and the scene block in `STORYBOARD.md`.

A placement bug is invisible: an off-canvas coordinate renders nothing and raises nothing.
Check coordinates against the canvas (default 1080x1920) and against a group's local bounds
before rendering, not after.

## Install

```
curl -fsSL https://kyfram-site-68056d.gitlab.io/install.sh | sh
kyfram --version
```

ffmpeg is required for mp4. `edge-tts` (python3) enables neural voices for `tts()`;
without it kyfram falls back to espeak-ng, then beeps.
