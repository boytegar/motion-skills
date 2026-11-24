Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Shadow Puppetry" style.

MATERIALS & RENDERING
**Rendering model.** Per pixel: `image = tonemap( lampLight × hideTransmission × clothTexture )`.
- `hideTransmission`: a 2D canvas that starts **white**; every piece is drawn with `multiply`. Holes stay white, overlaps darken by themselves.
- `lampLight`: per lamp a broad lobe plus a tight hotspot (~6 : 1), each flickering independently (a few % of ~7 Hz value noise plus a faster sine). Warm light, never white.
- `tonemap = 1 − exp(−hdr)`, exposure ∝ `1/√(lamps lit)`: more lamps bleach the frame but keep it readable. Lamp count is a dial the story can turn.
- **Cloth**: procedural weave (sin × sin, noise-warped), fibre speckle, low-frequency density blotches, darker edges at the frame. **Post**: warm-tinted bloom, a **warm vignette** (toward brown, never grey), slight saturation, soft S-curve, light grain; heat haze only when the story burns.
**Hide and carving.** Multiply a tileable hide texture (mottling, fibres, specks) into each piece and restore its alpha with `destination-in` so holes survive. **Carving is density**: leave thin strips, cut out most of the area. Cut vocabulary: crescent rows (scales, tiles), diamond lattice, coin holes (round hole, square of hide inside), cloud collars and scrolls, tapered slits (cracks, rain, fur), strata along a ridge, leaf holes in a canopy. A dark carved outline (1.5–2.4 px) on every region. With `destination-out`, set `fillStyle = '#000'` first: fill alpha decides how much is cut.
**Figures are jointed puppets**: 8–12 pieces, head ~1.2× natural with large headwear, sleeves in a different dye from the torso, round joint lobes with a rivet (dark ring + pale knot), arms by 2-bone IK from hand targets, held objects as separate pieces on the hand line. Sympathetic figures have *open-cut faces* (features as thin strips around a hole); antagonists solid faces.
**Scenery** is carved set pieces on mostly empty lit cloth: one lace piece per element, a narrow carved ground strip, separate liftable strips for water or sky. Fire, smoke and moving water are carved pieces on rods, swapped and shaken, never particles.
**Stage and backstage.** A lacquered frame (lintel, pillars, warm spill on its inner edges), optional blurred audience heads. Backstage = the same transmission canvas with the camera mirrored (`x → −x`), dark wood around, hanging lamps (additive flame glow; snuffed ones trailing dark smoke), hands and sleeves as **silhouettes with a warm rim** (mask minus mask shifted away from the light), rods sharp.

COLOUR LOGIC
- All colour is **transmitted**: a dye filters warm lamplight, so nothing is brighter than the lamp and every hue leans warm.
- 3–5 saturated dyes + **raw hide** (the neutral) + **ink** (outlines, hair, boots). Holes are the brightest value.
- Overlap is darkness: plan which pieces cross.
- Brightness comes from the number of lamps, not grading. Night is fewer lamps, never added blue.
- Thin things that must read (strings, shafts, text) are dark: pale pieces vanish on a bright cloth.
- Example sets: *court* (vermilion, ochre, malachite); *river* (indigo, jade, one ochre); *harvest* (flame orange, gold, one green).

TYPE & SUBTITLES
- Subtitles belong to the theatre: a **storyteller's placard** (narrow dark-lacquer board, thin gold line, small carved seal), a label on a rod, or letters carved into hide. Classical OFL serif (e.g. Cormorant Garamond 600), 40–50 px, cream on dark, short slide in. Hold ≥ max(1.8 s, speech + 0.6 s).
- **Titles are puppets**: dark hide plaques with letters cut out (`fillText` + `destination-out`, so words glow), a carved border, carried on rods, pressed on to enter, lifted off to exit.

MOTION QUALITY
- **24 fps continuous** for puppets and camera; only swapped effect pieces (fire, water) step at 12 fps.
- **Hand tremor** on every puppet (a few px, ~0.02 rad, low-frequency), stronger under tension.
- Body glides with the main rod; **legs, plumes, tassels are simulated** (damped pendulums / spring chains driven by body acceleration, pre-simulated at a fixed rate for determinism).
- Operatic phrasing: shuffling run → hop → crouch → **freeze pose on a percussion accent** (~1 s, only loose parts quiver) → head turn.
- Faces never change; attitude = head angle + body tilt (up = resolve, down = sorrow).
- **Press on / lift off**: arrivals come large and blurred onto the cloth and snap sharp (~0.5 s); exits grow, blur and fade (~1 s).
- A new lamp flares with overshoot; a snuffed lamp gutters with fast flicker.
- Never moves: the cloth, the frame. Nothing morphs: pieces rotate on rivets, slide on rods, or are swapped.

