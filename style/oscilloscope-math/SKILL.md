---
name: oscilloscope-math
description: Render in the Oscilloscope Math style: The maths is real. Docs-only import from Lemo-Opuscar (CC BY-NC 4.0); it reads and scripts here but does not render until an engine is written for it. Use when the user asks for maths and physics lessons, signals and electronics, science explainers.
---

# Oscilloscope Math
How do you roll a square out of circles? Spinning circles stack into a square wave, until a dive into one corner finds the 9% spike that never goes away.

Docs-only import from [Lemo-Opuscar by Lemomo (LemoLab)](https://github.com/lemomo-ai/lemo-opuscar) (`styles/oscilloscope-math`), CC BY-NC 4.0. Style content is unchanged; only the file layout was adapted.

**Not renderable yet.** This pack has no `remotion/`, `fframes/` or `kyfram/` engine, so `new_episode.sh` cannot scaffold it and the render skills cannot run it. `motion-strategy`, `motion-hook`, `motion-script` and `motion-storyboard` work normally: read `docs/style-guide.md` for the DNA and `docs/style-prompt.md` for portable prompts.

## Files

| file | what it gives |
|---|---|
| `docs/style-guide.md` | the style prompt: essence, materials, colour, type, motion, camera, sound, native moves, pitfalls |
| `docs/style-prompt.md` | the same DNA as standalone copy-paste blocks for another agent or generator |
| `style.json` | manifest: title, tagline, category, source, `engines: []` |

## What it suits

- Maths and physics lessons
- Signals and electronics
- Science explainers

## Promoting it

When you have an engine for it, `tools/lemo_import.py --promote oscilloscope-math` copies the pack into `style/` with the engine folders it needs filled in.
