---
name: spreadsheet
description: Render in the Spreadsheet Film style: Cells are the pixels, and they are wide. Docs-only import from Lemo-Opuscar (CC BY-NC 4.0); it reads and scripts here but does not render until an engine is written for it. Use when the user asks for data literacy, finance & costs, analytics explainers.
---

# Spreadsheet Film
The total says treatment B; unhide the stone-size columns and A wins every group. Simpson's paradox told in one sheet, 1 cell = 1 patient.

Docs-only import from [Lemo-Opuscar by Lemomo (LemoLab)](https://github.com/lemomo-ai/lemo-opuscar) (`styles/spreadsheet`), CC BY-NC 4.0. Style content is unchanged; only the file layout was adapted.

**Not renderable yet.** This pack has no `remotion/`, `fframes/` or `kyfram/` engine, so `new_episode.sh` cannot scaffold it and the render skills cannot run it. `motion-strategy`, `motion-hook`, `motion-script` and `motion-storyboard` work normally: read `docs/style-guide.md` for the DNA and `docs/style-prompt.md` for portable prompts.

## Files

| file | what it gives |
|---|---|
| `docs/style-guide.md` | the style prompt: essence, materials, colour, type, motion, camera, sound, native moves, pitfalls |
| `docs/style-prompt.md` | the same DNA as standalone copy-paste blocks for another agent or generator |
| `style.json` | manifest: title, tagline, category, source, `engines: []` |

## What it suits

- Data literacy
- Finance & costs
- Analytics explainers

## Promoting it

When you have an engine for it, `tools/lemo_import.py --promote spreadsheet` copies the pack into `style/` with the engine folders it needs filled in.
