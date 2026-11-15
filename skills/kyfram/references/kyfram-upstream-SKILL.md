---
name: kyfram
description: Render keyframe-driven motion graphics — declare states at times, Go tweens to mp4/png via gg + ffmpeg. Covers every kyfram declaration, kind, flag, interpolation rule, the Logos essentials needed to write scripts, and the agent process for generating precise output without silent mistakes.
version: 0.4.1
---

# kyfram — video you declare, not direct

You write a `.lgv` script in Logos declaring **states at times**. Go owns the tween. Mental model is CSS `@keyframes` for video: `Timeline → Interpolate(t) → canvas → ffmpeg`. Retiming is editing a number, not re-directing callbacks.

> **Flags work anywhere.** `kyfram render --out out.mp4 script.lgv` and `kyfram render script.lgv --out out.mp4` both work (cobra/pflag). Every example uses flags-first by convention.

## Install

```sh
curl -fsSL https://kyfram-site-68056d.gitlab.io/install.sh | sh
kyfram --version
# pin a version: curl -fsSL https://kyfram-site-68056d.gitlab.io/install.sh | sh -s -- v0.4.1
```

Requires `ffmpeg` on PATH to encode mp4 (png works without it), `python3 + edge-tts` for neural TTS (falls back to `espeak-ng` → beeps). Default canvas 640×360, default font bundled (Archivo Black).

## Minimal script

```lua
scene(2, 30)

shape("ball", "circle", table{
    "0": table{x: 100, y: 180, r: 40, color: "#e64c40", ease: "easeOut"},
    "2": table{x: 540, y: 180, r: 40, color: "#e64c40"},
})
text("caption", table{
    "0": table{content: "hello kyfram", x: 320, y: 300, size: 28, color: "#ffffff"},
})
```

```sh
kyfram render --out out.mp4 intro.lgv
kyfram render --format png --out frame.png intro.lgv   # t=0 only, no ffmpeg
kyfram view intro.lgv                                   # http://localhost:7070 — scrub, loop, PNG, fullscreen, hot reload
```

## Declarations — the only calls that produce video

Loops, variables, `fn`, `if`, arithmetic are free — they just generate more declarations. File IO, network, shell, and process exit are blocked except through the kyfram builtins below.

### `scene(duration, fps[, transition[, fadeDur]])`
Required — repeat to **chain scenes**. Each call opens the next scene; declarations after it belong to that scene, and total length = sum of durations. Transitions: `"cut"` (default, instant), `"fade"` (cross-dissolve), `"black"`/`"white"` (dip to solid), `"flash"` (white pop), `"push-left|right|up|down"` (slide in), `"wipe-left|right|up|down"` (curtain), `"iris"` (grows from center), `"zoom"` (scales up from a dot). Aliases: `to-black`→`black`, `to-white`→`white`, `push`→`push-right`, `wipe`→`wipe-right`. Unknown name → build error. 4th arg `fadeDur` overrides transition length — default flash 0.2s, everything else 0.5s.

`Width`/`Height` 0 → 640×360 (override via `--width`/`--height`, CLI wins). No `scene()` at all → build error.

### `shape(name, kind, keyframes[, group])`
`name` for layering/groups, `kind` (12 below; unknown kind → build error), `keyframes` table of quoted times → prop tables. Optional 4th arg `group` must name an existing `group()`.

### `text(name, keyframes[, group])`
Sugar for `shape` with `kind="text"`. Props: `content` (snaps, never interpolates), `x y` center, `size` default 28, `font` TTF path, `points` flat `[x1,y1,…]` rides glyphs along a polyline, plus `alpha/rotate/scale/layer`.

### `group(name, keyframes)`
Holds `x y scale rotate alpha layer`. Group's own `x/y` default to **canvas center**. Members keep **local** `x/y` (default 0,0 for circle/star/ellipse/text; rect/roundRect default to -w/2, -h/2). World = group × child. `gAlpha < 1` multiplies child alpha. **One flat level, never nested.**

```lua
group("g", table{"0": table{x: 320, y: 180, scale: 1}, "3": table{scale: 1.4, rotate: 90}})
shape("c", "circle", table{"0": table{x: 0, y: 0, r: 30, color: "coral"}}, "g")
```

**Every child's `x`/`y` is relative to the group's own center, not the canvas.** This is the single most common way output silently goes missing — see the Agent process section.

### `camera(keyframes)`
Single global. `x y` world point lands on screen center, scaled by `zoom` (clamped to ≥1 if ≤0). Empty = identity.

