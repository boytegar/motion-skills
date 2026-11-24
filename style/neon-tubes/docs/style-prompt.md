Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Neon Tubes" style.

MATERIALS & RENDERING
- **Tube = centreline polyline in metres** (`lib.js tubeFromStrokes`): corners filleted with a radius of about one tube diameter, strokes joined by painted double-backs, painted sleeves and an electrode cap at each end (a broken end has neither). Diameters 1.0–1.9 cm; ties every 22 cm.
- **Cross-section profile layer**: each tube is stroked into a canvas as concentric bands of the cylinder height `h = sqrt(1 - r²)` with a material code (clear, coated, black paint, metal). The shader takes the normal from a Sobel gradient of that profile: gas column inside 70–85 % of the radius, glass rim, Fresnel edge, specular streak, refraction of what is behind.
- **Emission layer** at half resolution, blurred into a pyramid, used twice: as light on surfaces (lobes sized in metres, 2 cm to 3 m) and as bloom on screen. Bricks and tiles get relief toward the light.
- **Characters from silhouettes.** Build each pose from a small rig (haunch, body, chest, neck, head, muzzle, ears, two-segment legs) as a smooth union of signed-distance shapes; take the outer contour with marching squares, smooth it and open it at one point for the electrodes (usually under the body). Eyes, whiskers and tail are separate thin tubes. One pose = one tube; a pose library of 30–40 covers a short film.
- **Hanzi from the font's skeleton.** Lay each character on a 10 × 10 grid over the skeleton of a sans-serif CJK font (Noto Sans SC): horizontals level, verticals plumb, strokes joined into as few bent tubes as the shape allows, double-backs routed behind lit strokes, never across the open face of the character. Check every glyph overlaid on the font before use.
- **The street is procedural and anchored to the world**: brick, mosaic tile, plaster, shopfronts with dim interiors, roller shutters, AC units, drainpipes, cables; a dark alley is a near-black slot between buildings.
- **Wet street**: paving, kerb, asphalt with aggregate glints; puddles grow with wetness. The reflection mirrors the facade about the wall base: sharp in puddles, smeared on wet asphalt, rippled by rain rings and single drops. Keep two screen heights of facade above the frame in the layers so a low camera can mirror signs far above it.
- **Screen space only**: camera motion blur, a slight chromatic fringe, a filmic curve that pushes cores toward white, vignette, dither, grain.

COLOUR LOGIC
- The ground is dark and neutral (night facade, wet asphalt); hue comes only from tubes and the light they cast.
- Gas colours follow the physics above. Give each sign its own hue so the street reads as a sequence.
- **Characters own their hues**: a hero hue for the protagonist and a contrasting one for whoever it meets (cool against warm reads best); no sign in the same shot uses either. A character keeps its hue wherever it goes.
- Values: tube core > halo > lit wall > unlit wall > sky. Nothing painted may be brighter than a lit tube.
- Boards are deep, desaturated versions of their tube colour (navy, oxblood, aubergine, bottle green).
- A failing tube turns pale lavender-white before it dies; a dying character dims step by step toward a dull red or grey.
Example (our demo, an example only): protagonist turquoise `#1ff2d9` with pink eyes `#ff3d9e`, second character `#ff2a08`, argon blue `#3d75ff`, boards `#0a1626` `#2a0b0d` `#1a0a24`.

TYPE & SUBTITLES
- **Words are tubes.** Every word the viewer reads is bent glass on a sign: hanzi as above (5–14 strokes in 1–7 tubes), Latin capitals monoline at 15–20 % of the hanzi height, Chinese on one line, English beneath.
- **Printed type only where the world has paint or paper**: a stencil on a roller shutter, a tag, a clock face. Use open fonts lit by the scene's light.
- **Readable at 1080p**: hanzi tubes ≥ 60 px tall, Latin ≥ 18 px; hold ≥ CJK ÷ 4.5 + Latin ÷ 15 + 1.5 s from the moment the sign settles after its strike. A flickering or chasing sign does not count as readable.
- No subtitle bar. Names of places come from the signs, time from a clock; a story told in this style is best told without voice.

