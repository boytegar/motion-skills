Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Crayon World 3D" style.

MATERIALS & RENDERING
- **Two passes per frame.** (1) An invisible half-resolution render: region id (R), light (G), depth. (2) Marching squares trace each region into smoothed rings; crayon strokes are drawn from them into Canvas2D pressure layers (RGB = colour, A = pressure), and a WebGL2 compositor lets wax stick only to the paper's tooth.
- **A region is a drawing recipe**, not a material: crayon colour, pressure, hatching angle, row gap and width, overshoot, outline width (0 = none), layer group, and whether its dark side gets a second, denser pass.
- **Outlines** follow each region's ring but are never drawn along the frame edge (that would box the picture in). One dark outline colour for everything.
- **Hatching** is a child's scribble: bowed, leaning rows, uneven spacing, a skipped row now and then, long rows broken into overlapping strokes; a lighter cross-hatch on big areas. Every row and random value is counted from an anchor projected from the object, so it is coloured the same way in every drawing.
- **Occlusion like a child.** Layer groups (background, characters, foreground) are composited in order; before each group its silhouette is knocked out of what lies behind it, so the paper stays clean behind a figure.
- **Faces and tiny things are drawn in 2D**: eyes and mouths placed by projecting points of the 3D head; anything too small to trace (a far bird) becomes a 2D doodle at its projected position.
- **A child's vocabulary**: squashed spheres for crowns and hills, boxes, a prism for a gable roof; people with big round heads and tube limbs, props bigger than real.

COLOUR LOGIC
- **A box of about a dozen crayons.** One dark outline colour (indigo, plum or dark brown), never black. Value comes from pressure and a dark-side pass, not from more hues.
- The sky is a light-pressure cross-hatch so the paper shows through; the ground is lighter than the things standing on it.
- White things are not coloured: the paper is their colour, with an outline only (clouds, a white wall, a gravestone).
- One colour may belong to the story (the flowers someone planted, a red scarf) and is used nowhere else at that strength.
- Example box (a village in summer): indigo line, sky, white, grass, olive, lemon, caramel, earth, brick, rose, cream, plum.

TYPE & SUBTITLES
- Words are **printed on the page**: a handwriting face in the outline colour, written in a character at a time, sitting on blank paper. Never a speech bubble, a box or a plate.
- Blank paper is made: leave part of the picture uncoloured (only a strip of sky) and put the words there, clear of what the eye follows. Thin the colour out just before the words arrive; blank paper held empty reads as a mistake.
- Bilingual: the Chinese line, then the English line a size smaller underneath, written in after it. Example faces (OFL): Long Cang for Chinese at 110–140 px, Patrick Hand for Latin at 50–64 px.
- A wordless film is native; a narrated one prints its lines the same way, at the bottom of the frame, on a paper patch the scene leaves free.
- Reading time: every block stays fully in frame ≥ max(1.8 s, CJK ÷ 4.5 + other ÷ 15 + 1.5 s) (readcheck).

MOTION QUALITY
- Drawings step at 12 per second; the film is rendered at 24 fps (each drawing held for two frames).
- No boil on still things: an object keeps its strokes from drawing to drawing while it doesn't move; it changes only because the camera or the object moves.
- Camera paths are smooth splines in time (continuous velocity), starting and ending at rest. Gaze is smoothed as a direction, with a lag, never as a look-at point.
- Living things move in a child's way: two key drawings for a repeated action (wings, steps), simple poses, held.
- Wind is the cheap way to keep a still frame alive: flowers sway from the base, washing on a line swings, clouds drift.
- **Frame rate and grain**: no film grain (mux grain 0).

CAMERA GRAMMAR
Framing: keep the subject of a held shot at least 1/4 of frame height; keep paper free where words will go. Transitions are cuts or camera moves; never a dissolve, never a 3D-looking whip with motion blur.

SOUND PALETTE
- Small wooden and metal-toothed instruments, no strings or grand piano: music box, nylon guitar picking, clarinet or bassoon long notes, recorder or flute as the singing voice, glockenspiel for a single sparkle. Sampled from CC0 libraries or synthesised.
- One theme can be held back and heard in full only once. Melodies end on the tonic; any coda comes after the resolution.
- Score the underlying feeling, not the surface action: a bright flight in a sad film is scored for the sadness.
- Foley follows the world, softly (water on soil, birds, wind); no crayon-scratch foley.
- Silence is a beat: one breath of nothing at the moment that matters.
- Mix to −14 LUFS; nothing harsh at the very start, when one instrument is alone (low-pass a bright music box until others join).

NATIVE MOVES
- **The camera is someone, never shown.** The viewer learns who from what the world does around them. *Fits content like:* a pet coming home; a returning soldier's walk through a town; a seed carried by the wind.
- **Cut to the other's eyes.** The last shot swaps point of view and shows, small, what the first person was. *Fits content like:* a parent seeing their child leave; a lighthouse keeper seeing the boat; a teacher seeing a former pupil.
- **Passing, not pointing.** Clues are flown past at the speed of the shot, never stopped at. *Fits content like:* a house's history in its objects; a company's first office; an old route to school.
- **Blank paper for words.** Colour only part of the world and print the line where the crayon stops. *Fits content like:* a dedication; a line of a letter; the name of a place.
- **Wind.** Sway, drift and flapping keep held frames alive without a cut. *Fits content like:* a field at dawn; a waiting room by an open window; a harbour.
Our demo opens riding something that sways in the *wind*, with *the camera as someone*, uses *cut to the other's eyes* as its turn and ends on *blank paper for words*. A new film opens and ends another way: eyes opening, a hand-drawn map that becomes the ground, a fixed window held for the whole film, or an ending where the camera lies down in the grass and the sky takes the frame.

PITFALLS OF THE MEDIUM
- The camera passes inside a solid (a hill, a roof) → clamp its height above the terrain and check flight paths against every big shape.
- A pop where two camera phases meet (waking, take-off, landing) → derive each phase's position from the same rest pose; ease clearances in instead of clamping them on.
- Smoothing a look-at point near the camera whips the view → smooth the direction.
- State carried between frames breaks parallel rendering → recompute smoothed values from the shot's start each frame.
- Hatching recomputed per drawing reads as heavy boil → anchor rows to the object.
- Two-pose flapping or stepping can poke through a nearby object on the extreme pose → limit the pose while the object is close.
- Things that shrink below a few analysis pixels flicker or vanish → draw them as 2D doodles below a size threshold.
- Region ids live in one 8-bit channel → at most 255 recipes per film.
- Uncoloured outlines (clouds) left floating on blank paper look unfinished → hide them where the colour has been taken away.
```
