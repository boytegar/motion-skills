Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Circuit Board" style.

MATERIALS & RENDERING
Layers bottom-up, all in board space under one world → screen camera:
1. **Laminate (FR-4):** dark green, a glass-weave basket texture in two crossfaded octaves so its on-screen size stays bounded at every zoom. Bottom-layer copper may show through faintly.
2. **Copper pour:** fills the board; everything is a **clearance** punched out of it (channels, lakes, sea, keep-outs around parts and traces). Rendered twice: a lighter rim pass with the plain clearance, then the fill pass with the clearance grown by ~1.6 px, so every copper edge gets a bevelled rim. A soft mottling texture sits only on the pour (solder mask is never perfectly flat).
3. **Traces:** rim colour stroke + narrower fill stroke, round caps; a channel of bare laminate runs on both sides (the clearance). Parallel traces keep the pour out of the gap between them when they are a pair.
4. **Exposed metal:** ENIG-gold pads (rect, round, square pin-1) with a sheen band on the top third and a dark drill hole; vias are gold rings with dark bores; bare copper (salmon with a vertical gradient and brushed streaks) only for one heavy, special part. Fiducials and mounting holes at board corners.
5. **Silkscreen:** off-white monoline strokes with a pinhole texture punched out, slightly blurred (0.35 px). Outlines, designators, polarity marks, component symbols, labels.
6. **Light:** an additive layer (glow) with two blurs (≈24 px and ≈5 px) under a sharp copy. Current, water, lit pads all live here.
7. **Screen space (once per frame):** a soft warm key light drifting from the upper left, shallow depth of field (a blurred copy faded in at the top and bottom bands, stronger in macro shots, weak or off where text must be read), out-of-focus motes of light drifting with parallax in front of the board, a vignette, film grain at mux. A window's light (a soft parallelogram with a mullion shadow) may fall on the board when the whole board is on a desk.
Level of detail: textures crossfade between octaves; small features (vias, designators, rims) fade out before they go sub-pixel, so zooms never shimmer. Nested boards (a module inside a board) are separate boards composited into their footprint, so a continuous zoom can cross six or more orders of magnitude without precision loss.

COLOUR LOGIC
- Ground: green solder mask in three values (dark laminate, mid pour, light rim/trace). The film is green; never recolour the mask per scene.
- Metal: gold for pads and vias, one bare-copper accent at most.
- Ink: one off-white for all silkscreen.
- Light carries meaning and is the only saturated colour: **amber** for energy/current (with a near-white core), **silver-cyan** for a second kind of flow (water, data, goods) when the story has two. A third flow gets a third light colour only if it must be told apart; red is reserved for a fault and appears at most once.
- Dark (bare laminate, sea, holes) for what is absent.
Example: laminate #05190F, pour #1C6A44, rim #4AA977, gold #D9B25B, silk #F2EEE1, current (255,190,84), water (170,214,226).

TYPE & SUBTITLES
- All type is silkscreen in one **monoline stroke font routed like traces** (0/45/90° strokes, chamfered corners, round caps), drawn in code. Chinese uses the same rules: every hanzi hand-routed on a 0/45/90° grid with the same stroke width as the Latin line beside it. Use simplified characters only and proof every glyph at large size before use.
- Text is world-anchored and **plotted stroke by stroke**; it stays on the board and is still there when the camera pulls back.
- Roles: labels (a Chinese line above a smaller English line, same stroke width), board-native text in one language (designators, voltages, timestamps, km, values), micro texture (pinyin tags, notes, REV/sheet lines) which may fade below reading size.
- Reading: a label must be fully in frame and stay there for its reading time (≥ 1.5 s + characters at 4.5/s CJK, 15/s Latin). Never let text sit in the depth-of-field bands; never cut text at the frame edge in a held shot. Title caps ≥ 60 px at 1080p.
- Moving camera: `readcheck.mjs` starts a label's clock when it is first fully in frame and stops it the first time it leaves, so a label that slides out during a ride or whip fails even if it comes back. Before plotting a label, check where the camera path takes it for its whole reading time: place it where the path keeps it in frame, print it once the camera settles, or let it be micro texture (under ~9 px tall it is not reported).
- Glyph coverage: the routed font draws only the characters it has. The demo's `hanzi.js` holds just the hanzi its film needed, and the Latin set is capitals, digits, common signs and a few lowercase letters (`a b d e k l m o p s u z`, enough for units like ms, km, kV). A missing character prints as `?` with no error. Check every label string against the font before building, write English labels in capitals (or add the lowercase you need), and route the hanzi you lack (§10, New hanzi).
- With a voice, every line gets a subtitle in the bottom safe area: a dark translucent plate, Chinese over English (an open-licence sans, e.g. Noto Sans SC, subset to the characters used; never a system font), 34 px / 24 px at 1080p. The board then prints only the voice's key words and numbers, never whole sentences. Keep picture text above the subtitle band.

MOTION QUALITY
- 30 fps, everything on ones. `render(t)` depends on t only; motion blur from measured screen motion (180° shutter, sub-frames from 1 to ~20), never across a cut.
- Kinds of current, told apart only by how the light moves:
  - **AC / alternating:** short dashes rocking back and forth along the trace (a slowed mains frequency) over a faint energised line.
  - **DC / one-way:** a continuous lit stream behind a travelling head, with slow streaks inside and an afterglow that dims slowly behind the head.
  - **Step-down:** the trace gets thinner and the dashes smaller.
  - **Water / second flow:** several parallel streamlines of moving dashes at different speeds, faster in the middle, with short glints.
