---
name: instruction-manual
description: Render in the Instruction Manual Art style: One angle: true isometric, orthographic. Docs-only import from Lemo-Opuscar (CC BY-NC 4.0); it reads and scripts here but does not render until an engine is written for it. Use when the user asks for assembly and set-up guides, safety instructions, product onboarding.
---

# Instruction Manual Art
Two first-timers build a crib from a flat-pack manual, ten steps on one 45° page; step ten was never in the manual.

Docs-only import from [Lemo-Opuscar by Lemomo (LemoLab)](https://github.com/lemomo-ai/lemo-opuscar) (`styles/instruction-manual`), CC BY-NC 4.0. Style content is unchanged; only the file layout was adapted.

**Not renderable yet.** This pack has no `remotion/`, `fframes/` or `kyfram/` engine, so `new_episode.sh` cannot scaffold it and the render skills cannot run it. `motion-strategy`, `motion-hook`, `motion-script` and `motion-storyboard` work normally: read `docs/style-guide.md` for the DNA and `docs/style-prompt.md` for portable prompts.

## Files

| file | what it gives |
|---|---|
| `docs/style-guide.md` | the style prompt: essence, materials, colour, type, motion, camera, sound, native moves, pitfalls |
| `docs/style-prompt.md` | the same DNA as standalone copy-paste blocks for another agent or generator |
| `style.json` | manifest: title, tagline, category, source, `engines: []` |

## What it suits

- Assembly and set-up guides
- Safety instructions
- Product onboarding

## Promoting it

When you have an engine for it, `tools/lemo_import.py --promote instruction-manual` copies the pack into `style/` with the engine folders it needs filled in.
