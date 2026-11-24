Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Thermal Camera" style.

MATERIALS & RENDERING
- **Two passes, one camera.** A picture pass (PBR, real lights, night or day) and a thermal pass in which every material writes °C into a float target. The composite chooses per pixel: picture, thermal, or a split.
- **Thermal materials**: a surface temperature model (steady-state wall: T_out + ΔT·U·R_se; bridges as bands; local sources as Gaussians; plumes as animated noise), then `apparent = ε(θ)·T + (1−ε(θ))·T_reflected`, with emissivity falling off at grazing angles and the reflected scene temperature from the view's mirror direction (cold overcast sky above, snow or ground below). Glass ε ≈ 0.84, paint/brick/skin 0.9–0.98, bare metal 0.1–0.3.
- **Sensor simulation** in screen space: render the thermal pass at ~3× the sensor grid, integrate with an optics blur into the sensor resolution, add temporal noise (~0.1 °C), fixed column/row patterns and corner shading, read the sensor back for all measurements, then upsample with bicubic and a mild detail enhancement. Optional edge overlay from the thermal depth buffer (never from the picture, which carries snow, rain or bokeh into the heat image).
- **Transparent heat** (exhaust, breath, steam) blends in temperature space: alpha-composite °C, not colour.
- **The picture pass must hold up on its own** (real light sources, believable materials); the thermal reveal is only as strong as the baseline.

COLOUR LOGIC
- One palette per film (iron: black → indigo → purple → magenta → orange → yellow → white).
- The scale (level and span) is part of the story: wide span for overview, narrow span to reveal faint structure, the same scale when two images are compared. Out-of-range values clip to the palette ends.
- The isotherm owns one colour outside the palette (a saturated green) and means only "above the threshold".
- Overlay graphics are white with a dark halo; a single accent (pale green) is reserved for a verdict.
- The visible picture keeps natural colour; it is never tinted to match the heat.
Example (our demo): palette stops #000008 #120554 #4D0894 #991294 #DB3352 #FA7812 #FFBF21 #FFFADB; isotherm #2BD14A; verdict border #7DFFA0.

TYPE & SUBTITLES
- **Roles**: a monospace for readings, scale ends, time stamp and measurement names (IBM Plex Mono, 22–40 px); a sans for labels (Noto Sans SC for Chinese 24–40 px, Inter for English 20–32 px). OFL, self-hosted, CJK subset to the characters used.
- **Labels belong to the camera or to the world**: a spot meter's value sits beside its crosshair; a label hangs on a leader line to the thing; the scale and the time stamp sit at the frame edges. No title bars, no centred captions.
- Bilingual labels are two lines, one language each. Numbers keep one decimal and flicker in the last digit at the meter's refresh rate; the value registered for reading time is the measured median.
- Nothing below 20 px at 1080p. Each label holds ≥ max(1.5 s, characters ÷ reading speed + 1.5 s) (CJK 4.5 chars/s, Latin 15 chars/s), measured from the moment it is fully on screen.

MOTION QUALITY
- 30 fps; sensor noise is new every frame, meter readouts refresh at ~6 Hz, overlays snap in with a short ease (crosshairs collapse onto their point, numbers roll up from the ambient value and land).
- Heat moves the way heat moves: plumes rise and waver, breath and turbulence flicker, cooling is slow. Bodies breathe.
- A scan line travels at a steady, mechanical pace (1–3 s across the frame) with a thin white edge and a faint warm glow on the thermal side.
- Never moves: the scale bar and time stamp positions; a locked comparison frame.
- **Frame rate and grain**: mux grain 1, a light film grain.

CAMERA GRAMMAR
Framing rules: the subject that carries the reading sits away from the scale bar (right edge) and the time stamp (bottom left); readings never cover the hot spot they measure. Transitions: scan lines, splits, level/span changes, the shutter freeze of the detector; cuts on beats. No cross-dissolves between visible and thermal; no wipes that are not a sensor action.

