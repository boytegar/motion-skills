---
name: motion-build
description: Build and render a papercut episode from a storyboard with the library. Scaffolds the project, builds scenes with parallel agents, assets and audio, renders the mp4, then harvests reusables. Use for "build the reel", "scaffold an episode", "rebuild scene 4", "render it".
---

# Papercut build

Build every episode on the library instead of writing a Remotion project from scratch. Read `$MOTION_LIB/core/README.md` once.

## Preflight

- Install: **Standard** or Full. Needs the library, node 20+, npm, ffmpeg, Python tools (numpy, Pillow, soundfile) and Chrome or Chromium for the audio render. Probe with the commands in `motion-explainer`; if something is missing, run `./install.sh --doctor` and hand over to `motion-setup`.
- Library: `$MOTION_LIB`, else the `MOTION_LIB=` line in `~/.config/motion-explainer/config.env`, else ask once. With no library at all, build from the sketches in `$MOTION_LIB/style/$MOTION_STYLE/remotion/RECIPES.md`.
- Load the Remotion best-practices skill first (`remotion-best-practices`, from the Remotion plugin, or `npx skills add remotion-dev/skills -g -y -a <your harness id>`; `install.sh --mode standard` does this) and follow its routing for markup, captions and the Studio.
- Needs `video/STORYBOARD.md` and `narration/script.json` from the earlier stages.
- Smoke test of the whole toolchain: `$MOTION_LIB/core/tools/selftest.sh` builds `examples/hello-reel/` in a throwaway folder and renders one still.

## 1. Scaffold & Verify Engine

The episode project should already be scaffolded from the intake / `motion-strategy` stage. Verify `video/episode.json` for the recorded `engine`:

- If not yet scaffolded, ask the user (`fframes` or `remotion`) and run:
  ```
  $MOTION_LIB/new_episode.sh <series-dir> NN-slug CompId [out-name] --engine fframes|remotion
  ```
- Preview commands from `video/`:
  - **fframes**: `cargo run --release -- preview` (or install CLI: `cargo install --locked cargo-fframes`)
  - **remotion**: `npx remotion studio` (or `npm run dev`)

- **fframes**: episode project is a Rust crate (`media/`, `src/main.rs`). Render with `cargo run --release -- render` or `$MOTION_LIB/core/tools/render.sh`.
- **remotion**: React project (`src/`, `package.json`). Render with `npx remotion render <CompId> out/<out>.mp4` or `render.sh`.

## 2. Plan: `episode.json`, voice, timing

From the storyboard, fill `video/episode.json`:
- `scenes`: `[["S01Hook", "l1_hook", 0], ["S02Name", "l2_name", "called"], ...]` (scene id, narration line id, first word as an index or the exact word).
- `banners`: `{"S01Hook": "TAKEAWAY IN CAPS"}`. `hook`: see `motion-hook`. `cuts`: the Camera / Cut table's cut column. `bed`, `whipScene`, `cutSfx`, `gaps` (extra beat after a line, in seconds).

Then run `motion-voice` through its timing step (it ends with `build_vo.py` and `timing_md.py`). Then, from `video/`:

```
python3 $MOTION_LIB/core/tools/stubs.py        # generates multi-file scene stubs based on engine
```

`build_vo.py` flags any scene under 1.2 s; merge it into a neighbour. `timing_md.py` writes the Timing section of `STORYBOARD.md` (absolute frames and word cues), so agents never open `plan.json`.

## 3. Assets

Follow `$MOTION_LIB/style/$MOTION_STYLE/docs/asset-prompts.md`: an image tool (anything that outputs transparent PNGs) for backgrounds and complex static props, code for characters, all text, tags, gauges, buttons and UI. No text inside generated images. With no image tool, build every prop in code with the paper filter; the style holds up. Backdrops: use the library's `<Backdrop>` set (`style/$MOTION_STYLE/remotion/public/bg/backdrops/_sheet.jpg`); generate a new one only when an episode needs a place the library lacks. The proof screenshot: `node $MOTION_LIB/core/tools/capture.mjs <url> public/proof/x.png`, shown with the kit's `RealShot`.

## 4. Scenes (Strict Multi-file Architecture)

**NEVER put all scenes in one file.** Each scene must live in its own isolated file to allow parallel agent work, independent testing, and clean git diffs.

Check `engine` from `video/episode.json`:

