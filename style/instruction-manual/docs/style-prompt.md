Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Instruction Manual Art" style.

MATERIALS & RENDERING
- **Projection**: world units in centimetres, y up; `makeView({cx, cy, s, az: 45, el: 35.264})` maps world → screen. A sphere always projects to a circle, so limbs, hands and cylinders are hulls of circles and stay right at any heading.
- **Painter's order by view depth.** Long parts (rails, posts) are sorted by their nearest point; an assembly that interlocks is drawn in groups from back to front (far end and back rail → the board and what lies on it → front rail, near end, bolt heads), so large flat parts never cut through bars.
- **Fill, then outline, per part**: overlapping parts are separated by a line, a part has no lines inside itself.
- **Paper**: off-white stock with faint fibres that travel with the page on a pan and keep their size on a zoom (printed stock, not a photo of it).
- **Line weight is set in screen pixels** (2.6 px at 1080p) and follows the zoom only partly (about z^0.55, clamped), so the whole page stays crisp when pulled back and lines don't go clumsy close up.
- **Appearing** (reference): a drawing first shows its complete line art with paper-coloured fills, then its colours come in over it. A part is never drawn half-built or floating.
- **Motion blur** (reference): pans smear one frame along the camera path; fast zooms average sub-frames across half a shutter.
- **Hands in close-up** (reference): when only hands are in frame (a magnifier, a "press here" shot), draw them finer than the full-body figures: a palm, a thumb and four separate fingers at real size (≈ 18.5 cm wrist to fingertip), white like the skin elsewhere, and the line where one finger overlaps the next is all that says "fingers" (one or two lines, no nails, creases or shading). The sleeve is the wearer's shirt colour (her light grey `#CFCBC2` reads best against white hands and parts; a white sleeve keeps its cuff line) and runs on until the frame or the magnifier's rim cuts it: an arm never ends inside the frame. Bring the arm in from the nearest edge, so the forearm crosses at most about a third of the frame, and place the hand by what it touches (the fingertip on the button, the palm on the jar). `demo/hand.js` does this; its check sheet is `?f=hands`.

COLOUR LOGIC
The exact hexes are reference; the logic (paper, ink, white parts in three values, one reserved yellow, red/blue only for verdicts) is the style.
In a procedure that removes or changes things rather than adding them (opening, cleaning, a safety step), yellow marks the one thing the reader acts with or on in that step (the tool, the button, the part that comes off), and returns to white once the step is done.

