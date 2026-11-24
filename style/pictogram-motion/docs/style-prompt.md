Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Pictogram Motion" style.

MATERIALS & RENDERING
- **Flat Canvas2D**, rendered clean; grain only at mux time. **One square grid** (cell ≈ 1/6 of the frame height) rules bands, transitions and walls.
- **Pattern model**: fill with a mid-dark tone; on grid nodes randomly place cell-radius discs in random tones and discs-with-inner-discs; then a fraction of cells get one motif from a library of generic geometry (quarter-discs, fish-scale waves, dot and triangle scales, concentric arcs, sun wheels, checkers, crescents…), rotated by multiples of 90°; never mascots, landmarks or logos. Draw it wider than the screen so any row can shift one cell.
- **Pictogram model**: one stroke weight family per film (round caps), solid fills, at most two tones per pictogram (a darker gore, a far side). Objects keep their silhouette at icon size; anything that cannot be read as a sign at 60 px is cut.
- **Figure model** (optional): unit = body height, origin at the hip; draw far limbs (mixed ~40 % toward the background), torso, near limbs, head. A second person sits behind, mixed further (~60 %). People can also appear only through their traces (footprints, a queue of feet).
- **Stage**: the hero pictogram (any object or figure) sits on a large **sun disc** in a neighbouring tone, crossed by one faint light sweep.
- **Environments** (water, nets, ramps, rails, rain) live in the hero's own buffer, masked to zero before the text column.
- **Frame rate**: 24 fps; motion blur by sub-frame averaging only inside transitions, whips and fast camera moves.

COLOUR LOGIC
- **A set of five hues + one dark neutral**, each hue in **five tones** (darkest → lightest, the middle one is the base). A chapter = one hue; a multi-hue chapter rotates through the set per card.
- Under type the pattern runs at **low contrast**; on chapter cards at **full contrast**.
- **Light hues take dark type and dark figures**, and their disc uses the lighter tone; dark hues take cream. On the dark neutral, the disc uses a lighter tone or it vanishes; its accent is one warm hue.
- One cream and one ink serve every hue. No outlines, no highlights except the sweep.
- Choose the set from the topic. Examples: warm festival (red, violet, forest green, gold, ochre); coastal (navy, teal, coral, sand, sky); kitchen (tomato, basil, saffron, aubergine, flour cream). Invent names for your hues; never borrow an event's colour names.

TYPE & SUBTITLES
- Title: a 900-weight grotesk (e.g. Inter Tight), 150–330 px at 1080p, tight tracking, auto-fitted. Second language: a heavy face under it. Meta: a mono (e.g. DM Mono) ~22 px, wide tracking; tags as mono chips.
- **HUD** on every card: mono corner text and a **progress bar of N ticks** (current tall, done dimmed, rest faint), N derived from the item list.
- **Bilingual rule**: one language is always the big title; the second language is chosen per version by a switch, never by hand-editing cards.
- The card type *is* the text layer. With narration, captions are two lines on a solid band in the chapter's darkest tone (an ink band on light hues), bottom left, never over the title: the second language in a heavy face over the English in mono; hold ≥ max(1.8 s, speech + 0.6 s). A card that must be read (a spec, a number) holds ≥ 1.6 s.
- Deliver an `.srt` of the title cards, generated from the edit list.

MOTION QUALITY
- **A grid of time**: pick one tempo for the film; every shot starts on a beat and every key action in a pose (release, contact, landing) lands on an integer beat.
- **Pace ladder**: normal cards last one bar; related sub-items become half-length rapid-fire runs. A second tempo comes from dividing the beat, never from changing tempo.
- Easing: exponential ease-out for entrances, exponential in-out for whips and wipes; a back ease once at most.
- **Type enters by mask slide-up**: title first, second line ~0.08 s later, tag after; meta lines type on without a caret. Short cards compress all timings.
- **Pictogram assembly**: the hero pictogram and its stage arrive as horizontal bands sliding into register from alternating sides.
- **Backgrounds breathe**: rows drift in alternating directions; on chapter cards they jump a quarter-cell on beats.
- **Transitions** snap to the grid: sliding rows, dropping columns, a push, quarter-discs from cell corners, an iris from the last figure, a tile flip. Rotate them; each lasts a few frames around the cut.
- **Motion blur only where things move**: sub-frame averaging inside transition and whip windows; static frames render once.

CAMERA GRAMMAR
Mostly a locked flat frame; energy comes from row shifts, transitions and figure actions. When the camera moves, it moves along the grid. Opening and ending come from the topic.
Framing: text and figure never overlap (one text column, one stage); horizontal poses move the stage outward. The HUD stays in the corners. Allowed transitions are the grid ones in §5; never dissolves, 3D camera moves, handheld shake or depth of field.

SOUND PALETTE
- **Type and music can carry the film alone**; a voice is optional, calm, one or two lines per chapter.
- **Percussion is the spine**: choose a drum family that suits the topic (frame drums, taiko, marching snares, steel pans, body percussion, an electronic kit) plus rim clicks, claps, shakers and crashes.
- Colour instruments on top: Karplus-Strong plucks (any lute, koto or harp-like voice, with pitch bends), mallets (marimba, glockenspiel), detuned-saw chord stabs, a synth bass, pads, risers and a drone.
- **Per-family accents**: every shot onset gets a hit chosen by chapter (a splash, a pok, a clank, a crack, a whoosh, a sizzle): foley reads the chapter's matter.
- **Picture-locked by data**: the score reads the edit list's shot times and places accents there; check every onset to ±15 ms.
- **Silence as a tool**: half a beat of drums-and-bass-muted before a big hit, or one whole bar stripped to a single ticking sound under a count.
- Mix: glue compressor and limiter, −14 LUFS, ≤ −1 dBTP.

NATIVE MOVES
A menu: use the ones your film needs.
- **One card = one item.** The count is the hook ("ALL N"). *Fits content like:* every tool in a workshop; every station of a subway line; every dish of a regional cuisine.
- **Chapter = colour.** Families become hues so the viewer always knows where they are. *Fits content like:* instrument sections of an orchestra; food groups; departments of a company.
- **One verb per item.** One body action readable in a bar, or a prop action (a gear turns, a cup pours). *Fits content like:* yoga poses; kitchen techniques; jobs on a construction site.
- **Pace ladder run.** A sub-family at double speed. *Fits content like:* four knife cuts; the bones of a hand; weekday routines.
- **Long take down a track.** Stations one screen apart, the camera whipping on each cut. *Fits content like:* the steps of a recipe; checkpoints of a marathon; stages of a rocket launch.
- **Count-up slam.** A big number rolls to its value on a bar. *Fits content like:* tonnes recycled; members of a club; years of a company.
- **Icon wall.** All pictograms return as a grid and turn one colour. *Fits content like:* a summary of a year; a periodic table of habits; a team roster.

PITFALLS OF THE MEDIUM
- Figure environments spread into the text column → render the figure layer into its own buffer and fade it out with a gradient mask before the text.
- Cream on light hues fails contrast → dark type and dark figures on light hues; their disc is the lighter tone.
- The sun disc vanishes on the dark neutral → use a lighter tone for it.
- Pattern canvases are large and seeded per card → cache with a bounded LRU; an unbounded cache runs out of memory with parallel renderers.
- Horizontal poses (swimming, lying, crawling) cross the title → move the stage outward and narrow the text column.
- Imitating a real identity too closely → your own palette, mark and slogan from the grid motifs.
```
