---
name: lowpoly-island
description: Render in the Low-poly Isometric Island style: A miniature world built from flat-shaded low-poly geometry (hexagonal tile columns, box buildings with prism roofs, cone and icosahedron. Docs-only import from Lemo-Opuscar (CC BY-NC 4.0); it reads and scripts here but does not render until an engine is written for it. Use when the user asks for game trailers, world building, calm content.
---

# Low-poly Isometric Island
An island and its village grow tile by tile from an empty sea, each tile a note, into a starry night.

Docs-only import from [Lemo-Opuscar by Lemomo (LemoLab)](https://github.com/lemomo-ai/lemo-opuscar) (`styles/lowpoly-island`), CC BY-NC 4.0. Style content is unchanged; only the file layout was adapted.

**Not renderable yet.** This pack has no `remotion/`, `fframes/` or `kyfram/` engine, so `new_episode.sh` cannot scaffold it and the render skills cannot run it. `motion-strategy`, `motion-hook`, `motion-script` and `motion-storyboard` work normally: read `docs/style-guide.md` for the DNA and `docs/style-prompt.md` for portable prompts.

## Files

| file | what it gives |
|---|---|
| `docs/style-guide.md` | the style prompt: essence, materials, colour, type, motion, camera, sound, native moves, pitfalls |
| `docs/style-prompt.md` | the same DNA as standalone copy-paste blocks for another agent or generator |
| `style.json` | manifest: title, tagline, category, source, `engines: []` |

## What it suits

- Game trailers
- World building
- Calm content

## Promoting it

When you have an engine for it, `tools/lemo_import.py --promote lowpoly-island` copies the pack into `style/` with the engine folders it needs filled in.
