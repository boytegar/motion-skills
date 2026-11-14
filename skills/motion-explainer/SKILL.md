---
name: motion-explainer
description: Make a papercut reel, video or explainer end to end. Short vertical (9:16) cut-paper explainers from a link (repo, product page, announcement), idea to mp4. Also routes partial asks to the stage skills and hands out the style prompt for other tools.
---

# Papercut Explainer

A ~20-45 s vertical explainer that looks like layered construction paper brought to life: grainy paper props with drop shadows, rigid cut-out animation, a slow camera push with stop-motion paper boil, a taped headline banner per beat and word-by-word torn-paper caption tags. Built with Remotion, optional AI-generated textured assets, and any TTS.

Every beat runs three tracks:
- the voiceover tells the story
- the taped banner adds a hard fact the voiceover skips
- a fresh paper set illustrates the voiceover's words, with a rough papercut rebuild of the product UI doing the demo

Reuse the style and the storytelling, not the worked examples' content. Every product, company and number in the examples is invented. The style rules are in `$MOTION_LIB/style/$MOTION_STYLE/docs/style-guide.md` (section 2 is the Style DNA).

## Preflight

- **Library.** Resolve `$MOTION_LIB`, else the `MOTION_LIB=` line in `${XDG_CONFIG_HOME:-~/.config}/motion-explainer/config.env`, else ask once. Never hard-code a machine path. Stages 1 to 4 need no library.
- **Install mode.** Probe it before promising a stage (see Install modes below).
- **Series.** Find the series folder (it holds `episodes/`). Read its `SERIES.md` if it exists.

## Run modes (ask first)

1. **Full reel**, idea to mp4: stages 1 to 8, then stage 9 after posting.
2. **Script only**: strategy, hook, script (stop after the staging table).
3. **Storyboard from an existing script**: `motion-storyboard`. Ask for the script, the facts and the HOOK block.
4. **Style prompt for another tool**: give the right block from `$MOTION_LIB/style/$MOTION_STYLE/docs/style-prompt.md` and stop.
5. **Fix or rebuild one scene**: `motion-build` (scenes step), then `motion-qa`.

## Intake (ask only what is missing)

- **Source**: usually a link (repo, product site, announcement). Everything else is researched from it.
- **The one claim** the video proves, and the topic.
- **Role framing**: who or what does the work, who advises or approves. Confirm it with the user and never invent product behaviour.
- **Engine**: `fframes` (Rust) or `remotion` (React).
- **CTA keyword** and what the viewer gets (the deliverable).
- **Length**: default 22-30 s.
- **Audience level** (from `SERIES.md`) and any accent colour or mascot.

Write the answers as the BRIEF block at the top of `episodes/NN-slug/research/facts.md` (format in `motion-strategy`) and scaffold the episode project immediately (`new_episode.sh ... --engine <engine>`).

## Stages

| # | Stage | Skill | Reads | Writes |
|---|---|---|---|---|
| 1 | Strategy & Scaffold | `motion-strategy` | `SERIES.md`, the source | BRIEF in `research/facts.md`, scaffolded project, `video/episode.json` |
| 2 | Hook | `motion-hook` | BRIEF | HOOK block at the top of `video/STORYBOARD.md` |
| 3 | Script | `motion-script` | BRIEF, HOOK | `research/facts.md`, `narration/script.json`, staging table in `STORYBOARD.md` |
| 4 | Storyboard | `motion-storyboard` | `script.json`, `STORYBOARD.md`, `episode.json` | Camera/Cut table, scene blocks (engine-specific), SFX plan in `STORYBOARD.md` |
| 5 | Voice and timing | `motion-voice` | `script.json`, `voice/voice.json` | `narration/` takes, `public/vo/captions.json`, `src/plan.json` |
| 6 | Scenes, sound, render | `motion-build` | `plan.json`, `STORYBOARD.md`, `episode.json` | `src/scenes/*` (TSX or Rust), `tools/cues.json`, `out/*.mp4` |
| 7 | QA | `motion-qa` | stills, `STORYBOARD.md` | fixes, the delivery summary |
| 8 | After posting | `motion-strategy` (autopsy), `motion-build` (harvest) | platform numbers, the project | `research/analytics/`, library promotions |

