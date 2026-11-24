---
name: rubber-hose
description: Render in the 1930s Rubber Hose Cartoon style: Hand-inked black-and-white animation from the early sound era: characters with rubber-hose limbs (equal-width tubes, no elbows, no knees). Docs-only import from Lemo-Opuscar (CC BY-NC 4.0); it reads and scripts here but does not render until an engine is written for it. Use when the user asks for food & drink, retro ads, kids.
---

# 1930s Rubber Hose Cartoon
A coffee cup chases a runaway sugar cube around the kitchen, 1930s-cartoon style.

Docs-only import from [Lemo-Opuscar by Lemomo (LemoLab)](https://github.com/lemomo-ai/lemo-opuscar) (`styles/rubber-hose`), CC BY-NC 4.0. Style content is unchanged; only the file layout was adapted.

**Not renderable yet.** This pack has no `remotion/`, `fframes/` or `kyfram/` engine, so `new_episode.sh` cannot scaffold it and the render skills cannot run it. `motion-strategy`, `motion-hook`, `motion-script` and `motion-storyboard` work normally: read `docs/style-guide.md` for the DNA and `docs/style-prompt.md` for portable prompts.

## Files

| file | what it gives |
|---|---|
| `docs/style-guide.md` | the style prompt: essence, materials, colour, type, motion, camera, sound, native moves, pitfalls |
| `docs/style-prompt.md` | the same DNA as standalone copy-paste blocks for another agent or generator |
| `style.json` | manifest: title, tagline, category, source, `engines: []` |

## What it suits

- Food & drink
- Retro ads
- Kids

## Promoting it

When you have an engine for it, `tools/lemo_import.py --promote rubber-hose` copies the pack into `style/` with the engine folders it needs filled in.
