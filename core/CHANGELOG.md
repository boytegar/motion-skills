# Changelog

## Unreleased

Kyfram engine (third engine, next to `remotion` and `fframes`).

- `core/kyfram/lgv/core.lgv`: style-free helpers (`textW`, `pad`, `clamp`, `popText`, `popPlate`,
  `drawTag`, `drawCaptionRow`, `pushIn`).
- `core/kyfram/README.md`: how an episode compiles, and the DSL traps the render is built around.
- `core/tools/render_kyfram.py`: one `.lgv` per scene, cached per scene, concat + one x264 encode,
  `--voice` mux. The scene header (`SCENE_ID`, `BANNER`, `DUR`, `FPSR`, `CAPS`, `scene()`) is generated
  from `episode.json` + `src/plan.json`, and a frame count that disagrees with the plan fails the render.
- `core/tools/kyfram_header.py`, `kyfram_stubs.py`, `kyfram_qa.py`: header builder, scene stubs
  (also reached through `stubs.py`), and QA stills lifted from cached segments (`kyfram` only writes
  a still at t=0).
- `core/tools/render.sh`, `qa.sh`, `new_episode.sh`, `stubs.py`: dispatch on `"engine": "kyfram"`.
- `skills/kyfram/`: the DSL skill, with the measured traps (colour does not hold across keyframes,
  `..` is not concat, arity panics) and `references/kyfram-upstream-SKILL.md` vendored.
- `style/papercut/kyfram/PRELUDE.lgv`: papercut tokens and furniture for `.lgv` scenes;
  `style/papercut/style.json` lists `kyfram`; `docs/style-prompt.md` gains block A2.
- `style/<every style>/kyfram/PRELUDE.lgv`: all ten styles ship a prelude, so `--engine kyfram`
  works with any of them. The nine without hand-authored furniture get a minimal one built from
  that style's real `fframes/src/tokens.rs` palette (`drawBackdrop`, `drawBanner`, `drawCaptions`,
  `drawPanel`, `drawDot`, `drawFrame`, plus `CANVAS_W/H`, `BANNER_Y`, `CAPTION_Y`). `drawFrame`
  exists because five of those palettes have `C_SURFACE == C_BG` — a panel fill would be invisible
  against the ground — so it strokes a `polygon` outline (kyfram `rect` has no stroke). No style
  TTFs ship outside papercut, so those preludes use kyfram's built-in bold sans, not a `font:` path.
- `install.sh`: `--doctor` and `step_deps` check `kyfram`; fixed `LIB` pointing at a `library/`
  folder that does not exist, the `papercut_find_chrome` name, and the unbound `PAPERCUT_PY`.
  `--doctor` now counts every style's PRELUDE.lgv, not just papercut's.

## 0.1.0: first public release

What ships.

- Kit (`papercut/kit/`): paper primitives, set builders, a UI kit, a default critter, props sets, `tokens.ts`, local fonts and the `CtaPhoneDM` scene template. `papercut/CATALOG.md` lists every export with its props.
- 24 backdrops (`papercut/public/bg/backdrops/`, `<Backdrop name="..." />`), each with a floor line and a calm band for the banner and captions. The sources of the eight code-drawn ones are in `papercut/backdrops-src/`.
- Camera moves (`paper/camera.tsx`): orbit with depth layers, foreground pass, rack focus, snap zoom, pull-back, freeze frame, dolly zoom, crab track, crane, POV masks, time ramp, dutch tilt, tilt shift, focus pulse, focus hunt and handheld.
- Scene transitions (`paper/transitions.tsx`, `<CutSeries>`): whip, zoom-through, page turn, tear-away, tape pull, stamp cut, infinite zoom, pop-up book, keyhole and burn-away.
- Film-look layers (`paper/look.tsx`): gate weave and flicker, moving felt fibre, lens dirt, lamp flicker, halftone and graph paper.
- Audio: 79 sound effects (one file each), 8 music beds (marimba and kalimba presets plus a rain-and-piano night bed), a Web Audio engine and an offline renderer.
- Tools: scaffold (`new_episode.sh`), kit sync, scene stubs, voiceover and frame-plan builder, SFX cue builder, cached per-scene render, audio mix, QA contact sheets, page capture, catalog generator, harvest report, placeholder narration and a self-test.
- Optional voice pipeline (`voice/`): Qwen3-TTS narration, take picking, pace and loudness pass, word timing, with a setup script.
