Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Blueprint" style.

MATERIALS & RENDERING
- **Sheet**: procedural cyanotype in a shader in *sheet coordinates*, crisp at any zoom: fbm between two Prussian blues, coating streaks, fibres, fold creases (bright edge, dark edge, whitening), bleached edges, specks. Around it: dark drafting cloth with a drop shadow.
- **Lines**: pale blue-white, slightly soft (mipmap bleed), a faint ink breakup driven by paper grain. Four weights (outline, detail, thin, hair; e.g. 3.2 / 2.0 / 1.25 / 0.9 sheet units). **Screen width = sheet width × zoom^0.65**, so close-ups don't turn into sausages and wides don't vanish.
- **Dash conventions**: centre line (long-short), hidden (short dashes), phantom / cutting plane (long-short-short), scaled with zoom so the pattern stays on the paper.
- **Occlusion by painter's order**: every solid is filled with "paper" before its outline is stroked. Oblique extrusion: stroke the outline at N depth steps at 2× width, then fill the same steps with paper; a clean silhouette survives.
- **Composite in buffers**: ink (white = exposed line); fx (shadow, wetness, stamp ink as channels); a subtitle overlay last. Shadows darken lines too: that is how a lifted object proves it has left the paper.
- **Hatching** (fine-line shading) for sections, soil, weights, at 45°, spacing 8–14 units.
- **Cyanotype and water**: a drop blooms deeper blue with a pale tide line; white lines inside the stain go soft; ink signatures bleed.
- **The one real thing** (optional, at most once): a drawing symbol that leaves the drawing language gets soft volume, one gradient across its whole shape, glow, and a shadow falling across the lines. It leaves a pale unexposed imprint where it was drawn.

COLOUR LOGIC
- **Blue and white only.** The blue varies in value (coating, stains, folds); the white varies in weight and softness, never in hue.
- **One non-blue colour, used exactly once**: a rubber-stamp ink (red is classic; a green APPROVED or black VOID work too). It carries the verdict.
- The "real" object at the climax may carry light (white glow, soft value), not a new hue.
- Example blues: `#18336B` → `#2A58A6` with lines `#E7F0F9` and table `#0D1116`; a faded sun-bleached sheet `#3A6DA8` → `#5B8CC4`; a deep over-exposed sheet `#0F2350` → `#1D3F80`.

TYPE & SUBTITLES
- **Hand lettering** for header, labels, notes and subtitles (e.g. Architects Daughter); a technical sans for title-block fields (e.g. B612); a script only for the signature (e.g. Allura). All OFL.
- **Text is written on left to right** with a glowing nib, like everything else.
- **Subtitles are drawing notes**: a light translucent box with double white rules, a "NOTE / n" cell, the text lettered on in ~0.35 s, a small leader tick. **Placed per shot in empty sky**, never fixed to the bottom (the bottom of a drawing is ground line, base and title block). Wrap at ~880 px. Hold ≥ max(1.8 s, speech + 0.6 s).
- Every subtitle is also lettered into the sheet's **GENERAL NOTES** column, so the final wide shows the whole voiceover as drawing notes.
- The **title is the sheet header**; the **title block** holds status and verdict fields.

MOTION QUALITY
- **Camera and draw-on on ones (24 fps).** Continuous moves within a section, hard cuts on downbeats between sections; tiny camera bumps only for physical impacts (paper slap, stamp).
- **Draw-on** truncated by arc length with a glowing nib at the head; items staggered within a layer; each layer starts on a beat.
- **Drafting order**: centre lines → outlines → detail → hatching → dimensions → balloons → labels.
- **Exploded views**: parts leave in sequence on subdivisions of the beat, eased, and return one per beat-group with a click.
- **Mechanisms step on the grid** like an escapement: a gear advances one tooth per subdivision with a short ease-out; meshing phases computed so teeth really interlock; chains move with their sprockets.
- **Machine acting**: angles, droops, volumes and gauge needles are the face. Failure = slowing, sagging, winding down over ~1.5 s; joy = snapping upright, a needle overshooting (back-ease).

