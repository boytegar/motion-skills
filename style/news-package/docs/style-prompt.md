Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Broadcast News Package" style.

FRAME AND LAYOUT (1920×1080)
Content layers sit lower than a subtitled layout would put them (a 60–110 px downward shift) so the bottom strap does not leave an empty band above it. When you remove subtitles and the crawl, re-lay the page; never leave the space they used.

PACKAGE PARTS (SPECS)

COLOUR
Structure to keep when you change the brand: one accent + one gold-like secondary + paper + ink, and a reserved alarm colour. Ink text on the accent; on paper, English secondary lines in a dark olive (`#6E5A1E`) because gold is too weak there.

TYPE
- Chinese: Noto Sans SC 900 (headlines, strap) / 700 (labels).
- Latin: Barlow Condensed 800 (numbers, names, the plate), 600 (secondary lines, quotes), 600 italic (the end-card English title).
- Mono: IBM Plex Mono 500 for datelines, sources, axis labels, all caps with 1–4 px tracking.
- All OFL, subset from Google Fonts by `demo/tools/fonts.sh` (the character list is read from `main.js`; delete `fonts/*.woff2` after a text change). No handwriting faces in this style.
- Every label is bilingual on one line each (Chinese first, English smaller); never wrap.

PICTURE LAYER AND GRADE
- When there is no footage, the picture layer is data graphics drawn in code plus, at most, a few **illustrative plates** (generated or stock), each tagged `ILLUSTRATION` for as long as it is on screen. They set a mood (a server room behind a model launch, a wafer behind chip stocks); they never stand in for the event, the place or the people.
- One grade for every plate: greyscale → auto black/white points (0.5 / 99.6 percentile) → γ 1.05 → 35 % S-curve → black 8, white 232 → duotone `#0E1013` → `#ECE7D8` (`tools/prep.py: grade()`), plus a left-dark ink gradient for type, a 50–55 % vignette and 7 % overlay grain cycling through six tiles. Graphics and the package are never grained.
- Real footage, if you have it, takes the same grade path; modern footage ≤ 1.5× scale, archive ≤ 2.5× after deblocking; crop out station logos.

EDITING GRAMMAR
- **120 BPM grid**, beat 0 at the end of the sting (1.5 s). Every cut and wipe on a beat or half-beat.
- **Shot lengths**: sting 1.5 s; rundown 4–5 s; story shots 3–7 s; a map long take (6–12 s) as the bulletin's breath; a comparison stays until every item has been fully on screen ≥ 2.5 s.
- **A package action every 0.5–1 s** inside a shot (flip, count, stamp, tick, strap change, map light, line step). No frame flat for more than 1 s; a strong action within 0.5 s of every cut.
- **Date every story**: a weekly bulletin's items happen on different days; the date sits in the rundown row, the eyebrow line, the readout or the document.
- **The order inside an item**: what (the name or the number) → how big (the line, the bars, the price board) → who it hits or who said it (bars, documents, a quote) → the verdict stamp.

SOUND PALETTE
Master −14 LUFS, true peak ≤ −1 dBTP (`core/render/mux.sh`). Samples: VSCO 2 CE and VCSL (CC0); foley synthesised (`core/audio/sfx.py`).

NATIVE MOVES
1. **Sting becomes the bug** — the brand plate slams in, then flies to the corner and stays there for the whole bulletin; opening and programme are one object.
2. **Strap tab = category** — the strap's tab changes category (模型 MODELS, 市场 MARKETS, 监管 POLICY …) with a white flash when a new story starts; no separate item cards between stories.
3. **Rundown, then ticked off** — the same list opens and closes the bulletin; at the end each row gets its tick.
4. **Spec card** — a launch reduced to its proportions: the whole and the part on one scale, then what it runs on, joined by a bracket to the total.
5. **Strap that never leaves** — one strap group from the first story to the recap; words flip, the strap itself never exits mid-programme.
6. **Split-flap before/after** — old and new value on one board, each tile flipping from the old to the new.
7. **Breaking the threshold** — a line advances one period per step towards a dashed threshold; the music stops for one beat before the last step and the crossing gets the heaviest hit.
8. **Date in the eyebrow** — every graphic's eyebrow line carries source and date in mono caps, so a weekly round-up never blurs which day each item happened.
9. **City sends the letters** — on a map long take, documents fly out of the sending city to their addressees; one world→screen function keeps labels and documents attached through push and pan.
10. **Quote card + verdict stamp** — the official's own words in type, with name, occasion and date; an underline on the key word as the voice says it; one stamp.
Where each move lives in the demo's code: [DEMO.md](DEMO.md) (build notes).
```