MOTION QUALITY
- 30 fps; every switch lands on the music grid (beats, 8ths, 16ths).
- **Strike**: 100 % for 50 ms, dark 70 ms, 70 %, dark, ramp to full in 0.1 s. Argon-mercury tubes start dim violet and warm to blue. A figure can also come alive by light running along its tube from one electrode to the other.
- **Characters switch, they never tween.** One key pose per beat or per 8th note; pose changes are instant. Make poses strong: a leap stretches into one line, a landing gathers into a ball; a gallop alternates gathered and extended frames. Afterglow, if any, only on jumps: the previous outline at ≤ 8 % for two frames. Never on runs: one character must never read as two.
- **Failure**: random flicker; a crack flashes white at the impact and leaves a visible gap in the tube; the column turns pale; the glow retreats a section at a time toward the electrodes. Anything stacked (water levels, steps, a meter) dies a level at a time.
- **Chasers and alternate circuits**: arrows and borders fill in 16ths; two outlines of one object in different gases swap on the beat.
- A slight mains shimmer (±3 %) on everything lit; rain falls at 6–8 m/s; ripples die within a second.
- Boards and walls never move. Characters move across the facade plane; a sign hanging out from the wall gives them a nearer plane to jump to.

CAMERA GRAMMAR
- Signs face the camera straight on (lens shift: verticals stay vertical). For a macro, set the eye height to the subject's height so the tubes' stand-off adds no parallax.
- Screen direction is story: one way out, the other way home. Keep it.
- Cuts land on beats. Match cuts work on shapes of the same size: a round clock to a round eye, a tube on one sign to the same tube on another.

SOUND PALETTE
- The medium's own sounds: transformer hum at 100/120 Hz with buzzy harmonics, the click of each flash, relay clacks for chasers and switched frames, crackle for a failing tube, a glass ting and sizzle for a crack, rain on wet asphalt, drips, the plop of a drop in a puddle.
- **Characters speak in neon**: a creature's breath is the hum with a tremor, footsteps are relay clicks, a landing is a soft thump; no animal voices.
- Water inside a sign: a falling gurgle for draining, a drip on dry glass, a splash.
- Music: synthesised city pop / electronic pop at a dance tempo; each sign that strikes plays the next note of a rising line; signs going off play a falling one. Silence is hum: when the music stops, the transformers are left.
- Mix: foley on cues at or above the music (duck the music under it); one bar of near-silence before the turn of a story lands hard. Master to −14 LUFS, true peak ≤ −1 dB.

NATIVE MOVES
- **Strike in sequence.** Signs flash on one per beat as the camera passes, or as a character lands on them. *Fits content like:* a route; a list of places; members of a team; steps of a timeline.
- **Frame-switched character.** A figure moves by one tube per pose on the beat grid. *Fits content like:* a mascot's journey; a gesture; a chase; a small fable.
- **Strike = life.** The two-flash strike used on a character or object at the emotional peak. *Fits content like:* a revival; a launch; a yes.
- **Crack and drain.** White flash, a gap in the tube, pale colour, then levels going out one by one. *Fits content like:* a mistake; damage; loss; a running-out countdown.
- **Alternate circuits.** Two outlines swap on the beat (a bird's wing, a microphone, an eye). *Fits content like:* before/after; two states; a heartbeat.
- **Chaser.** Segments of an arrow fill in 16ths, and can carry a character like a conveyor. *Fits content like:* direction; progress; flow from A to B.
- **Puddle-level chase.** The character runs along the base of the wall with its upside-down twin in the water. *Fits content like:* urgency; a return; a race.
- **One sign left.** Everything switches off but one, doubled in a still puddle. *Fits content like:* closing time; what remains; a single message.
Our demo opens on a macro of cold glass pulling back from the first strike, uses *crack and drain* followed by *strike = life* as its turn and ends on *one sign left*. A new film opens and ends another way: a dark street where one sign comes on by itself, a puddle with nothing in it until a reflection lights, an ending on the whole street switching off at dawn, or a pull back and rise to the whole street.

PITFALLS OF THE MEDIUM
- **Flat glowing strokes** look like vector art: always render the cross-section and the board behind.
- **Wrong gas colours** (a green clear tube, a blue neon): only neon and argon-mercury may be clear.
- **Hand-guessed hanzi**: strokes in the wrong place, double-backs crossing the character, one tube per stroke with caps everywhere. Lay them over the font.
- **Tweened or morphing figures** break the medium; so do bright, long afterglows that multiply a character.
- **Macro bloom**: very close to a lit figure the halo floods the frame; pull back a little or keep most of it unlit.
- **Low cameras need off-screen content**: a mirror at puddle level shows signs far above the frame; keep them in the layers or frame higher.
- **A small figure in a wide shot** (≈ 100 px) reads as a streak; use wides for the street's answer, mediums for the character.
- **Readable text vs flashing**: let a sign settle before it chases.
```
