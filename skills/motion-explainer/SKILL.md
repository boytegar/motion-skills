---
name: motion-explainer
description: Plan a short vertical video, reel or explainer end to end, from a link (repo, product page, announcement) to a build-ready plan folder. Writes the brief, hook, script, storyboard, voice timing and handoff contract, then stops. Engine-agnostic: it names no renderer and runs none. Also routes partial asks to the stage skills and hands out the style prompt for other tools.
---

# Motion Explainer

A ~20-45 s vertical explainer in whichever visual style the series picked (`$MOTION_STYLE`, e.g. layered construction paper: grainy props with drop shadows, rigid cut-out animation, a slow camera push with stop-motion boil, a taped headline banner per beat and word-by-word caption tags).

Every beat runs three tracks:
- the voiceover tells the story
- the taped banner adds a hard fact the voiceover skips
- a fresh set illustrates the voiceover's words, with a rough rebuild of the product UI doing the demo

Reuse the style and the storytelling, not the worked examples' content. Every product, company and number in the examples is invented. The style rules are in `$MOTION_LIB/style/$MOTION_STYLE/docs/style-guide.md` (section 2 is the Style DNA).

**This skill set plans.** Every stage up to the handoff writes files into one episode folder. Drawing, rendering and pixel QA belong to the engine that builds the folder, and are out of scope here.

## Preflight

- **Library.** Resolve `$MOTION_LIB`, else the `MOTION_LIB=` line in `${XDG_CONFIG_HOME:-~/.config}/motion-explainer/config.env`, else ask once. Never hard-code a machine path. Stages 1 to 4 need no library.
- **Install mode.** Probe it before promising a stage (see Planning modes below).
- **Series.** Find the series folder (it holds `episodes/`). Read `agents.md` first when present (the chosen style), then `SERIES.md` (audience, voice, palette). If `agents.md` also names an engine, that is a note about the user's tooling: record it in `HANDOFF.md` when you get there, and let it change nothing in the plan.

## Run modes (ask first)

1. **Full plan**, idea to a build-ready folder: stages 1 to 6, then the handoff.
2. **Script only**: strategy, hook, script (stop after the staging table).
3. **Storyboard from an existing script**: `motion-storyboard`. Ask for the script, the facts and the HOOK block.
4. **Style prompt for another tool**: give the right block from `$MOTION_LIB/style/$MOTION_STYLE/docs/style-prompt.md` and stop.
5. **Re-plan or re-check one scene**: `motion-storyboard` for the block, then `motion-build`.

## Intake (ask only what is missing)

- **Source**: usually a link (repo, product site, announcement). Everything else is researched from it.
- **The one claim** the video proves, and the topic.
- **Role framing**: who or what does the work, who advises or approves. Confirm it with the user and never invent product behaviour.
- **Style**: read the series `agents.md` first when it exists — it is the initialized source of truth and must not be asked again. Else ask which style folder to plan for.
- **CTA keyword** and what the viewer gets (the deliverable).
- **Length**: default 22-30 s.
- **Audience level** (from `SERIES.md`) and any accent colour or mascot.

Write the answers as the BRIEF block at the top of `episodes/NN-slug/research/facts.md` (format in `motion-strategy`) and create the episode folder immediately (`motion-strategy`, step 5).

## Stages

| # | Stage | Skill | Reads | Writes |
|---|---|---|---|---|
| 1 | Strategy & folder | `motion-strategy` | `SERIES.md`, the source | BRIEF in `research/facts.md`, the episode folder tree |
| 2 | Hook | `motion-hook` | BRIEF | HOOK block at the top of `video/STORYBOARD.md` |
| 3 | Script | `motion-script` | BRIEF, HOOK | `research/facts.md`, `narration/script.json`, staging table in `video/STORYBOARD.md` |
| 4 | Storyboard | `motion-storyboard` | `script.json`, `STORYBOARD.md` | Camera/Cut table, scene blocks, SFX plan in `video/STORYBOARD.md` |
| 5 | Voice and timing | `motion-voice` | `script.json`, `voice/voice.json` | `narration/` takes, `public/vo/captions.json`, `src/plan.json`, Timing section |
| 6 | Plan and handoff | `motion-build` | `plan.json`, `STORYBOARD.md`, `episode.json` | `video/episode.json`, `tools/cues.json`, `video/HANDOFF.md` |
| 7 | Build, render, QA | out of scope: whatever the user's engine is | the episode folder | scenes, mp4, fixes |
| 8 | After posting | `motion-strategy` (autopsy) | platform numbers | `research/analytics/`, `SERIES.md` |

Stages 1 to 6 end the plan. Nothing here builds, previews, renders or QA-passes pixels. The set does ship one engine-side skill, `motion-qa`: a checklist and delivery summary to run after an engine renders. It is not a stage and never runs during planning.

## After the plan: handoff

Stage 6 ends by writing `video/HANDOFF.md`, a contract that asks for an engine by name and says what the folder needs. The plan itself is engine-free: nothing in it imports, installs or configures a renderer, and no stage picks one for the user.

