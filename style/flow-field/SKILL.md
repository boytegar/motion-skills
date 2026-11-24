---
name: flow-field
description: Render in the Flow-field Wind Lines style: Every line is a streamline of a real field. Docs-only import from Lemo-Opuscar (CC BY-NC 4.0); it reads and scripts here but does not render until an engine is written for it. Use when the user asks for storms, monsoons and other weather, ocean currents and river floods, any flow measured against a threshold.
---

# Flow-field Wind Lines
Thousands of ink wind lines, computed from the best track, draw Super Typhoon Yagi: the eye opening over the South China Sea, the landfall on Hainan, the spiral coming apart in Vietnam, and a whole week folded into one last line.

Docs-only import from [Lemo-Opuscar by Lemomo (LemoLab)](https://github.com/lemomo-ai/lemo-opuscar) (`styles/flow-field`), CC BY-NC 4.0. Style content is unchanged; only the file layout was adapted.

**Not renderable yet.** This pack has no `remotion/`, `fframes/` or `kyfram/` engine, so `new_episode.sh` cannot scaffold it and the render skills cannot run it. `motion-strategy`, `motion-hook`, `motion-script` and `motion-storyboard` work normally: read `docs/style-guide.md` for the DNA and `docs/style-prompt.md` for portable prompts.

## Files

| file | what it gives |
|---|---|
| `docs/style-guide.md` | the style prompt: essence, materials, colour, type, motion, camera, sound, native moves, pitfalls |
| `docs/style-prompt.md` | the same DNA as standalone copy-paste blocks for another agent or generator |
| `style.json` | manifest: title, tagline, category, source, `engines: []` |

## What it suits

- Storms, monsoons and other weather
- Ocean currents and river floods
- Any flow measured against a threshold

## Promoting it

When you have an engine for it, `tools/lemo_import.py --promote flow-field` copies the pack into `style/` with the engine folders it needs filled in.
