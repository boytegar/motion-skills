Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Blob Simulation" style.

MATERIALS & RENDERING
- **Pip** (three.js, instanced: one InstancedMesh per part, hundreds per frame):
  - **Body**: a lathe profile, a wide-based, round-bellied, domed "mochi" about 0.9 tall and 1.07 wide, flat bottom sitting firmly on the board. A soft curled tuft on top (thick root, thin tip, round end). Two flat oval feet in front, body colour × 0.8.
  - **Face**: ink only. Eyes are tall glossy beads with one large and one small white glint; a pale blush on each cheek (body colour mixed 30% towards `#FF9A8F`); the mouth is a matte arc.
  - **Expressions change only eyes, mouth and brows** (geometric parts): neutral, happy / fed, eating (open mouth), sleepy / tired, surprised, determined (brows low inside), sad (brows high inside). Expressions change through a blink (closed for about 0.15 s), never in one frame.
  - **Traits on the body**: the trait being studied is the body colour (a ramp, §3); size = scale; perception = bigger eyes.
- **Flock, opaque**: MeshPhysicalMaterial, roughness 0.9, specular 0.1, sheen 1 tinted by each Pip's colour, sheen roughness 0.35; an extra fibre edge glow (`pow(1 − N·V, 2.6) × 0.6`, towards a paler shade of the same colour) that separates outlines in a dense crowd; a very fine fibre normal map (1024² value noise, normal scale 0.22) that reads as fuzz close up and as clean matte from afar. No transmission, no clear coat; only the eyes are glossy.
- **The board**: a thick rounded slab of warm white matte paper; the grid is drawn in the shader (1-unit fine lines, 5-unit heavy lines, fading with distance) so it stays crisp at any distance; a fine paper grain; a thin ink edge line. Under it a floor the colour of the haze catches the shadows.
- **Printed matter**: bars, axes, ramps, labels and the end card are canvas-textured planes lying on the board, never standing up. Type printed on the board is pre-stretched against the perspective of the shot it is read in.
- **Grounding**: each Pip has a clean radial contact shadow (multiply blending; plain alpha cannot darken the bright board in linear space) plus the key light's soft shadow. No screen-space AO (noise, bright halos).
- **Light**: warm key front left above (`#FFF4E6`, 2.8, PCF soft shadows, the shadow camera fixed for the whole film); cool rim from behind (`#F3F6FF`, 1.3) that lights the flock outlines; RoomEnvironment at 0.42 and a sky/ground fill. Neutral tone mapping, exposure 0.94. Fog in the haze colour; close shots use dense fog so the far crowd melts away.
- **Post**: 2× supersampling, physical depth of field (shallow on close-ups, almost none on the board-wide views), light vignette, film grain 1 in the mux.

COLOUR LOGIC
One palette, defined once in `palette.js`; every colour on screen comes from it.
- Up to five team colours in a frame, taken in the order of the table.
- For a continuous trait the five team colours are the stops of a ramp (OKLab interpolation, lightness rising along it), so the colour of the whole board reads as the state of the population.
- Type is ink; secondary type is ink at reduced opacity (0.75 / 0.55), never another grey.
- Derived colours follow the rules and are never picked on their own: feet = body × 0.8; cheeks = body towards `#FF9A8F` 30%; kicked-up fluff `#FFF7EA`; printed bar grounds = team colour towards `#F4EEE3` 78%; glints white.
- The accent never enters the ramp, and food is always the accent.

TYPE & SUBTITLES
- **Fredoka 600** for titles and the one big number per frame (120–168 px); its rounded shapes share the Pips' character.
- **Inter 500/600** for explanations (24–30 px), labels and ticks; eyebrows in capitals, 15 px, letter-spacing 2 px.
- **Noto Sans SC 500** for Chinese, at least 34 px.
- **Hierarchy**: each frame has exactly one largest piece of type and it is the result (a head count, a generation, a mean). Never an outline, never a drop shadow, never a header bar.
- **Where type goes**: only on clean ground. In close-ups over the far crowd that has melted into the fog; in overhead shots on the floor beside the board edge (frame the board so a third of the picture is clean floor); in board-wide shots the picture shifts down so the top band is haze. When a frame has no clean margin, a faint board-coloured plate fading to nothing sits behind the column.
- **Tags that travel with a Pip**: placed beside its head (offset by its projected height), its value in Fredoka, a small one-word label for its state and, when relevant, an energy bar in mint that turns spent grey when empty. Use them for the one or two individuals the story follows, never for the crowd.
- **Subtitles** (when there is a voice): lower left on a board-coloured rounded plate at 86%, Chinese 36 px above, English 24 px below at 90% ink; one line per language, never wrapped.

