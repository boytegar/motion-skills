Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "City Symphony" style.

MATERIALS & RENDERING
- Footage only (2D). Sources mixed freely: tripod time-lapse, long single-camera time-lapse (≥ 12 h, sunrise and sunset in the same frame), hyperlapse, drone aerials (top-down is gold), high-frame-rate slow motion, archive film (pre-1931), photographs (a pair from the same camera position is gold), a scanned page of a historic text.
- Everything is unified into one **35 mm release-print** look: 1.85:1 matte (21 px bars on 1920×1080), warm highlights, lifted blacks, highlight bloom, clumped mid-tone grain, ±0.5 px gate weave, soft vignette.
- Never generated images; never restored, colourised or speed-corrected archive copies; never YouTube mirrors of archive films.
- Upscale limits: archive ≤ 2.5× (denoise → lanczos → light sharpen → grain); modern ≤ 1.5×.

COLOUR LOGIC
Grade (`tools/render.py → make_luts / grade`): per-shot partial grey-world white balance first (only pixels with luminance 0.12–0.88, strength 0–0.6; **0 for night shots**, which keep their sodium and blue), then one LUT for all shots: S-curve (sigmoid slope 6.2, pivot 0.47, blended 55/45 with linear) → black 7/255, white 250/255 → R ×1.025, B ×0.965 (warm highlights, a little cold in the shadows) → saturation 0.86. Bloom: pixels above 170/255, blurred 6 px at ⅛ size, added back × (1, .82, .62); 0.09 by day, 0.15–0.2 at night. Grain: half-resolution Gaussian noise up-sampled and blurred 0.6 px, weighted `0.35 + 1.3·L·(1−L)`, strength 9/255. Archive film stays black-and-white and goes through the same LUT, so it reads as part of the same print.

TYPE & SUBTITLES
- Latin: **Michroma** (OFL), capitals, tracking 0.06–0.32 em. Chinese: **Noto Sans SC** variable, weight 300–350, tracking 0.08–0.14 em. Credits: Noto Sans SC 17 px.
- **Left-aligned**, left margin 168–196 px, never centred, never on a box. Legibility comes from a very soft elliptical darkening behind the block (α ≤ 0.6), not a panel.
- **Number card**: the number in Michroma 84 px (44 px when three numbers stack); a 2 px sodium rule grows under it; one Chinese line (28 px) and one English line (14–16 px) say what it counts and the source year. A census-type number locks digit by digit on sixteenths (one glockenspiel note per digit); a flow number counts up from 0 in 0.6 s (ease-out). "About" numbers carry `≈` and "约".
- **Archive date**: just the year, Michroma 22 px, bottom-left of the archive panel.
- **Closing quote**: on the page shot the whole sentence is lit; on the card, a short fragment over the last moving shot — Chinese 60 px, English original in capitals 20 px, source line 14 px sodium, fading in within 0.25 s and held just past the reading time.
- One line per language, never wrapped; every text block stays fully in frame at least CJK ÷ 4.5 + other ÷ 15 + 1.5 s (`readcheck.mjs`). Text may stay up across cuts underneath it, but only while the shots under it belong to what it counts.
- `.srt`: the film has no voice, so the subtitle file carries the on-screen text (one Chinese and one English line per cue) for muted or accessible playback.

MOTION QUALITY
- Speed is the main instrument. Shot speeds run from 0.3× to 75×; every speed above 1× uses **frame blending** (a 360° shutter): each output frame averages the source frames it covers, capped at N frames. Headlights become continuous lines, crowds become fog, clouds become water.
- **Speed ramps** are smooth (smoothstep between key speeds) and always mean something: a corridor accelerates out of real time (1× → 6×), a time-lapse ramps into a frenzy (1× → 7×), a fast shot brakes into slow motion (2.2× → 0.3× in 0.55 s) on the bar where the music stops.
- Camera moves are added sparingly in post: slow push-ins (1.00 → 1.05–1.10 over the shot), a falling push on the opening still (1.0 → 1.3 in 0.35 s, ease-out, then accelerating to ×2 by the cut). No shakes, no whip pans, no digital zooms past the upscale limit.

