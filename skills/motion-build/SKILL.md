---
name: motion-build
description: Turn a finished storyboard and voice timing into a build-ready plan folder, then stop. Finalises episode.json, builds the scene plan, the caption track and the sound cue list, writes the HANDOFF.md contract, and ends the pipeline. Engine-agnostic: it names no renderer and runs none. Use for "finalise the plan", "build the handoff", "re-plan scene 4", "what does the builder need".
---

# Plan and handoff

This stage ends the planning pipeline. It produces every file a builder needs and then **stops**: it writes no scene code, runs no renderer, and produces no still. Building the folder belongs to the user's engine, outside this skill set.

Read `$MOTION_LIB/core/README.md` once for what the shared tools do.

## Preflight

- Install: **plan + audio files** for the caption track and the cue list (ffmpeg, python3, numpy/Pillow/soundfile). Probe with the commands in `motion-explainer`; if something is missing, run `./install.sh --doctor` and hand over to `motion-setup`.
- Library: `$MOTION_LIB`, else the `MOTION_LIB=` line in `~/.config/motion-explainer/config.env`, else ask once.
- Needs `video/STORYBOARD.md`, `narration/script.json`, `narration/timing_narrator.js` and the per-line WAVs from the earlier stages.
- No renderer is loaded, installed or probed here. If the user has none, that is not a blocker: say so in one line in `HANDOFF.md`.

## 1. Verify the episode folder

The folder was created by `motion-strategy` and holds no renderer:

```
episodes/NN-slug/
  research/facts.md
  narration/script.json  narration/voice/final_narrator/*.wav  narration/timing_narrator.js
  video/STORYBOARD.md  video/episode.json  video/src/  video/public/vo/  video/tools/
```

If a piece is missing, name the stage that owns it and stop: `motion-strategy` for the folder and BRIEF, `motion-hook` for the HOOK block, `motion-script` for `script.json` and the staging table, `motion-storyboard` for the Camera/Cut table and scene blocks, `motion-voice` for the takes and `timing_narrator.js`. Never fill a gap by guessing.

## 2. Fill `episode.json`

From `STORYBOARD.md`, into `video/episode.json`:

- `scenes`: `[["S01Hook", "l1_hook", 0], ["S02Name", "l2_name", "called"], ...]` — scene id, narration line id, and the first word as an index or the exact word.
- `banners`: `{"S01Hook": "TAKEAWAY IN CAPS"}`, one per scene, from the staging table.
- `hook`: from the HOOK block, see `motion-hook`.
- `cuts`: `{"S04Name": {"kind": "tearAway"}}` from the Camera/Cut table (no entry means a hard cut).
- `bed`, `cutSfx`, `gaps` (an extra beat after a named line, in seconds).

Keep `style`, `width`, `height`, `fps` and `comp` as they are. **Never write an `engine` key**: the plan is engine-neutral, and the choice is recorded only in `HANDOFF.md`.

## 3. Build the plan files

From `video/`:

```
python3 $MOTION_LIB/core/tools/build_vo.py     # public/vo/voiceover.wav, public/vo/captions.json, src/plan.json
python3 $MOTION_LIB/core/tools/timing_md.py    # the Timing section of STORYBOARD.md
```

Then write the per-scene SFX rows from the storyboard's `SFX:` lines into `tools/cue_sheet.d/<Scene>.json` as `[word_prefix, nth_match, sfx_name, offset_frames]`, and merge them:

```
python3 $MOTION_LIB/core/tools/cues.py         # tools/cues.json
```

`build_vo.py` flags any scene under 1.2 s: merge that scene into a neighbour and rerun rather than letting the builder cope with it. `cues.py` prints a note when air sounds exceed one per 2.5 s; treat it as a note, fix the loud spots, and keep the count under budget.

The plan is now complete and engine-independent: `src/plan.json` (scene frames, word cues, duration), `public/vo/captions.json` (one entry per word, milliseconds), `public/vo/voiceover.wav`, `tools/cues.json` (every sound cue with its time).

## 4. Write `HANDOFF.md`

The contract the builder reads. It is a description of the folder, not a recommendation: you record whatever engine the user already works with, and you never pick one for them. Ask once, in one line, if you don't know it, then write the file by hand in this shape:

```md
# HANDOFF

## Ask for
`@<engine>` with this folder as the working directory: build each scene from `src/plan.json`,
mix `tools/cues.json`, render to `out/`.

## Canvas
- Style, size, fps, total frames, scene count, hook title frames.

## Scenes
| Scene | Line | From (f) | Dur (f) | Dur (s) | Cut in | Banner |

## Files
| Path | What |   (mark anything missing)

## Sound
Bed preset, cue list path, the SFX and bed catalogs.

## Rules the build must keep
- Scene boundaries are frames in `src/plan.json`; never retime a scene by editing a shape.
- One caption tag per word, from `public/vo/captions.json`.
- Every sound fires at a cue time from `tools/cues.json`.
- The storyboard's camera and cut per scene are the plan, not a suggestion.
```

Name exactly one engine, the one the user says they use — for example `@remotion`, `@fframes` or `@kyfram`. If they have none, write `Ask for: an engine skill` and say so in the reply. Do not compare engines, do not suggest one, do not scaffold for one.

## 5. Stop

Report the folder, the scene count, the total duration, the cue count, and the engine named in `HANDOFF.md`. Do not scaffold, do not preview, do not render.

## Gate

- [ ] `episode.json` has scenes, banners, hook and cuts, and no `engine` key.
- [ ] `src/plan.json`, `public/vo/captions.json`, `public/vo/voiceover.wav` and `tools/cues.json` all exist.
- [ ] No scene under 1.2 s; air sounds within budget.
- [ ] `HANDOFF.md` names one engine and lists every file, marking any gap.
- [ ] No scene code written, no renderer run, no still claimed.
- [ ] No engine recommended, compared or installed.

## Hand-off

This is the end of the plan. The engine named in `HANDOFF.md` owns scaffolding, scene code, assets that need its runtime, the render and every visual QA pass. After the video is posted, `motion-strategy` runs the autopsy.
