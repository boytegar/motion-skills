Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Constructivist Montage" style.

MATERIALS & RENDERING
- **Four kinds of element only**: **bands** (red or ink, along the axis), **windows** (rectangles or parallelograms holding film or photographs), **cut-outs** (people scissored from photographs with a paper-white border), **type blocks**. Plus one paper-fibre texture over everything.
- **The print grade** (one function for film, photos and documents): luminance → percentile stretch (film: one pair of percentiles per clip, so it never pumps; 0.7 / 99.5; photos 0.8 / 99.4) → S-curve `x + s·(x−0.5)·(1−|2x−1|)·2`, s = 0.32 (documents 0.18, they keep their paper) → map to ink→paper: `rgb = INK + (PAPER − INK)·g`. Archive film keeps a ±2.5 % per-frame flicker inside its window (`hash(frame)`), photographs and documents do not.
- **Cut-outs**: a person-segmentation model (u2net_human_seg via onnxruntime) → threshold 0.5 → median filter → largest component → 7 px paper border dilated outside the mask. The border is the medium's own look and hides the model's edge errors. Group photos rarely segment cleanly: use them whole in windows.
- **Documents**: scans at full resolution, never retyped. They are marked: a red highlighter bar in `multiply` over the exact sentence, a hand-drawn red ellipse round a heading, a red circle on a map. Drawings on a document (a chart, a map) may be lifted off the page and reused, keyed from their paper (`alpha = (205 − luminance) / 90`).
- **Resolution ceiling**: archive film ≤ 2.5× its source pixels on screen, modern footage ≤ 1.5×, photos ≥ 1 source px per screen px where possible. Prepare clips with `hqdn3d → lanczos → light unsharp` before grading; add grain only in the mux. A 720-line film transfer can fill a 1,700 px window, not a full 1080p frame: frame it (a window, a split strip, an iris) instead of stretching it.
- **Paper fibre**: a fixed seeded noise canvas multiplied over the frame at α 0.5; film grain 4 added by `mux.sh`.

COLOUR LOGIC
- Everything photographic is ink-on-paper; nothing else has colour. Red is never laid on red: a red block on a red ground becomes ink.
- Alternate grounds to make cuts land: paper scene → ink scene → paper. In an acceleration the background itself alternates red / ink per cut.

TYPE & SUBTITLES
- **Bilingual pairs**: Chinese block, then one English line in Oswald directly under it (or on the neighbouring band). Each language one line, measured.
- **Placement**: along the −18° axis on a band, or as an ink / red block; never centred on the frame, never a header. Numbers can be bigger than the words that explain them.
- **Tags**: every archive window carries its footage tag (`FILM · <TITLE> · <PLACE> <YEAR>`, `PHOTO · <AUTHOR> <YEAR>`), every fact a source tag (`来源 · <PUBLISHER> <YEAR>`). If the footage is not of the subject itself, the tag says what it really is.
- **Subtitles** stay bottom-left in a 960–1056 px strip; key type keeps above y 930 while narration runs. Subtitles hold ≥ max(1.8 s, speech + 0.6 s). On-screen text follows `readcheck.mjs` (CJK ÷ 4.5 + other ÷ 15 + 1.5 s, fully in frame); a comparison holds every item ≥ 2.5 s before its verdict.

