Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "1920s Newsreel" style.

MATERIALS & RENDERING
- **Gate**: 1480 × 1080 inside 1920 × 1080 (1.37:1), corner radius 26 px, pure black outside. Nothing is ever drawn outside the gate.
- **Two layers**: the picture layer (footage, print, maps, cards) gets the film look; the type layer (place cards, labels, subtitles, the record inset) stays sharp and steady. Grease-pencil marks belong to the picture (they weave with it).
- **Base grade** for every source, before anything else (`tools/prep.py`, `grade()`): greyscale → percentile stretch (0.6 % → black, 99.6 % → white; print: 0.2–0.3 % / 99.0–99.7 % with the black lifted to 26–30, like newsprint) → mild S-curve `x + 0.18·(x−0.5)·(1−|2x−1|)` → output 14–238. **Moving footage uses one level set per clip** (sampled across all its frames), never per frame.
- **Projection look** (`filmLook()`, picture layer only): warm silver (`multiply #efe7d6`, `screen #17140f`), vignette 0.42 H → 0.95 H, flicker ±3.5 % per frame + 3 Hz drift, gate weave ±1.6 px / ±2.2 px (value noise), 0–3 vertical scratches that live for 9 frames, 0–4 one-frame dust specks. Grain is added at mux (`mux.sh … 24 7`), not in the page.
- **Archive film upscaling**: at most ~2.5×. NTSC transfers (720 × 480) are treated as 4:3, cropped to 1.37 and scaled 2.31× to 1480 × 1080: `hqdn3d=4:3:6:5` → `lanczos` → `unsharp=5:5:0.55` → base grade. Frames are exported at 24 fps as JPEG sequences and played by `floor((t − t0) · 24)`, so renders are deterministic.
- **Print**: newspapers, magazines and books at their scan resolution; crop pages to the relevant fold so unrelated (and sometimes ugly) old stories never come into view.
- **Code-drawn packaging only**: the ident (sunburst, two searchlights, wire globe), item title cards, maps, the end card. Never generate "archive" pictures.

COLOUR LOGIC
- **Monochrome warm silver** for the picture; values do the work (check every shot as a thumbnail).
- Type is cream on near-black (`#f3eee4` / `#d6cfc0` on `rgba(12,11,10,.8)`), framed by a 2 px cream rule; marks are a slightly warm white grease pencil; underlines on printed pages are dark soft lead.
- Packaging silver: Art Deco gradient lettering `#fff → #d9d4ca | #9e978a → #efebe2` with a hard "metal" break at the middle and a black outline (0.12 em).
- No colour accents, no sepia wash on top of the silver, no colourised archive.

