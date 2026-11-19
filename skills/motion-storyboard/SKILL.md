---
name: motion-storyboard
description: Storyboard a script before any code. Writes the Camera / Cut table, one block per scene with frame-level events on each stressed word, the SFX plan and the proof beat. Names effects, not engine syntax. Use for "storyboard this script" or "plan the camera moves and cuts".
---

# Storyboard

The storyboard is the plan every scene is built from. It names the camera move and the cut for every scene, so whoever builds the scene doesn't invent them. A brief that says "a push or your own transform" gets a flat push in every scene.

Write the camera and cut vocabulary, not engine syntax: every row names an effect, and the engine that builds the video maps it to its own components.

## Preflight

- Install: **plan only** is enough. With a library, read `style/$MOTION_STYLE/docs/style-guide.md` sections 4-6 (canvas, palette, fonts) and `$MOTION_LIB/core/audio/CATALOG.md` (SFX and bed names).
- Resolve `$MOTION_LIB` from the environment, else the `MOTION_LIB=` line in `~/.config/motion-explainer/config.env`, else ask once.
- Needs `narration/script.json` and `video/STORYBOARD.md` with the HOOK block and the staging table (banner, set, prop, UI moment, mascot, gag per row). Missing: run `motion-hook` and `motion-script`.
- Backdrop sheet for choosing sets: the style's own, `style/$MOTION_STYLE/` wherever its kit keeps `bg/backdrops/_sheet.jpg` (the papercut style has it under `remotion/`). Open only what the step needs.

Never inspect a renderer's source or catalogs here. The storyboard names an effect; how that effect is drawn is the engine's decision.

## 1. Global rules

Put these at the top of `STORYBOARD.md`, under the HOOK block. Palette by role (rotate background colours per set): `$MOTION_LIB/style/$MOTION_STYLE/docs/style-guide.md` section 5. Fonts: section 6. Canvas and zones: section 4. A feature-unlock that suits one extended metaphor can use section 12.
- 1080x1920, 30 fps. Banner at 14-21% height, caption band 68-80% (it shifts per scene to stay off faces), bottom 20% clear.
- Light from the top-left. Saturation only on characters, captions and signals.

## 2. Camera / Cut table

One row per scene, before the scene blocks. Every row names one camera effect and the kind of cut into that scene.

| Scene | Camera | Cut in |
|---|---|---|
| S01Hook | slow push, pull back to reveal | none |
| S04Name | crane rise to <focal prop> | tear away |
| S09Peak | lateral track along <lineup> | whip pan |
| S10CTA | rack focus <prop> to phone | tape pull |

- Camera effects: slow push, pull back, orbit with parallax depth, snap zoom or punch-in, crane rise, lateral track, dolly zoom, rack focus, foreground pass, dutch tilt, focus pulse, freeze frame, point-of-view mask, time ramp, tilt shift, focus hunt, handheld.
- Cut kinds: hard, whip pan, zoom through, page turn, tear away, tape pull, stamp cut, infinite zoom, pop-up book, keyhole, burn away.
- A plain slow push is allowed on at most a third of the scenes.
- Copy each row's camera into its scene block header. The cut column goes into `episode.json` as `"cuts": {"S04Name": {"kind": "tearAway"}}` (no entry means a hard cut), which `motion-build` fills.

The engine maps these names onto its own components. If an effect has no counterpart there, it says so when it builds; the storyboard does not pre-empt that by picking syntax.

## 3. Scene blocks

One block per scene, 1.5-5 s each, cutting on sentence boundaries:

```
## S03: <beat>  (5.6-8.2 s, 78 f)   BANNER: <TAKEAWAY IN CAPS>
Location/bg: <world location + bg colour>     Camera: <effect + params, from the table>   Cut in: <kind>
Cast: <who, accessories>                       Caption band: y=0.76
| VO word (stressed) | frame | event                                    | motion spec                   |
| "<word>"           | +6    | <prop enters / state flips / acts>       | <spring/slide/swap + frames>  |
Ambient: <LED blink, idle bob, stars, screen waveform>
Boil: banner + labels, +-1.5 px every 2 frames
SFX: <pop, chime, click...>
```

- Expand each staging-table row into frame-level events: one per stressed word (every 0.4-0.8 s), ambient motion always on. Frames are offsets from the word, so a builder reads them against the Timing section.
- Product UI is the rough rebuild on a paper monitor or laptop, with fake interactions (cursor click, typing, progress, waveform, list scroll).
- **Proof:** once per video, a real screenshot of the source page (a browser capture, not a redraw) presented as a physical card, with a red marker circle drawing itself around the key line. Everything else stays a stylised rebuild. Note the capture as a step for the build, and mark the image as a required asset.
- One callback to an earlier set before the CTA.

## 4. Camera and transition rules

- Every scene has one move. A slow push is the baseline. Add a parallax orbit on lit sets for depth, a punch-in or snap zoom on the stressed word for numbers and jokes, a foreground pass (blurred paper object across the lens) or rack focus, a pull-back to reveal scale, and a freeze frame with a label on the struggling-mascot beat.
- Mostly hard cuts on the banner change. A whip-pan with motion blur on 2-3 topic changes, a zoom-through when going "inside" a screen, door or mirror, at most one page turn or tear per video (on the catch or a before/after contrast), and a tape pull into the CTA where the style allows one (a plain kraft strip with an optional printed word).
- The cut out of the hook carries motion through it. Apply the rule from `motion-hook` to S01 and S02.
- Transitions run 6-12 frames, each with its SFX (whip, zoom, page-flip, rip, tape-peel).
- Scene starts stay on their plan frames, so the voice stays in sync whatever transition the engine uses.

## 5. SFX plan

- Air sounds (whoosh, slide, whip, zoom) together: at most one every 2.5 s of video. `motion-build` prints a note above that. For scale, 12 in 33 s is fine and 24 in 26 s is distracting.
- Entrances get contact sounds (pop, thud, stamp) or silence. Count them in `tools/cues.json` before the audio render.
- Name SFX from `$MOTION_LIB/core/audio/CATALOG.md` in each block's `SFX:` line. `motion-build` writes the real rows to `tools/cue_sheet.d/<Scene>.json`.
- Pick the bed preset (`episode.json` "bed") and set `cutSfx` to match the cut kinds: tear away rip, tape pull tape-peel.

## Gate

- [ ] Every scene has a camera and a cut-in, and a plain slow push is on at most a third of them.
- [ ] Every stressed word has an event; ambient motion is listed.
- [ ] One proof beat, one callback, the tape pull or whip into the CTA.
- [ ] At most one page turn or tear; air sounds within budget.
- [ ] No engine component name, no file path into a renderer, no render command.

- Name the anticipation and the one exaggerated beat in each scene block's event table. The 12 principles are in `$MOTION_LIB/style/$MOTION_STYLE/docs/motion-principles.md`.

## Hand-off

Next is `motion-voice` (or straight to `motion-build` if the voiceover already exists). `motion-build` turns the Camera / Cut table and scene boundaries into `episode.json`, the cue list and the handoff manifest. It reads `STORYBOARD.md` and `narration/script.json`. The Timing section of `STORYBOARD.md` is generated later, after the voice, so leave it out.
