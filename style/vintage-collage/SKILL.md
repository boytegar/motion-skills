---
name: vintage-collage
description: Render in the Vintage Engraving Collage style: Real printed matter, cut by hand. Docs-only import from Lemo-Opuscar (CC BY-NC 4.0); it reads and scripts here but does not render until an engine is written for it. Use when the user asks for the history of an object, magazine-style explainers on ideas and society, archive and collecting stories.
---

# Vintage Engraving Collage
A 1918 airmail sheet printed with its plane upside down: bought for $24, cut up and scattered, and number 66 is still missing.

Docs-only import from [Lemo-Opuscar by Lemomo (LemoLab)](https://github.com/lemomo-ai/lemo-opuscar) (`styles/vintage-collage`), CC BY-NC 4.0. Style content is unchanged; only the file layout was adapted.

**Not renderable yet.** This pack has no `remotion/`, `fframes/` or `kyfram/` engine, so `new_episode.sh` cannot scaffold it and the render skills cannot run it. `motion-strategy`, `motion-hook`, `motion-script` and `motion-storyboard` work normally: read `docs/style-guide.md` for the DNA and `docs/style-prompt.md` for portable prompts.

## Files

| file | what it gives |
|---|---|
| `docs/style-guide.md` | the style prompt: essence, materials, colour, type, motion, camera, sound, native moves, pitfalls |
| `docs/style-prompt.md` | the same DNA as standalone copy-paste blocks for another agent or generator |
| `style.json` | manifest: title, tagline, category, source, `engines: []` |

## What it suits

- The history of an object
- Magazine-style explainers on ideas and society
- Archive and collecting stories

## Promoting it

When you have an engine for it, `tools/lemo_import.py --promote vintage-collage` copies the pack into `style/` with the engine folders it needs filled in.
