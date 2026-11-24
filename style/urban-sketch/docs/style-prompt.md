Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Urban Sketch · Pen & Wash" style.

MATERIALS & RENDERING
- **Paper**: two octaves of fbm mottling (a few %), a tooth tile multiplied over everything, figures included.
- **Ink**: ~96 % coverage; world line width thinner far, thicker near (e.g. 1.3–3.4 px); wobble ~1 px; overshoot 3–8 px; a third to half of long strokes get one small gap. Strokes are baked with their **draw time per pixel**, so the shader plays back stroke order.
- **Wash**: 2–3 rough-edged layers, multiply, alpha ~0.4–0.65, offset from the line by about one line width, a dried edge at 30–40 %; granulation = pigment × (0.82 + 0.36 · fbm).
- **Colour arrival field**: every pixel has a time at which colour reaches it. The wash is revealed through it with a ragged wet front (darkened band just behind the front) and noisy edges. Figures bloom from a point with a noisy circular clip.
- **Foliage**: pale base wash, then light and shadow masses, then small leaf dabs coloured by light direction, holes at the rim, scalloped ink only on the outline.
- **Sky**: a few big soft washes at low alpha, fading upward; clouds are reserves with a tinted belly.
- **Buildings**: local colour; shadow side mixed ~45 % with a cool violet-grey; distant blocks mixed ~50 % with a haze colour.
- **Figures**: a 3D skeleton in orthographic projection; torso from cross-sections; limbs as organic tapered tubes; hair and head as one mass; face left blank (one nose tick in profile); shadow side mixed with violet.
- **Page edge**: a noisy boundary, a white-dab band inside, a coloured-dab spray outside.
- Deterministic: same seed, same marks.

COLOUR LOGIC
- **Ink is one colour** (dark sepia or warm black); it never changes.
- **Wash is local colour, lightly**: few pigments, mixed on the page; shadows are the local colour mixed toward one shared cool violet; distance mixes toward one haze.
- **Paper white is the brightest value**; it is reserved, never painted.
- **Colour can be withheld**: a page may be ink-only with colour as an event (one tinted object, colour arriving with a cause).
- Example palettes: park in summer (lawn `#c3c96f`, leaves `#7fa04a`, sky `#9fbcd6`, limestone `#d8c6a0`); harbour morning (sea `#7fa6b8`, hulls `#c9573f`, rope `#c8a86b`, sky `#bcd0dc`); winter market (roofs `#8b93a6`, lights `#e2b04a`, awnings `#b8483e`, snow = paper).

TYPE & SUBTITLES
- **All text is handwriting on the page**, written on left to right with pen sound, and moves with the camera: a corner note (e.g. Reenie Beanie: day, time, weather, place), titles and colophon in a casual hand (e.g. Caveat 500–600), left-aligned in the blank margin.
- **Subtitles**, if there is a voice: handwritten lines in the page margin or a caption strip written into the lower margin, never a generic overlay box. Hold ≥ max(1.8 s, speech + 0.6 s).
- The style works well without any voice.

MOTION QUALITY
- **Figures boil at 12 fps** (line wobble and wash seeds change every 1/12 s). The static page does not boil (it is a real page); the camera moves every frame, smoothly.
- **Performance**: anticipation, action, follow-through; footsteps on the beat.
- **Invisible forces are drawn**: wind as ink curls that draw on and erase from the tail, trailing dabs; sound as ink lines; heat as wavy strokes.
- **Colour never snaps**: it blooms from a point or arrives through the field.
- Stroke-order draw-on for anything that "appears".

CAMERA GRAMMAR
A 2D camera over a large page (e.g. 5120 × 2880). Designed moves use cubic easing; follow shots use a critically damped spring on a lead-ahead target, so the camera lags like a hand-held follow. A vocabulary, not a route. Opening and ending come from the topic.
Framing: subjects ≥ ⅓ of frame height at story beats; a small key object gets a faint paper halo (reserve) so it reads over busy foliage. Transitions: camera travel on the page, page turns, or colour arriving; no dissolves.

SOUND PALETTE
- **Ambience from the place**: paper room tone first, then the location once it has colour (birds, traffic, harbour gulls, market chatter). J-cut a place's sound a bar before we see it.
- **Pen on cold-press paper**: band-passed noise (~1.6–7.5 kHz) with a fibre texture and grain clicks, one burst per real stroke; thousands overlap into a hiss during a fast sketch; brighter short strokes read like a hi-hat.
- **Water and brush**: a wet brush "shhh" for blooms, a jar rinse, a drip.
- **Foley follows objects** at sketch scale: fabric or paper flapping for things in the wind (rate and level from speed, pan from screen x), steps on the beat, bells, splashes.
- **Music families**: a jazz waltz (walking bass in three, brushes, piano, clarinet or flute); a café accordion trio; solo nylon guitar; a string quartet pizzicato; a brass band for crowds. One colour instrument per area of the page works well.
- **Silence as a tool**: a bar of true silence (ambience included) at the height of suspense; the first sound after it is the next gesture.
- **Mix**: music ducks under key hits; foley high-passed ~90 Hz; −14 LUFS, moderate LRA.

NATIVE MOVES
A menu: use the ones your story needs.
- **Line before colour.** The pen draws a subject in real stroke order, then the wash drops in; the drawing is the spectacle. *Fits content like:* a cathedral façade; a chef plating a dish; a street musician.
- **Colour as arrival.** Wash spreads from a source with a wet front, so an event paints the page; figures stay frozen line drawings until colour touches them. *Fits content like:* a sunrise over a harbour; a parade moving down a street; the lights of a market switching on.
- **The one coloured thing.** A colourless sketch with a single tinted object that becomes the protagonist. *Fits content like:* a red umbrella in the rain; a yellow taxi; a child's kite.
- **The page as frame.** Pull back until the whole page shows, edges dissolving, handwritten notes in the margin. *Fits content like:* a travel diary entry; a neighbourhood portrait; a day in a place.
- **Carrot people.** Distant crowds as one wash blob, two leg lines and a head dot. *Fits content like:* a station at rush hour; a stadium; a beach.

PITFALLS OF THE MEDIUM
- **Straight edges in the colour field** from clipping by a bounding box → clip by the actual noisy distance.
- **Rectangular masks around complex silhouettes** leave notches → mask with the real stepped silhouette.
- **Characters losing colour** when they move into unpainted areas → sample arrival once (or tie it to the event), never at the current position.
- **Static ink can't be occluded by later washes** → erase far ink inside nearer shapes before drawing; use carrot people for far crowds.
- **IK feet pinned while blending poses** splay the legs → blend foot targets toward under-hip.
- **Front-facing heads go bald** → draw the hair mass, then knock the face out of it.
- **Big canvases in headless Chrome**: pack ink and time into RGBA byte textures yourself and upload the arrival field as a filterable float texture (`R16F`).
- **Painting edge to edge** kills the sketchbook feel → leave margins and fray the edge.
```
