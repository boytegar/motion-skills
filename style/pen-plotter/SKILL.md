---
name: pen-plotter
description: Render in the Pen Plotter style: Every mark is a stroke. Docs-only import from Lemo-Opuscar (CC BY-NC 4.0); it reads and scripts here but does not render until an engine is written for it. Use when the user asks for maps and terrain, measurements and records, generative and data drawings.
---

# Pen Plotter
At night under a desk lamp, a pen plotter draws Mount Qomolangma from real elevation data with four pens, and writes its height last.

Docs-only import from [Lemo-Opuscar by Lemomo (LemoLab)](https://github.com/lemomo-ai/lemo-opuscar) (`styles/pen-plotter`), CC BY-NC 4.0. Style content is unchanged; only the file layout was adapted.

**Not renderable yet.** This pack has no `remotion/`, `fframes/` or `kyfram/` engine, so `new_episode.sh` cannot scaffold it and the render skills cannot run it. `motion-strategy`, `motion-hook`, `motion-script` and `motion-storyboard` work normally: read `docs/style-guide.md` for the DNA and `docs/style-prompt.md` for portable prompts.

## Files

| file | what it gives |
|---|---|
| `docs/style-guide.md` | the style prompt: essence, materials, colour, type, motion, camera, sound, native moves, pitfalls |
| `docs/style-prompt.md` | the same DNA as standalone copy-paste blocks for another agent or generator |
| `style.json` | manifest: title, tagline, category, source, `engines: []` |

## What it suits

- Maps and terrain
- Measurements and records
- Generative and data drawings

## Promoting it

When you have an engine for it, `tools/lemo_import.py --promote pen-plotter` copies the pack into `style/` with the engine folders it needs filled in.