CAMERA GRAMMAR
A vocabulary, not a route. Opening and ending come from the topic.
Keep storytelling shots close; full-sheet wides are rare. Keep the main figure at least `960/zoom` from any sheet edge you will frame. Transitions: cuts on downbeats, or fig-number rewrites on the sheet; no dissolves.

SOUND PALETTE
- **Contrapuntal, mechanical music**: harpsichord, bassoon, strings spiccato, harp, celesta, music box, clavichord, a small wind ensemble. Imitative lines (an invention, a canon, a fugato) feel like interlocking parts; a short repeated motive can be the "gear".
- **Foley follows materials**. Paper: roll crackle and slap, T-square wood slide, technical pen scratch (2–6 kHz grain, length = stroke length), lettering bursts, compass squeak, hatching strokes, eraser rub, tracing-paper rustle. Brass and wood: part clicks (resonances ~3–5 kHz), escapement ticks, chain rattle, spring twang, wind-down ticks that slow geometrically. Water: soft damped pats per drop, a light rain bed. Rubber stamp thump, pen cap click.
- **Options for the arc** (in any order, use what the story needs): instruments dropping out with a written-out ritardando into true silence; a single questioning figure alone; a suspended chord held without resolution under the magic; a full cadence on the stamp; the theme in augmentation for a close; adding a section per mechanism that joins.
- **Silence** may contain paper sounds only.
- **Voice**: dry, understated, reading design notes, a few short lines. Leave the most emotional moment voiceless.
- **Mix**: music ducks well under voice (~0.36 ×), foley by half; whooshes low-passed so they don't mask consonants. −14 LUFS. Little added grain (the paper already has it).

NATIVE MOVES
A menu: use the ones your story needs.
- **Drafting order.** The object draws itself as an engineer would, a layer per beat. *Fits content like:* a bridge; a violin; a kitchen renovation.
- **Live annotations.** Dimensions measure moving parts and the numbers change; balloons fly with parts; a section line reveals the inside. *Fits content like:* a heart valve; a folding bicycle; a rocket stage separating.
- **Fig. numbers as transitions.** "FIG. 1" struck through and rewritten "FIG. 2 · EXPLODED VIEW". *Fits content like:* versions of a product; phases of a construction; a recipe's stages.
- **Projection as camera.** The oblique factor animates; exploded parts fly along the depth axis. *Fits content like:* a floor plan becoming a building; a circuit board; flat-pack furniture.
- **A symbol made real** (once, if at all). A convention leaves the drawing language: a dimension arrow flies, a hatch becomes rain, a centre line becomes a road, a weld symbol sparks. *Fits content like:* an invention finally working; a city plan becoming a street; a dream becoming a plan.
- **Cyanotype and water.** Drops bloom and soften lines. *Fits content like:* a flood-defence design; grief; a harbour wall.
- **The title block as story.** Status field = the verdict; revision rows = history. *Fits content like:* a rejected permit; REV. 1–5 of a city; an approval after years.

PITFALLS OF THE MEDIUM
- **Main figure near the sheet edge** → wides show the desk. Keep margins or enlarge the sheet.
- **Upward explosions hit the header** → explode sideways and flatten long parts during the explode.
- **Subtitles on the ink buffer** vanish off-sheet and can't occlude lines → separate overlay, premultiplied alpha.
- **An object darkened by its own shadow** → zero the shadow channel only in its area.
- **Per-lobe shading on a blobby shape reads as grapes** → one gradient across the union; crescents from a union-minus-shifted mask; fill unions as one path (`nonzero`) or overlapping fills make bubbles.
- **A small payoff undersells the return** → the payoff object ~⅓ of frame height, centred, drawn stroke by stroke, subtitles off it.
- **Line weights not scaled with zoom** → sausages in close-ups, vanishing wides.
```