```lua
camera(table{"0": table{x: 200, y: 180, zoom: 1}, "2": table{x: 320, y: 180, zoom: 1.6, ease: "sine"}})
```

### `audio(path[, offset[, loop]])`
Lays a file under the video. `offset` seconds shifts forward (must be ≥0). `loop` repeats to fill to last frame. Multiple `audio()` calls mix. Missing files fail at render time; no pre-check.

### `transparent(bool)`
`transparent(true)` clears to a transparent background for png overlays. Mp4 has no alpha, so only useful with `--format png`.

### `text_file(path)` → string
Reads a text file (capped 1 MB) for use with `--var`.

### `tts(text, outPath[, opts])` → `{audio, duration, words}`
Falls back automatically: neural voice → espeak → beeps, same return shape always. `opts` is a voice name string, or a table `{voice, rate, pitch, volume}`. `words` is `[{word, start, end}]` for caption timing.

```lua
let v = tts("hey my name is uthman", "/tmp/voice.mp3", table{voice: "en-US-AriaNeural", rate: "+5%"})
scene(v.duration + 0.5, 30)
audio(v.audio)
for i, w in v.words {
  text("w"..str(i), table{ str(w.start): table{content: w.word, x: 320, y: 180, size: 56, alpha: 0}, str(w.start+0.08): table{alpha: 1}})
}
```

### `tts_voices()` → `[string]`
Lists available neural voice names.

### `stamp(base, overrides)` — keyframe reuse
Returns a **fresh** table: `overrides` shallow-merged into *every* keyframe of `base`. One animation, N copies with a field changed. `base` is never mutated.

```lua
let hop = table{"0.40": table{x: 0, y: -70}, "0.58": table{x: 0, y: 0}}
shape("a", "circle", stamp(hop, table{x: -330, color: "mint"}), "rowA")
shape("b", "circle", stamp(hop, table{x: 0, color: "coral"}), "rowA")
```

### `sin(x)`, `cos(x)`, `pi()` — trig
Radians. **Crash guard (verified):** a deeply nested expression *inside* the call can crash the interpreter — compute the argument into a variable first:

```lua
let a = 6.28318 * ((t - t0) / period) + phase   // the angle, as a plain variable
let y = baseY + amp * sin(a)                     // then call sin/cos with it
```

## Keyframes — the rules

- Times are **quoted strings** `"0"`, `"1.5"`. Sorted numerically — declaration order irrelevant. Duplicate times hold the earlier value.
- `ease` lives on the **earlier** key of a segment.
- Unknown prop keys → build error — as do wrong-typed values, bad colors, bad `blend`/`cap`/`join`, and missing `font`/`image` files.

## Interpolation

For each prop: find bracketing keys around `t`, compute `f = (t - t0)/(t1 - t0)`, apply ease, then:

| type | behaviour |
|---|---|
| numbers | `lerp = v0 + (v1-v0)*f` with ease applied |
| hex/named colors | per-channel lerp — red→blue passes through purple |
| numeric arrays (`points`) | element-wise morph when **same length**, else snap to earlier |
| strings (`content`, `font`, `path`) | snap to earlier — text never renders halfway |
| missing prop | holds earlier value |
| past last key | holds last value — **never extrapolates** |
| before first key | holds first value |

### Eases (8)
`linear` (default, `f`), `easeIn` (`f²`), `easeOut` (`1-(1-f)²`), `easeInOut` (`2f²` / `1-2(1-f)²`), `cubicIn` (`f³`), `cubicOut` (`1-(1-f)³`), `cubicInOut` (`4f³` / `1-4(1-f)³`), `sine` (`0.5-0.5*cos(πf)`). Applied after clamping `f` to 0..1.

### Colors
`#rrggbb` or `#rgb`, case-insensitive, or names: `black white red green blue yellow cyan magenta orange purple pink teal navy gray/grey dark gold coral sky mint lavender slate silver`. Unknown strings are not colors — build error.

## Kinds — 12 kinds (11 shapes + text)

Common props on everything: `alpha` default 1, `rotate` 0° around anchor, `scale` 1 around anchor, `layer` 0 (higher on top), `blend` (`multiply`/`screen`/`add`), `gradFrom gradTo gradAngle` (filled kinds only — strokes/images/text stay flat). Unknown kinds, bad values, and unloadable `font`/`image` paths → build error. (Hand-built timelines bypassing `Build()` still skip silently — one bad entity never kills the frame.)

