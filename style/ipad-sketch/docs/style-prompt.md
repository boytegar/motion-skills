Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Tablet Sketch Explainer" style.

MATERIALS & RENDERING
- **Paper:** cream canvas with a fixed paper grain (fine value noise plus a slow mottle), anchored to the canvas so it pans, zooms and turns with it. A light screen-space corner falloff only.
- **Pencil:** a stamped-dot brush along a Catmull-Rom spline, one stamp every ~0.5 canvas units; pressure = taper in and out × slow noise, driving both width and density; a small low-frequency wobble along the normal. Within one stroke take the max coverage (no self-darkening).
- **Grain threshold:** at composite, pencil opacity = clamp((coverage · 0.9 − grain · 0.85 + 0.32) · 2.1). Light pressure is eaten by the paper tooth: broken, grainy edges. Lettering goes through the same threshold, so words look pencilled too.
- **Colour blocks:** the object outline, translated off the line and displaced along its normal by noise (ragged edge), filled flat and blended *multiply* under the pencil. Highlights, seams and inner layers are holes cut out of the block (the paper shows through). Objects in front cut the blocks behind them.
- **Colouring path:** a block is revealed along a back-and-forth (boustrophedon) path across its shape, so a half-coloured block ends in visible strokes. A block can stop at any fraction.
- **Re-registering:** to change an object's colour, erase what the new block covers (slightly wider, so no old fringe peeks out) and colour again; the line stays.
- **Layers, in order:** paper → colour blocks → light construction pencil (≈55% opacity) → graphite → coral accent pencil.
- Draw the canvas into buffers at ≥ 2× its unit size so a 2.5× push-in still shows clean grain; the camera scales the buffer, the grain is never generated in screen space.

COLOUR LOGIC
- Neutral ground: a warm cream paper; graphite (warm dark brown-grey, never pure black) for every line and word.
- **System colours (every film):** a light graphite for construction lines, one coral for the "explaining" marks (arrows, underlines, burst ticks, the one number that matters), one mustard for the highlight block under the key word of a title.
- **Subject colours (per film):** soft, warm, chalky versions of the subject's real colours; each gets a darker variant of the same hue for a second, smaller shadow block (≈40% multiply). Four to eight subject hues in a film.
- Colour is information: when a thing changes (ripens, dries, cooks, ages) change its block, not its line. A swatch strip that records each state is the style's way to chart.
- Example: paper #F3EADB, graphite #43362E, light #A8998A, coral #DC6A4C, mustard #F1C04F.

TYPE & SUBTITLES
- **Lettering on the page** is handwriting written stroke by stroke along each glyph's skeleton, then put through the pencil threshold. A casual marker-hand Latin (e.g. Caveat Bold for titles, Medium for notes) and a pen-hand Chinese (e.g. Long Cang) for one short line; all OFL.
- Roles: a title (≈100 canvas px) with a mustard block under one word and a coral underline; chapter capsules (circled number + word on an offset block) when a film has chapters; notes 32–50 px with a short pencil leader to what they name. Chinese on the page ≥ 34 px on screen.
- Notes are placed in the empty space next to their object, never centred stacks, never a header bar.
- **Subtitles** live outside the canvas, as a small paper chip: a light cream rounded strip over a darker paper block offset down-right (the colour-block grammar), Chinese on top in a readable text face (e.g. LXGW WenKai, 40 px), English below in the handwriting face (40 px), left aligned, one line each. The chip wipes in left to right like a colour block (≈0.2 s), the words fade in after; bottom left by default, bottom right when the subject sits there. Hold ≥ max(1.8 s, speech + 0.6 s).

MOTION QUALITY
- Marks appear at the speed of a sped-up drawing: a long line 0.3–1.4 s (slow only when the camera is close), small strokes 0.03–0.1 s, a block 0.05–0.4 s. One stylus: marks are mostly sequential; the stylus lifts and travels in an arc between them, and leaves the frame in pauses longer than a second.
- 24 fps on ones; no boiling, no paper shake. What moves: the camera, the stylus, the mark being made, and anything the story animates in screen space (particles, a colour flood).
- Construction lines come first and fast; outlines next; colour last. A group is drawn white, then coloured as a group.

