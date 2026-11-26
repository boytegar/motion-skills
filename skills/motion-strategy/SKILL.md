---
name: motion-strategy
description: Set up a series and decide what a short video should be. Writes SERIES.md when the series has none (audience, language, regions, voice, look, funnel), picks the topic and one familiar format, sets the comment-keyword funnel and caption, studies competitor videos, and runs the post-publish autopsy. Start here before any hook or script; come back when reviewing how a posted video did.
---

# Strategy

These notes decide what a video says and why it holds attention. The other planning skills decide how it is written and staged; building it is outside this skill set. The rules come from studying high-performing faceless tech videos and from reading platform insights after posting. Keep your own evidence in the series' `research/analytics/`.

## Preflight

- Install: **plan only** is enough. No library needed.
- Find the series folder (it holds `episodes/`). Read `agents.md` when it exists: it supplies the initialized default style, so do not ask for it again. Then read `SERIES.md`. **If `SERIES.md` is missing, stop and run step 0 first** — no other skill in this set creates it.
- For a competitor study: `yt-dlp`, `ffmpeg` and any speech-to-text tool. For an autopsy: the platform's insights numbers from the user.

## 0. Init the series (only when `SERIES.md` is missing)

`SERIES.md` holds the decisions that outlive one episode, so every later stage reads it instead of re-asking. This stage owns it: no installer writes it and no other skill may. Do this before step 1 and do not continue until the file exists.

