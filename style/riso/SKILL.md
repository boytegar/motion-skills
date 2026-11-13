---
name: riso
description: Render a reel in the Risograph style: a three-ink risograph print on cream paper with halftone screens at their own angles, overprints instead of mixed colours, plates a few pixels off register, and ink dropouts that sparkle in the darks. Use when the user says risograph, riso print, screen print, halftone, overprint, zine, indie print, or asks for a worn bright printed-poster look.
---

Three-ink risograph print on cream paper: halftone screens, overprints, misregistration and ink dropouts on 2s.

This is one style of the motion-explainer library. It ships the same three things every style ships: a `style.json` manifest, a Remotion engine, and an fframes engine. The pipeline skills (`motion-explainer`, `motion-script`, `motion-storyboard`, `motion-build`, `motion-qa`) all take the style name as `$MOTION_STYLE`, so nothing else in the workflow changes when you pick this one.

## When to use it

Reach for this style when the brief matches its DNA below. Read `docs/style-guide.md` first: it is the authority, this file is the index.

## Style DNA

1. Nothing is painted, everything is printed. A frame is a sheet of paper run through three spot-ink plates.
2. A plate only says how much ink goes where, as a tone from 0 to 1; the screen turns that tone into dots at the plate's own angle.
3. Inks multiply. There is no black ink and no white ink.
4. Plates sit a few pixels off register, deliberately.
5. Ink dropouts sparkle in the dark areas. Motion is on twos.

Palette: paper `#FFFDF5`, pink `#FF48B0`, blue `#0078BF`, yellow `#FFE800`; every other colour is an overprint of these three

## Kit components

Remotion, importable from `../lib/<file>` (the kit is synced into each project's `src/lib/`):

| component | what it is |
|---|---|
| `<RisoPlate ink dx dy opacity>` | one spot-ink plate, multiplied over the ones below it |
| `<HalftoneDotFilter>` | the halftone dot pattern used to screen a plate's tone |
| `<RisoBanner text>` | the overprinted banner |
| `<RisoCaptions text>` | the overprinted caption strip |

## Nothing boils in a vacuum

docs/style-guide.md lists the plate angles, the screen frequencies and the registration offsets. Read it before composing any colour.

## Engines

| engine | path | entry |
|---|---|---|
| remotion | `remotion/` | `template/`, `kit/`, `sync_kit.sh`, `CATALOG.md`, `AGENT_BRIEF.md` |
| fframes | `fframes/` | `Cargo.toml`, `src/` (lib, tokens, kit, look, video, main), `tools/ff.sh` |

Registered engines: `remotion`, `fframes`.

## Scaffold an episode

```sh
# from the library root, with MOTION_LIB resolved
./core/new_episode.sh <series-dir> 01-slug Episode out-name --style riso --engine remotion
./core/new_episode.sh <series-dir> 01-slug Episode out-name --style riso --engine fframes
```

Or name the style in a series `SERIES.md` / `episode.json` as `"style": "riso"` and let `motion_style_of` pick it up.

## QA for this style

Check these on every contact sheet before calling a scene done; the rest of the checklist in `motion-qa` still applies.

- [ ] Every colour on screen is an overprint of the three inks.
- [ ] Shading is a halftone screen at a plate angle, never a gradient.
- [ ] Plates are visibly a few pixels off register.
- [ ] There is no pure black and no pure white anywhere.
- [ ] Motion is on twos.

## Files

- `docs/style-guide.md`
- `docs/style-prompt.md`
- `remotion/CATALOG.md`
- `remotion/AGENT_BRIEF.md`
- `style.json`: the manifest (canvas, engine list, doc paths, audio roots)
- `sample.png`: one representative frame
