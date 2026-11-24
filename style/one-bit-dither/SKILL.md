---
name: one-bit-dither
description: Render in the 1-bit Dither style: Two inks, no grey. Docs-only import from Lemo-Opuscar (CC BY-NC 4.0); it reads and scripts here but does not render until an engine is written for it. Use when the user asks for history told through a chain of media, printing, scanning and signals, night scenes and space.
---

# 1-bit Dither
February 1966: the first close-up of the Moon arrives at an English radio telescope as a newspaper fax, printed one line at a time in two inks, a day early and stretched too tall.

Docs-only import from [Lemo-Opuscar by Lemomo (LemoLab)](https://github.com/lemomo-ai/lemo-opuscar) (`styles/one-bit-dither`), CC BY-NC 4.0. Style content is unchanged; only the file layout was adapted.

**Not renderable yet.** This pack has no `remotion/`, `fframes/` or `kyfram/` engine, so `new_episode.sh` cannot scaffold it and the render skills cannot run it. `motion-strategy`, `motion-hook`, `motion-script` and `motion-storyboard` work normally: read `docs/style-guide.md` for the DNA and `docs/style-prompt.md` for portable prompts.

## Files

| file | what it gives |
|---|---|
| `docs/style-guide.md` | the style prompt: essence, materials, colour, type, motion, camera, sound, native moves, pitfalls |
| `docs/style-prompt.md` | the same DNA as standalone copy-paste blocks for another agent or generator |
| `style.json` | manifest: title, tagline, category, source, `engines: []` |

## What it suits

- History told through a chain of media
- Printing, scanning and signals
- Night scenes and space

## Promoting it

When you have an engine for it, `tools/lemo_import.py --promote one-bit-dither` copies the pack into `style/` with the engine folders it needs filled in.