### For Remotion (`engine: "remotion"`)
- **File Structure**:
  - `src/scenes/<Scene>.tsx` (e.g. `S01Hook.tsx`, `S02Name.tsx`) — exactly **one file per scene**.
  - `src/scenes/<Scene>.parts.tsx` (optional) — for private sub-components of that scene only.
  - `src/scenes/index.ts` — registry array `SCENES` automatically managed by `stubs.py`. Never inline scene bodies here.
1. Build `S01Hook` yourself first in `src/scenes/S01Hook.tsx`, from the HOOK block. It is the reference scene the agents match (cue constants, the camera, clamped `interpolate`).
2. Hand the rest to scene agents in parallel, grouped by scene (2-3 scenes each). The brief is `style/$MOTION_STYLE/remotion/AGENT_BRIEF.md`.
3. File ownership: an agent writes only `src/scenes/<Scene>.tsx`, an optional `<Scene>.parts.tsx` and `tools/cue_sheet.d/<Scene>.json`. Never edit other scenes or `index.ts`.
4. Agents read `style/$MOTION_STYLE/remotion/CATALOG.md` and `$MOTION_LIB/core/audio/CATALOG.md`.

### For fframes (`engine: "fframes"`)
- **File Structure**:
  - `src/scenes/<scene_snake>.rs` (e.g. `s01_hook.rs`, `s02_name.rs`) — exactly **one file per scene**, defining its struct and `impl Scene for <Scene>`.
  - `src/scenes/mod.rs` — automatically registers all scene modules via `stubs.py` (`pub mod s01_hook; pub use s01_hook::*;`).
  - `src/main.rs` / `lib.rs` — only mounts the scenes via `Video::define_scenes()`. Never write scene drawing code inside `main.rs`.
1. Build `s01_hook.rs` first as the reference scene.
2. Hand remaining scenes to agents in parallel (each agent owns only `src/scenes/<scene_snake>.rs`).
3. Read `style/$MOTION_STYLE/fframes/src/` for available camera, set, and prop structs instead of TSX components.

Build rules: 1080x1920, 30 fps, scenes chained in the kit's transition series. Global layers: VO and music `<Audio>`, the taped banner with `<Boil>`, `<WordTagCaptions>`, `<GrainOverlay>`. Every prop is wrapped in `<Paper>`, entrances use `<PopIn>`, each scene runs inside its storyboard camera. No CSS animations. Keep the bottom 20% clear. Neighbouring scenes never share a backdrop. The default hero is a boxy orange critter (token `C.hero` `#D67354`, shade `C.heroLo` `#A9553B`, alias `you`); accessories and name pills tell the cast apart.

## 5. Sound

Pick SFX by name from `$MOTION_LIB/core/audio/sfx/*.js` and the bed by preset from `$MOTION_LIB/core/audio/beds/*.json`. From `video/`:

```
python3 $MOTION_LIB/core/tools/cues.py && node $MOTION_LIB/core/audio/render.mjs
```

`cues.py` merges the agents' rows into `tools/cues.json` and prints a note if air sounds exceed one per 2.5 s. `render.mjs` writes `public/audio/music.wav` and `fx.wav`.

## 6. Render

Preview: `cargo run --release -- preview` (fframes) or `npx remotion studio` (remotion). Render only when asked, always through the wrapper — it picks the engine from `episode.json`:

```
$MOTION_LIB/core/tools/render.sh        # -> out/<out>.mp4
```

Flags: `--workers N` (default 3), `--conc N`, `--force`, `--keep`. Each scene is cached, so after a fix only the changed scene re-renders; a change to the kit, plan, voice or captions re-renders everything. The first run downloads Remotion's headless Chrome, and `out/cache` takes several GB of ProRes.

To fix or rebuild one scene: edit `src/scenes/<Scene>.tsx`, check it with `motion-qa`, then run `render.sh` again.

## 7. Harvest (after the episode)

```
python3 $MOTION_LIB/core/tools/harvest.py .       # from video/; read-only report
python3 $MOTION_LIB/core/tools/catalog.py         # regenerate both catalogs
```

Promote kit fixes, reusable components, new SFX and recurring beats as `library/HARVEST.md` describes, and add an entry to `library/CHANGELOG.md`.

- When a move looks stiff or muddy, check `$MOTION_LIB/style/$MOTION_STYLE/docs/motion-principles.md` for the matching principle and its kit helper.

## Hand-off

Next is `motion-qa`, which reads stills from the project and `STORYBOARD.md`. Run it per scene while building and once on the whole reel before the final render. After posting, `motion-strategy` runs the autopsy.
