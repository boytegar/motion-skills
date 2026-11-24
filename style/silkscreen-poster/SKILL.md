---
name: silkscreen-poster
description: Render in the Silkscreen Travel Poster style: Registration: every layer offset 1–3 px at 1080p; a knocked-out shape leaves a crescent of bare paper. Docs-only import from Lemo-Opuscar (CC BY-NC 4.0); it reads and scripts here but does not render until an engine is written for it. Use when the user asks for destination guides, tourism, event posters.
---

# Silkscreen Travel Poster
A traveller crosses the world one full-bleed screen print at a time, a ticket stub counting the distance from home, until the far side turns into the way back.

Docs-only import from [Lemo-Opuscar by Lemomo (LemoLab)](https://github.com/lemomo-ai/lemo-opuscar) (`styles/silkscreen-poster`), CC BY-NC 4.0. Style content is unchanged; only the file layout was adapted.

**Not renderable yet.** This pack has no `remotion/`, `fframes/` or `kyfram/` engine, so `new_episode.sh` cannot scaffold it and the render skills cannot run it. `motion-strategy`, `motion-hook`, `motion-script` and `motion-storyboard` work normally: read `docs/style-guide.md` for the DNA and `docs/style-prompt.md` for portable prompts.

## Files

| file | what it gives |
|---|---|
| `docs/style-guide.md` | the style prompt: essence, materials, colour, type, motion, camera, sound, native moves, pitfalls |
| `docs/style-prompt.md` | the same DNA as standalone copy-paste blocks for another agent or generator |
| `style.json` | manifest: title, tagline, category, source, `engines: []` |

## What it suits

- Destination guides
- Tourism
- Event posters

## Promoting it

When you have an engine for it, `tools/lemo_import.py --promote silkscreen-poster` copies the pack into `style/` with the engine folders it needs filled in.
