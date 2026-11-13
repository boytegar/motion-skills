---
name: isometric
description: Render a reel in the Isometric style: calm precise product line art in 2:1 dimetric isometric on a white or dark ground, drawn only with constant 2px hairlines, white-filled faces so nearer objects hide farther lines, rounded slabs with visible thickness, stacked layers, repeated-part grids, and one tiny dark accent per frame. Use when the user says isometric, product line art, technical illustration, exploded hardware view, or asks for a clean documentation-grade diagram look.
---

Calm, precise product line art in 2:1 dimetric isometric with constant hairlines and spring settles.

This is one style of the motion-explainer library. It ships the same three things every style ships: a `style.json` manifest, a Remotion engine, and an fframes engine. The pipeline skills (`motion-explainer`, `motion-script`, `motion-storyboard`, `motion-build`, `motion-qa`) all take the style name as `$MOTION_STYLE`, so nothing else in the workflow changes when you pick this one.

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

Remotion, importable from `../lib/<file>` (the kit is synced into each project's `src/lib/`):

| component | what it is |
|---|---|
| `<IsometricGrid>` | the faint isometric construction grid |
| `<IsometricBlock x y w h d color>` | a slab with a top, a left and a right face at correct 2:1 angles |
| `<IsometricBanner text>` | the hairline banner plate |
| `<IsometricCaptions text>` | the dark caption strip |

## Nothing boils in a vacuum

docs/style-guide.md carries the measured reference line weights and the accent rule. The line weight is non-negotiable; read it before drawing.

## Engines

| engine | path | entry |
|---|---|---|
| remotion | `remotion/` | `template/`, `kit/`, `sync_kit.sh`, `CATALOG.md`, `AGENT_BRIEF.md` |
| fframes | `fframes/` | `Cargo.toml`, `src/` (lib, tokens, kit, look, video, main), `tools/ff.sh` |

Registered engines: `remotion`, `fframes`.

## Scaffold an episode

```sh
# from the library root, with MOTION_LIB resolved
./core/new_episode.sh <series-dir> 01-slug Episode out-name --style isometric --engine remotion
./core/new_episode.sh <series-dir> 01-slug Episode out-name --style isometric --engine fframes
```

Or name the style in a series `SERIES.md` / `episode.json` as `"style": "isometric"` and let `motion_style_of` pick it up.

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
- `remotion/CATALOG.md`
- `remotion/AGENT_BRIEF.md`
- `style.json`: the manifest (canvas, engine list, doc paths, audio roots)
- `sample.png`: one representative frame
