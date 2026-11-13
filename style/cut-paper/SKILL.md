---
name: cut-paper
description: Render a reel in the Cut Paper (Collage) style: torn-edge paper shapes with crayon in the fills, pattern papers such as stripes, dots, gingham and wood grain, full-bleed colour worlds, and simple-faced characters. Use when the user says collage, cut paper, paper craft characters, kindergarten collage, torn paper, or asks for a playful warm busy picture-book look.
---

Torn-edge paper collage with drop shadows, crayon fills, playful patterns and character faces.

This is one style of the motion-explainer library. It ships the same three things every style ships: a `style.json` manifest, a Remotion engine, and an fframes engine. The pipeline skills (`motion-explainer`, `motion-script`, `motion-storyboard`, `motion-build`, `motion-qa`) all take the style name as `$MOTION_STYLE`, so nothing else in the workflow changes when you pick this one.

## When to use it

Reach for this style when the brief matches its DNA below. Read `docs/style-guide.md` first: it is the authority, this file is the index.

## Style DNA

1. Every shape is torn paper: a wobbly torn edge, a soft shadow, paper grain and an optional crayon scribble inside the fill.
2. Patterns come from a small paper library clipped inside the shape: stripes, dots, gingham, wood grain.
3. Full-bleed colour worlds: the background is a colour field, not a room.
4. Characters have simple faces and stubby limbs; the orange spark is reserved for the hero and never used as decoration.
5. Playful, warm, busy with small life: at least one small moving detail per scene.

Palette: ink `#2a1d18`, paper `#fbf6ea`, cream `#f3ead6`, navy `#26315f`, wood `#c08048`, mustard `#d9a441`, yellow `#f6cf55`, teal `#3f9f9a`, mint `#a9d9c6`, pink `#f3a3bf`, sky `#cfe2ee`, purple `#8c7ad8`; hero orange `#ec7a4f`

## Kit components

Remotion, importable from `../lib/<file>` (the kit is synced into each project's `src/lib/`):

| component | what it is |
|---|---|
| `<CutPaperDefs>` | the `torn-edge` displacement filter |
| `<CutPaperSlab x y w h color rot>` | a torn paper shape with crayon-friendly fill and drop shadow |
| `<SparkHero x y r>` | the orange hero critter with eyes: the viewer's stand-in |
| `<CutPaperBanner text>` | the collage headline strip |
| `<CutPaperCaptions text>` | the caption card |

## Nothing boils in a vacuum

docs/style-guide.md holds the torn-edge recipe, the pattern library names and the crayon strength. Read it before choosing a fill.

## Engines

| engine | path | entry |
|---|---|---|
| remotion | `remotion/` | `template/`, `kit/`, `sync_kit.sh`, `CATALOG.md`, `AGENT_BRIEF.md` |
| fframes | `fframes/` | `Cargo.toml`, `src/` (lib, tokens, kit, look, video, main), `tools/ff.sh` |

Registered engines: `remotion`, `fframes`.

## Scaffold an episode

```sh
# from the library root, with MOTION_LIB resolved
./core/new_episode.sh <series-dir> 01-slug Episode out-name --style cut-paper --engine remotion
./core/new_episode.sh <series-dir> 01-slug Episode out-name --style cut-paper --engine fframes
```

Or name the style in a series `SERIES.md` / `episode.json` as `"style": "cut-paper"` and let `motion_style_of` pick it up.

## QA for this style

Check these on every contact sheet before calling a scene done; the rest of the checklist in `motion-qa` still applies.

- [ ] Every object has a torn edge, a soft shadow and grain.
- [ ] Pattern papers are clipped inside their shape, never overflowing.
- [ ] The orange spark appears only as the hero, never as decoration.
- [ ] The colour world is full-bleed and changes with the beat.
- [ ] At least one small piece of life moves in every scene.

## Files

- `docs/style-guide.md`
- `docs/style-prompt.md`
- `remotion/CATALOG.md`
- `remotion/AGENT_BRIEF.md`
- `remotion/RECIPES.md`
- `style.json`: the manifest (canvas, engine list, doc paths, audio roots)
- `sample.png`: one representative frame