`HANDOFF.md` names whatever the user already has, e.g. `@remotion` (React/TypeScript, one TSX per scene), `@fframes` (a Rust crate) or `@kyfram` (Logos `.lgv`) — those are examples, not a menu to choose from. If the user has no engine, write `Ask for: an engine skill` and say so in one line.

The engine skill owns scaffolding, scene code, assets that need its runtime, the render and every visual QA pass. This set writes the plan those steps consume and nothing else. Never improvise a renderer inside a planning stage.

## Project files

The pipeline lives in files, not in chat. Each stage reads the files above and writes its own, so a new session can resume from any stage. One folder per episode:

```
<series>/
  SERIES.md                  optional: audience, voice, palette, mascot, keyword log
  voice/voice.json           optional: voice settings (copy library/voice/voice.example.json)
  video-assets/              optional: overlay textures and props
  research/analytics/        optional: competitor studies and autopsies
  episodes/NN-slug/
    research/facts.md        BRIEF + facts with sources
    narration/
      script.json            [{"id": "l1_hook", "text": "..."}, ...]
      voice/final_narrator/  one WAV per script line
      timing_narrator.js     word times
    video/
      STORYBOARD.md          HOOK, staging table, Camera/Cut table, scene blocks, Timing
      episode.json           scenes, banners, hook, gaps, bed, cuts
      HANDOFF.md             the build contract; names the engine and lists every file
      src/plan.json          scene frames, word cues, duration
      public/vo/             voiceover.wav, captions.json
      tools/cues.json        every sound cue with its time
```

Library catalogs: the style's staging vocabulary and `$MOTION_LIB/core/audio/CATALOG.md` (every SFX and bed). The style catalog sits with the style's own kit (for papercut today, `style/$MOTION_STYLE/remotion/CATALOG.md`). Both are generated, so a fresh checkout may lack them or have stale ones. Run `python3 $MOTION_LIB/core/tools/catalog.py` to rebuild. Agents read the catalogs, not the kit source.

Reuse before writing: pick SFX and beds by name, and name existing camera moves and transitions in the storyboard instead of inventing new ones.

## Planning modes

- **Plan only** (skills only): strategy, hook, script, storyboard, style prompts, the handoff. Enough for stages 1 to 4 and stage 6.
- **Plan + audio files**: adds ffmpeg and the Python tools, so the placeholder voice, the captions, the scene plan and the sound cue list can be built.
- **Plan + real voiceover**: adds local Qwen3-TTS voiceover and word-timed captions from real audio.

Probe (read-only):

```
. "$(grep '^MOTION_LIB=' "${XDG_CONFIG_HOME:-$HOME/.config}/motion-explainer/config.env" | cut -d= -f2-)/env.sh"   # fills MOTION_PY, QWEN_TTS_PY
ffmpeg -version | head -1
"$MOTION_PY" -c 'import numpy, PIL, soundfile'
"$QWEN_TTS_PY" -c 'import qwen_tts'
```

The first two probing lines passing means **plan + audio files**. The `qwen_tts` import passing as well means **plan + real voiceover**. If a stage needs more than the install provides, say so in one line, run `./install.sh --doctor` from the repo root, and hand over to `motion-setup` instead of improvising. Find the root with `readlink` on the installed skill folder (under `.claude/skills`, `.agents/skills`, `.config/opencode/skills` or another harness's skills folder), or `MOTION_HOME` in the config file.

No render engine ships with any planning mode here, and none is installed by this set. A renderer's dependencies belong to its own skill.

## Rules across stages

- Strategy notes win over style notes: 22-30 s, and no news-framed hooks.
- Parallelise independent work (scene blocks, staging rows, research) with agents in one go.
- Keep the plan engine-neutral: no scene code, no render command, no still that nobody produced.

## Shared references

- `$MOTION_LIB/style/$MOTION_STYLE/docs/style-guide.md`: Style DNA, drawing rules, palette, fonts, texture, camera and jitter, motion timings, captions, sound.
- `$MOTION_LIB/style/$MOTION_STYLE/docs/style-prompt.md`: portable prompts for other agents and generators.
- `$MOTION_LIB/style/$MOTION_STYLE/docs/motion-principles.md`: the 12 principles of animation, with a use-in-each-stage note.
- `$MOTION_LIB/style/$MOTION_STYLE/docs/asset-prompts.md`: what to generate versus build in code, with prompt templates.
- `$MOTION_LIB/style/$MOTION_STYLE/docs/frame-breakdown.md`: a worked ~28 s example at 0.2 s resolution. Copy its event density, not its content.

## Hand-off

Start at stage 1 for a full plan, or at the stage the run mode names. After intake, run `motion-strategy`; it reads `SERIES.md` and the BRIEF and hands to `motion-hook`. Each stage skill names the next one. Stage 6 ends the plan by writing `HANDOFF.md`, and this set stops there: building the folder is the user's engine, chosen outside these skills.
