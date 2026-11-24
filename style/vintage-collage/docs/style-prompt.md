Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Vintage Engraving Collage" style.

MATERIALS & RENDERING
- **Cut-outs**: a scan, a hand-traced or computed mask, the scan's grey turned into ink amount on a cream card, a white margin dilated around the mask with a slightly torn edge. Line art with a transparent ground gets its silhouette by closing and filling holes. Hand-colour (brass, mustard) only inside the cut, never as a gradient.
- **Separated plates**: a two-colour print (a poster, a label) is split into its ink plates by hue and re-inked in the palette; each plate can then be printed, shifted or turned on its own. Upscale before tightening the ink edge so lines stay crisp.
- **Torn colour papers**: flat colour with a faint mottle, the edge displaced by fractal noise, a thin white fibre fringe outside it.
- **Old paper**: a mottled cream base with fibres and faint foxing, lighter than a raw scan; it can be tiled with mirrored copies, so the table extends without seams.
- **Halftone**: any photograph enters as a 45° dot-screen news clipping on newsprint, torn edges, a typed caption.
- **Paper physics in the composite**: under each piece a cut-edge copy in a darker paper colour, offset down-right; two pre-blurred shadows mixed by height; the back side is paper colour with about 10 % of the mirrored ink showing through.
- **Bake, then move**: the heavy work (masks, separations, tearing, textures, engraving, halftone, typed slips) is baked once into sprites; the page only moves sprites. Grain is added in the mux, not drawn.

COLOUR LOGIC
- A restrained palette of six: old paper, cream card, warm black ink, ochre red, teal, mustard. Large areas are paper and flat colour; heavy black is limited to engraved lines, small slips and silhouettes.
- A printed subject keeps its own inks, re-inked once into two tones close to the palette (for a two-colour poster: its red and its black), and only the subject uses them.
- One colour carries meaning per film beyond decoration (ochre for the data wash and one key figure, mustard for the slip that names it); flat colour papers are scenery.
- Example (our demo): paper #ECDFC2, card #F6EEDB, ink #221C17, ochre #C65F3A, teal #2F7F7A, mustard #E1AB3B; subject inks #C8463F and #2F5D8C; cut edge #B49D78, paper back #E9DFC8.

TYPE & SUBTITLES
- All type is printed on its own slip of paper and placed like any other piece: no type floats on the frame, no outlines or drop shadows on letters, only the slip's shadow.
- Faces with a period feel, all OFL: an old-style serif (e.g. IM FELL English roman, italic and small caps) for titles and labels; a typewriter (e.g. Courier Prime) for slips, receipts and data; a CJK serif (e.g. Noto Serif SC, subset) for Chinese.
- **Ransom letters** for a hero word or number: one slip per character, rotating fonts, papers and inks, slapped in one per eighth note. Old-style serifs have old-style figures (1 like I, 0 like o): digits come from a lining face.
- **Typewriter slips** type out letter by letter on a blank slip (monospace advance), with a key sound per letter.
- On screen: Chinese ≥ 34 px, English labels ≥ 24 px, data numbers large enough to read at the widest framing they appear in.
- **Subtitles** (when there is a voice): two slips stacked at the bottom left, a black newsprint slip with the Chinese line (about 40 px) over a cream typewriter slip with the English line (about 30 px) typing out. One line per language, split long lines into two cues. Keep that corner (about 1100 × 180 px) free of anything the viewer must see.
- Comparisons and verdicts: every compared item fully in frame and held ≥ 2.5 s.

MOTION QUALITY
- **On twos** (12 drawings a second) for every paper piece; the camera moves smoothly every frame.
- Slide in from off-frame with a small overshoot and a 2–3 drawing side-to-side settle; drops accelerate and bounce once; a slap arrives in two drawings (a slightly larger copy high above, then down).
- Moving pieces carry a hand jitter (≈ 0.7 px, 0.25° per drawing); resting pieces are still.
- Hinges turn around a pin and overshoot; a sheet can turn 180° flat on the table.
- Flips around the vertical axis show the paper back and a wider cut edge mid-turn.
- State changes are substitutions, not morphs: swap in a new cut-out (an open lid for a closed one, a printed sheet for a blank one).
- Never a smooth software scale, never transparency fades between pieces.

CAMERA GRAMMAR
A rostrum camera over a paper table: 2D only, but it can push, pull, pan, rise and tilt across a table much larger than the frame.
- Wide establishing on the assembled collage; follow a moving piece; push to read a detail; pull back for scale.
- **Through a shape**: push into a hole in the paper (an oval frame, a window, an empty album slot) and the next scene is already inside it.
- **Whip pan** sideways along the table, a paper seam between the two scenes.
- **Match cuts** on the same piece at a new size; match on action (a piece leaves the top of frame, enters the next shot from below).
- **Rise** along a growing data engraving, then pull back so the whole comparison is in frame.
- Keep the view on the paper: zoom ≥ 1 on a single-sheet table, or tile the table for wide pulls.

SOUND PALETTE
- Paper foley for every piece: slides (band-passed noise with fibre grain), pats when a piece lands, slaps for type, flutter for flips and wings, scissors snips, typewriter keys and the carriage bell, a rubbing roller, the scrape of a growing chart.
- Score from small-ensemble period instruments: tuba and bassoon staccato, pizzicato strings, clarinet, xylophone, glockenspiel, harp, woodblock, timpani; a march or music-hall oom-pah, with a minor turn where the story turns. No generic piano and strings.
- One near-silence before the turn, and transitions carried by sound (a whoosh through a shape, a whip).
- With a voice: compress it and set it about 10 dB over the music; duck the music's 300 Hz–4 kHz band while it speaks.

NATIVE MOVES
- **Cut it out**: a found print enters as a cut-out with its white margin; it can be cut again on screen with scissors and scatter.
- **Assemble**: torn colour papers and cut-outs slide in one by one and build the scene or the subject.
- **Turn the sheet**: a whole sheet or piece turns 180° flat on the table, or flips to show its back.
- **Through the hole**: the camera enters a shape cut in the paper into the next scene.
- **Ransom and typewriter**: words arrive letter by letter as slapped clippings or typed slips.
- **Data engraving**: a value grows as engraved lines, the camera rises with it, a torn break marks a broken scale.
Our demo opens on *assemble* with ransom letters, uses *turn the sheet* as its turn and ends *through the hole* back into the first scene. A new film opens and ends another way: a single cut-out falling into frame and landing with a thud, a typewriter slip typing the first line before anything else appears, an ending on a last stamp or seal pressed onto the collage, or a rise until the whole table reads as one poster.

PITFALLS OF THE MEDIUM
- A cut-out without a margin or shadow reads as a digital sticker; one with a fat, even margin reads as a sticker too. Keep the margin thin and slightly torn.
- Shadows from different light directions, or shadows that do not change with height, flatten the table.
- Smooth scaling, fades and easing curves that never overshoot read as software, not paper.
- Too many pieces: one subject per frame, about twenty flying pieces at most, leave the bottom-left free for subtitles.
- Upscaled scans go soft and blotchy: separate and tighten the ink, use the largest public-domain scan, and keep cut-outs at or below their source size where possible.
- Old-style serif figures in numbers, letters with outlines or glows, system fonts.
- Copyrighted or "no known restrictions" images: only public domain and CC0 prints, each one in the credits.
```