TYPE & SUBTITLES
- On the page only step numbers (a geometric sans, e.g. Inter 700, about 150 px at 1:1, top left of each step's area, printed on the paper and moving with it) and counts in parts boxes ("4×", Inter 600). Everything else is said by drawings and the voice.
- **Subtitles** bottom left, left-aligned: Chinese (e.g. Noto Sans SC 500, 38 px) over English (Inter 500, 32 px, ink at about 78 %), one line each, an 8 px yellow bar on the left marking the line being spoken. No box, outline or shadow; a soft paper-coloured bed (same colour as the page) keeps them readable over line art. Keep subjects out of that corner.
- A title, if any, is printed on the page like a manual's cover or footer, never a title card over the picture.

MOTION QUALITY
- Parts fly along dotted guide lines in straight paths, ease in, and stop with a small settle; a beat of stillness after each one lands.
- Arrow first, motion second: the arrow is drawn, then the part follows it once and holds.
- The new part's yellow fades to white once it is fitted.
- People move between a few key poses with eased interpolation; walking is short shuffled steps when carrying.
- Never moves: the angle, the page, the step numbers' positions.
- **Frame rate and grain**: 30 fps (parts glide along their guides without stepping); mux grain 1, since the paper fibres carry the texture.

CAMERA GRAMMAR
The camera is a 2D camera over one long sheet: pan, whip, tilt, push, pull, and cuts. It never rotates the view.
Leave a step held still long enough to read its count and its arrow; the subject is at least a third of the view when it is the point.

SOUND PALETTE (REFERENCE)
- A small tidy ensemble that counts steps: pizzicato strings, marimba, glockenspiel, felt piano; one bright note per part landing, rising through the chord.
- Paper and flat-pack foley: a carton set down, a lid, wood knocking, a pencil, paper sliding, bolts, a ratchet, the click, a rubber stamp for ✗ and ✓.
- A mistake stops the music dead before the ✗; the ending is near silence, then a single chime.
- Voice (optional): a calm, dry narrator, as if the manual itself were speaking; few words, never reading out the numbers on the page.
- Mix: voice compressed and about 10 dB over the music, music gives way at 300 Hz–4 kHz under it; −14 LUFS, true peak ≤ −1 dB.

NATIVE MOVES
- **Parts explosion.** The box opens and its parts fly along their own dotted lines to a counted parts list. *Fits content like:* unboxing; a kit's contents; what a product is made of.
- **Line art grows.** Floor line → object → people → arrows → magnifier, then the flat colours. *Fits content like:* a step being built up; a scene appearing.
- **Arrow-driven assembly.** A flat arrow in the isometric plane, then the part follows it and lands. *Fits content like:* any fitting, inserting, sliding step.
- **New part yellow.** The new part is yellow, then turns white. *Fits content like:* what changed in this step; the one new thing at the end.
- **Magnifier circle.** A small ring on the spot, a leader line, a big circle redrawn at 3–4× with a hand and a rotation arrow. *Fits content like:* tightening, pressing, a latch, a connector.
- **Do / don't box.** One white rounded box split in two: red ✗ beside blue ✓, drawn as strokes. *Fits content like:* safety rules; common mistakes; two ways to hold something.
- **Push through a shape.** The camera dives into the ✓ cell or a magnifier circle and comes out in the next picture. *Fits content like:* zooming into the right way; changing scene inside one page.
- **The whole page.** A final pull back shows every step laid out on one sheet. *Fits content like:* a summary; the full procedure at a glance.
Our demo opens on the parts explosion, uses the push through the ✓ cell as its turn and ends on the whole page. A new film opens and ends another way: a single part flying between cells, a shape-matched cut, a magnifier that opens the film, or an ending on the one last yellow part.

PITFALLS OF THE MEDIUM
- **The assembly must be one model with a state table.** Every picture of the object (every step, the parts list, the final page) is drawn from the same function with a state; write the table first (step → what is added → what the object has when the step ends → parts used) and add one thing per step. Counts on the parts list must add up across the steps (e.g. bolts 2 + 2 + 4 = 8). Separate drawings per step drift: parts appear early, vanish, or sit in the wrong place.
- **Footprints and guide lines come from the same geometry.** A dotted footprint must be the finished object's real corners; a guide line runs from where the part lies to where it lands. Parts in a box lie flat, stacked at real sizes, and the box is empty only after the last one leaves.
- **Knees bend with the body, not the world.** Set the knee hint from the body's heading, or a pose that is right facing one way breaks when the figure turns.
- **Kneeling hip height comes from leg length**: knee on the floor + one thigh length. A half-kneel: the kneeling knee under the hip, thigh vertical, shin and instep flat behind; the other foot flat, knee about 90°, shin vertical.
- **The far leg is one shade darker** than the near leg, so crossed or overlapping legs never merge into one shape.
- **Sitting on the floor**: cross-legged with the knees open and low and the shins crossed in front reads as sitting; a pelvis on the floor with legs straight forward reads as falling over.
- **Standing**: weight on one leg, the other relaxed and turned out, hips and shoulders tilted against each other. Feet together and arms hanging make figurines.
- **Placement**: never put a person between the viewer and what the step is about, and keep their feet outside the object's footprint. Behind slatted furniture their legs show between the bars and they read as standing inside it: put them at a front corner, or at one end with their back to the viewer.
- **A gesture that matters** (a hand on a shoulder) lands on the side nearest the viewer, with nothing in front of it.
- **Front-on (facing the viewer squarely) reads worst** at this angle; use three-quarter and profile headings.
- **Hair and buns** live in the head's own 3D frame, so they turn, tilt and hide with the head.
- **Small mistakes are hard to see at full-page scale**: a part on upside down needs the ✗ on it and a close-up, not only the voice.
- **No perspective, no turning the view**, no 3D camera orbiting the paper.
- **No brands, logos, product codes, faces or cartoon round-headed figures.**
```