MOTION QUALITY
- **Hop by distance**: the hop phase is distance travelled ÷ stride (stride grows with speed); when a Pip stops, the hop amplitude fades with a smoothed "moving" factor, so feet never slide and no Pip freezes in mid-air. Lean forward 0.12–0.22 rad, squash and stretch ±8%; fast Pips kick up two or three cream fluff balls that stay where they land and shrink.
- **Anticipation, action, follow-through**: turn and crouch before a run; the body bulges as it eats and the bean flies into the mouth; a Pip that runs out of energy slumps and fades to spent grey; at a verdict the ones that go stretch up once and shrink away (never squashed into a puddle), a newborn pops out beside its parent with a ring printed on the board.
- **Breathing**: idle Pips breathe (±2% squash, each with its own phase), until the final hold, which is perfectly still.
- **Bodies never interpenetrate**: in the simulation Pips are points; every frame the drawn positions are relaxed apart (three passes by body radius) so neighbours touch but never merge. This changes only the drawing.
- **Time is a tool**: the simulation clock can run at any rate against the film clock (monotone, continuous); slow motion is kept for the one moment a rule bites.
- **Frame rate and grain**: 24 fps.

CAMERA GRAMMAR
- **From one to all**: low close-up on one Pip (camera 1–1.3 units high, 3–3.5 away) → rise to overhead or crane to a board-wide view. Between set-ups the camera moves along a spherical interpolation around a sliding look point (direction slerped, distance interpolated in log space), so it never pans across empty board or looks down at bare paper.
- **Overhead** for rules and for time-lapses: nearly vertical, the screen "up" fixed to the board, slowly tilting and orbiting so it never sits dead.
- **Tracking** at Pip height, from the side, for a race; the azimuth is chosen by code so no other Pip crosses the line of sight and the followed Pip's face is visible at the moment that matters.
- **Focus pull** to hand the story from one Pip to another.
- **Hard cuts only on rhythm points** (a start, a cut into and out of a close-up); everything else is one continuous camera.
- **The result is held**: when a verdict or a final chart lands, picture and camera stop for at least 2.5 s.
- The key light and its shadow camera never follow the camera (no swimming shadows).

SOUND PALETTE
- Warm mallets and plucks: marimba, kalimba, hang, glockenspiel, xylophone, vibraphone (struck and bowed), harp, pizzicato and sustained cellos, woodblock; a synthesized shaker. No piano-and-strings pad, no drum kit.
- **Foley from the simulation**: one soft felt step per hop (gain by distance to the camera, pan by screen position), a pitched pop per bean appearing, a two-note "nom" per bean eaten, a falling sigh when a Pip runs dry, a soft poof when one goes.
- **Data as pitch**: births, generations and Pips landing in a chart are pitched by their value (a higher value, a higher note), so the score follows the numbers.
- Narration, when used, sits about 10 dB above the bed; the music leaves 300 Hz–4 kHz to it.

NATIVE MOVES
- **Bean rain**: food pops up out of the board in rings around the subject.
- **The race**: a low tracking shot of one Pip running a rule, its energy bar draining.
- **Running dry**: slow motion as one Pip's colour drains to spent grey and it slumps.
- **The verdict sweep**: a line of judgement crosses the board; the ones that go shrink away and leave a grey ring, newborns pop up with a mint ring; the head count rolls.
- **The morning wave**: between generations a mint band sweeps the board diagonally; behind it the new generation pops up, ahead of it the old one sinks.
- **Pips into the chart**: every Pip walks into its slot in bars printed on the board, one Pip = one individual, with an earlier generation as a dashed outline.
- **The cast on a number line**: a few Pips standing on a ramp printed on the board, each at its own value.
Our demo opens on a low close-up of one Pip in the *bean rain*, uses *the race* and *running dry* as its turn and ends on *Pips into the chart*. A new film opens and ends another way: overhead on a board already full, *the morning wave* bringing the first generation up, *the cast on a number line* introducing the few the story follows, an ending on *the verdict sweep* held still, or two populations side by side.

PITFALLS OF THE MEDIUM
- Translucent, jelly, glass or clear-coated Pips look cheap and refract each other; keep them opaque flock.
- Fibre normals shimmer in motion without mipmaps and anisotropy; supersample.
- Shadow cameras that follow the camera make shadows swim; fix the sun.
- Screen-space AO on hundreds of small bodies flickers and halos; use contact blobs.
- Charts as floating cards with shadows break the bench: print them, or let the Pips be the chart.
- More than one main chart plus one small chart in a frame turns into a slide.
- Type over the crowd is unreadable; find or make clean ground first.
- The food colour must never fall inside the trait ramp.
- A wandering simulation can produce a dull outcome; tune the rules (and say so in the facts) rather than placing Pips by hand.
```
