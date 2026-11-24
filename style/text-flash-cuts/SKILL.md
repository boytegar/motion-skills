---
name: text-flash-cuts
description: Render in the Text Flash Cuts style: Type as picture. Docs-only import from Lemo-Opuscar (CC BY-NC 4.0); it reads and scripts here but does not render until an engine is written for it. Use when the user asks for suspense & drama trailers, title sequences, manifesto films.
---

# Text Flash Cuts
A letter, a phone call, a wall of windows, thirteen steps: the girl who told him not to look back is always one step closer, until she turns her head.

Docs-only import from [Lemo-Opuscar by Lemomo (LemoLab)](https://github.com/lemomo-ai/lemo-opuscar) (`styles/text-flash-cuts`), CC BY-NC 4.0. Style content is unchanged; only the file layout was adapted.

**Not renderable yet.** This pack has no `remotion/`, `fframes/` or `kyfram/` engine, so `new_episode.sh` cannot scaffold it and the render skills cannot run it. `motion-strategy`, `motion-hook`, `motion-script` and `motion-storyboard` work normally: read `docs/style-guide.md` for the DNA and `docs/style-prompt.md` for portable prompts.

## Files

| file | what it gives |
|---|---|
| `docs/style-guide.md` | the style prompt: essence, materials, colour, type, motion, camera, sound, native moves, pitfalls |
| `docs/style-prompt.md` | the same DNA as standalone copy-paste blocks for another agent or generator |
| `style.json` | manifest: title, tagline, category, source, `engines: []` |

## What it suits

- Suspense & drama trailers
- Title sequences
- Manifesto films

## Promoting it

When you have an engine for it, `tools/lemo_import.py --promote text-flash-cuts` copies the pack into `style/` with the engine folders it needs filled in.
