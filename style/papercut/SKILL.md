---
name: papercut
description: Render a reel in the Papercut style: layered construction paper brought to life with paper grain, torn edges, stacked shade strips, drop shadows, rigid cut-out animation, a slow camera push, taped headline banners and word-by-word torn-tag captions. Use when the user says cut paper, papercraft, paper cutout, paper collage, construction paper, or asks for the default handcrafted explainer look.
---

Layered construction paper: grain, torn edges, stacked shade strips, a slow camera push and stop-motion paper boil.

This is one style of the motion-explainer library. It ships the same three things every style ships: a `style.json` manifest, a Remotion engine, and an fframes engine. The pipeline skills (`motion-explainer`, `motion-script`, `motion-storyboard`, `motion-build`, `motion-qa`) all take the style name as `$MOTION_STYLE`, so nothing else in the workflow changes when you pick this one.

## When to use it

Reach for this style when the brief matches its DNA below. Read `docs/style-guide.md` first: it is the authority, this file is the index.

## Style DNA

1. Every object is flat construction paper: one solid fill, fine paper-fibre grain multiplied at ~10%, a slightly irregular scissor-cut edge, and a soft drop shadow (0 x, 3-6px y, 6-10px blur, 30% black). No gradients or outlines.
2. Shading is stacked cut pieces: a darker underside strip and a lighter highlight strip, never a blend.
3. Orthographic views only, chunky and legible at phone size, light from the top-left.
4. Saturation is earned: muted dusty backgrounds, bright colour only on characters, captions and state signals.
5. The world camera drifts smoothly; only the banner and labels boil, stepped on twos.

Palette: cream `#DACEC0`, ink `#1C1B17`, tape `#C3B284`, hero `#D67354`, captions `#3968AD`, tags `#C9573C`

## Kit components

Remotion, importable from `../lib/<file>` (the kit is synced into each project's `src/lib/`):

| component | what it is |
|---|---|
| `<Paper lift torn>` | the base wrapper: paper grain, rough or torn SVG filter, drop shadow |
| `<Slab x y w h color shade hi>` | the primitive: a cut-paper rectangle with a shade strip, a highlight strip and grain |
| `<Disc x y d color>` | a circular cut piece for portholes, wheels, rivets |
| `<Pill x y text>` | a small paper label chip |
| `<Card x y w h lines>` | a paper card with ruled filler lines |
| `<PopIn at>` | spring entrance with a small settle tilt |
| `<Backdrop name>` | a prepared paper set with a known `floorY` |
| `<CameraPush>` | the world camera move |
| `<TapeBanner text>` | the taped headline banner, one hard fact per beat |
| `<WordTagCaptions captions>` | torn tag captions, one per word, popping on onset |
| `<CutSeries scenes cuts>` | the transition engine between scene sets |

## Nothing boils in a vacuum

docs/style-guide.md section 2 is the Style DNA. docs/frame-breakdown.md is a worked ~28 s example; copy its event density, not its content.

## Engines

| engine | path | entry |
|---|---|---|
| remotion | `remotion/` | `template/`, `kit/`, `sync_kit.sh`, `CATALOG.md`, `AGENT_BRIEF.md` |
| fframes | `fframes/` | `Cargo.toml`, `src/` (lib, tokens, kit, look, video, main), `tools/ff.sh` |

Registered engines: `remotion`, `fframes`.

## Scaffold an episode

```sh
# from the library root, with MOTION_LIB resolved
./core/new_episode.sh <series-dir> 01-slug Episode out-name --style papercut --engine remotion
./core/new_episode.sh <series-dir> 01-slug Episode out-name --style papercut --engine fframes
```

Or name the style in a series `SERIES.md` / `episode.json` as `"style": "papercut"` and let `motion_style_of` pick it up.

## QA for this style

Check these on every contact sheet before calling a scene done; the rest of the checklist in `motion-qa` still applies.

- [ ] Everything reads as paper: grain, rough edge, drop shadow, stacked-piece shading, no outlines or gradients.
- [ ] The world camera moves smoothly; only the banner and labels boil.
- [ ] The banner is taped, 3-7 words, one fact per beat, never repeats the voiceover.
- [ ] Captions are one torn tag per word, tilted, popping on onset, in the 68-80% band.
- [ ] A visual event on every stressed word, with ambient motion always running.

## Files

- `docs/style-guide.md`
- `docs/style-prompt.md`
- `docs/motion-principles.md`
- `docs/asset-prompts.md`
- `docs/frame-breakdown.md`
- `remotion/CATALOG.md`
- `remotion/AGENT_BRIEF.md`
- `remotion/RECIPES.md`
- `style.json`: the manifest (canvas, engine list, doc paths, audio roots)
- `sample.png`: one representative frame
