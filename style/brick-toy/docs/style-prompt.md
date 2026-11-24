Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Brick Toy" style.

MATERIALS & RENDERING
- **Plastic**: ABS with strong clearcoat (~0.85, clearcoat roughness ~0.07); a roughness map with faint fingerprints and scratches (base ~0.17, scratches up to ~0.65); a bevel so edges catch a highlight.
- **Geometry**: rounded box bodies (bevel ~0.045 stud), studs with a thin lip. Unit = 1 stud pitch (8 mm); brick height 1.2, plate 0.4. Large baseplates are **matte** (lower clearcoat than bricks), or they reflect softboxes as a coloured wash.
- **Set**: a real surface with a real texture and real props at true scale (1 m = 125 studs). Household objects become landscape. Light it with a **neutral daylight studio HDRI**, never a tungsten interior (white bricks turn orange).
- **Light**: a toy-photography setup. A directional key from one upper side, warm-white, casting shadows (fit its shadow frustum to each shot so shadows stay crisp); two or three **RectAreaLight softboxes** (key front, cool rim behind, one over any big set) so glossy plastic shows clean rectangular highlights; environment kept low (~0.45) for contrast. Add a soft contact-shadow blob where the key's shadow falls behind a hero object.
- **Ambient occlusion is mandatory**: GTAO (radius a few studs) darkens seams between stacked bricks, stud bases and contact points. Without it bricks look pasted on.
- **Supersample 2×** (render 3840×2160, deliver 1080p): thin stud edges and bevel highlights shimmer without it.
- **Lens**: physical depth of field (CoC ∝ |1/focus − 1/z|). Scale aperture with shot size: widest for close-ups, less for mediums, least for a wide (too much blurs the hero at mid distance). Focus is always on a character's head or the brick being placed.
- **Effects are bricks**: fire, smoke, water, light, weather, celestial bodies are all built from plates, round bricks and translucent pieces, jittered or rebuilt per drawing. Nothing is a particle system or a soft gradient.

COLOUR LOGIC
- A **toy palette**: a handful of saturated brick primaries plus white, black, greys, tan. Each built object has one dominant colour and at most one or two trim colours; characters and hero objects take the most saturated ones.
- The **real world stays natural and muted**: wood, paper, ceramic, daylight. The saturated colour belongs to the toys, which makes them pop against the room.
- **Whites must read white**: grade slightly cool so white bricks don't go cream from the room bounce; soft contrast, saturation lifted ~10 %.
- Translucent pieces (orange, yellow, clear, blue) are reserved for effects: fire, light, water, glass.
- Example palette (the classic brick primaries): red `#c91a09`, blue `#0055bf`, yellow `#f2cd37`, green `#237841`, white `#f2f2ee`, black `#1b2a34`. Others: pastels and tan; greys with one accent.

TYPE & SUBTITLES
- A rounded, friendly OFL face (e.g. Fredoka, in `core/fonts`) for titles and subtitles.
- Subtitles live on a **toy element**: a dark rounded pill low in the frame with a small brick icon beside the text, or a printed tile, or a sign built into the set. White on dark, ≤ 2 lines. A second speaker type (radio, device, narrator) gets its own icon colour or tag. Hold ≥ max(1.8 s, speech + 0.6 s).
- Titles pop in on 12 fps steps, like a piece being placed; a short sub-line in a brick colour.

MOTION QUALITY
- **Characters and bricks step at 12 fps** (on twos); **camera, focus, fire and flying vehicles move on ones** at 24 fps. Stepping the camera too reads as judder, not charm.
- Bricks **fly in on arcs** and **snap** with a tiny overshoot (a fraction of a stud → 0 in ~4 frames). Every snap is a click on the soundtrack.
- Characters act with **big readable poses** held for several drawings: a V of arms for pride, arms flung up for alarm, a bowed head for sadness, a hop for joy. Walk cycles swing legs about ±0.55 rad with a small bob. Arms and legs rotate at their joints; nothing bends.
- Collapses and falls use **real rigid-body physics**: tip about the base edge, then independent bodies with gravity (a lower-than-true value reads better at this scale), bounces (restitution ~0.3), friction, settling flat. Pre-simulate so every render is identical.
- Tiny imperfections are allowed. Bricks never stretch, squash or morph.

