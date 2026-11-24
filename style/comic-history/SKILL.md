---
name: comic-history
description: Render in the Comic History Cartoon style: Every change of territory is a paper action. Docs-only import from Lemo-Opuscar (CC BY-NC 4.0); it reads and scripts here but does not render until an engine is written for it. Use when the user asks for territorial changes in history, light explainers on geopolitics, market share and turf-war stories.
---

# Comic History Cartoon
Alsace-Lorraine changed hands four times between 1871 and 1945: each time someone tore off the paper and pasted a new piece, and the woman living on it never moved.

Docs-only import from [Lemo-Opuscar by Lemomo (LemoLab)](https://github.com/lemomo-ai/lemo-opuscar) (`styles/comic-history`), CC BY-NC 4.0. Style content is unchanged; only the file layout was adapted.

**Not renderable yet.** This pack has no `remotion/`, `fframes/` or `kyfram/` engine, so `new_episode.sh` cannot scaffold it and the render skills cannot run it. `motion-strategy`, `motion-hook`, `motion-script` and `motion-storyboard` work normally: read `docs/style-guide.md` for the DNA and `docs/style-prompt.md` for portable prompts.

## Files

| file | what it gives |
|---|---|
| `docs/style-guide.md` | the style prompt: essence, materials, colour, type, motion, camera, sound, native moves, pitfalls |
| `docs/style-prompt.md` | the same DNA as standalone copy-paste blocks for another agent or generator |
| `style.json` | manifest: title, tagline, category, source, `engines: []` |

## What it suits

- Territorial changes in history
- Light explainers on geopolitics
- Market share and turf-war stories

## Promoting it

When you have an engine for it, `tools/lemo_import.py --promote comic-history` copies the pack into `style/` with the engine folders it needs filled in.
