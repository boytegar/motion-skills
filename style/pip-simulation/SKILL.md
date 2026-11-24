---
name: pip-simulation
description: Render in the Blob Simulation style: The simulation is real. Docs-only import from Lemo-Opuscar (CC BY-NC 4.0); it reads and scripts here but does not render until an engine is written for it. Use when the user asks for evolution and natural selection, game theory and agent-based models, epidemics, queues and markets.
---

# Blob Simulation
160 flocked Pips race for beans: the fast ones eat first and run dry first, and after 20 generations the board turns blue.

Docs-only import from [Lemo-Opuscar by Lemomo (LemoLab)](https://github.com/lemomo-ai/lemo-opuscar) (`styles/pip-simulation`), CC BY-NC 4.0. Style content is unchanged; only the file layout was adapted.

**Not renderable yet.** This pack has no `remotion/`, `fframes/` or `kyfram/` engine, so `new_episode.sh` cannot scaffold it and the render skills cannot run it. `motion-strategy`, `motion-hook`, `motion-script` and `motion-storyboard` work normally: read `docs/style-guide.md` for the DNA and `docs/style-prompt.md` for portable prompts.

## Files

| file | what it gives |
|---|---|
| `docs/style-guide.md` | the style prompt: essence, materials, colour, type, motion, camera, sound, native moves, pitfalls |
| `docs/style-prompt.md` | the same DNA as standalone copy-paste blocks for another agent or generator |
| `style.json` | manifest: title, tagline, category, source, `engines: []` |

## What it suits

- Evolution and natural selection
- Game theory and agent-based models
- Epidemics, queues and markets

## Promoting it

When you have an engine for it, `tools/lemo_import.py --promote pip-simulation` copies the pack into `style/` with the engine folders it needs filled in.
