Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Visual Investigation" style.

MATERIALS & RENDERING
- **Three kinds of source at least**: moving image (newsreel, CCTV, phone video, launch camera), documents (testimony, memo, telegram/teletype, report page), photographs. Add sound (original commentary, radio, recordings), data (weather, telemetry, counts) and maps whenever they exist.
- **Documents**: a scanned page is shown as a strip or a page, never re-drawn; only the decisive line is highlighted (red, `multiply`, α ≈ 0.6, wiped left to right). Text that exists only as a transcription (a report, radiograms) is re-typeset by code and labelled as a transcription.
- **Archival picture pipeline** (pre-1950 film, max ≈ 2.5×): deinterlace (`bwdif=0`) → crop borders → deblock (`hqdn3d=2:2:3:3`) → lanczos upscale → light unsharp (`5:5:0.5`) → greyscale; one grain for everything. Modern footage: ≤ 1.5×. Never colourised or "restored" copies.
- **One grade** (canvas `ctx.filter`): footage and photos `grayscale(1) contrast(1.22) brightness(0.94)`; paper `grayscale(1) contrast(1.05) brightness(0.86)` (white pages pressed to light grey); maps greyscale, inverted, `brightness(.62)` so they read as dark line drawings.
- **One grain**: six 256² Gaussian tiles rotated per frame, `overlay`, α 0.22 on footage/photos, 0.10 on paper. Grain only on sources, never on type or lines (`mux.sh … 0`).
- **Drawn evidence** (plan, side schematic, charts, dot matrix) is thin line work on the panel colour, marked *schematic* when it is not measured.
- Remove channel bugs, watermarks and the source's own title cards; never generate an image to stand in for a historical one.

COLOUR LOGIC
Sources are greyscale; red is the only colour and it only ever points: circles, the chart playhead, sync lines, highlights, ✓ badges (a drawn tick in a red box, no words), the zone everything converges on, the *unresolved* column, a strike-through on a wrong claim.

