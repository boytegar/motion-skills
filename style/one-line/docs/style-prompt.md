Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "One-line Drawing" style.

MATERIALS & RENDERING
- **Paper**: warm white (around `#F4EFE4`), procedural mottling (soft-light, large tile), long fibres that fade out when zoomed out (or they shimmer), a sparse paper-tooth speckle over the ink at close zoom, light vignette. All textures anchored to the paper; they move with the camera.
- **Ink** is a filled polygon around the path (not `ctx.stroke`), mixed with paper colour to an opaque tone per point.
  - Width = base width of the current "voice" of the line × pen pressure × fine noise (±8 %); pressure from speed: `0.66 + 0.8·exp(−v/300)` (fast = thin, slow = fat).
  - The line's character can change along the path (base width, alpha, tremble, dryness); blend over a short stretch at boundaries.
  - Ink bleed: the same polygon slightly wider, blurred on an offscreen layer, composited at ~20 %.
  - Pooling: extra round dots where speed drops low.
  - Tremble (perpendicular noise) and dry-brush gaps (long-scale noise + thin bristle strands) for age, fear or fatigue.
  - A stop leaves a teardrop-shaped blot: tip at the nib, belly sagging downward as it spreads, a dark tide line at the rim.
  - **A screen-space minimum width**: widths scale with zoom like a line under a macro lens, but clamp the zoom used for width during pull-backs so a wide view keeps its weight.
- **The pen is never drawn**: a soft blurred wedge of shadow from the nib and a tiny contact dot. Hands, if any, appear only as shadows, each hand composited as one flat layer.
- **Path design**: chapters written as SVG path strings, sampled by arc length, joined by tangent-continuous connectors; time assigned per point weighted by curvature.

COLOUR LOGIC
- **Ink on paper.** A warm black (around `#1D1A17`) on warm white; nothing else fills the frame.
- **One accent at most, used once**, on a stretch of the line that means something; it returns to ink afterwards and stays visible in the final picture if the picture is revealed.
- Examples of an accent's job: red for love or danger; blue for water or cold; gold for money or a prize. Never a second accent.

TYPE & SUBTITLES
- **Text is written, like the line**: a handwriting face (e.g. Caveat) for subtitles, ink at ~84 %, no box, centred near the bottom, **written on** left to right (~0.2 s + ~17 ms per character) behind a soft paper-coloured halo so passing lines never cross the letters; fade out ~0.45 s. Hold ≥ max(1.8 s, speech + 0.6 s).
- Titles in a monoline connected script (e.g. Sacramento: itself a one-line drawing), written on in empty paper away from the drawing, gone before the camera moves into them.

MOTION QUALITY
- **Everything on ones** (24 fps): the appeal is the continuous act of drawing; stepping breaks it.
- **Timing = chapter windows + curvature**: within a window, time per point is weighted by `1 + K·turn` (sharp turns get more time, smoothed so speed never jumps). Pin musical beats with marks: by arc-length fraction or by position ("the second time the pen passes here"). A stop is two marks at the same place.
- Corners (speed minima at high turn) can be exported to the soundtrack as candidate note onsets.
- Speeds: ~300–700 units/s for travelling lines, ~50–150 for small details. The most important stroke is usually the slowest.
- **Retracing** over an existing line is legal and invisible (it only thickens slightly).

CAMERA GRAMMAR
A vocabulary, not a route. The camera is one continuous move: composition centre, zoom, a few degrees of roll, and a follow weight (0 = composed, 1 = locked to the nib); the follow point averages the nib over roughly −0.55…+0.4 s (lag and anticipation), with a soft clamp keeping the nib inside the central ~70 % of the frame. Opening and ending come from the topic.
No cuts. Transitions are morphs: the last stroke of one image is the first stroke of the next.

SOUND PALETTE
- **Music: one solo instrument, one unbroken melodic line**, the sound equivalent of the drawing: a bowed or plucked string (cello, violin, viola, harp), a solo woodwind (clarinet, flute, bassoon), a hummed voice, a single guitar. A second colour only as a rare accent (a chime, a pizzicato stretch). No pads.
- **Write the score to the drawing**: key notes on marks and corners; fast line = short notes; a stopped line = one long note fading into silence; the highest note on the most important touch.
- **The pen on paper is the most important sound**, synthesised from the pen speed: band-passed noise (brighter when fast, duller when slow), random paper-fibre ticks with density ∝ speed, amplitude ∝ √speed, panned with the nib's screen position; broken up by dry gaps; lighter and jerkier for a young hand. Plus: a wooden tap on the first touch, a faint wet swell for a blot, a tiny rub for a handoff, a page lift.
- **Silence is literal**: zero the music bus including reverb tails; drop room tone.
- **Mix**: music ducks ~5 dB and the pen ~4 dB under voice; −14 LUFS; grain 0 (the paper is in the render).
- **Voice**: warm, few short lines that leave space for the line; place key words on picture beats with whisper word timestamps.

NATIVE MOVES
A menu: use the ones your story needs.
- **The pen never lifts.** Anything that is one continuous thing. *Fits content like:* a river from source to sea; a relay race; a production line.
- **The line is time.** The stroke's character changes with age, mood or strain. *Fits content like:* a building aging; a runner tiring; a machine wearing out.
- **Transformation.** One image morphs into the next. *Fits content like:* a seed becoming a tree becoming a table; a sketch becoming a product; a word becoming its object.
- **Stopping is an event.** The nib stops, ink pools, music cuts to silence (1.5–3 s). *Fits content like:* a power cut; a pause for a minute's silence; a hesitation before a signature.
- **Reverse-designed scale reveal.** Every vignette is secretly part of one big picture seen at the end. *Fits content like:* a city's landmarks forming its skyline; a company's milestones forming its logo; a recipe's steps forming the dish.
- **Handoff.** The pen passes to another hand (as a shadow) and a new line begins. *Fits content like:* a teacher and a student; a founder and a successor; one generation to the next.
- **Line as graph.** The line becomes a chart while staying a drawing. *Fits content like:* a heartbeat; a temperature record; a football match's momentum.

PITFALLS OF THE MEDIUM
- **Stray connectors ruin a reveal**: every extra line reads as a wrinkle, beard or mask. Fix the topology (where each chapter enters and leaves), not the drawing. Plan it like an Euler path: a closed motif is a circuit (you leave where you came in); retrace or reorder chapters.
- Each region of a final picture is drawn exactly once; connectors must fall on natural lines of that picture.
- **Test the reveal early and often**: render the whole path at once at final scale after every change.
- Lines drawn too close braid together once width is applied; keep a gap.
- A round blot reads as a mole → a sagging teardrop.
- Thin lines at wide zoom → clamp the width zoom.
- Translucent hand shadows double-darken where they overlap → one layer per hand.
- Titles collide with the drawing as the camera moves → check every frame of the title's window.
- Whisper mishears short lines → rephrase rather than fight it.
```
