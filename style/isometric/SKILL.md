---
name: isometric
description: Render a reel in the Isometric style: calm precise product line art in 2:1 dimetric isometric on a white or dark ground, drawn only with constant 2px hairlines, white-filled faces so nearer objects hide farther lines, rounded slabs with visible thickness, stacked layers, repeated-part grids, and one tiny dark accent per frame. Use when the user says isometric, product line art, technical illustration, exploded hardware view, or asks for a clean documentation-grade diagram look.
---

Calm, precise product line art in 2:1 dimetric isometric with constant hairlines and spring settles.

This is one style of the motion-explainer library. It ships the docs and tokens: `docs/style-guide.md` for the visual authority, `docs/style-prompt.md` for portable prompts, and a `style.json` manifest with the canvas, palette, fonts and audio roots. The pipeline skills (`motion-explainer`, `motion-hook`, `motion-script`, `motion-storyboard`, `motion-build`, `motion-qa`) all take the style name as `$MOTION_STYLE`, so nothing else in the workflow changes when you pick this one.

## When to use it

Reach for this style when the brief matches its DNA below. Read `docs/style-guide.md` first: it is the authority, this file is the index.

## Style DNA

1. 2:1 dimetric isometric only. No other perspective.
2. Constant 2.0px hairlines at 1080 wide, divided by the current zoom so they never thicken.
3. Faces are filled white so a nearer object hides the lines of a farther one.
4. Rounded slabs and plates with visible thickness, stacked layers, and grids of repeated parts such as keys, vents, pillars and ports.
5. One tiny dark accent per frame, nothing more. Motion on ones with spring settles, otherwise still.

Palette: ground `#ffffff`, silhouette and edges `#a4a4ac`, inner and receding lines `#e0e0e4`, faces white, one dark accent `#2a2b36`

## Kit components

Components the style calls for, named here so a plan can request them by name:

| component | what it is |
|---|---|
| `<IsometricGrid>` | the faint isometric construction grid |
| `<IsometricBlock x y w h d color>` | a slab with a top, a left and a right face at correct 2:1 angles |
| `<IsometricBanner text>` | the hairline banner plate |
| `<IsometricCaptions text>` | the dark caption strip |

## Nothing boils in a vacuum

docs/style-guide.md carries the measured reference line weights and the accent rule. The line weight is non-negotiable; read it before drawing.

## QA for this style

Check these on every contact sheet before calling a scene done; the rest of the checklist in `motion-qa` still applies.

- [ ] Every shape sits on a 2:1 isometric axis; no perspective and no 3/4 views.
- [ ] Line weight is constant at about 2px; nearer faces hide farther lines.
- [ ] Faces are white; ground is white or the declared dark ground.
- [ ] Exactly one small dark accent per frame.
- [ ] Motion is on ones with spring settles, not on twos.

## Files

- `docs/style-guide.md`
- `docs/style-prompt.md`
- `style.json`: the manifest (canvas, engine list, doc paths, audio roots)
- `sample.png`: one representative frame