CAMERA GRAMMAR — EDITING
**Cut-point table (the score decides; numbers at 120 BPM and 30 fps: 1 beat = 15 frames, triplet = 5, sixteenth = 3.75)**
Rules:
- **Cuts land on beats.** All cut times come from one function, `bar(n, beat)` in `tools/timeline.py`; shot lengths are never typed in seconds. The music is rendered from the same file.
- **One long single-camera time-lapse is the clock.** It returns three or four times (night → dawn, noon clouds, a one-second nightfall, a day/night flicker), always from the same position, so the viewer knows the time without a caption.
- **Density follows the score.** The editor never cuts faster than the current subdivision, and the slowest picture sits under the place where the arpeggios stop.
- **Shape match cuts** on the beat: clock → clock, arch → arch, grid of streets → grid of cells. A small-to-large punch-in on the same shape is allowed.
- **The grid** is the split screen of this style: 1×2 → 2×2 → 3×3 → 4×4, one stage per half bar or bar, the new cells lighting up in a scattered order over the previous stage within 0.3 s (never back to black), gutters 6 px black; in the last beat the cells re-cut on every sixteenth.
- **Counterpoint split**: two halves, archive left, today right, the same kind of place or shape; the gutter grows from the centre in 8 frames.
- **Day/night flicker**: the same camera, a noon frame and a night frame alternating every 2 frames; a pair of same-position photographs alternating every 2 frames.
- **Reverse flashback**: in the last bar before the hard stop, earlier shots return in reverse order, 4 frames each, then 2 frames each.
- Transitions: hard cuts only, plus the hard cut to black. No dissolves except fades from black for the page and the end card.

SOUND PALETTE
- **Music only.** No narrator, no source sound (the footage soundtracks are other people's music), no sound effects. A narrator would turn watching into listening and would fight the score for the structure; that is why the style has none. If a voice is ever required, one short line at the very start or end, never over the cuts.
- Orchestration (sampled, CC0 — `core/audio/sampler.py`): pipe organ (full and soft stops, pedal), winds staccato (flute, clarinet), strings (spiccato cellos and violins, sustained cellos, violins, double bass), horn pair for the last two bars, concert bass drum on section downbeats, suspended-cymbal crescendos into the hits, glockenspiel only for numbers locking.
- Writing: arpeggios over a one-chord-per-bar progression; additive layering (eighths → triplets over eighths → sixteenths); the *break* is all arpeggios stopping at once into one held chord (maj7 or 9th); the return is a full-organ hit; the hard stop is a dry cut of every instrument in 15 ms with the reverb tail left ringing; the coda is one soft chord that does not resolve, with the opening figure returning in quarter notes under the end card.
- Reverb: `S.room(size .7, mix .28)` for the body, `size .78, mix .36` for the coda. Master to −14 LUFS / true peak ≤ −1 dBTP with `core/render/mux.sh` (grain 0: the grain is already in the picture).

NATIVE MOVES (AND HOW THEY ARE BUILT)

PITFALLS OF THE MEDIUM
- **Watermarks and slates hidden in CC footage**: corner logos (scale from the centre until they leave frame, within 1.5×), opening thank-you cards, end-card music credits, title text over the first shots. Check a 1-second contact sheet (`tools/secsheet.py`) of every source before choosing in-points.
- **Hidden cuts inside sources.** Hyperlapse reels hide 2–3-frame jump cuts and whip transitions; frame blending across a cut makes ghosts. Scan with `ffmpeg … select='gt(scene,0.25)',showinfo` and keep every clip's source window inside one shot.
- **Too much blending** turns noon clouds into grey mush: about 40 % shutter (24 of 60 frames) looks better than 100 %.
- **Grey-world balance kills night**: it washes sodium light to white. Strength 0 for night shots.
- **Tracking needs a fixed camera.** A drifting drone flags every building as motion. Use a median background only on tripod shots; for moving aerials use blending instead.
- **Motion-compensated slow motion tears thin lines** (cables, rails, text): use the crossfade mode there.
- **Text over bright sky** disappears: put number cards over the dark half of the frame or add the soft darkening; never a box.
- **A slow face is not a free subject.** Do not make an identifiable ordinary person the slowed or close-up hero. Slow a thing; show people as crowds, tracks or ghosts. Public figures and people in news events are a different case.
- **Numbers drift.** Official pages update; quote the page and the year, keep the quote in `FACTS.md`, and recheck before the last render. Prefer one clean figure over a range.
- **CC BY credits are long.** Each one needs author, full title and licence on the end card. Treat them as fine print (16 px, 29 px line pitch, the licence and changes on one header line) and keep the end card short (4–5 s); a 10 s card of text on black loses the audience. The film's last moving shot runs on under the card, darkened.
- **Don't end on dead black.** After the hard stop, the coda is a page (≈1 s), then the quote laid over one last moving city shot (≈4 s, just long enough to pass `readcheck`), then the end card over the same shot.
- Commons rate-limits (HTTP 429): search and download serially with back-off (`tools/s.sh`, `tools/f.sh`); for long 4K files download only the needed excerpt (`tools/seg.py`) or the 1080p transcode.
```
