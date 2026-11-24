Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Woodcut Print" style.

MATERIALS & RENDERING
- **Print frame**: the printed image area sits inside a paper margin; the wider bottom margin holds captions. The image edge is the block edge: ragged ink, never a clean rectangle.
- **Knife classes** (widths in 1080p pixels at 1× camera):
- **Hatching (Doré grammar in wood)**: line **direction follows form** (a round object is wrapped in arcs, slopes follow the fall line, fire and light radiate, cloth follows its folds). **Width follows tone**: bright = wide cuts almost merging into white, mid-tone = thin cuts, dark = no cuts. Far = dense and fine, near = sparse and coarse. Every line is a separate cut (30–160 px, blunt entry, tapered exit, chipped edge), never a continuous machine line.
- **Figures**: black silhouette + white knife halo + grazing-light hatching on the lit edge only (light z ≈ 0.2, so flat areas stay black). **Skin carved white, features left black** (the wordless-novel rule).
- **Close-ups (hands, objects)**: sculpt a small height field (capsules, blobs, planks, creases), light it and run it through `woodcutFilter` (§10), so cuts follow the isophotes like an engraver's.
- **Printing**: the mask (white = cut) goes through a WebGL2 compositor: knife burr on edges, ±6 % ink unevenness, wood-grain streaks, specks where ink didn't take, a mottled colour plate with a small fixed misregistration. Block mode: glossy inked wood with grain sheen, pale wood in the grooves.

COLOUR LOGIC
- Two values carry everything: near-black ink and warm laid-paper white. The raw wood colour appears only in shots that show the block itself.
- **At most one colour plate**, printed only where white has been carved, so it glows out of the cuts and never sits on the black. A colour plate over ink is nearly invisible, so plan it on carved white.
- The colour is reserved for the one thing that matters (a flame, molten metal, a heart, a signal lamp). It can spread at a peak and shrink to a single highlight afterwards; it is never decoration.
- Examples of a plate: a burnt copper-orange for heat; a cold Prussian blue for water or night; a deep oxblood for a wound or a seal. Choose it from the topic.

TYPE & SUBTITLES
- **Captions are letterpress in the paper margin**, below the plate, like a plate caption in a woodcut novel. They never cover the image. A letterpress old-style face (IM Fell English, OFL) around 40–44 px, in ink, pushed through the same print shader. The ink comes up in 0.1 s with no slide and fades over 0.15 s. Hold ≥ max(1.8 s, speech + 0.6 s).
- **Titles are carved, not typed**: cut into the block (IM Fell English SC), mirrored if the block is shown before the print is pulled.
- Any closing text is a fresh print: letterpress in the margin or carved out of a black panel.

MOTION QUALITY
- **Characters on twos (12 fps)**; camera, particles, fire and light on ones (24 fps). Hatching lives in each body part's local space, so cuts move with the limb and don't boil; only fire and liquids re-cut every 2 frames, as deliberate boil.
- **Anticipation / action / follow-through** on every key action: a few frames back → a 2-frame strike → settle + a short shake.
- **Carve reveal**: each stroke has its own start time and speed (0.08–0.35 s). Reveal keys: `light` (bright areas first, coarse to fine), `radial` (from a point: rays, a pour), `down`, or any function.
- **Hard cut = a new impression**: two frames of misregistration (a few pixels, then back).
- **Paper transition** (the one transition grammar): brayer rolls ink (a wet highlight sweeps) → paper lays down → baren spirals, the image soaks through → the sheet peels on a cylinder toward camera, heavy then fast, its back showing the mirrored show-through.
- What never moves: the paper margin.

CAMERA GRAMMAR
The camera moves over the paper; the margin stays fixed. The knife tip, or the brightest cut, is the focus.
Opening and ending come from the topic. Framing: black masses on carved white or white on uncut black, never grey on grey; text only in the margin. Transitions: hard cut (new impression) or the peel; never a dissolve.

SOUND PALETTE
- **Instruments**: low strings (contrabass, cellos; spiccato, pizzicato), wood and skin (log drum, woodblock, frame drum), gran cassa, timpani, one tam-tam, synthesised struck metal (modal: 3–5 inharmonic partials + hammer transient); a bowed saw or hurdy-gurdy drone for a folk colour.
- **Options**: withhold the timbre of the story's object until it appears; a metric modulation instead of a tempo jump; a modal shift inside a long decay; one drone under a locked plate.
- **Foley follows the material**, all synthesisable: knife into wood (transient + band-passed fibre tearing tied to knife speed), chips, brayer tack, paper air, baren rasp, the wet peel; metals by size (large = low, small = high and pure); resonant objects as modal models plus echoes for distance.
- **Silence is a tool**: near-silence with one faint ambience, or true digital silence before the loudest moment; the first sound after it is chosen.
- **J/L cuts** carry sound across plates and peels.
- **Mix**: music ducks ~8 dB under voice and a little under key foley; **foley ducks too** under voice. Pan hard transients off centre. −14 LUFS.
- **Voice**: few lines, low and plain, like captions spoken; the plates tell the rest.

NATIVE MOVES
A menu: use the ones your story needs.
- **Light is carved.** A burst of radial cuts from a point. *Fits content like:* a power grid switched on; a scientific insight; a lighthouse restored.
- **Carving = time.** Patient work on screen, cut by cut. *Fits content like:* a year of training; a building rising; a craft apprenticeship.
- **Mirrored block → true print.** A backwards word reads true when the sheet is pulled. *Fits content like:* a vote result; a name cleared; a misunderstanding put right.
- **Stab cuts as texture.** White dots can freeze mid-air or be inked back in until the black is clean. *Fits content like:* stars going out over a city; rain stopping; a disease receding on a map.
- **The one colour plate spreads and withdraws.** *Fits content like:* a blood donation; a sunrise over a harvest; a single lit window in a blackout.
- **Filter a sculpt or a frame** into form-following cuts. *Fits content like:* a portrait of a real person drawn from a photo; a machine part; an archive image re-cut.
- **Wordless-novel sequencing.** Self-contained plates, few or no lines; a gap can be silent. *Fits content like:* a migration; a strike; a life in seven plates.

PITFALLS OF THE MEDIUM
- **White marks on white areas are invisible** → stab cuts need dark masses behind them.
- **Hatching every surface looks like fur or rain** → grazing light; 3–6 cuts on the lit edge only.
- **Black faces with white cuts don't read small** → skin white, features black, a black keep-line around white parts.
- **Hand-drawn close-up props look like mittens and blobs** → height field + `woodcutFilter`.
- **Short hatch segments with a tonal wobble turn into zig-zag chevrons** → long cuts (14–50 × spacing), one broad highlight band, a thin rim light, a stronger halo.
- **Steam or smoke on top of the subject reads as flames** → draw it behind, so only wisps escape.
- **Lazily cached strokes shared between shots** make frames depend on the render worker → every cache builds itself on first use.
- **Fine hatching + ink noise + grain inflate the file** → a higher CRF with `-tune grain`; check a 1:1 crop. `woodcutFilter` NaN in flat areas → clamp before `sqrt`.
```
