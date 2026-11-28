---
name: math
description: Render a reel in the Math style: a manim-style mathematical explainer on a near-black stage, with serif formulas written on, axes and number planes swept by ShowCreation, equations that Transform into one another, a tracking dot with a live decimal readout, and a small pi hero watching. Use when the user says math explainer, manim, 3blue1brown style, formula animation, proof animation, graph animation, or asks to explain a rate, a function or a geometric fact.
---

Manim-style mathematical motion graphics: chalkboard dark stage, written equations, smooth easing on 1s.

This is one style of the motion-explainer library. It ships the docs and tokens: `docs/style-guide.md` for the visual authority, `docs/style-prompt.md` for portable prompts, and a `style.json` manifest with the canvas, palette, fonts and audio roots. The pipeline skills (`motion-explainer`, `motion-hook`, `motion-script`, `motion-storyboard`, `motion-build`, `motion-qa`) all take the style name as `$MOTION_STYLE`, so nothing else in the workflow changes when you pick this one.

## When to use it

Reach for this style when the brief matches its DNA below. Read `docs/style-guide.md` first: it is the authority, this file is the index.

## Style DNA

1. Coloured math on a near-black stage, clean vector strokes, no texture and no grain.
2. Serif type is Written on; axes and graphs are swept by ShowCreation; equations Transform into each other.
3. Motion is on ones, eased with manim's sigmoid `smooth`. Nothing boils.
4. Colour coding does the work: each variable owns a colour and keeps it.
5. The look promises rigour, so the math on screen has to be true.

Palette: stage `#121317`, formula `#f7f7fa`, yellow `#f5d76e`, blue `#5dade2`, green `#58d68d`, red `#ec7063`, pi `#f39c12`, grid `#222530`

## Kit components

Components the style calls for, named here so a plan can request them by name:

| component | what it is |
|---|---|
| `<MathGrid>` | the faint stage grid |
| `<MathAxes cx cy>` | a pair of axes for a plane or a graph |
| `<Formula x y text color size>` | a serif formula with a per-variable colour |
| `<MathBanner text>` | the stage banner |
| `<MathCaptions text>` | the caption strip with a green rule |

## Nothing boils in a vacuum

docs/style-guide.md lists the manim move names and the colour assignment. Read it before writing a formula.

## QA for this style

Check these on every contact sheet before calling a scene done; the rest of the checklist in `motion-qa` still applies.

- [ ] Every formula on screen is mathematically true.
- [ ] Each variable keeps one colour across the whole reel.
- [ ] Axes and graphs are swept on, not faded in.
- [ ] Motion is eased on ones; nothing boils, no grain.
- [ ] Nothing is smaller than phone-legible at 1080 wide.

## Files

- `docs/style-guide.md`
- `docs/style-prompt.md`
- `style.json`: the manifest (canvas, engine list, doc paths, audio roots)
- `sample.png`: one representative frame