Ask for these in one call (your agent's question tool). Never invent an answer: a guessed audience level poisons every script written after it. Anything the user leaves open goes in as `TBD` with the open question next to it, and the gate below still passes.

| Field | What it decides | Read by |
|---|---|---|
| **Audience** | who it is for, their level, and the feeling to leave them with | `motion-hook`, `motion-script`, `motion-qa` |
| **Language** | the one the VO, banners and captions are written in | `motion-script`, `motion-voice` |
| **Regions** | which to aim for and which to avoid | `motion-hook`, this skill's topic step |
| **Voice** | tone, person, contractions, words to avoid | `motion-script`, `motion-voice` |
| **Palette** | base colours and the accent, taken from the style guide | `motion-storyboard` |
| **Mascot** | the recurring character, its own prop and its costume rule | `motion-script` (gags), `motion-storyboard` |
| **Style** | one style folder name; `agents.md` wins when both exist. A reference video instead of a name means `motion-style-extract` turns it into a pack first | every stage |
| **CTA convention** | the keyword shape and how the freebie is delivered | `motion-script`, this skill's funnel step |
| **Keyword log** | keywords already used, so they never repeat | this skill's funnel step |
| **What worked** | formats, topics and hooks that earned attention, with numbers | this skill's topic step |

Write `<series>/SERIES.md` in this shape:

```md
# SERIES: <name>

## Audience
Who: <who it is for>
Level: <how much they already know>
Language: <one language>
Regions: aim <...>; avoid <...>
Feeling: <what they should feel at the end>

## Voice
<tone, person, contractions, words to avoid>

## Look
Style: `<style folder>`
Palette: base <...>; accent <...>
Mascot: <who, its own prop, its costume rule>

## Funnel
CTA: comment <KEYWORD> -> <freebie>, delivered by <tool>
Keywords used: <KEYWORD> (<episode>, <date>)

## Evidence
Worked: <format / topic / hook> — <numbers, date>
Failed: <what, and why>
```

The file records no engine and no path into any renderer. The engine is a tooling note that lives in `HANDOFF.md`, and it never changes what the series or the plan says.

Two fields need care:
- **Style** comes from `agents.md` when both exist; `agents.md` wins, and step 0 must not offer a different one.
- **Palette** is read off `$MOTION_LIB/style/$MOTION_STYLE/docs/style-guide.md` (section 5) rather than invented. If there is no library yet, ask the user and mark it `TBD`.

Re-run step 0 only to fill a `TBD` or to change a decision deliberately. Every other stage reads the file and never edits it; the autopsy appends to `## Evidence` and nothing else.

## 1. Topic

- Re-skin proven winners. A topic that worked (for you or a competitor) in a new familiar format beats a new topic in a plain format. Example: a video about running several AI agents as a council, remade as "a jury of 12 AIs puts your idea on trial".
- The viewer must be able to use the outcome. If they can't do anything with it, skip it.
- Something to give away: every winner promises a free skill, prompt, template, cheat sheet or setup by DM.
- Speed on a hot repo. The first video on a trending repo takes most of the views. Ship within a day or two, on an angle the earlier ones didn't use (`../motion-hook/references/hook-patterns.md`).
- Demand signals: competitor outliers (likes at 5x or more of their median), trending repos (stars per day), front-page threads, same-day launches. Save candidates to the series' idea backlog.

## 2. Format

Pick one familiar real-world format for the whole video before writing a word: courtroom, pitch show, cooking show, heist movie, nature documentary, dating show, character-select screen, boss fight, job interview, Olympic podium. The viewer gets the premise in a second and every set and gag comes from it. A plain list of facts has no premise to grasp, so it gets less attention than the same facts told inside a format. The bank, with the story each format carries, is `../motion-script/references/comedy.md`.

## 3. Audience level

Write at the level step 0 recorded in `SERIES.md`. There is no default audience. If the file says `TBD`, ask now and write the answer back before scripting; a later stage must never guess. The feeling to leave behind: you're not behind, and now you know something most people don't.

## 4. Funnel and caption

- Spoken CTA: "Good thing I made [freebie]. For the link, comment KEYWORD." One short keyword tied to the topic.
- Deliver the freebie by DM with whatever comment-to-DM tooling the account allows. Keep sends slow and spaced, because platforms rate-limit bursts.
- Caption: the hook as the first line, a follow line so the DM lands, "READ BELOW" with the useful part written out, one send prompt ("Send this to the friend who..."), 3-5 hashtags.
- Shares matter most to the algorithm. A funny or useful video gets sent.
- Reuse no keyword: check `Keywords used` in `SERIES.md` first, then append the new one there when the episode ships.

## 5. Write the BRIEF and create the episode folder

Write this block at the top of `episodes/NN-slug/research/facts.md`. Every later stage (`motion-hook`, `motion-script`, `motion-storyboard`, `motion-voice`, `motion-build`) reads it to know the format and audience target.

```
BRIEF
Source:   <link(s)>
Claim:    <the one claim the video proves>
Role:     <who or what does the work vs who advises or approves>
Mode:     open-source drop | news launch / worth-it | feature unlock
Format:   <the one familiar format>
Audience: <level, copied from SERIES.md>
Style:    <style folder, from agents.md else SERIES.md "Look">
CTA:      <KEYWORD> -> <the freebie>
Length:   22-30 s
```

The BRIEF records no engine. The plan is engine-neutral: whichever renderer builds it is the user's choice, made outside this skill set, and it never changes what the plan says.

**Create the episode folder immediately once the BRIEF questions are answered.** Do not wait for the later stages, and do not scaffold a render project here:

```
mkdir -p <series>/episodes/NN-slug/{research,narration/voice/final_narrator,video/{src,public/vo,tools}}
```

Write `<series>/episodes/NN-slug/video/episode.json` with the identity only:

```
{"comp": "<CompositionId>", "out": "<out-name>", "style": "<style>", "width": 1080, "height": 1920, "fps": 30, "gap": 0.22, "scenes": [], "banners": {}, "hook": {}}
```

`motion-build` fills `scenes`, `banners`, `hook`, `cuts` and `bed` from the storyboard. Project files, render dependencies and preview commands belong to whichever engine the user builds with; nothing here creates them, and no preview command may be promised from this stage.

## 6. Competitor study (when the user shares videos)

Method, setup and the subagent prompt: `references/competitor-analysis.md`.
- One subagent per video, up to 4 at a time. Each downloads it, extracts frames at 2 fps, transcribes it, applies the six-question hook check and returns a fixed-format report.
- Synthesise into a comparison table (likes and comments vs the creator's typical video, age at fetch, look, length), what drove the outliers, what to borrow and what stays your difference.
- Save it to `research/analytics/YYYY-MM-DD-<creator>-videos.md`. One video is noise; look for a pattern across 4 or more.

## 7. After posting: the autopsy

- At about 2 h and 24 h collect views, skip rate, average watch time, the retention curve, likes, comments, saves, shares, follows, traffic source, top countries and ages.
- Read them in this order: hook, topic, format, audience, timing. Fix the earliest thing that failed before touching later ones.
- If most viewers are gone by 3-5 s, or the skip rate is clearly worse than your own median, the hook failed. Run `motion-hook`.
- Write `research/analytics/YYYY-MM-DD-autopsy.md`, then update only the `## Evidence` section of `SERIES.md` with what changed. The series file is step 0's; keep its shape, and change audience, voice or palette only when the numbers say so.

## Gate

- [ ] `SERIES.md` exists and was read; audience, language, regions and voice respected.
- [ ] Every step-0 field is filled or explicitly `TBD`; no field was guessed.
- [ ] Topic has a demand signal, a usable outcome and a freebie by DM.
- [ ] One format named in a line.
- [ ] Style recorded in the BRIEF; no engine named anywhere.
- [ ] Vocabulary and the DM match the audience level.
- [ ] Keyword absent from `Keywords used`.
- [ ] BRIEF written in `facts.md`; episode folder and `video/episode.json` created.

## Hand-off

Next is `motion-hook`. It reads the BRIEF in `episodes/NN-slug/research/facts.md` and `SERIES.md` for the audience level and regions, then writes the HOOK block at the top of `episodes/NN-slug/video/STORYBOARD.md`. The episode folder exists and holds no renderer.
