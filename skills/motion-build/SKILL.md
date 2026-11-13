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

## 1. Scaffold

```
$MOTION_LIB/new_episode.sh <series-dir> NN-slug CompId [out-name]
```

It creates `<series>/episodes/NN-slug/{research,narration/voice,video}` with the template project and a synced copy of the kit in `src/lib/`. `node_modules` comes from `$DONOR`, else the newest sibling episode, else `npm i`. If `<series>/video-assets/` exists, it is laid over the default textures and props.

The folder may already exist, because the earlier stages wrote `research/`, `narration/script.json` and `video/STORYBOARD.md` into it. The script merges into it and never overwrites a file that is there. It stops only if `video/package.json` exists (already scaffolded).

Never edit `src/lib/` in a project. It is a snapshot, so an old episode renders the same after the library changes. Fix the library, then re-sync with `video/sync_kit.sh`.

## 2. Plan: `episode.json`, voice, timing

From the storyboard, fill `video/episode.json`:
- `scenes`: `[["S01Hook", "l1_hook", 0], ["S02Name", "l2_name", "called"], ...]` (scene id, narration line id, first word as an index or the exact word).
- `banners`: `{"S01Hook": "TAKEAWAY IN CAPS"}`. `hook`: see `motion-hook`. `cuts`: the Camera / Cut table's cut column. `bed`, `whipScene`, `cutSfx`, `gaps` (extra beat after a line, in seconds).

Then run `motion-voice` through its timing step (it ends with `build_vo.py` and `timing_md.py`). Then, from `video/`:

```
python3 $MOTION_LIB/video/stubs.py        # src/scenes/index.ts + a stub per scene
```

`build_vo.py` flags any scene under 1.2 s; merge it into a neighbour. `timing_md.py` writes the Timing section of `STORYBOARD.md` (absolute frames and word cues), so agents never open `plan.json`.

## 3. Assets

Follow `$MOTION_LIB/style/$MOTION_STYLE/docs/asset-prompts.md`: an image tool (anything that outputs transparent PNGs) for backgrounds and complex static props, code for characters, all text, tags, gauges, buttons and UI. No text inside generated images. With no image tool, build every prop in code with the paper filter; the style holds up. Backdrops: use the library's `<Backdrop>` set (`style/$MOTION_STYLE/remotion/public/bg/backdrops/_sheet.jpg`); generate a new one only when an episode needs a place the library lacks. The proof screenshot: `node $MOTION_LIB/core/tools/capture.mjs <url> public/proof/x.png`, shown with the kit's `RealShot`.

## 4. Scenes

1. Build `S01Hook` yourself first, from the HOOK block. It is the reference scene the agents match (cue constants, the camera, clamped `interpolate`).
2. Hand the rest to scene agents in parallel, grouped by scene (2-3 scenes each; more agents cost more startup context). Use the `motion-scene` agent that `install.sh` wrote for your harness (Claude Code `~/.claude/agents`, OpenCode `~/.config/opencode/agents`, Codex `~/.codex/agents`, Gemini CLI `~/.gemini/agents`); a general agent also works. Spawn it with your harness's own mechanism (the Task tool in Claude Code, the subagent tool in opencode, Codex's subagents, Gemini's agent tool). **If your harness has no subagents, build the scenes yourself one at a time**, in the same order and with the same brief; it is slower, not different. The brief is `style/$MOTION_STYLE/remotion/AGENT_BRIEF.md`. The spawn message gives the project path, each agent's scenes with absolute frame ranges, a QA folder name and the value of `$MOTION_LIB`.
3. File ownership: an agent writes only `src/scenes/<Scene>.tsx`, an optional `<Scene>.parts.tsx` and `tools/cue_sheet.d/<Scene>.json`. Never `src/lib/**`, other scenes, `index.ts`, `plan.json`, `episode.json` or the storyboard.
4. Agents read `style/$MOTION_STYLE/remotion/CATALOG.md` and `$MOTION_LIB/core/audio/CATALOG.md`, not the kit source, and reuse components, prop sets and scene templates (`kit/scenes/`, for example `CtaPhoneDM` for the CTA) before drawing anything new.

Build rules: 1080x1920, 30 fps, scenes chained in the kit's transition series. Global layers: VO and music `<Audio>`, the taped banner with `<Boil>`, `<WordTagCaptions>`, `<GrainOverlay>`. Every prop is wrapped in `<Paper>`, entrances use `<PopIn>`, each scene runs inside its storyboard camera. No CSS animations. Keep the bottom 20% clear. Neighbouring scenes never share a backdrop. The default hero is a boxy orange critter (token `C.hero` `#D67354`, shade `C.heroLo` `#A9553B`, alias `you`); accessories and name pills tell the cast apart.

## 5. Sound

Pick SFX by name from `$MOTION_LIB/core/audio/sfx/*.js` and the bed by preset from `$MOTION_LIB/core/audio/beds/*.json`. From `video/`:

```
python3 $MOTION_LIB/core/tools/cues.py && node $MOTION_LIB/core/audio/render.mjs
```

`cues.py` merges the agents' rows into `tools/cues.json` and prints a note if air sounds exceed one per 2.5 s. `render.mjs` writes `public/audio/music.wav` and `fx.wav`.

## 6. Render

Preview in the Studio (`npx remotion studio`). Render only when asked:

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
