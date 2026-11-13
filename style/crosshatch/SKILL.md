---
name: crosshatch
description: Render a reel in the Crosshatch style: a sketchy ink line that boils on twos, crosshatch and coloured-pencil shading clipped inside shapes, stipple, paper grain, warm cream paper by day and a navy "inside the machine" world with periwinkle lines and glowing accents. Use when the user says crosshatch, sketchy ink, hand-drawn ink explanation, pencil shading, or asks for an annotated notebook-and-computer look.
---

Sketchy ink line that boils on 2s, crosshatch and pencil shading on cream paper and navy night worlds.

This is one style of the motion-explainer library. It ships the same three things every style ships: a `style.json` manifest, a Remotion engine, and an fframes engine. The pipeline skills (`motion-explainer`, `motion-script`, `motion-storyboard`, `motion-build`, `motion-qa`) all take the style name as `$MOTION_STYLE`, so nothing else in the workflow changes when you pick this one.

## When to use it

Reach for this style when the brief matches its DNA below. Read `docs/style-guide.md` first: it is the authority, this file is the index.

## Style DNA

1. Every filled shape carries a texture: hatch, cross, coloured pencil or stipple, shaped by a density function so light still reads from the top-left.
2. The line boils: strokes re-wobble every two frames even in a held shot, so the drawing is alive when nothing moves.
3. Shade with a density function rather than flat fills.
4. Two worlds that must look different on a contact sheet: cream paper and ink outside; navy and periwinkle inside the machine.
5. Grain over every frame.

Palette: day ink `#3a2417`, paper `#f4ecd9`, cream `#efe5cf`, wood `#d6a466`, yellow `#f0c55a`, green `#5f9c5c`, red `#d4382c`, sky `#9cc4d6`; night navy `#0c0d1c`, periwinkle `#c8d0f0`, cyan `#52d6c4`, magenta `#e0368f`; hero terracotta `#d97757`

## Kit components

Remotion, importable from `../lib/<file>` (the kit is synced into each project's `src/lib/`):

| component | what it is |
|---|---|
| `<CrosshatchDefs>` | the SVG `hatch-45`, `cross-45` patterns and the `ink-boil` turbulence filter |
| `<CrosshatchSlab x y w h hatch cross>` | an inked, textured shape that re-wobbles every two frames |
| `<ToolPill x y tool arg>` | an agent or tool-call pill that makes a step readable |
| `<CrosshatchBanner text>` | the steam-punk-free taped banner, boiling on twos |
| `<CrosshatchCaptions text>` | the caption band |
| `<GrainOverlay>` | the paper-grain post pass |

## Nothing boils in a vacuum

docs/style-guide.md names the day and night palettes, the texture pucks and the motion habits. Read the guide before drawing anything: the two worlds are not interchangeable.

## Engines

| engine | path | entry |
|---|---|---|
| remotion | `remotion/` | `template/`, `kit/`, `sync_kit.sh`, `CATALOG.md`, `AGENT_BRIEF.md` |
| fframes | `fframes/` | `Cargo.toml`, `src/` (lib, tokens, kit, look, video, main), `tools/ff.sh` |

Registered engines: `remotion`, `fframes`.

## Scaffold an episode

```sh
# from the library root, with MOTION_LIB resolved
./core/new_episode.sh <series-dir> 01-slug Episode out-name --style crosshatch --engine remotion
./core/new_episode.sh <series-dir> 01-slug Episode out-name --style crosshatch --engine fframes
```

Or name the style in a series `SERIES.md` / `episode.json` as `"style": "crosshatch"` and let `motion_style_of` pick it up.

## QA for this style

Check these on every contact sheet before calling a scene done; the rest of the checklist in `motion-qa` still applies.

- [ ] Every filled shape has a texture shaped by light; no flat fills except paper.
- [ ] The line boils in held frames; nothing is perfectly still.
- [ ] Dark-world frames read as navy plus periwinkle; paper-world frames as cream plus ink.
- [ ] Tool pills and labels are true statements about the thing being explained.
- [ ] Grain is present on every frame.

## Files

- `docs/style-guide.md`
- `docs/style-prompt.md`
- `remotion/CATALOG.md`
- `remotion/AGENT_BRIEF.md`
- `remotion/RECIPES.md`
- `style.json`: the manifest (canvas, engine list, doc paths, audio roots)
- `sample.png`: one representative frame
