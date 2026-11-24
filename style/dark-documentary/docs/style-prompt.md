Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Radar Sweep Documentary" style.

MATERIALS & RENDERING
- **One scan function** shared by every material in a space: `d = S − dot(p, D)` for a plane moving along `D`; `d < 0` is unseen (black), the front `exp(−d²/w²)` is the bright line, behind it a glow `exp(−d/L₁)` and a slow trail `exp(−d/L₂)` with `L₂ ≫ L₁`. Ghost copies of the front every few metres decay faster still. The scan is analytic, so every frame depends on `t` only.
- **Two scans can overlap**: behind the new front the new scan applies, ahead of it the previous one keeps ageing. This is how a scan rewrites a scene into another time.
- **Echo geometry**: contour lines from height, range rings around the subject, a coarse survey grid, all line-antialiased with `fwidth` and faded out where they would alias. Structure edges (EdgesGeometry) and panel seams follow the same scan.
- **Surfaces**: near-black base, lit only by the scan terms, a small daylight term when the scene is a grey day, and a rim term `(1 − n·v)³` in cold blue for the subject. Additive lines for wireframe, opaque surfaces for silhouettes.
- **Hard light when there is a real light source** (a low sun): flat-shaded facets, one strong direction, long planar shadows cast onto the ground. Crinkled foil or rough metal take a noise-perturbed normal and a narrow specular.
- **Sky**: either deep night (a dim Milky Way and stars that never flicker) or an overcast day (a layered, slowly rolling cloud deck, brighter at the horizon, with slanted rain). The two must be told apart at a glance: day is a lit grey, never night.
- **Post**: bloom, vignette, a soft shoulder, a static sub-LSB dither against banding. No grain that changes between frames.
- **Several spaces, one treatment**: terrain, a lunar surface, a globe in space, a room interior each get their own scan direction and the same echo rules.

COLOUR LOGIC
- Black is the ground. Brightness belongs to what the scan has just touched.
- Echo colour: a cold blue-white (example `#B3DBFF`); the scan front is white.
- One warm accent (example `#F2C48D`) reserved for the target and the signal path; at most one warm element per frame.
- A warning red (example `#FF8C78`) only for a value over a limit, and only on the readout.
- Text: three levels of cold grey-white (examples `#EEF2F6`, `#C4CED8`, `#96A2B0`).
- A grey overcast day may lift the whole frame (cloud base up to about `#3B3F44`), but keeps the low key and the cold rim.

TYPE & SUBTITLES
- **Title**: a thin humanist sans (IBM Plex Sans 200 in the demo), 50–70 px, sentence case, never centred, no shadow or outline.
- **Readings and labels**: a light monospace (JetBrains Mono 300), 14–17 px, all caps, tracked +1.2 to +2.4 px; big numbers (a time, an angle, a ratio) in the thin sans at 50–150 px.
- **Chinese** (Noto Sans SC Light): one short line under the title or as a section label, ≥ 34 px.
- **Every text is written by a scan bar**: a clip that grows left to right with a 1 px bright edge leading. No typewriter, no fade-in, no pop.
- Labels sit next to their object with a thin leader line; numbers sit on the instrument (an arc, a trace, a track). Never a header strip, never a centred stack.
- **Subtitles**: bottom left, one Chinese line (34 px) over one English line (≈ 23 px), joined by a 1 px echo-coloured vertical hairline; a soft dark patch behind them on night frames only. Hold ≥ max(1.8 s, speech + 0.6 s).
- Small text never sits on a busy picture without a plate: a dark rounded plate under a readout that overlaps an image.

MOTION QUALITY
- The scan advances in surges on the beat (an ease-out push per beat, mixed with a steady drift so it never stops dead). Its speed is the film's pulse.
- Echoes decay smoothly; nothing blinks.
- Machines move with weight: anticipation, a long ease, a bounce against the stop, and a wireframe after-image trail while moving.
- Weather, when a film has it (wind, rain, dust), moves continuously and slants with the gusts; the camera shakes only with the weather, more in close-ups.
- Stars never move on screen: on a star shot the camera only translates (no rotation) and the sky rides with it.
- **Frame rate and grain**: 24 fps.

CAMERA GRAMMAR
Framing: a large real object keeps its ground, horizon and base in frame; put the camera above the ground and check its height (a camera backed off along an elevated direction can end up under the terrain, and the object then floats). The subject fills at least a third of the frame height at key moments.
Transitions belong to the medium: a scan that rewrites the scene in place; a **scan wipe** between two live views (two renders, a 1 px bright edge sweeping, the next view behind it); a hard cut on an impact; a match cut from one rectangle or circle to another (a monitor's screen into the picture it shows); a CRT collapsing into a line that becomes the next scan. No cross-dissolves, no page turns.

SOUND PALETTE
- Ambience first: wind (low, slow gusts), rain hiss, room tone; it drops away in space.
- Scan foley: an air surge per push and a clean sonar ping (with two quieter echoes), panned with the scan; a struck metallic ring when the scan meets the subject.
- Machines: drive motors, gear teeth, a heavy clank on the stop, metal groan under load, an electric bell behind glass.
- Signal: a short quindar-like beep, a low carrier, TV hiss and line buzz that clean up when the source improves.
- Score: low strings and bowed vibraphone held long, a vibraphone pulse when the signal arrives, glockenspiel for stars; one bright chord at the turn. Leave 300 Hz–4 kHz for the voice; voice about 10 dB above the bed.
- At least two near-silences (a verdict panel; the moment before the turn).

NATIVE MOVES
1. **The scan reveal**: black → a light plane sweeps → the subject exists → it settles into echoes.
2. **Sky behind the beam**: stars and the Milky Way appear only where the scan has passed.
3. **Scan rewrites time**: in one shot, a second scan replaces night with day (or one state with another) and erases the old labels as it passes.
4. **Motion echo**: wireframe after-images left behind by a moving machine.
5. **Radial ping**: the subject becomes the source; a ring of light runs out across the ground and the range rings flash.
6. **Instrument readouts written by the scan**: an angle counting to its limit on an arc; a trace crossing one threshold line; a comparison held still (one quantity beside another).
7. **Scan line = TV raster**: a picture written line by line, rough at first and sharpening as the lines fill in.
8. **CRT to scan**: the picture collapses into a line, and the line is the next shot's scan light.
Our demo opens on *the scan reveal* with *sky behind the beam*, uses *scan line = TV raster* (entered on a match cut) as its turn and ends on *CRT to scan*, a pull back and up and a last *radial ping*. A new film opens and ends another way: a radial ping from the subject as the first light, a vertical sweep that reveals an interior, a cone from a source, or an ending on echoes fading back to black.

PITFALLS OF THE MEDIUM
- Night everywhere: a day scene left as black as night tells the wrong time. Lift the sky and the ground into a grey day.
- Too dark to read: subject and labels sinking into black. Give the subject a rim and lift the exposure of reading shots.
- A scan front passing right in front of the lens blows out the frame; fade the front near the camera.
- Star flicker: sub-pixel stars and a rotating camera. Keep stars ≥ ~1.5 output px, below the bloom threshold, and the camera unrotated.
- Dark patches behind text show as smudges on a grey sky; use them only on night frames, or use a proper plate.
- Floating models: a camera below the terrain or an object without ground contact; keep ground, horizon and base in frame, add contact shadows.
- Blocky stand-ins: a recognisable machine needs its recognisable parts (legs, pads, facets, windows, antennas), not two boxes.
- Fake data: a trace or gauge with invented numbers. Label only the sourced value; leave the rest unscaled.
```