EDIT GRAMMAR
- **Grid**: one tempo for picture and score (120 BPM: beat 0.5 s, bar 2 s). Every cut, slam, wipe and count lands on a beat, half-beat or sixteenth. The page exports these times as `window.EV`; the mix reads the same list.
- **Shot length**: 0.5–2 s for footage; one idea per shot. A section may accelerate by halving: 0.5 → 0.25 → 0.125 s (never shorter than 3 frames). Holds are reserved for reading: a text block, a freeze, a comparison.
- **Cut rule**: adjacent shots conflict in at least one of direction (left slam / right slam), value (paper / ink ground), or geometry (diagonal band / upright window). Never two paper documents in a row.
- **Nothing static for more than a second**: inside a hold something moves (film in its window, a counter, a slow push, a frame-within-the-frame). Planned exceptions only: the comparison hold, one silence beat, the end card.
- **Transitions inside the medium** (pick per cut, don't fade):
  - *red wipe*: a 260 px red band at 72° (perpendicular to the axis) sweeps the frame in 0.25 s, the next scene revealed behind it; it lands on the downbeat;
  - *iris*: a red marker circle drawn on a map or photo becomes an aperture and opens onto footage;
  - *slam*: the next element arrives from outside along the axis in 0.08–0.2 s, ease-out, hard stop, frame shake (amplitude 3–18 px, `exp(−14·Δt)`);
  - *flash cut*: one frame of 50–80 % paper over a window at a cut or freeze;
  - *hard cut on the beat* between grounds of opposite value;
  - *jump-in*: inside a freeze, a cut to a 1.2× crop of the same frame, the annotation scaling with it.
- **Two silences**: one where the orchestra stops and only an original recording remains, and one true half-bar of nothing (ink frame, one red dot) before the heaviest hit.

FORMATS (EACH IS A FUNCTION IN `DEMO/INDEX.HTML`)

SOUND PALETTE
- **Score**: an original machine ostinato (eighths, minor, e.g. D–D–F–D–A–D–E–F) in spiccato cellos + staccato bassoon, staccato tuba / trombone on the beat, snare sixteenth taps, xylophone for counts (one rising note per lit cell or built floor), horn / trumpet stabs and a short brass motif, timpani rolls into section changes, gran cassa + gong on the heaviest hit. Sampled CC0 libraries (VSCO 2 CE, VCSL). No piano-and-strings bed.
- **Original sound alternates with music**: in one section the orchestra stops and a real recording (a rivet gun, a machine, a crowd) carries the cut alone, each burst trimmed to its shot. Bring it in a beat early (J-cut).
- **Foley for every printed action**: slam = low thump + steel clank; stamp = thump + clack; wipe and iris = whoosh; marker = filtered noise scrub; paper = band-passed rustle; floors and cells = pitched clanks.
- **Layers**: ambience (a real site / street recording, about 20 dB under the voice, out for the silence), foley ducked 35 % under speech, music ducked 70 % under speech (≈ 10 dB under the voice), narration compressed and levelled by RMS. Master to −14 LUFS with the peaks limited to −1.8 dBFS before `mux.sh`, and a 25 ms fade at sample 0 (an AAC encode overshoots on a hit at the first sample).
- **Narration** (when the film has one): short declarative lines, newsreel cadence, never more than one sentence over a format; numbers spelled out for TTS, digits in subtitles.

NATIVE MOVES
1. **The document is marked, not quoted**: the actual sentence on the scan gets a red bar; the number then becomes a red block.
2. **The document becomes the chart**: drawings printed on a page are lifted off, turned and stood on a ground line to scale.
3. **A + B → idea**: two kinds of image alternate faster and faster (the machine / the hands; a heading / the people it leaves out) until a card or a number states what the collision meant.
4. **Count the many**: a counter driven by the cuts, a grid that fills cell by cell, a tower floor by floor.
5. **People build the structure**: cut-out figures stacked into the thing they made (photomontage).
6. **Silence, then the heaviest hit** into the result.

FOOTAGE
1. **Ask first.** Before searching, ask the user for their own material: film, video, photographs, scans of documents (reports, brochures, letters, minutes), data. What they give leads; archives fill gaps.
2. **Where to look, and how to judge the licence** (`demo/tools/footage.py` searches, downloads and records licence, author and source page into `SOURCES.jsonl`; `footage.py md` writes `SOURCES.md`):
   - **Internet Archive**: read each item's `licenseurl` and description. Public Domain Mark / CC0 / CC BY are usable; *no licence field* is not. For a pre-1978 US film, check the title and end cards for a copyright notice: published 1930 or earlier = public domain in the US by date; published without notice before 1978 = public domain; with a notice, check renewal. Corroborate date and producer elsewhere (a library catalogue, an archive exhibit). Avoid YouTube mirrors, re-uploads with no provenance, colourised or "restored" versions.
   - **Wikimedia Commons**: read the file page's licence and its source institution. Public domain, CC0, CC BY (credit in full) are usable; CC BY-SA is not used in library demos.
   - **US National Archives (NARA)**, **Library of Congress**, **NASA**, other US federal agencies: federal works are public domain; check each item's rights statement. "No known restrictions" (LOC, Flickr Commons) is **not** clear enough: name it to the user or find the same image elsewhere with a clear statement.
   - **Museums with open access** (Met, Rijksmuseum, Smithsonian): CC0 where the object page says so; check that the item is still marked open access.
   - **University and city libraries** (e.g. Columbia's digitised ephemera on the Internet Archive): use their rights statement; "copyright status undetermined" is not usable.
3. **Tell the user before downloading**: what (title, years), from where (URL), under which licence and why. Keep `raw/` under about 1.5 GB; download serially (Commons rate-limits).
4. Record every asset with the seconds used (`footage.py used <name> "<source s> → film s"`), list it in `demo/CREDITS`, and keep the trail for anything judged "no notice".
5. Footage that is not of the subject itself (another city, another year) may stand in only with a tag that says exactly what it is.
```
