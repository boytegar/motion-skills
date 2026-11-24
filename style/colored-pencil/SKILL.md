---
name: colored-pencil
description: Render in the Colored-Pencil Science Illustration style: The whole frame is one pencil drawing. Docs-only import from Lemo-Opuscar (CC BY-NC 4.0); it reads and scripts here but does not render until an engine is written for it. Use when the user asks for natural-science cutaways, picture-book explainers, hidden structures and processes.
---

# Colored-Pencil Science Illustration
Do trees feed each other? A pencil drawing sinks below the grass, follows one grain of sugar along a fungal thread to the next tree, and pulls back over a whole wood joined underground.

Docs-only import from [Lemo-Opuscar by Lemomo (LemoLab)](https://github.com/lemomo-ai/lemo-opuscar) (`styles/colored-pencil`), CC BY-NC 4.0. Style content is unchanged; only the file layout was adapted.

**Not renderable yet.** This pack has no `remotion/`, `fframes/` or `kyfram/` engine, so `new_episode.sh` cannot scaffold it and the render skills cannot run it. `motion-strategy`, `motion-hook`, `motion-script` and `motion-storyboard` work normally: read `docs/style-guide.md` for the DNA and `docs/style-prompt.md` for portable prompts.

## Files

| file | what it gives |
|---|---|
| `docs/style-guide.md` | the style prompt: essence, materials, colour, type, motion, camera, sound, native moves, pitfalls |
| `docs/style-prompt.md` | the same DNA as standalone copy-paste blocks for another agent or generator |
| `style.json` | manifest: title, tagline, category, source, `engines: []` |

## What it suits

- Natural-science cutaways
- Picture-book explainers
- Hidden structures and processes

## Promoting it

When you have an engine for it, `tools/lemo_import.py --promote colored-pencil` copies the pack into `style/` with the engine folders it needs filled in.