CAMERA GRAMMAR
The camera is the canvas under two fingers. Every move is a gesture: quick start, settles with momentum (≈0.7–0.9 s); between gestures the canvas drifts very slowly; it usually sits 1–4° off level and comes back to 0° for the whole page.
Framing: the subject fills ≥ 1/3 of the frame height at key moments; leave the bottom 160 px for subtitles. Transitions are made from the medium (a gesture, a colour flood, a match cut); no fades, no wipes, no hard cuts except a match cut.

SOUND PALETTE
- **Foley is the pencil:** graphite on paper (band noise 0.7–4.2 kHz with paper-tooth bumps and the slow rhythm of a hand), lighter and higher for construction lines, choppier for lettering, broad and low (0.3–2.6 kHz, 12–14 Hz back-and-forth) for colouring; a tiny tick when the lead touches down after a lift; the sheet sliding under two fingers for a gesture. Drive it from the page's marks, not from a library of one-shots.
- **Music:** warm, small, acoustic: nylon guitar, double bass, a soft shaker, a soft-mallet vibraphone or kalimba; one soft accent note per new idea. No generic piano and strings.
- Silence: a near-silence (pencil and room only) before a reveal; the first sound after it is the reveal.
- Mix: voice compressed and set ≈10 dB (RMS) above the music; the music dips 300 Hz–4 kHz while someone speaks; −14 LUFS, true peak ≤ −1 dB. Voice: warm, unhurried, a creator talking over their drawing (≈2.3–2.6 words/s).

NATIVE MOVES
- **Line, then colour.** A group is drawn white; a new "pencil" colours it as a pass. *Fits content like:* a plant part by part; a dish's ingredients; a city block before it is zoned.
- **Re-register a block.** The line stays, the colour block is replaced; record every state on a swatch strip. *Fits content like:* bread browning in an oven; leaves through autumn; a bruise healing; paint mixing.
- **Holes as layers.** Cut holes in stacked blocks to show a cross-section (shell, white, yolk). *Fits content like:* a geological core; a cake's layers; an onion; a cable's insulation.
- **Colour flood into the next shape.** Push into a block until it fills the frame, sweep a new colour over it, shrink it into the next object. *Fits content like:* a snowfield becoming a sheet of paper; a sunset becoming an orange; a stop sign becoming a tomato.
- **Shatter a block.** The colour of an object breaks into hundreds of small offset chips that fly elsewhere, leaving the empty line. *Fits content like:* a window breaking; seeds scattering; salt dissolving; a budget split into spending.
- **Match cut to the magnified copy.** Push into a small drawing, cut to a large copy elsewhere on the page with the same screen size and angle, then draw the magnifier circle around it. *Fits content like:* a cell inside a leaf; a gear inside a watch; a word inside a page.
- **Still drawing.** End a section with something unfinished (half a block, a line-only object, construction lines). *Fits content like:* any explainer that should feel live rather than finished.
Our demo opens on an extreme push on the first line, uses *colour flood into the next shape* as its turn and ends on the whole page with the title written last. A new film opens and ends another way: an empty page with a question written on it, a finished drawing rubbed out to start again, an ending on one object *re-registered* to its final state, or a swatch strip read from the first colour to the last.

PITFALLS OF THE MEDIUM
- **Boiling lines** turn it into a sketchnote: keep finished marks still.
- **Colour on top of the line** looks like a sticker: composite colour under the pencil, always multiply.
- **Perfect fills** look vector: keep the offset, the ragged edge and an occasional unfinished block.
- **Grain in screen space** crawls during camera moves: generate it in canvas space, at the buffer resolution, and let the camera scale it.
- **Stylus parked over the subject or a note:** park it in empty paper, or take it off frame.
- **Too many objects on one page:** a page for a 50 s film holds about six drawings and a dozen short notes; plan the whole page before drawing the first line, and leave the final title's space empty until the end.
- **A drawing that is too small to read when the page is whole:** size objects for the final pull-back, then push in for detail.
- **Thin flat transitions** (a fade between areas of the same page) break the canvas illusion: move the canvas instead.
```