- Pads flash on arrival (a quick bright bloom, then a steady glow). Components "conduct": the symbol's internal edges glow while current passes.
- Silkscreen appears only by plotting; copper and pads never pop in. What never moves: the board itself (only the camera moves).

CAMERA GRAMMAR
Straight-down camera always (rotation about the view axis allowed, small). Moves:
Framing rules: the lit subject sits on a third or dead centre for symbols; at least two layers of detail around it (background copper/vias/routes, foreground motes or depth-of-field blur); the board edge appears only when the whole board is the subject. Transitions: motion-into-motion and matched cuts; no fades, no wipes, no page turns.

SOUND PALETTE
All synthesised: sine/saw tones, pulse waves, FM bells and glockenspiel, relay clicks (two-click transients), soft kicks and 16th ticks, filtered noise. Tune the drone to the mains hum (50 Hz ≈ G1; 60 Hz ≈ B♭1) so the hum belongs to the key. Rule the ear can follow: an **amplitude-gated tremolo tone = alternating**, a **steady tone = direct**; a component that converts one into the other is heard as the tremolo smoothing into the long tone. Arrivals are relay clicks or a filtered noise burst; a step-down is a relay one step lower each time; plotting silkscreen is a faint scratch of grains. Outside sounds (water, cicadas, traffic) only behind a low-pass, as if heard through a wall. Silence: drop the rhythm before a verdict, keep only long tones. With a voice, the voice leads: duck the bed so it sits 15–18 dB under the speech, effects a few dB. Mix around −16 LUFS before mastering, true peak under −3 dBTP, lean low end (high-pass ≈ 55 Hz).
### Voice-over and subtitles
When the film explains something, a voice tells it and the board shows it at the same moment: each spoken line has one picture action (a pad lights, a stream changes shape, a tag prints). Every line gets a subtitle in the bottom safe area, a Chinese line over an English line on a dark translucent plate, appearing and leaving with the voice; the board itself prints only the key words and numbers the voice says (a name, a value, a verdict), never whole sentences. Keep the voice on top: the bed sits 15–18 dB under it and rises in the gaps. Pick one calm, clear voice for the whole film; numbers and units are spoken in full.

NATIVE MOVES
- **Flow becomes current.** A second flow (silver) enters a row of pads; each pad lights on a beat and amber leaves on its other side. *Fits content like:* raw material entering a factory and leaving as product; requests hitting servers and leaving as responses; donations becoming grants.
- **Pulse-path explainer.** Every stage of a process is a component on one trace; the way the light changes (rocking, streaming, thinning) tells what each stage does. *Fits content like:* a payment's clearing path; a parcel's sorting hubs; a message across a network.
- **The map is a board.** Geography re-routed octilinear as copper and channels; rivers, lakes and coasts are clearances with via fences; cities are test points; corridors are buses of parallel traces. *Fits content like:* a pipeline across countries; a migration route; the cables under an ocean. A board that covers China follows DIRECTOR.md §13.
- **Detail callout.** An assembly-drawing circle that shows another place of the same board (or another board) at a chosen scale next to the thing it is compared with. *Fits content like:* the same mechanism at two sizes; a lab prototype beside the plant; yesterday's version beside today's.
- **Nested boards.** A module footprint on a board is a complete board of its own; plunge into it, or pull out through every level in one move. *Fits content like:* a country → city → building → device; an organisation → team → person; a chip → core → transistor.
- **Matched step-down.** The same symbol held centred while the board around it changes scale on each cut. *Fits content like:* voltage levels; approval tiers; zoom levels of a map.
- **Timestamp tags.** Silk frames printing times at two stations, the second tag repeating the first digit-for-digit above its own. *Fits content like:* latency; a race between two routes; before and after.

PITFALLS OF THE MEDIUM
- **Flat pour.** A board that is all mid-green reads as an empty slide. Fill with real board texture: stitching vias that stay visible at map scale, fences along every clearance, mottling, micro silk, other unlit routes that exist in the subject.
- **Decorative routing.** Traces that mean nothing make viewers ask what they are. Draw only real routes; texture comes from vias and mask, not invented traces.
- **Lines crossing on one layer.** Two traces can't cross on the same copper: use a via to the bottom layer, or route around. A trace may cross a bare channel (a river, a street).
- **Overexposed light.** Wide traces with additive bloom turn into white bars. Cap glow widths and flash radii in pixels, keep the amber body and a thin core.
- **Text in the blur or under the frame edge.** Depth of field and the vignette eat the top and bottom bands: keep labels out of them, or turn the blur down in reading shots.
- **Too-fast holes.** A plunge into a hole under 0.6 s reads only as "darkness". Hold on the hole first, then plunge; come out of another hole the opposite way.
- **Precision.** Zooming a single coordinate system over many orders of magnitude breaks the canvas transform. Nest boards (each with moderate units) and convert cameras between levels in double precision.
- **Hanzi in a stroke font.** Automatic skeletons of fonts lose dots and strokes; route each character by hand and proof traditional/simplified forms (鹤 not 鶴, 换 not 換). A glyph that only works at small size (one component missing, two strokes merged) reads as another character on a title: proof it large (§10).
- **The board edge by accident.** When the camera's view comes near the board outline, `Board` switches to drawing the desk and clips to the board, so a close shot near the edge suddenly shows black desk. Keep the view well inside the outline (the check uses the frame's half-diagonal plus 4% of the board width), or make the board larger than the shot needs; show the edge only when the whole board is the subject.
```