SOUND PALETTE
- **The instrument**: a faint fan hum whenever the camera is on; scan = electric hum + filtered noise sweep; lock-on beep with a quick roll of ticks; a two-tone alarm for the isotherm; a soft mechanical clack for the detector's flat-field shutter (NUC); a click for a level/span change.
- **The world's heat as sound**: wind and cold air, water moving in pipes, an engine ticking as it cools, the sounds of the living subject (breath, a rustle, a footstep) kept close and dry.
- **Music**: sparse and cold (bowed vibraphone, glass, harp, celesta-like glockenspiel, strings in a low register); a pulse that can turn into a heartbeat when the subject is circulation; one warm chord for the verdict.
- **Mix**: tones and foley over the music; music ducks under readouts and living sounds; low end under control (high-pass ~40 Hz); about −16 LUFS before mastering, true peak ≤ −3 dBTP.

NATIVE MOVES
- **The scan line.** A line crosses the frame and everything behind it turns to heat (or back). *Fits content like:* a server room at night; a kitchen before service; a forest edge at dusk.
- **The split that travels.** A fixed vertical split while the camera moves: the world slides from heat into plain sight. *Fits content like:* pipes in a floor; wiring behind a panel; blood flow under skin after exercise.
- **Level and span.** Narrowing the scale until faint structure appears (veins, joists, wet patches). *Fits content like:* underfloor heating; moisture in a ceiling; a cooling solder joint.
- **Isotherm alarm.** Everything above a threshold turns one colour. *Fits content like:* overheating breakers; fever screening of a crowd of animals; leaking seals on a cold-store door.
- **The hot-spot tracker jumps.** A max marker goes to the hottest point and leaps when something new warms up; between leaps it holds still on that point. *Fits content like:* a phone charger on a desk; an eye opening; a bearing starting to fail.
- **Same scale, two times.** Before and after side by side, locked, read, judged. *Fits content like:* a repaired roof; a fever going down; a machine after maintenance.
- **Ghosts.** Heat left behind where something was: a car that drove off, a hand on a table, a body on a bed. *Fits content like:* a crime-free mystery; who sat where; how long ago.
Our demo opens on *the scan line* across the whole subject, uses *the split that travels* (with *level and span*) and *the hot-spot tracker* as its signature shots and *same scale, two times* as its verdict, and ends on a scan back to the ordinary picture. A new film opens and ends another way: a visible close-up where a warm hand print appears only in heat, an *isotherm alarm* that fires before we know why, an ending on *ghosts* left where something was, or the hot-spot tracker settling on the last warm thing.

PITFALLS OF THE MEDIUM
- **Glass is not transparent.** Infrared does not see through windows; lit rooms look no warmer. Never show interiors through glass in the thermal pass.
- **Reflections lie.** Shiny metal and glass show the sky's cold or a lamp's heat, not their own temperature. Measure on matte, high-emissivity surfaces, or say what is reflected.
- **Too clean reads as CG.** Without noise, optics blur and a coarse grid, the image looks like a gradient render. Simulate the sensor, then upsample.
- **Moiré from fine geometry** (railings, grilles, branches): supersample the thermal pass and blur before decimating to the sensor grid.
- **Numbers must match colours.** Read every value from the same temperature buffer that is coloured; register the measured median for reading time; keep the scale range visible whenever a value is shown.
- **Comparisons on different scales are meaningless.** Lock level and span before judging.
- **Trackers must not chatter.** The per-frame hottest pixel is noisy: two equally warm spots (two eyes, two bulbs) swap places from frame to frame and the marker jitters by whole sensor pixels. Find the hot point once from the buffer, pin the marker to that point on the object (in world or object coordinates) and let it ride the camera; only the reading refreshes.
- **Thin hot features vanish at distance.** A crack or a gap one sensor pixel wide reads cooler from far away; frame closer or use a box maximum.
```
