---
name: motion-qa
description: Check a built video against the plan and the style DNA. Engine-side: run it after your renderer produces stills, in the engine's own project. Compares frames to the storyboard, the scene frames and the captions, runs the checklist, and writes the delivery summary. Use for "QA the video", "check my scenes match the style", "is this ready to post".
---

# QA

**Engine-side skill.** It is not part of the planning pipeline: it runs after an engine has produced stills, and it checks what that engine produced against what the plan asked for. It never plans, never scaffolds, and never chooses a renderer. Any engine can use it; the only engine-specific thing in it is your still command.

## Preflight

- Install: **plan + audio files**. Stills come from the render project; the contact sheets need Python with numpy and Pillow (`$MOTION_PY`). Missing pieces: `./install.sh --doctor`, then `motion-setup`.
- A built project plus its plan folder: `video/src/plan.json`, `video/public/vo/captions.json`, `video/tools/cues.json`, `video/STORYBOARD.md`, `video/HANDOFF.md`.
- Library: `$MOTION_LIB`, else the `MOTION_LIB=` line in `~/.config/motion-explainer/config.env`, else ask once.
- Read first: the Style DNA in `$MOTION_LIB/style/$MOTION_STYLE/docs/style-guide.md` section 2, and the backdrop sheet if the style ships one (`style/$MOTION_STYLE/`, wherever its kit keeps `bg/backdrops/_sheet.jpg`; the papercut style has it under `remotion/`). Each image costs about 0.7-1.5k tokens, so open only what the step needs.
- For density, `$MOTION_LIB/style/$MOTION_STYLE/docs/frame-breakdown.md` ("Pacing stats" at the end): scenes average 2.8 s, 2-4 discrete visual events per scene, one every 0.4-0.8 s on a stressed word, a new caption page about every 0.6 s. Copy the density, not the content.

The engine named in `HANDOFF.md` owns the still command, the preview, and any fix inside a scene file. Ask it for stills; this skill reads them.

## 1. Get stills

Ask the engine for 4-6 timestamps per check, or use its own still command. For a scene, take about one still per quarter of its length in frames, read from `src/plan.json`.

- One pass per scene, never two renders at once. Keep fixes to two rounds per scene.
- Check against the plan: the scene's frame 0, its key events on their words, nothing clipped, nothing covering the banner or caption bands.

## 2. Checklist

**Plan fidelity — compare the frame to the plan, not to taste:**

- [ ] Every scene starts and ends on its `src/plan.json` frame and runs the stated duration.
- [ ] The banner text matches `episode.json` `banners` and never repeats the VO under it.
- [ ] Captions match `public/vo/captions.json`: one tag per word, pages of 1-3, in the 68-80% band.
- [ ] Every scene has the camera move and the cut-in its storyboard row names.
- [ ] Every sound in `tools/cues.json` has a visible cause at its frame, and none is doubled.

**Style DNA — the chosen style's look (papercut's checklist shown; swap in the style's own tokens and re-read its guide before judging):**

- [ ] Every object reads as paper: grain, rough edge, drop shadow, stacked-piece shading, no outlines or gradients.
- [ ] The world camera moves smoothly. Only the banner and labels boil (stepped, subtle).
- [ ] The banner is fixed in position and taped, 3-7 words, one fact each; the banners alone read as a fact sheet.
- [ ] Every set illustrates a concrete word in its VO line, numbers are physical props, and there is one callback set before the CTA.
- [ ] Saturation only on characters, captions and signals.
- [ ] A visual event on every stressed word, with ambient motion always running.
- [ ] Cuts on banner changes, bottom 20% clear.

**Content — the story:**

- [ ] Every number and UI label traces to the facts list. The script has at least one honest limitation and a feature stack of exactly three.
- [ ] The HOOK block (SAY / WRITE / SHOW / ALIGN) is on screen exactly as written: frame 0 shows the SHOW moment mid-action with the WRITE title on it, and the first VO line is SAY.
- [ ] At most one page turn or tear, a tape pull or whip into the CTA, and every transition has its SFX.
- [ ] Humour pass visible on screen: one format throughout, a costume prop per idea, one struggling beat, the frame-0 prop returns changed in the CTA set, every gag reads in under 1 s and never covers the banner, the captions or the key number.
- [ ] Vocabulary in the VO and banners matches the audience level in `SERIES.md`.
- [ ] Voiceover, captions and events are in sync at every cut.

A failed **plan fidelity** item is a renderer bug or a plan bug: decide which, then fix the plan through the owning motion skill or send the scene back to the engine. A failed **style** or **content** item is a scene fix in the engine's own files.

## 3. Deliver

List the scenes with timings, and say what is placeholder: voice, music, SFX, assets. If the voice is a placeholder, say how to re-voice it (`motion-voice`). After posting, the numbers go to `motion-strategy` for the autopsy.

## Principles pass

Scrub one scene at 5 fps and check against `$MOTION_LIB/style/$MOTION_STYLE/docs/motion-principles.md`: anticipation before each big move (2), everything eased (6), something trailing after impacts (5), one focal prop (3), one exaggerated beat (10), stacked shading and a consistent shadow (11).

## Hand-off

A plan bug goes back to `motion-storyboard` or `motion-voice` through `motion-build`, which rewrites the plan files. A scene bug goes back to the engine. A clean pass ends the episode; the render is the engine's call, when the user asks.

## Pipeline note

`motion-qa` is not one of the planning stages that `motion-explainer` routes. It is a checklist plus a delivery summary any engine can run after it renders.