| kind | anchor | props (defaults) |
|---|---|---|
| `circle` | center x,y | `x y r 50 color` |
| `rect` | **top-left** x,y | `x y w 100 h 100 color` |
| `roundRect` | top-left | `x y w 100 h 100 radius 10 color` |
| `ellipse` | center | `x y rx→r→50 ry→rx color` |
| `line` | midpoint | `x1 y1 x2 y2 width 2 color` |
| `arc` | center | `x y r 50 start 0 end 360° width 4 color` — full sweep = ring |
| `wave` | — | `x 0 y 180 width canvas amplitude 30 frequency 1 phase 0° thickness 3 color` |
| `path` | bbox center | `points [x1,y1…] ≥4 even width 3 cap butt/round/square join round/bevel/miter closed/filled bool color` |
| `polygon` | bbox center | `points ≥6 even filled true width 2 color` |
| `star` | center | `x y r 50 points 5 inner 0.45 filled true width 2 color` — apex at 12 o'clock |
| `image` | center | `path required x y` — ignores alpha/color/gradient |
| `text` | center | `content x y size 28 font TTF color points [x1,y1…]` |

**Note on `rotate`:** for kinds with a non-center anchor (`rect`, `roundRect`), the exact pivot point isn't documented and shouldn't be assumed to be the visual center. For anything needing a precisely rotated shape (a diamond, an angled bar), use `polygon` with literal hand-computed vertices instead — no pivot to guess wrong.

## CLI

```
kyfram render [--out f] [--fps n] [--width n] [--height n] [--font f] [--crf n] [--preset s] [--format mp4|png] [--var k=v ...] <script.lgv>
kyfram view   [--port n] [--font f] <script.lgv>
kyfram help [render|view|new]    kyfram --version
```

| flag | default | effect |
|---|---|---|
| `--out` | `out.mp4` | output path |
| `--fps` | scene | override scene fps (>0) |
| `--width`/`--height` | 640×360 | override canvas size |
| `--font` | bundled | default typeface |
| `--crf` | 23 | x264 quality 0–51, lower=better |
| `--preset` | medium | ultrafast…veryslow |
| `--format` | mp4 | mp4 or png (png = t=0 only) |
| `--var k=v` | — | Logos var, repeatable |
| `--port` | 7070 | view only |

## Logos essentials — the language underneath

You don't need the whole language, just enough to declare and generate keyframe tables.

```lua
let x = 1                 // mutable
const N = 10              // immutable
let s = "hello ${name}"   // string interpolation
let t = table{name: "Alice", years: 2}  // bare keys → string keys
let a = [1, 2, 3]

if x == 1 { print("yes") } else { print("no") }
for i, v in a { print("${i}: ${v}") }      // in-array
for i in range(0, n) { print(i) }          // integer count, end exclusive — use this, not "while" (dead keyword)

let add = fn(a, b) { return a + b }
let inc = fn(x) -> x + 1          // arrow shorthand
let v = condition ? "yes" : "no"  // ternary
i++ ; i--                         // postfix
```

Arrays are `[...]`, never `list{...}`. Hex colors in keyframe tables must be quoted strings: `color: "#ffffff"`.

**Useful builtins for keyframe generation:**
- `toStr(n)` / `str(n)` — convert a number to a string for use as a table key
  (capital S in `toStr` — lowercase `tostr` fails with "identifier not found")
- `range(start, end, step?)` — integer loop bounds
- `push/pop/first/last/reverse/sort` — array helpers, if building keyframe tables programmatically
- `mathAbs, mathCeil, mathFloor, mathMax, mathMin, mathRandom(), mathRandomInt(min,max), mathRound, mathSqrt`

That covers essentially every pattern you'll need. Skip the rest unless a specific script calls for it.

## Programmatic patterns — the .lgv is a program

A `fn` can be a keyframe builder: args in, a `shape()`-ready table out. Loops + data arrays replace hand-typed, hand-copied tables.

```lua
// keyframe-builder fn: one definition, called N times with different args
let cap = fn(content, x, y, size, t0, t1) {
  let kf = table{}
  kf[toStr(t0)] = table{content: content, x: x, y: y, size: size, alpha: 0}
  kf[toStr(t1)] = table{content: content, x: x, y: y, size: size, alpha: 1}
  return kf
}

// data array + loop declares N entities from one pattern
let letters = ["K", "Y", "F", "R", "A", "M"]
for i, ch in letters {
  text("l" .. toStr(i), cap(ch, 120 + i * 80, 180, 64, 0, 2))
}

// computed keyframe table via a range loop
let kf = table{}
for i in range(0, 40) {
  let t = 0.05 * i
  let a = 0.3 * i                      // compute the angle before sin()
  kf[toStr(t)] = table{x: 320, y: 180 + 60 * sin(a)}
}
shape("sine", "circle", kf)
```