TYPE & SUBTITLES
- **Place card** (the newsreel's segment title, instead of full-screen cards for every story): top left of the gate (gate x + 48, y 56), dark box with an inset 2 px cream frame, English "PLACE · DATE" over one Chinese line; it is pulled out left to right in 0.22 s and fades 0.15 s before it leaves. Hold ≥ CJK ÷ 4.5 + Latin ÷ 15 + 1.5 s from the moment it is complete (`readcheck.mjs`), so keep both lines short.
- **Item title card** (one per film at most, to turn to a new story): black, a double Art Deco frame drawn in 0.35 s with quarter-fan corners, Limelight line + Chinese line, slow push; ≥ 2.5 s.
- **Labels inside split screens**: one per panel, top centre, same box style, enter one beat apart.
- **Subtitle strip**: the subtitles sit on a dark strip across the bottom of the gate (180 px, transparent at the top edge, ≈ 93–96 % `#0a0908` below), on every shot, fading in 0.25 s before the first cue of a run and held through gaps under 1 s, so it never blinks; print and pale footage would otherwise swallow the letters (`subBand()`).
- **Subtitles**: Chinese line over English line, **one line per language, never wrapped** (`tools/subcheck.mjs` measures every cue in the page); a cue lasts max(1.8 s, spoken + 0.6 s) and ends 50 ms before the next. Print in shot is never re-typed as a caption: the announcer and subtitles carry its meaning, an underline points at it.

EDITING GRAMMAR
- **The band is the clock**: pick a hot-dance tempo (190–220 BPM); every cut, landing, slam and label entrance sits on a beat, most cuts on bar lines; quick inserts on beat 3 or on the Charleston accent (beat 1 and the "and" of 2). When a period record plays, cut on the record's own beat (measure it), not on the grid.
- **One story = one place + one kind of material + one line of voice.** Stories get shorter towards the end (2–3 bars → 1 bar → beats), then one held button.
- **Shot lengths**: archive shots 1–2.5 s; print and documents 2.5–4 s (they must be read); comparisons ≥ 2.5 s fully in frame before the verdict; never a static frame longer than ~1 s without something moving (push, roll, pencil, flicker of a counter).
- **Transitions are the newsreel's own, each used once, as punctuation**: iris close/open, diagonal hard wipe with a bright 6 px edge, vertical-axis flip, venetian blinds (12 slats turning), clock wipe from 12 o'clock, spinning paper, page turn, panel push-over, freeze. No cross-dissolves between stories, no modern whip pans, no zoom blur.
- **L-cuts**: the announcer may run 0.3–0.5 s over a cut; a new music state may start one beat before its picture (J-cut).

CAMERA ON STILLS AND ARCHIVE
Archive footage is not reframed with fake camera moves bigger than ~6 % (it already has its own camera); stills always get a move with a purpose.

SOUND PALETTE
- **Announcer** (Kokoro): a bright, energetic male voice; a blend of two voices (e.g. `am_liam:0.6,bm_george:0.4`) gives a mid-Atlantic tinge; speed ≈ 1.15–1.2; 6–12 words per line, exclamations and rhetorical questions. Treatment as a 1930 optical track (`period_voice()` in `audio/mix.py`): high-pass 260 Hz, low-pass 3.7 kHz, +4 dB presence at 1.5–2.3 kHz (horn loudspeaker), tanh drive 2.0, compression 4:1, wow 0.22 % at 0.55 Hz + flutter 0.07 % at 6.3 Hz, two short wall reflections (31 ms, 67 ms). The speech check runs on the clean voice and again on the finished film (`tools/final_asr.py`).
- **Band** (original): tuba on 1 and 3, banjo four to the bar (accent 2 and 4), trombones on the Charleston figure (1 and the "and" of 2), snare backbeat, honky-tonk piano right hand, muted then open trumpet melody with clarinet a sixth below, woodblock pickups; a brass fanfare with timpani for the ident; a dead stop with one stab for the punchline; "da – DA!" to close.
- **Counter-music**: when the story has two sides, give the other side its own band (here an old-time reel: fiddle, plucked dulcimer, upright piano, tuba) and let them trade bars, then half bars, in a split screen, panned left and right.
- **Archive record**: a public-domain 78 can take over the soundtrack as "the real thing": needle drop (click + surface noise), band-pass 110–6500 Hz, crackle and rotation rumble only while the needle is down, varispeed by ≤ 2 % to sit on the film's tempo; end it on its own final chord and lift the needle.
- **Foley** for every move: carbon-arc snap, rising whoosh with flutter for the spinning paper, thump + paper slap, telegraph clicks under a map line, camera shutter per still, paper flicks per flip-book page, slat flutter, page turn, grease-pencil squeak, soft-lead scratch.
- **Bed**: projector (24 Hz claw clatter, motor hum, optical hiss) from first to last frame.
- **Mix**: the band and record ducked ≈ 4–5 dB under the voice (voice ≈ 12 dB over the music), the band band-limited 95–7500 Hz with a low shelf and light saturation; two silences (a dead stop before a turn, only the projector before the button); −14 LUFS.

NATIVE MOVES
A menu; use the ones the story needs, each once.
- **The ident ritual.** Searchlights snap on with a flash in the first 0.2 s, the globe grows and turns, the name lands with overshoot, the iris closes on the globe. Code: `sceneLogo()` — `sunburst()`, `beams()`, `globe()`, `decoTitle()`. *Fits:* the opening of any news-shaped film; the reprise is the sign-off.
- **The spinning paper.** Grows by `u^2.3`, rotation decays by `(1−u)^1.25` over 2.5 turns, three ghost copies 18 ms apart as motion blur, a vertical jolt (damped sine, 16 px) and a 3 % bounce on the slam, then push and punch in on the next beat. *Fits:* a headline, a decree, a press release.
- **The map spread.** A projected outline (Natural Earth, Albers; maps follow DIRECTOR.md §13), dotted arcs from the source city drawn on the beat with telegraph clicks, star pulses on arrival, place labels in screen space, an odometer of years in a corner; push into the city of the next story. *Fits:* a craze, a product launch, a migration, a campaign.
- **Grid deal → flip-book.** Stills land one per beat (shutter click each), then the camera dives into the first cell and the series plays on the beat as stop motion. *Fits:* a how-to, the stages of a process, a collection.
- **Document underline.** Slam a book or letter, turn the page, push to the paragraph, draw soft-lead underlines word-exact (positions from the scan's OCR boxes, e.g. archive.org hOCR). *Fits:* a quote, a rule, a contract clause.
- **Grease-pencil freeze.** Freeze the telling frame, jolt it, ring the detail with a hand-drawn loop (`pencil()` + `ring()`), add an arrow. *Fits:* the evidence, the punchline, "look here".
- **Split-screen duel → push-over.** Two panels slide in, each labelled, each with its own music in its own channel trading bars; the winner pushes the loser out of frame. *Fits:* old vs new, rival products, two cities.
- **Needle drop.** The band cuts dead, a 78 spins in the corner, the period record plays under the archive and ends on its own chord at the freeze. *Fits:* any era that has public-domain recordings.
- **Band stop + button.** One stab, silence with only the projector, one short line from the announcer. *Fits:* the last word of any story.

MATERIAL: WHERE IT COMES FROM
1. **Ask the user first** whether they have their own material: film or video, photographs, newspapers, letters, documents, data, a period recording. Their archive beats anything found online.
2. **If not, search sources whose licence is clear**, with `demo/tools/footage.py` (`search archive|commons|nasa`, `files`, `fetch`, `used`, `md`; it records licence, author and page in `SOURCES.jsonl`):
   - **U.S. National Archives** (films, often via Internet Archive "FedFlix" copies): federal works are public domain; donated collections need their own rights note (e.g. the Ford Motor Company Film Collection: all rights transferred to NARA in 1962, "copyright- and royalty-free" except contractor films). Universal Newsreel (1929–1967) is in NARA too.
   - **Internet Archive**: check every item's licence field and provenance; skip anything that is a YouTube mirror, a colourised or "restored/AI enhanced" version, or has no stated source.
   - **Wikimedia Commons**: read the licence template on the file page; use PD-US-expired, PD-US-no-notice, PD-USGov, CC0, CC BY. A Library of Congress "No known restrictions" (often with "Copyright-tag-still-required") is **not** enough, nor is DPLA "no known restrictions"; CC BY-SA is not used in library demos.
   - **Library of Congress** (loc.gov, tile.loc.gov; Chronicling America newspapers before 1931), **NASA**, other national archives.
   - **Print**: anything published in the US before 1931 (newspapers, magazines, books on archive.org / HathiTrust / Commons) is public domain in the US.
   - **Recordings**: US sound recordings published before 1926 are public domain in the US from 1 January 2026 (Music Modernization Act: 100 years after publication for pre-1947 recordings); UCSB Library / Internet Archive transfers (Edison, Victor) and the LOC National Jukebox give dates and matrix numbers. Check the composition's date too.
   - **Maps**: Natural Earth (public domain) for outlines drawn in code. Its default borders are not usable for China: DIRECTOR.md §13.
3. **Before downloading, tell the user** what you will fetch, from where, under which licence, and how big it is; keep `raw/` under ~1.5 GB.
4. Everything used goes into `demo/CREDITS` and `SOURCES.md` (title, author, date, licence, page, which seconds). Real people: historical public figures and people in news events may appear; ordinary people's faces are never slowed down or made the close-up subject.
5. Every fact in the voice and on screen goes into `demo/FACTS.md` with its source and the quoted wording; read the catalogue description of each film before using it (it may say what is really happening in the shot).
```
