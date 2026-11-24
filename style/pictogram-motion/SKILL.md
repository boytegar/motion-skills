---
name: pictogram-motion
description: Render in the Pictogram Motion style: The grid pattern: big discs and half-discs spanning square cells, about half the cells carrying a small motif, all in five tones of one. Docs-only import from Lemo-Opuscar (CC BY-NC 4.0); it reads and scripts here but does not render until an engine is written for it. Use when the user asks for public services and city stories, brand and event identity films, explainers told through signs and objects.
---

# Pictogram Motion
A yellow umbrella left on a bus waits out a sunny day on the lost-property shelf, then opens with 312 others when the storm comes back: objects, vehicles, weather and signs as beat-locked geometric pictograms.

Docs-only import from [Lemo-Opuscar by Lemomo (LemoLab)](https://github.com/lemomo-ai/lemo-opuscar) (`styles/pictogram-motion`), CC BY-NC 4.0. Style content is unchanged; only the file layout was adapted.

**Not renderable yet.** This pack has no `remotion/`, `fframes/` or `kyfram/` engine, so `new_episode.sh` cannot scaffold it and the render skills cannot run it. `motion-strategy`, `motion-hook`, `motion-script` and `motion-storyboard` work normally: read `docs/style-guide.md` for the DNA and `docs/style-prompt.md` for portable prompts.

## Files

| file | what it gives |
|---|---|
| `docs/style-guide.md` | the style prompt: essence, materials, colour, type, motion, camera, sound, native moves, pitfalls |
| `docs/style-prompt.md` | the same DNA as standalone copy-paste blocks for another agent or generator |
| `style.json` | manifest: title, tagline, category, source, `engines: []` |

## What it suits

- Public services and city stories
- Brand and event identity films
- Explainers told through signs and objects

## Promoting it

When you have an engine for it, `tools/lemo_import.py --promote pictogram-motion` copies the pack into `style/` with the engine folders it needs filled in.