Prefer this over hand-copying near-identical `shape()` blocks — one `fn` or loop, called repeatedly, instead of N pasted variants with one field changed.

## Gotchas

- `while` is a dead keyword — use `for i in range(0, n)` or `for i, x in arr` or `for i < 10 { ... }`.
- `list{...}` is invalid — arrays are `[...]`.
- Hex colors in keyframe tables must be quoted strings.
- Compute `sin`/`cos` arguments into a variable first — see crash guard above.
- Flags before the script path, always.
- `transparent(true)` only matters for png output.
- Images ignore `alpha`/`color`/gradient props.
- Groups are one flat level — never nested, and always local-coordinate (see Agent process).
- Gradients only apply to filled kinds; strokes stay flat.
- Array morph (`points`) needs matching length on both sides, or it snaps.
- `rotate`'s pivot for non-center-anchored kinds (`rect`, `roundRect`) is unverified — use `polygon` instead when precision matters.
- Build fails fast on unknown kind/prop/group, wrong-typed values, bad colors/enums, and missing font/image files — read the error, don't re-render blindly.

## Agent process — how output lands precisely

Kyfram still fails silently on placement. An off-frame shape renders nothing with no error — no build rule can catch a coordinate that's valid but wrong. Declaration mistakes (unknown kind/prop, bad values, missing font/image) are build errors now, so the agent's checking burden is coordinates, bounds, and formula-derived values. These are the steps that actually produce exact, correct output rather than plausible-looking guesses.

### 1. Ask what the scene actually is — never assume content

If the request doesn't name what's on screen ("make an ad", "an intro"),
don't invent placeholder subject matter and proceed. Ask what it's for, what
copy exists, what feel is intended. A wrong guess costs a full regenerate; a
question costs one turn.

### 2. Derive every number, don't estimate it

Before writing a coordinate, ease value, or keyframe time, know exactly
where it comes from:
- **Group children:** first write out the group's own local bounds (e.g. a
  300×620 body centered at 0,0 spans local `x: -150..150, y: -310..310`).
  Every child coordinate gets checked against those bounds before it's
  accepted — not eyeballed, checked.
- **Ease/formula-driven curves:** sample the actual documented formula
  (`1-(1-f)³` for cubicOut, `y+amp*sin(...)` for wave) at each keyframe
  point, rather than approximating a curve by feel. State which formula and
  which sample points were used.
- **Timing:** a "too fast/slow" note becomes a specific numeric multiplier
  applied to every relevant time value, not a re-guess of each one from
  scratch. State the multiplier and the new total duration.

### 3. Route around undocumented behavior instead of guessing through it

If a behavior isn't specified (an anchor's rotation pivot, an edge case in
interpolation) and getting it wrong is invisible — silently skipped, silently
offset, not a build error — don't ship a guess. Pick the primitive that has
no ambiguity instead (e.g. `polygon` with literal vertices instead of
`rect` + `rotate`). If there's truly no ambiguity-free alternative, say the
assumption plainly and name the one check that would confirm it, before
treating the output as final — not as an afterthought once it's already
been read as settled.

### 4. Ground visual metaphors in the tool's actual mechanism

For a logo, icon, or visual identity: don't reach for the nearest
convention from an adjacent tool. Find the one thing that's true of this
tool specifically and not generically true of the category — then check:
does the representation still make sense with every label removed? If it's
just a generic symbol for "animation tool," it isn't done.

### 5. Verify before calling it final

Read back what was just written against the rules above — do the
coordinates fall inside their group's bounds, does the curve match its
formula, does the pivot assumption get flagged if unverified. Say plainly
which of these were checked and which weren't, rather than presenting
everything with equal confidence.

### 6. Workflow

1. Confirm the scene content (step 1) before writing anything, unless the
   request already fully specifies it.
2. Write the `.lgv` declaring states; never hand-tween. Run every group
   child and formula-derived value through steps 2–3 before finalizing.
3. `kyfram view script.lgv` to scrub and confirm visually; `kyfram render
   --out out.mp4 script.lgv` to ship.
