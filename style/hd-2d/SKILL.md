---
name: hd-2d
description: Render in the 2.5D Pixel Art style: A 3D miniature set of boxes and cylinders wrapped in pixel-art textures, populated by flat 2D pixel sprites that receive light and cast. Docs-only import from Lemo-Opuscar (CC BY-NC 4.0); it reads and scripts here but does not render until an engine is written for it. Use when the user asks for game trailers, fantasy, story films.
---

# 2.5D Pixel Art
The lighthouse goes dark; a girl carries the last flame through a night forest and up the storm cliffs.

Docs-only import from [Lemo-Opuscar by Lemomo (LemoLab)](https://github.com/lemomo-ai/lemo-opuscar) (`styles/hd-2d`), CC BY-NC 4.0. Style content is unchanged; only the file layout was adapted.

**Not renderable yet.** This pack has no `remotion/`, `fframes/` or `kyfram/` engine, so `new_episode.sh` cannot scaffold it and the render skills cannot run it. `motion-strategy`, `motion-hook`, `motion-script` and `motion-storyboard` work normally: read `docs/style-guide.md` for the DNA and `docs/style-prompt.md` for portable prompts.

## Files

| file | what it gives |
|---|---|
| `docs/style-guide.md` | the style prompt: essence, materials, colour, type, motion, camera, sound, native moves, pitfalls |
| `docs/style-prompt.md` | the same DNA as standalone copy-paste blocks for another agent or generator |
| `style.json` | manifest: title, tagline, category, source, `engines: []` |

## What it suits

- Game trailers
- Fantasy
- Story films

## Promoting it

When you have an engine for it, `tools/lemo_import.py --promote hd-2d` copies the pack into `style/` with the engine folders it needs filled in.
