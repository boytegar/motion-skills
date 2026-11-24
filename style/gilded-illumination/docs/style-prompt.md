Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Gilded Illumination" style.

MATERIALS & RENDERING
- **Pigment shapes**: `fill` + `rimShade` (the same path stroked inside its own clip in a darker tone) + ink `stroke` + white-lead `hiLine` + white-lead dots. This recipe is the whole look; keep line widths tied to the camera so they never thin out to a hairline in wide shots.
- **Gold leaf**: a tileable texture laid in leaf squares (each square its own tone, a thin dark seam with a bright overlap ridge, a few cracks, specks of red bole), filled through a pattern into any path, then a burnish, a speck layer, an inner rim and an ink outline. Punched decoration is rows of tiny dark dots with a light fleck. Raised (gesso) gold gets a small cast shadow.
- **Light only on gold**: every gold path is also written into a screen-space mask; light from the picture's own sources is added only inside that mask, per leaf square (each square has its own tilt, so the light moves across the squares unevenly), with star-shaped flashes where it is brightest. Pigment never receives specular light.
- **Lit gold stays gold**: light pulls the leaf toward a warm gold with a cap on each colour channel (ours stops at 82% of the way), so the brightest leaf still shows its squares and seams and never burns to white or flat yellow. A soft bloom, if any, belongs only to the most strongly lit leaf and stays small; never a wash over the frame.
- **Ground pigment (ultramarine and friends)**: a granular pattern (coarse grains, not noise) mapped between a dark and a light tone of the hue.
- *Reference:* vellum (procedural sheet with grain and stains) is kept for the parts that are written on: speech scrolls and the end plaque.
- *Reference:* static and live layers: anything that does not move can be pre-rendered under the camera; anything that grows, flashes or is lit is vector, every frame (ours draws everything every frame at ~50 ms).
- **Light comes from inside the picture**: a lamp, the sun, a band of light crossing the painting. Night is the pigments sinking toward blue; lit gold rises above it.
- *Reference:* infinite zoom: textures that must survive a big push (the leaf in a macro shot) are generated again at higher resolution for that region and cross-faded by log zoom.

COLOUR LOGIC
- The ground is ultramarine (sky) or gold (gold-ground panels and frames). Everything else is a short list of medieval pigments: lapis/ultramarine blue, vermilion, verdigris green, rose, azure, lilac-grey, white lead, iron-gall brown ink, gold, rarely silver. Ochre only in small, clearly matte places (hair, a roof), never on large areas next to gold.
- Each pigment comes in three values only: body, darker rim, white-lead highlight.
- Blue and gold are the pair the eye goes to; keep the biggest blue field and the brightest gold for the moment that matters. Vermilion is for the hero, frames and marks of importance.
- *Reference palette:* ultramarine `#22379a`, vermilion `#c8321f`, verdigris `#3f8f7a`, rose `#d97a8a`, ink `#2a1a10`, gold leaf (texture, mid tone `#c99a3e`); vellum `#efe2c4` only for scrolls and the plaque.
- Brightness is carried by gold, not by the whole frame: from the darkest night to the brightest ending, the average brightness of the picture moves little; what changes is how much gold is lit and how brightly.
- Never: neon, pastel gradients, photographic skies, flat yellow used as gold, black outlines thicker than the shapes they hold, a pale or washed-out sky after the light returns, a global brightness or saturation boost to show "day".

TYPE & SUBTITLES
- *Reference:* titles and inscriptions in Uncial Antiqua (display capitals) or Grenze Gotisch (lowercase Gothic); explanatory words and subtitles in IM Fell English (roman and italic) for English and Noto Serif SC for Chinese, in brown ink, written on scrolls and plaques. One language per line.
- **Sizes**: text meant to be read ≥ 40 px at 1080p for titles, ≥ 32 px for English subtitles, ≥ 34 px for Chinese.
- **Reading time**: hold ≥ CJK ÷ 4.5 + other ÷ 15 + 1.5 s, and ≥ 1.8 s, from the first frame the block is complete. Text is revealed by a pen wipe, never typed letter by letter.
- **Subtitles are a speech scroll, fixed in one place**: a vellum banderole at the bottom centre of the frame (Chinese line, English line) that unrolls when a line is spoken. It is part of the style. It never moves sideways and never changes size: one width for the whole film (set by the longest line, shorter lines centred inside it), one height, one type size. Between two close lines the scroll stays open: the old words fade, then the new ones are pen-wiped on; two lines are never seen on top of each other.
- **Compose for the scroll**: the hero is kept out of the lower-centre band while a line is up (higher in frame, to one side, or the camera lowered), rather than moving the scroll.

MOTION QUALITY
- 30 fps, smooth (on ones). Things appear the way they are made: shapes pop up with a small spring, strokes are laid along their path, writing is wiped on, gold is laid then shines, a figure is painted in layers (gold, colour, ink and white lead).
- Easing is calm (ease-in-out, ease-out); springs only on small things appearing.
- What never moves: the painted picture and the gold already laid. Only the light, the hero and the camera move across them.
- Paint and gold build up; they never fade in as a whole.