## Project files

The pipeline lives in files, not in chat. Each stage reads the files above and writes its own, so a new session can resume from any stage.

```
<series>/
  SERIES.md                  optional: audience, voice, palette, mascot, keyword log
  voice/voice.json           optional: voice settings (copy library/voice/voice.example.json)
  video-assets/           optional: overlay textures and props
  research/analytics/        optional: competitor studies and autopsies
  episodes/NN-slug/
    research/facts.md        BRIEF + facts with sources
    narration/script.json    [{"id": "l1_hook", "text": "..."}, ...]
    video/
      episode.json           scenes, banners, hook, gaps, bed, cuts
      STORYBOARD.md          HOOK, staging table, Camera/Cut table, scene blocks
```

Library catalogs: `style/$MOTION_STYLE/remotion/CATALOG.md` (every kit component) and `$MOTION_LIB/core/audio/CATALOG.md` (every SFX and bed). They are generated, so a fresh checkout may lack them or have stale ones. Run `python3 $MOTION_LIB/core/tools/catalog.py` to rebuild. Agents read the catalogs, not the kit source.

Reuse before writing: a kit component, a props set or a scene template beats new drawing code. Pick SFX and beds by name.

## Install modes

- **Minimal** (skills only): strategy, hook, script, storyboard, style prompts. No rendering.
- **Standard**: Minimal + build and render with a placeholder voice (silent, or macOS `say`) via `$MOTION_LIB/core/tools/placeholder_vo.py`, stills QA, SFX and bed audio, the full mp4.
- **Full**: Standard + local Qwen3-TTS voiceover and word-timed captions from real audio.

Probe (read-only):

```
. "$(grep '^MOTION_LIB=' "${XDG_CONFIG_HOME:-$HOME/.config}/motion-explainer/config.env" | cut -d= -f2-)/env.sh"   # fills MOTION_PY, QWEN_TTS_PY
node -v; ffmpeg -version | head -1
"$MOTION_PY" -c 'import numpy, PIL, soundfile'
"$QWEN_TTS_PY" -c 'import qwen_tts'
```

The first three lines passing means Standard. The `qwen_tts` import passing as well means Full. If a stage needs more than the install provides, say so in one line, run `./install.sh --doctor` from the repo root, and hand over to `motion-setup` instead of improvising. Find the root with `readlink` on the installed skill folder (under `.claude/skills`, `.agents/skills`, `.config/opencode/skills` or another harness's skills folder), or `MOTION_HOME` in the config file.

## Rules across stages

- Strategy notes win over style notes: 22-30 s, and no news-framed hooks.
- Parallelise independent work (scene groups, backdrops and props, re-renders) with agents in one go.
- Render only when asked. Preview in the Remotion Studio.

## Shared references

- `$MOTION_LIB/style/$MOTION_STYLE/docs/style-guide.md`: Style DNA, drawing in papercut, palette, fonts, texture, camera and jitter, motion timings, captions, sound.
- `$MOTION_LIB/style/$MOTION_STYLE/docs/style-prompt.md`: portable prompts for other agents and generators.
- `$MOTION_LIB/style/$MOTION_STYLE/docs/motion-principles.md`: the 12 principles of animation mapped to kit components and helpers, with a use-in-each-stage note.
- `$MOTION_LIB/style/$MOTION_STYLE/remotion/RECIPES.md`: Remotion component sketches, for when there is no library.
- `$MOTION_LIB/style/$MOTION_STYLE/docs/asset-prompts.md`: what to generate versus build in code, with prompt templates.
- `$MOTION_LIB/style/$MOTION_STYLE/docs/frame-breakdown.md`: a worked ~28 s example at 0.2 s resolution. Copy its event density, not its content.

## Hand-off

Start at stage 1 for a full reel, or at the stage the run mode names. After intake, run `motion-strategy`; it reads `SERIES.md` and the BRIEF and hands to `motion-hook`. Each stage skill names the next one.
