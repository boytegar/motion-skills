---
name: bauhaus-shapes
description: Render in the Bauhaus Shapes style: Shapes are amounts. Docs-only import from Lemo-Opuscar (CC BY-NC 4.0); it reads and scripts here but does not render until an engine is written for it. Use when the user asks for finance and economics explainers, mechanisms and principles, titles and chapter cards.
---

# Bauhaus Shapes
One dot doubles every ten years: 128 after seventy, 6.02 without compounding, and the last ten years are worth all the rest.

Docs-only import from [Lemo-Opuscar by Lemomo (LemoLab)](https://github.com/lemomo-ai/lemo-opuscar) (`styles/bauhaus-shapes`), CC BY-NC 4.0. Style content is unchanged; only the file layout was adapted.

**Not renderable yet.** This pack has no `remotion/`, `fframes/` or `kyfram/` engine, so `new_episode.sh` cannot scaffold it and the render skills cannot run it. `motion-strategy`, `motion-hook`, `motion-script` and `motion-storyboard` work normally: read `docs/style-guide.md` for the DNA and `docs/style-prompt.md` for portable prompts.

## Files

| file | what it gives |
|---|---|
| `docs/style-guide.md` | the style prompt: essence, materials, colour, type, motion, camera, sound, native moves, pitfalls |
| `docs/style-prompt.md` | the same DNA as standalone copy-paste blocks for another agent or generator |
| `style.json` | manifest: title, tagline, category, source, `engines: []` |

## What it suits

- Finance and economics explainers
- Mechanisms and principles
- Titles and chapter cards

## Promoting it

When you have an engine for it, `tools/lemo_import.py --promote bauhaus-shapes` copies the pack into `style/` with the engine folders it needs filled in.