CAMERA GRAMMAR
A virtual camera over a flat painted world (centre, log zoom, roll).
Framing: the painted world fills the frame edge to edge; a patterned border can be the camera's frame and, at the end, grow into the whole painting's frame. Allowed transitions: continuous zoom, push through a frame, foreground wipes, match cuts on shapes, hard cuts on the beat. Forbidden: page-turn wipes, slide transitions, cross-dissolves between unrelated scenes.

SOUND PALETTE
- *Reference band:* bowed fiddle (a solo violin played plainly stands in), harp, portative organ (a soft organ register), frame drum, recorder; bells (tubular, glockenspiel, hand chimes) for gold.
- Modes: dorian, mixolydian, aeolian; a Picardy third for arrival. No piano, no modern string section, no synth pads.
- *Reference foley:* the painted world's own sounds: wind, running water, footsteps on stone, a door, a far bell. Workshop sounds (quill, brush, mortar) are optional, for a film about making.
- Silence is a tool: drop the band to one held note when the picture asks a question.
- Gold gets a bell when it lights; the brightest moment of gold gets the fullest bar of music. Mix: voice on top, music under it, −14 LUFS.

NATIVE MOVES
- **The touch.** A small light (a flame, a spark) touches a large piece of leaf; two thin rings of gold light spread from the point of contact and the leaf squares light one after another as they pass, while rays around it light in turn. Restrained: no white flash, no full-frame bloom; the climax is gold lighting up, not the screen going bright. *Fits content like:* a launch; a first sale; the moment a thing starts working.
- **Gold laid, then lit.** Gold appears dark and warm; later a light crosses it and each leaf square flashes in turn while pigment stays matte. *Fits content like:* revealing a finished seal; an inventory of treasures; a medal list.
- **Lit by a lamp.** A light moves through the picture and the gold brightens only near it, then sinks back after it passes. *Fits content like:* a guide, a teacher, a product that changes what it touches.
- **Into the material.** Pull out of a macro of leaf (squares, seams, punches) into the picture. *Fits content like:* where a thing comes from; an opening that starts from texture.
- **Continuous narration.** The same figure appears several times in one painting, each at a later moment of the story; the final pull back shows them all. *Fits content like:* a journey; a process with stations; a life.
- **Through the frame.** A quatrefoil or foliate frame holds a close-up; the camera pushes through it into the big picture. *Fits content like:* introducing a hero; a chapter change.
- **Painted in layers.** A figure or a title appears in the order it is made: bole, gold and its flash, then colour, inner rim, ink and white lead. *Fits content like:* a reveal; a signature; a product being made.
- **Holy card.** One object centred in a painted frame on a gold ground, with its name on a scroll. *Fits content like:* a herb; a tool; a coin.
- **Brush fill.** A blank area is filled stroke by stroke in ground pigment, from the centre out. *Fits content like:* a design completed; a sky; a filled map region.
Our demo opens *into the material*, out of a macro of dark leaf, uses *the touch* as its climax and ends on the pull back to the whole painting, its border growing from the corners. A new film opens and ends another way: a gold ground with nothing on it yet and the first stroke of colour, a *holy card* whose gold is *laid, then lit*, a *brush fill* of the first blank area, or an ending on a locked frame while light rakes across one finished object.

PITFALLS OF THE MEDIUM
- **Shading creeps in.** Soft gradients make objects look plastic; model with the rim band and white-lead strokes only.
- **Flat-illustration drift.** Thin lines, no white dots, gold as plain yellow discs: the picture turns into a modern vector illustration. Keep the ink firm, add white-lead dots, show the leaf squares.
- **Gold everywhere, lit all the time.** If gold shines all the time, the light has nothing to reveal. Keep gold dark until its moment.
- **The white-out climax.** Pushing the lit leaf's brightness up (or adding a big glow) turns the brightest moment into a yellow-white blur and erases the leaf squares. Light the climax with more gold, not with more white: rings, squares lighting in turn, a capped gold tone.
- **Day by brightening.** Raising the whole frame's brightness or saturation, or turning the night sky light blue, to show that the light came back looks like a colour-correction slip and kills the deep ultramarine. The sky stays deep; the gold does the work.
- **Wandering subtitles.** A scroll that jumps between left, centre and right, or changes width with each line, reads as an error. Fix its place and size; move the camera or the hero instead.
- **Picket-fence mountains.** A row of spires of the same height reads as a fence. Group spires in clusters (one tall, smaller ones leaning off it), vary the gaps, and stack two or three ranges, paler and smaller behind.
- **Stacked-box mountains.** Rectangular ledges read as buildings; spires must lean and taper, with small stair-steps on both flanks.
- **The hero as a nun.** A hood over the face makes a small figure read as a nun or monk; keep the face and hair visible.
- **Blackletter legibility.** Grenze capitals are hard to read; set display capitals in Uncial and keep Gothic for lowercase words. Never set Chinese or long English in blackletter.
- **Fake Latin.** Use real public-domain Latin sentences or none.
- **Infinite zoom seams.** A single texture stretched over a big push turns to mush; regenerate the leaf at the zoomed region's resolution.
- **Render state.** Canvas state and cached patterns leak between frames; reset the context every frame and clear offscreen buffers.
```
