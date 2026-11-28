---
name: motion-style-extract
description: Turn a reference video into a reusable style pack in the library. Measures the clip with ffmpeg (or reads screenshots when ffmpeg is unavailable), reads the look, type, motion and beat off its contact sheet, writes the pack into style/<name>/, and gates it with lint and probe before any episode uses it. Use for "make a style from this video", "extract this video's style", "sample this look so I can reuse it", or handing a reference to another tool as a style prompt.
---


A style pack is the reusable object: a folder under `style/` the pipeline reads by name (`$MOTION_STYLE`). This skill builds one from a video the user points at, so the look survives past the one reference and can be applied to an unrelated topic.

The measurement decides the facts; the reading of the contact sheet decides the style. Keep them separate: the probe tells you the canvas, the cut rhythm, how much of the frame is saturated and which rows carry type. It does not tell you what the medium is.

## Preconditions

- `$MOTION_LIB` — the environment, else the `MOTION_LIB=` line in `${XDG_CONFIG_HOME:-~/.config}/motion-explainer/config.env`, else ask once. The pack goes in `$MOTION_LIB/style/<name>/`.
- Measurement path: `ffmpeg` and `ffprobe` on PATH and PIL on `$MOTION_PY` (`ffmpeg -version`, `"$MOTION_PY" -c 'import PIL'`). Both missing → the screenshot path below; say which path you took.
- Nothing here renders. A pack without an engine still scripts and storyboards; that is not a failure.

## 1. Get the frames

**Video path** — one command produces everything measured:

```sh
"$MOTION_PY" "$MOTION_LIB/tools/style_ref.py" ingest "$SOURCE" refs/<slug>/
```

`$SOURCE` is a local file or a URL (ffmpeg reads `https://` directly; when a site needs auth the user downloads it first). The reference repo or product page that the clip came from is worth keeping as the `--source` note later. Output: `refs/<slug>/probe.md`, `probe.json`, `sheet_0.jpg`…, and `frames/*.jpeg`. Read `probe.md` and the sheet.

**Screenshot path** — when ffmpeg is unavailable or the user brings stills: ask for 6–12 shots that cover the opening, a mid-beat, a transition and the end, and which canvas they came from. Read the images as the sheet; there are no measured numbers. Write the probe table by eye and mark each row *(observed)* so later readers know it was not measured. Set `canvas` and `extra.reference` from what the user says and leave `extra.probe` out.

## 2. Read the reference

Read the sheet once, in this order, and write one line per section of `docs/style-guide.md` before writing any file. A style pack that describes a medium but not its rhythm produces a reel that looks right and cuts wrong.

| what to settle | what in the sheet answers it | where it goes |
|---|---|---|
| The medium | what material or process the frames look made of, and what it refuses | Look |
| The non-negotiables | the 4–6 rules every frame obeys; test each on two different frames | Look |
| Palette roles | `probe.palette`, plus which colour only ever appears on the subject | Colour |
| Type | families, case, weight, where type is allowed, the text bands | Type |
| Motion | `probe.motion`, whether parts are rigid or redrawn, camera drift, ambient motion | Motion |
| Beat | `probe.cuts`, the fixed furniture (bar, corner mark, caption band) and its zone | Beat |
| Signature | the one move a viewer would name | Signature move |
| Traps | the readings that look right and break the style | Traps |

Rules that survive a topic change are the ones that belong in the pack. Anything that only describes the reference's subject goes in the pack's worked example, never in its DNA.

## 3. Write the pack

```sh
"$MOTION_PY" "$MOTION_LIB/tools/style_ref.py" scaffold <name> \
  --source "$SOURCE" --probe refs/<slug>/probe.json \
  --title "<Title>" --tagline "<one line on the medium>" --search-terms "<words a user would say>" \
  --fonts "<display family>,<text family>"
```

`scaffold` copies `$MOTION_LIB/mold/` into `$MOTION_LIB/style/<name>/` with the identity filled and the probe numbers folded in. Then replace every `TODO` — the ones left are the style itself, not paperwork. Keep the section headings: `lint` checks them, and `motion-explainer` routes by them.

Pack layout, and what each file is for:

| file | content |
|---|---|
| `SKILL.md` | description with the trigger words, the six DNA rules, measured numbers, the token table, the gate, traps |
| `style.json` | manifest: `name`, `title`, `tagline`, `description`, `canvas`, `status`, `docs`, `extra.tokens`, `extra.fonts`, `extra.reference`, `extra.probe` |
| `docs/style-guide.md` | the authority: Look, Colour, Type, Motion, Beat, Signature move, Traps |
| `docs/style-prompt.md` | the same DNA as standalone copy-paste blocks |
| `extra.tokens` / `extra.fonts` / `canvas` | the engine-free token layer every renderer reads; no engine folder is written |

Set `"status": "draft"` until the docs and tokens hold. Scripting and storyboarding work at `draft`; the pack is plan-ready before it is build-ready.

**Done when** every TODO is gone, `docs/style-guide.md` carries all seven sections with rules that hold on two different frames, and `docs/style-prompt.md` block A alone is enough for another agent to produce a still in this style.

## 4. Gate it

```sh
"$MOTION_PY" "$MOTION_LIB/tools/style_ref.py" lint "$MOTION_LIB/style/<name>"
```

`lint` failing on `extra.tokens` means the colours are still placeholders. Fill them in `style.json` from `probe.palette`, one role each, and mirror the same hexes in the guide's Colour table.

`probe` needs a rendered still, so it runs once something produces one — a builder the user already has, or a still drawn elsewhere:

```sh
"$MOTION_PY" "$MOTION_LIB/tools/style_ref.py" probe "$MOTION_LIB/style/<name>" <one-still.png>
```

A token that `probe` cannot find means whatever drew that still is not using the pack's palette — fix the drawing, not the token. The pack itself stays free of any builder.

## 5. Hand back

- Say which path was taken (video or screenshots), what was measured, and what was read off the sheet.
- Name the pack path and what it can do today: script and prompt only until a renderer reads it.
- Name the placeholders: fonts not licensed or not shipped, tokens not yet checked by `probe`.
- Give the user the `style-prompt.md` block A when they want the look for another tool.
- A series can adopt it by putting the name in `SERIES.md`/`agents.md` style field; that is `motion-strategy`'s edit, not this skill's.

## Traps

- **Describing the reference's subject instead of its style.** "Explains planets" is content; "flat torn paper, stacked shade, one fact per beat" is style. Rewrite any rule that would die with a topic change.
- **Trusting the sheet for timing.** Cut rate and shot length are measured; the sheet samples one frame per shot and flattens rhythm. Use `probe.cuts`.
- **Copying the reference's palette wholesale.** Keep the roles, not every hex; a pack needs a small set with rules, or later episodes drift frame to frame.
- **Building before the docs hold.** A kit written over an unfinished guide is a one-off, not a style. Docs and tokens first, gate, then let a builder read the pack.
- **Writing build code into the pack to make it reusable.** Portability comes from tokens in `style.json` plus rules in the guide; build code belongs to whoever builds, not to the extraction.
