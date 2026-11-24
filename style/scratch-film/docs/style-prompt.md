Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Scratch Film" style.

MATERIALS & RENDERING
Reference: how our engine does it (`demo/js/base.js`); another stack is fine if the marks read the same.
- **Layers.** One 2D canvas per frame: a mottled emulsion plate (warm and cool blotches, a rotating set of four), the scene's marks in painter's order, then the light pass: two blurred additive copies for halation (about 9 px and 28 px at 1080p), dust specks and short curls, a corner falloff, a one-drawing flicker, a weave of about 1 px, grain added at mastering. No long vertical hairlines (they read as film damage).
- **The needle line** (`scratch`): a Catmull-Rom path resampled in screen space, 2–3 passes offset by the tremble, width varying along the stroke, about 1 % skips, small triangular chips on heavy lines, a thin overshoot tail. Width follows the picture's scale; the tremble is in screen pixels (about 0.7 × width, capped at 5 px), so a close-up and a wide shot both look hand-cut.
- **Dye** (`dye`): 4–8 thin translucent bristles along a path, tapered at both ends: reflections, rays, wakes.
- **Fills** (`hatch`): back-and-forth zigzag hatching that overshoots the outline, two passes at different angles. Large light (a sky, a glow from above) is several sparse hatch layers stacked into a gradient, or drawn on its own layer and faded with a gradient mask; never stepped bands (they read as film frames).
- **Text as cuts.** Words are rendered once as images with thin skips through them and cut in left to right behind the hot point; they tremble by less than a pixel and are never re-lettered per drawing.
- **Depth.** Depth of field is a blurred copy masked by a disc around the subject; background creatures can be drawn on their own layer and blurred. Keep blur ≤ 5 px on bright hatching or it melts into a flat fill.

COLOUR LOGIC
- Ground: warm black `#050403` with brown and blue mottles; never grey.
- Light: one cream `#FFF4DC` for every cut. Strong light (a sun, a glow from above, the hot point) is the same cream, denser.
- Dye: a few transparent colours laid by hand, alpha 0.5–0.9, always with cream lines on top. **One dye, one kind of thing, for the whole film.** Our demo: teal `#1FB39A` the hero, blue `#2F86FF` living light, red `#FF3220` danger, yellow `#FFC81C` sun and moon, green `#46D65A` small food.
- Values: black dominates (≥ 50 % of most frames); the brightest point is a fresh cut or a light source.

TYPE & SUBTITLES
- Words belong to the drawing: a depth figure beside a scratched ruler, a title cut into the dark. No boxes, no outlines, no drop shadows.
- Faces (all OFL): a heavy geometric sans filled with diagonal cuts for titles (Jost 800), a sturdy mono for numbers (IBM Plex Mono SemiBold), a marker-like Chinese face (LXGW Marker Gothic). These are references; any face with the same roles works.
- Subtitles sit low left, Chinese above English, cream on the black ground, revealed left to right in about 0.3 s. Cap height ≥ 30 px for anything the viewer must read.

MOTION QUALITY
- Everything steps on twos: 15 drawings a second at 30 fps. Each drawing reseeds the tremble.
- Camera moves and characters move on the same steps; a stepped camera is part of the look, but keep it smooth in time (eased), not jerky in position.
- Cuts appear progressively behind the hot point; dye sweeps in behind a brush edge; lights switch on with a short bloom.
- Characters: anticipation, a flick, follow-through (2–4 drawing anticipation, big poses held 4–6 drawings).
- At a cut between shots, nothing jumps: the same subject keeps its place, size and angle across the cut, new elements come in after it.

CAMERA GRAMMAR
- Keep the subject ≥ 1/3 of the frame; never hold a frame that is mostly empty black for more than a second without a mark being made.
- Transitions happen inside the medium: a cut line that becomes a divider, a push into a point of light, a pull to a wide shot. No wipes, no page turns, no white flashes.

SOUND PALETTE
Reference, not rule:
- **Hand-made instruments**: struck and bowed vibraphone, glockenspiel for each new light, pizzicato double bass, log drum for a threat, brushes. Avoid generic piano and strings.
- **The cut is heard**: a gritty high scrape with a thin wandering squeal, a dry tick per small mark.
- **Drawn tones**: a sine read at 24 drawings a second flutters like a hand-drawn sound stripe; good under each light that switches on.
- **Silence**: cutting all music before a key moment is the strongest accent; the first sound after it should be a mark being made.

NATIVE MOVES
- **Scratch-in.** A line or a whole character cut live behind the hot point. *Fits content like:* a title; a first diagram line; a character being born; a correction.
- **Hatching is light.** A light source is a field of hatching; an uncut shape inside it is a shadow; hatching that fills the shape makes the shadow vanish. *Fits content like:* a searchlight; being seen; switching something on.
- **One mark to a whole place.** A single drawn subject becomes one of thousands of glowing points in one continuous pull, each level of detail appearing only while it suits the frame. *Fits content like:* a city's population; crowds; networks; "you are not alone".
- **Lights on, one by one.** A row of dots each switching on with a bloom and a note. *Fits content like:* a sequence; a count; a signal; a city at dusk.
- **A cut across the horizon.** The scene's key line (a horizon, a road, a timeline) cut from one edge to the other, opening and closing a film.
Our demo opens on *a cut across the horizon*, uses *hatching is light* and *lights on, one by one* as its turn and *one mark to a whole place* as its climax, and ends on the horizon line again with every light gone dark but one point. A new film opens and ends another way: a title cut letter by letter, a split screen with two views on one beat, a single point of light pushed into, or an ending on the last mark held until the dust settles.

PITFALLS OF THE MEDIUM
- Flat bright areas look digital: always hatch; stack sparse layers for gradients.
- Large light drawn as segments or bands reads as film frames: fade it with a mask instead.
- Tremble scaled with zoom shakes macros and blurs wide shots: tremble in screen pixels, width from the picture.
- Boiling text becomes unreadable: tremble ≤ 1 px on text, never re-letter a word per drawing.
- Too many full characters overlapping on screen turn into noise: fewer, smaller, placed by hand around the hero.
- Particles positioned by speed × absolute time jump whenever the speed changes: offset them by the camera's displacement instead.
- A crowd of glowing points generated in boxes shows the box edges: use soft (Gaussian) distributions per level of detail.
```
