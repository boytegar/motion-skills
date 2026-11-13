---
name: sketchbook
description: Render a reel in the Sketchbook style: graphite and one accent colour on a warm dot-grid sketchbook page, every line hand-drawn and boiling on twos, things pencilled in then inked over then coloured with a quick scribble, hand lettering narrating, with the page's spiral binding, smudges and coffee ring always in view. Use when the user says sketchbook, notebook, graphite, pencil sketch, moleskine, doodle page, or asks for a hand-lettered personal-notes look.
---

Graphite pencil and vivid single accent on warm dot-grid sketchbook paper, hand lettering on 2s.

This is one style of the motion-explainer library. It ships the same three things every style ships: a `style.json` manifest, a Remotion engine, and an fframes engine. The pipeline skills (`motion-explainer`, `motion-script`, `motion-storyboard`, `motion-build`, `motion-qa`) all take the style name as `$MOTION_STYLE`, so nothing else in the workflow changes when you pick this one.

## When to use it

Reach for this style when the brief matches its DNA below. Read `docs/style-guide.md` first: it is the authority, this file is the index.

## Style DNA

1. The page is the world: dot grid, spiral binding, smudges and a coffee ring stay in view, and the camera moves across the page instead of cutting away.
2. Every line is hand-drawn and boils on twos.
3. The build order is visible: pencil construction lines first, then ink over them, then a quick accent-coloured scribble.
4. Graphite is a value range, not one grey: settling and unsettled lines are different greys.
5. The accent colour belongs to the hero only.

Palette: paper `#f2ead8`, ink `#2a2420`, graphite `#6f6a63` (light `#9a948b`), accent terracotta `#d97757` (shade `#b85c3f`), line `#888078`

## Kit components

Remotion, importable from `../lib/<file>` (the kit is synced into each project's `src/lib/`):

| component | what it is |
|---|---|
| `<DotGridPage>` | the dot-grid paper the whole scene sits on |
| `<SketchScribble x y w h color>` | the quick accent scribble fill |
| `<SketchBanner text>` | hand-lettered banner with a ruled underline |
| `<SketchCaptions text>` | hand-lettered caption text |

## Nothing boils in a vacuum

docs/style-guide.md carries the pencil-to-ink build order and the graphite values. Read it before drawing a line.

## Engines

| engine | path | entry |
|---|---|---|
| remotion | `remotion/` | `template/`, `kit/`, `sync_kit.sh`, `CATALOG.md`, `AGENT_BRIEF.md` |
| fframes | `fframes/` | `Cargo.toml`, `src/` (lib, tokens, kit, look, video, main), `tools/ff.sh` |

Registered engines: `remotion`, `fframes`.

## Scaffold an episode

```sh
# from the library root, with MOTION_LIB resolved
./core/new_episode.sh <series-dir> 01-slug Episode out-name --style sketchbook --engine remotion
./core/new_episode.sh <series-dir> 01-slug Episode out-name --style sketchbook --engine fframes
```

Or name the style in a series `SERIES.md` / `episode.json` as `"style": "sketchbook"` and let `motion_style_of` pick it up.

## QA for this style

Check these on every contact sheet before calling a scene done; the rest of the checklist in `motion-qa` still applies.

- [ ] Every line boils on twos; nothing is machine-perfect.
- [ ] Pencil construction lines are visible under the ink.
- [ ] The accent colour appears only on the hero.
- [ ] The page furniture stays in view: dot grid, binding, smudge, coffee ring.
- [ ] The camera moves across the page instead of cutting away.

## Files

- `docs/style-guide.md`
- `docs/style-prompt.md`
- `remotion/CATALOG.md`
- `remotion/AGENT_BRIEF.md`
- `style.json`: the manifest (canvas, engine list, doc paths, audio roots)
- `sample.png`: one representative frame
