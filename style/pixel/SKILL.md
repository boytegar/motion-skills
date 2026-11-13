---
name: pixel
description: Render a reel in the Pixel Art style: every picture drawn at a low internal resolution such as 90x160 or 135x240 and upscaled nearest-neighbour to 1080x1920, hard square pixels, ordered Bayer dithering instead of gradients, a 5x7 bitmap font, sprites from string arrays, and device effects such as an LCD grid, scanlines or a curved-tube vignette. Use when the user says pixel art, 8-bit, 16-bit, retro game, arcade, CRT, DOS, or asks for a chronology where resolution tells time.
---

Low-res pixel art drawn at native raster grids and upscaled nearest-neighbour to 1080x1920.

This is one style of the motion-explainer library. It ships the same three things every style ships: a `style.json` manifest, a Remotion engine, and an fframes engine. The pipeline skills (`motion-explainer`, `motion-script`, `motion-storyboard`, `motion-build`, `motion-qa`) all take the style name as `$MOTION_STYLE`, so nothing else in the workflow changes when you pick this one.

## When to use it

Reach for this style when the brief matches its DNA below. Read `docs/style-guide.md` first: it is the authority, this file is the index.

## Style DNA

1. Draw at a low internal resolution that divides 1080x1920 exactly, so every pixel is a whole square on screen.
2. Ordered Bayer dithering instead of gradients; a limited palette; no anti-aliasing anywhere.
3. A built-in 5x7 bitmap font for every glyph; no system fonts.
4. Sprites are string arrays with a palette key, not image files.
5. Device effects belong to the hardware: LCD grid for a handheld, scanlines and tube vignette for an arcade.
6. Resolution can tell time: an early era is chunkier than a later one.

Palette: 16-colour PICO-8-style set: black `#000000`, dark blue `#1D2B53`, dark purple `#7E2553`, dark green `#008751`, brown `#AB5236`, dark grey `#5F574F`, light grey `#C2C3C7`, white `#FFF1E8`, red `#FF004D`, orange `#FFA300`, yellow `#FFEC27`, green `#00E436`, blue `#29ADFF`, indigo `#83769C`, pink `#FF77A8`, peach `#FFCCAA`

## Kit components

Remotion, importable from `../lib/<file>` (the kit is synced into each project's `src/lib/`):

| component | what it is |
|---|---|
| `<CrtScanlines>` | the scanline and vignette post pass |
| `<PixelBox x y w h bg border>` | a hard-edged pixel panel with a stacked drop shadow |
| `<PixelBanner text>` | the chunky banner bar |
| `<PixelCaptions text>` | the bitmap caption strip |

## Nothing boils in a vacuum

docs/style-guide.md carries the resolution table and the dithering recipes. Pick the internal resolution before drawing a single sprite.

## Engines

| engine | path | entry |
|---|---|---|
| remotion | `remotion/` | `template/`, `kit/`, `sync_kit.sh`, `CATALOG.md`, `AGENT_BRIEF.md` |
| fframes | `fframes/` | `Cargo.toml`, `src/` (lib, tokens, kit, look, video, main), `tools/ff.sh` |

Registered engines: `remotion`, `fframes`.

## Scaffold an episode

```sh
# from the library root, with MOTION_LIB resolved
./core/new_episode.sh <series-dir> 01-slug Episode out-name --style pixel --engine remotion
./core/new_episode.sh <series-dir> 01-slug Episode out-name --style pixel --engine fframes
```

Or name the style in a series `SERIES.md` / `episode.json` as `"style": "pixel"` and let `motion_style_of` pick it up.

## QA for this style

Check these on every contact sheet before calling a scene done; the rest of the checklist in `motion-qa` still applies.

- [ ] Every pixel is a whole square; no fractional or anti-aliased edges.
- [ ] The internal resolution divides 1080x1920 exactly.
- [ ] Shading is dithered, never a smooth gradient.
- [ ] All type uses the bitmap font.
- [ ] Device effects match the hardware the scene is set on.

## Files

- `docs/style-guide.md`
- `docs/style-prompt.md`
- `remotion/CATALOG.md`
- `remotion/AGENT_BRIEF.md`
- `style.json`: the manifest (canvas, engine list, doc paths, audio roots)
- `sample.png`: one representative frame
