---
name: swiss-motion
description: Render in the Swiss Motion Graphics style: Pure 2D graphic design animated on a grid. Docs-only import from Lemo-Opuscar (CC BY-NC 4.0); it reads and scripts here but does not render until an engine is written for it. Use when the user asks for brand identity, events, design education.
---

# Swiss Motion Graphics
A concert poster lays itself out by five Swiss design rules; the fifth is to break just one.

Docs-only import from [Lemo-Opuscar by Lemomo (LemoLab)](https://github.com/lemomo-ai/lemo-opuscar) (`styles/swiss-motion`), CC BY-NC 4.0. Style content is unchanged; only the file layout was adapted.

**Not renderable yet.** This pack has no `remotion/`, `fframes/` or `kyfram/` engine, so `new_episode.sh` cannot scaffold it and the render skills cannot run it. `motion-strategy`, `motion-hook`, `motion-script` and `motion-storyboard` work normally: read `docs/style-guide.md` for the DNA and `docs/style-prompt.md` for portable prompts.

## Files

| file | what it gives |
|---|---|
| `docs/style-guide.md` | the style prompt: essence, materials, colour, type, motion, camera, sound, native moves, pitfalls |
| `docs/style-prompt.md` | the same DNA as standalone copy-paste blocks for another agent or generator |
| `style.json` | manifest: title, tagline, category, source, `engines: []` |

## What it suits

- Brand identity
- Events
- Design education

## Promoting it

When you have an engine for it, `tools/lemo_import.py --promote swiss-motion` copies the pack into `style/` with the engine folders it needs filled in.