TYPE & SUBTITLES
- Chinese headings and findings: Noto Sans SC 700 (30–124 px). Tags, timecodes and ticks: IBM Plex Mono 400/600. Quotes: Source Serif 4 Italic for the original wording, Noto Serif SC 500 for a translation underneath in the secondary colour (only when the subtitles don't already say it). All OFL, subset to the words on screen.
- **Few words per frame.** Each frame keeps only what this beat needs: the main readout or heading, the quoted original (the highlighted line), and one short source tag per piece of evidence. No eyebrow labels, no English twin under every Chinese label, no archive numbers, dates or "estimate / schematic / not to scale" footnotes unless the beat is about them, nothing the subtitle is saying at the same moment.
- **Source tag**: a 6×12 chip + `type · source` in mono, ≤ about 12 characters (`照片 · Pasquarella`, `FBI 电传 · 1937-05-08`), sitting just above its frame. Full credits go on the end card, not on the panels.
- **Lower third**: a red bar + place name (36 px bold) + one mono line (place code).
- **Subtitles** (bilingual films): at the bottom of the frame (Chinese baseline y ≈ 984, English y ≈ 1022): Chinese 34 px bold over English 23 px, centred, 10 px black shadow; **one line per language**, measured in the page against 1,500 px; a long sentence becomes two cues. Original sound gets subtitles too, marked `【原声】` / `[1937 newsreel]`.
- **Reading time**: every key text stays still and whole for CJK ÷ 4.5 + other ÷ 15 + 1.5 s (`readcheck.mjs`); verdict and comparison frames hold ≥ 2.5 s with every item fully in frame. Counters (timecode readouts, the stopwatch) and tick labels are exempt.

MOTION QUALITY
- 30 fps. Elements **cut in** (≤ 0.1 s opacity) with a 3–4-frame white flash, as if pulled from the evidence locker; nothing flies, bounces or scale-pops.
- Only things that are *drawn* are progressive: circles drawn clockwise in 0.5 s (ease-out cubic), leaders and links 0.4–0.8 s, plans stroke by stroke, highlights wiped, typewriter text for telegrams.
- Inside a frame: a slow push (≤ 1.9× on a map, ≤ 1.06× on a photo) is allowed; the "camera" itself never moves. Layouts change on cuts.
- The event clock (chart playhead, stopwatch) eases between keyframes and moves at constant speed inside a timed run (each tick = one real second).
- Scale changes (minutes → seconds → days) happen on section cuts, carried by the readout changing form (clock → `T+` stopwatch → date), not by a zooming band.
- Something changes at least every second; holds are for verdicts only.

SHOT & LAYOUT GRAMMAR
- 1920×1080, 60 px margins; content y 60–900, subtitles below it (y ≈ 950–1030). Layouts use the full height; no strip is reserved for a timeline. When a section has no subtitle, centre its content vertically.
- Panels are hard-edged rectangles with 1 px frames; footage panels keep the source's aspect.
- Typical layouts: **map → plan** (left) with a reference photo and a footage panel (right); **one big statement + one quote + one wide schematic**; **five-source board** (three evidence frames on top, sound and report below, schematic right, red links converging); **three-column sync grid** (report / photo / video under one stopwatch, each column lighting up when its source begins); **document strip + two-column verdict**; **claim card + fact-check card + dot matrix**; **evidence map** (plan, broken-axis mini timeline, schematic, thumbnails with links).
- Avoid centred layouts, page headers and slide-like page turns. Transitions are hard cuts on a 0.5 s grid, white flashes and the rewind; no dissolves.

SOUND PALETTE
1. **Bed**: low D held by contrabass and cellos; a filtered room tone about 30 dB under the voice.
2. **Clockwork**: a pendulum tick every 0.5 s in the minute-scale sections, louder in the waiting minutes; in a timed run one tick per **event** second (accent every 5) — the audible stopwatch.
3. **Harmony**: a quiet pizzicato ostinato in eighths (Dm – B♭ – Gm – A) under the slow sections; tremolo minor-second clusters for tension; slow sustained chords for the verdict columns; a resolving D minor add9 with vibraphone for the evidence map.
4. **Foley** (procedural): paper for documents, a shutter for photos, Morse "dits" and keystrokes for telegrams, teletype chatter, a rewind sweep, pencil scratches for plans, one deep thump for the moment of impact — then near-silence.
5. **Original sound** (archival commentary, radio, recorded sound): band-passed 180 Hz–5.5 kHz with sparse crackle, shown as a waveform with a red play-line and word-by-word lighting; music ducks −12 dB under it. Original sound and score alternate; never both loud.
6. **Voice**: calm and flat (ours: Kokoro `am_michael`); music ducks about −9 dB under it; −14 LUFS, true peak ≤ −1 dBTP.

NATIVE MOVES (AND HOW TO CODE THEM)
- **Freeze, circle, magnify.** Freeze a frame with a white flash; draw a 2.5 px red circle clockwise; at 60 % a leader and a bold label appear; a ×2 loupe opens beside it, cropped from the same source pixels (never beyond 1 source px per screen px). *Fits:* a licence plate in CCTV; a crack in a weld.
- **Hard sync point.** Pick one instant every source can recognise (impact, flash, a public clock, a gunshot peak) and align each source to it. A source that starts late just appears late; if the gap matters, say it in the voice or a subtitle. *Fits:* which phone started filming first; when the launch camera lost the vehicle.
- **Three-column sync grid.** Document / photo / video side by side under one big stopwatch; each column cuts in (white flash) at the moment its source begins, so the grid fills left to right as the clock runs; freeze on the sync point with a red dashed line across the pictures. *Fits:* two dashcams and a police log.
- **Readouts change scale.** The time readout changes form at each section cut — card timestamps and a chart playhead (minutes), a `T+` stopwatch (seconds), dates (days) — as the story narrows or widens. *Optional:* a bottom timeline band with tracks per source kind, only for a film that stays on one clock throughout; keep it free of labels and drop it when the clock stops. *Fits:* a market crash from the week to the minute.
- **Rewind cold open.** Start on the decisive frozen frame, slam the other sources into a grid, sweep a sync line, then rewind (footage playing backwards, scan lines) to the start. *Fits:* any reconstruction.
- **Telegrams on the clock.** Typeset radiograms / messages land at their timestamps with Morse and keystrokes; red dashed leaders tie each card to a pin at its time on the data chart's axis. *Fits:* chat logs, ATC calls, emails.
- **Data chart with a playhead.** A chart (wind, speed, temperature) across the lower part of the frame with a time axis, event bands (storm, outage) labelled with one word, the message pins, and a red playhead carrying the clock. *Fits:* power-grid load before a blackout.
- **Map → plan.** A real survey map pushes in to the site, then cuts to a measured plan with the subject's position, a bearing, a distance ring, a camera point and an inferred view cone; the map stays as a locator inset. *Fits:* where each witness stood.
- **Evidence → model links.** Every circle in every source sends a red dashed link to the same zone of one schematic; ✓ badges land 0.12 s apart with small mallet notes. *Fits:* which room a fire started in.
- **Then vs. record.** Play a period claim as original sound, then a fact-check card; strike through the wrong words in the claim. *Fits:* a press release against the filed numbers.
- **Known / unresolved.** Two columns, ticked vs. dashed boxes, every item held whole for ≥ 2.5 s, one source tag for the board. *Fits:* any inquiry's conclusion.
- **Dots for people.** One dot per person; the dead become red open rings one by one; a single large number. *Fits:* passengers, workers, victims of any event.
- **Evidence map.** Plan, broken-axis mini timeline, schematic and thumbnails of every source, all links converging; the poster frame. *Fits:* the end of any investigation.

PITFALLS
- **Inference passed off as fact.** Camera positions, photo times and offsets need `≈ / inferred / unlocated` on screen and in `FACTS.md`.
- **Early reports as facts.** The first-night memo and the period newsreel often disagree with the final report; use them as images or as claims to check, never as the source of a number.
- **Faking a multi-camera event.** With one licensable film, do not borrow clips of unclear origin; cross-check film + photos + documents + sound instead. The structure still holds.
- **Seams in "continuous" shots.** An exposure jump or splice breaks timing; time durations from the official record and state the alignment error.
- **Time zones and clocks.** Old reports may use standard time while the place ran on daylight time; pick one and state it once on screen (`18:25:00 EST` on the first readout).
- **Too much at once.** One layout, one finding; at most five sources on a board, and the subtitle never repeats the heading word for word.
- **Too many words on screen.** Keep only what this beat needs (main readout or heading, the highlighted original, one short tag per source). Eyebrows, English twins of every label, archive numbers, footnotes and translations the subtitle already gives turn the frame into noise.
- **Empty placeholder boxes for missing material.** A grey "no footage" or "waiting for photo" panel looks like a render error. Fill the slot with real material, change the layout, or let the source appear when it starts.
- **A strip reserved at the bottom** (timeline band, caption rail) leaves an empty band when it has nothing to show. Content runs to y ≈ 900; nothing lives in the subtitle area (y ≈ 950–1030).
- **Catastrophe as spectacle.** No slow motion of fire or impact, no full-screen flames, no sound effects on deaths.
```