CAMERA GRAMMAR
A vocabulary, not a route. Opening and ending come from the topic.
Framing: keep foreground clutter out of the frame edges (piles of parts love to block the lens); check headroom in every shot; check every real prop against every camera, because an object placed on one shot's axis sneaks into the background of another. Cut on beats or bars; a snap can be a cut point. No generic dissolves; transitions can be a hand-placed brick covering the lens, a whip along the desk, or a pull out of one set into the next.

SOUND PALETTE
- **Instruments**: small, acoustic, playful (glockenspiel, toy piano, pizzicato, ukulele, whistling, claps, melodica); a brass or orchestral swell for scale; a heartbeat drum for suspense. Licensed or original, edited to the picture (cut on bars, bar-aligned jumps, chroma similarity at joins) rather than looped under it.
- **Foley** (synthesizable): plastic creak before a failure, tiny plastic footsteps, a crash of many clacks, a brick click (short transient + resonances near 2.9 kHz and 4.6 kHz), clack on wood, rattle of a parts pile being stirred, whoosh on throws, a bell "ding" for discovery, bricks sliding over paper, motors and engines built from filtered noise.
- **Silence is a tool**: cut music dead at a failure or a surprise; leave only room tone and one small real-world sound (a ticking clock, a fridge hum, a distant bird). Music can also drop out before a big moment, leaving a pulse or a countdown.
- **Voice**: a warm storyteller, few short lines with room to breathe; secondary voices through a device filter (band-pass ~380–2800 Hz + soft saturation), optionally with beeps. Verify every line with whisper.
- Mix: voice compressed, music ducked ~40 % under voice, loudness −14 LUFS.

NATIVE MOVES
A menu: use the ones your story needs.
- **Snap-together building.** The plot is an act of building; pieces fly in on arcs and click home. *Fits content like:* a company assembling its first product; a neighbourhood rising street by street; a recipe put together ingredient by ingredient.
- **Break without harm.** Things fall apart and the parts become something new; failure yields spare parts. *Fits content like:* a pivot after a failed plan; recycling; a team reshuffled into a better shape.
- **Toy on a real desk.** Real objects become landscape at toy scale. *Fits content like:* a laptop keyboard as a city grid; a houseplant as a jungle; a coffee spill as a flood.
- **Hand-made stop-motion.** 12 fps poses, tiny imperfections, even fire and water made of bricks. *Fits content like:* a weather report; a kitchen explained with brick flames; a science demo of a volcano.
- **Rebuild into something else.** The same pieces, rearranged on screen, become a different object. *Fits content like:* one career turning into another; a before/after renovation; a word re-spelled.
- **The scale reveal.** Pull back until the whole world is a toy on a table. *Fits content like:* a big ambition that is still small; a year in review; a child's imagination.

PITFALLS OF THE MEDIUM
- Tungsten HDRI → orange whites. Use a neutral studio HDRI and a slightly cool grade.
- Clearcoat on a large flat baseplate reflects softboxes as a coloured wash. Make it matte.
- Flat, slightly jagged first renders: even light, no AO, single-sample rendering, stepped camera. Fix: softboxes + low environment, GTAO, 2× supersampling, cool grade, camera on ones.
- Too much aperture blurs the hero at mid distance. Scale aperture with shot size.
- A fast-rising or fast-moving object becomes a dot on a blank wall. Track it and keep its destination in frame.
- Curved transparent parts made from a sphere segment vanish inside a cylindrical part. Use an open-cylinder panel.
- Real props on a later shot's camera axis sneak into the background. Check props against every camera.
- Random jitter used as a material or colour index can go negative → undefined material crash. Clamp.
```