CAMERA GRAMMAR
A vocabulary, not a route. Opening and ending come from the topic.
Puppets in profile; most of the cloth stays empty. When zoomed in, clamp the camera to the screen rectangle (no black bars); push on frame centre unless diving into a lamp. Transitions: lamps dimming, a blurred foreground pillar wipe, a piece lifting off in light, a flash to warm white; never a generic crossfade or digital wipe.

SOUND PALETTE
- **Percussion first, not piano and strings** (gong-and-drum pattern thinking, 锣鼓经): low barrel drum, hard high clapper drum (woodblock), cymbals, small gong with rising pitch ("tai"), big gong with falling pitch ("kuang"); also a bamboo clapper board, pellet drum, sanxian pluck, a bamboo flute for quiet.
- **Bowed and reed voices**: a bright nasal fiddle (erhu-like: pitched up, nasal formant, pitch-curve resampling for slides and vibrato), plain pentatonic (**huan-yin**) or sorrowful microtonal (**ku-yin**: raised 4th, lowered 7th); a shrill suona-like reed (oboe + 1.2–3 kHz formant + saturation) at most once.
- **Techniques, as options**: a single tap inside a silence; one big gong per decisive hit; a heartbeat drum under dread; accelerating rolls into an entrance; one hit per visual step; a gong scrape to change space; a hard stop (≤ 30 ms, tails included).
- **Silence**: hard-cut the whole bed; at most one sound inside (a breath, a tap, a sizzle).
- **Foley follows the materials**: wooden clapper (醒木), match strike, lamp "fwoomp" and "pfft" + sizzle, rod taps on cloth, dry leather flaps on lift-off, leather steps, creaks of hide and wood props, leather thwacks, cloth rustle, backstage room tone.
- **Voice**: a storyteller, few short lines, small-theatre reverb; music ducked ~12 dB, foley ~4 dB, voice ≥ 12 dB above the bed; −14 LUFS; check each line with whisper alone and on the final mix.

NATIVE MOVES
A menu: use the ones your story needs; put the strongest at the peak.
- **The light is the world.** Lamp count drives heat and colour. *Fits content like:* city lights spreading at night; a fever rising and falling; servers switching on one by one.
- **Off the screen = big and blurry.** A piece pulled toward the lamp grows and blurs. *Fits content like:* a rumour growing; a memory fading; a deadline pressing onto the cloth and snapping sharp.
- **Go behind the screen.** Lamp, hands, rods. *Fits content like:* how a product is really made; who controls a market; a teacher revealing the method.
- **Carved and translucent.** Holes glow, overlaps darken, open faces vs solid. *Fits content like:* an x-ray of a system; good and bad actors in a fable; layers of a city.
- **Hand-held.** Tremor and swinging parts. *Fits content like:* a nervous interview; a tightrope; a toddler's first steps.
- **Piece swap.** One carved piece on a rod replaced by another. *Fits content like:* seasons; a machine's states; weather over a harvest.
- **Strip lift-off.** A band of scenery lifted to reveal another. *Fits content like:* geological layers; before/after a flood; an old town under a new one.

PITFALLS OF THE MEDIUM
- Flat clip-art look → hide texture, denser openwork, deeper dyes, carved outlines everywhere.
- Figures unreadable in profile → bigger head, fuller lower body, collar piece, contrasting sleeves.
- Radiating shapes read as saw blades or petals → flame tongues with a fat base, belly and hooked tip, alternating curl.
- Grey corners when overexposed → warm vignette.
- Thin pale pieces vanish → make them dark.
- Mirror view flips pieces when rotation uses the x scale for both axes → x scale for x, y scale for y.
- Smoke with `lighter` is invisible on bright cloth → dark wisps, warm additive only near a lamp.
- Push-in centred on an edge lamp shoves figures out → push on frame centre.
- Beds leaking into a silence → hard-cut them.
- Headless screenshots time out under GPU load → retry.
```
