Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Caption News" style.

FRAME AND LAYOUT (1920×1080)
Under the caption: a bottom gradient (0 → 84 % from y 560) plus a soft dark radial from the bottom-left corner; a top gradient (55 % → 0 over 200 px) under the bug and the credit. Data cards use a lighter gradient that starts at y 820. Graphics and charts sit right of centre and bleed to the frame edge; their title and source sit top left, never centred.
Vertical 9:16 is a different layout, not a crop: put the caption under a 1080×1560 footage band (horizontal sources then scale ≤ 1.5×), stack the chunks on two lines there (the one-line rule is for 16:9 subtitles), and move the bug and source tag into the top safe area.

EDITING GRAMMAR
- **Cuts on a 120 BPM half-beat grid** (every cut at a multiple of 0.5 s). Footage shots 1.5–3 s; data cards and documents 3–4.5 s; a shot that holds a comparison stays until every item has been fully on screen ≥ 2.5 s.
- **Captions cross cuts.** A caption can run over two shots (picture changes, words don't); captions change at most every ~2.5 s even when shots change every 1.5 s. Reading time, not shot length, sets the caption clock.
- **The order of a fact**: footage (feeling) → the number on a card → the document line that says it → the consequence. Never a number without its source tag in the same shot.
- **Transitions are few and fixed**: hard cut (default); a 1–2-frame white flash on the turn and on the freeze; a card sliding in from the right with speed streaks (0.18 s); a split screen opening from its divider; grid panels rising staggered (0.1 s apart); a punch-in (0.3 s ease-out to ~1.25×) on the culprit; a **mask transition** where a photo grows out of a map shape. No dissolves, no page turns, no 3D flips.
- **Pace**: a strong action within 0.5 s of every shot; no frame flat for more than 1 s (rolling counters, line drawing, highlighter sweeps, slow pushes keep it alive); one held breath (2–3 s) after the turn; two real silences.

CAPTION CARD (THE SUBTITLE *IS* THE GRAPHIC)
- **Chinese line**: Noto Sans SC 900, 88 px (verdict lines 110 px), ivory `#F7F5EF`, 20 px dark soft shadow. Split into 2–3 chunks with `|` in the timeline; each chunk slams 0.12 s after the previous: scale 1.3 → 1 in 0.13 s (back ease, overshoot 1.6), opacity full in 0.04 s.
- **Keyword**: `{…}` inside a chunk; a honey-yellow block `#FFC21A` wipes left → right in 0.12 s behind it, text turns ink `#0E0E0C`. One keyword per caption, at most.
- **English line**: Inter Tight 500, 33 px, ivory, fades and rises 10 px in 0.18 s, 0.1 s after the first chunk. It is the spoken line (numbers as digits), not a translation of the Chinese.
- **One line per language, always.** If either language would wrap, split the sentence into two captions. A caption is on screen ≥ 1.8 s and ideally ≥ the spoken line + 0.6 s; the Chinese is a paraphrase short enough to read in that time (≤ ~14 characters).
- The `.srt` is generated from the same timeline (`timeline.json` → `tools/cues.py` → `core/render/srt.py`): one Chinese line, one English line per cue.
- A text-only beat (caption with no voice) is native to the style: use it for the breath after the turn.

GRAPHICS
- **Ledger card** (the data-card ground): `#151512` sheet, faint ruled lines every 54 px (4.5 % white), a thin red double margin at x 78/86 — the "account book" that ties every chart together. It slides in from the right over 0.18 s; inside, a slow push (×1.025–1.03 over the shot).
- **Colour meaning**: honey `#FFC21A` = the point / gains / the highlight; red `#FF4A3D` = losses, the spike, the bad number; ivory = everything else; grey `#8D8B83` = labels and units; `#5E5C55` = the bars that don't matter.
- **Numbers**: Inter Tight 800 with fixed-width digit cells (`num()` in `demo/main.js`), so a rolling counter never jitters. Counters roll with ease-out and turn their colour when they arrive. Units in Chinese (箱, 万箱) set smaller after the number.
- **Charts**: titled top left in Chinese (Noto Sans SC 900 64 px, ≤ 6 characters so it can be read in the shot) with one grey English line; lines draw segment by segment (eio); the key point gets a bigger dot, an expanding ring (0.7 s) and a block label; bars grow from the left one after another (first one slower and longer if it is the point).
- **Map**: state shapes from Natural Earth (maps follow DIRECTOR.md §13), projected (Albers), sequential fill in five steps of one hue, states without data in dark grey with a "未单列" legend swatch; the subject state outlined in yellow with a leader and a label; the map pushes in on it.
- **Documents**: the PDF page rendered at 220 dpi as white paper on a dark desk (±1–2° tilt, soft shadow, vignette), pushed in slowly; highlighter rectangles placed from `pdftotext -bbox` coordinates (1 pt = 3.0556 px), `multiply` blend, swept left → right in 0.3–0.5 s; the number read from it is re-set as a big yellow block beside the page. The printed text is picture; what must be read is re-set in type.
- **Split screen**: two halves with a 4 px yellow divider that drops first; the "bad" half desaturated, darkened and tinted red, the "good" half in full grade; a label block (Chinese + English word) and a counter in each; a shared stamp centred on the divider.
- **Grid**: three panels (632 px, 12 px gaps) rising staggered; each one lights up (yellow border, label turns yellow) on the word that names it; numbered 1–2–3.
- **Freeze frame**: the shot stops on a frame, a 0.14 s white flash, punch 1.0 → 1.12, saturation drops to 45 %, yellow corner brackets close in on the subject, a timecode chip with a pause mark appears top right.
- **Title sting**: a yellow ruled page (the ledger again, in yellow) wipes across the left 45 % over live footage; the title slams, the English title and a dateline chip follow; the page wipes off left and the title becomes the channel bug.

GRADE AND IMAGE QUALITY
- One grade for every piece of footage: `contrast(1.07) saturate(0.92) brightness(0.98)` + a 6 % warm soft-light layer; documents 4 %. Light grain in the master (`mux.sh … 30 1`).
- **Scale limits**: modern footage ≤ 1.5× (prepare frames at their final size with ffmpeg: crop, lanczos, `unsharp=5:5:0.45`); photos ≤ 1:1 of their source; archive material ≤ 2.5× after deblocking. Pushes stay inside those limits — a 720p source already at 1.5× gets no push.
- Remove station logos by cropping (never cover them with a caption), blur legible private details (phone numbers, names on hives, number plates) with `delogo`, skip shots with burned-in text.
- Captions, numbers and charts are always vector, never scaled footage.

SOUND PALETTE
Master −14 LUFS, true peak ≤ −1 dBTP (`core/render/mux.sh`).

NATIVE MOVES
1. **Frame-4 hook** — kicker line, then a giant number in a yellow block rolling to its value in 0.45 s, then the unit and the subject word (`hook()`).
2. **Chunk slam with one yellow keyword** (`capCard()`).
3. **Proportion by darkness** — a yellow line sweeps across the footage to the percentage; what it passed goes grey and dark; the figure appears on the dark side (`shotHalf()`). Works for any "X % of …".
4. **The document says it** — headline or sentence highlighted from PDF bbox coordinates, then the number popped as a yellow block beside the paper (`paper()`, `shotDoc6()`, `shotQuote()`).
5. **Freeze and question** — the image stops on the viewer's question, music cut to silence (`shotFreeze()`).
6. **Ledger split** — two counters racing side by side, the difference stamped after both settle (`shotSplit()`).
7. **Lit grid** — three ways / three causes, each panel lighting on its word (`shotGrid()`).
8. **Punch and circle** — 0.3 s punch-in on the culprit, a circle drawn round it, a leader to its name and its Latin name (`shotSem()`).
9. **Map to ground** — the map pushes into a region, then a photo of that place grows out of the region's outline (`shotMap()`).

MATERIAL
1. **Ask the user first** whether they have their own footage, photos, documents or data (and the rights to them). Their material comes first; the sources below are the fallback.
2. **Where to look** (judge every file on its own page, not the search result):
   - **Wikimedia Commons** (video and photos): read the licence on each file page. Fine: CC0, public domain, CC BY (credit author, title, licence on screen and in `CREDITS`). Not used: CC BY-SA, "no known restrictions", unclear licences, YouTube re-uploads, colourised or "restored" versions.
   - **U.S. federal agencies** (USDA, USGS, NASA, NOAA, NARA): works of federal employees are public domain; check that the page does not credit a contractor or a third party. NASA: https://images.nasa.gov. NARA: https://catalog.archives.gov.
   - **Internet Archive**: only items whose own page states public domain or a CC0/CC BY licence; U.S. works published before 1931 are public domain.
   - **Library of Congress**: read each item's rights statement; "no known restrictions" is not enough.
   - **Official documents and data** (reports, statistics, court filings) from the agency's own site: the source of every fact; keep `FACTS.md` (claim, URL, quoted wording, derived sums with their arithmetic).
   - **Map geometry**: Natural Earth (public domain). Its default borders are not usable for China: DIRECTOR.md §13.
3. **Before downloading, tell the user** what you will fetch, from where and under which licence, and keep within a size budget. The finder copied into this style, `demo/tools/footage.py` (`search commons|archive|nasa`, `fetch`, `used`, `md`), logs licence, author, page and the seconds used of each file into `SOURCES.jsonl` and writes `SOURCES.md`; `demo/tools/commons-tools/peek.py` previews a Commons video from thumbnails before downloading it, `seg.py` downloads only a segment of a long one, `commons_thumb.py` takes a standard-width thumbnail when the original is rate-limited. Wikimedia rate-limits bursts (HTTP 429): one request at a time with pauses.
4. **People on screen**: no close-up or slow motion of an identifiable private person; public figures and people in the news event are fine. Never use generated images as footage.
```
