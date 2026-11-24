Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Stained Glass" style.

MATERIALS & RENDERING
One formula for everything (a WebGL compositor), so every element sits in the same light:
```
glass   = tonemap( backlight(x,y,time) × transmittance × glass texture ) + weathering haze × room light
surface = albedo × (room ambient + glow spilled from lit panes + point lights)          // stone, lead, iron
floor   = flagstones × (ambient + projected colour-patch map, blurred + halo)            // perspective or top-down
+ additive light shafts with dust · bloom (bright glass eats into the lead: the "irradiation" effect)
```
- **Hue-preserving tone mapping**: per-channel `1-exp(-x)` turns cobalt pastel; blend to per-channel only at the very top so a sun disc burns white while blue stays jewel blue.
- **Unlit panes**: weak sky light + room-lit weathering haze, figures ghostly but readable. **Lit panes**: backlight ~2–3×, a soft, slightly skewed moving band edge.
- **Pieces**: smooth Catmull-Rom cuts (no deep concave corners). **Every piece is textured** (`multiply` in its clip): streaks at a per-piece angle, hue drift (±10 %), darker aged edges by the lead, ±9 % thickness variation. The shader adds seed bubbles, scratches and thickness clouds in world space; fade small features below ~0.6× zoom or they sparkle.
- **Lead**: near-black, one planar network, thinner on small pieces, round joins, faint grey flange, solder blobs; only where colours meet (one colour = one piece).
- **Grisaille**: tapered trace lines in translucent dark brown; **matting** (stippled wash, graded shade, soft band inside the lead); **scraped highlights** (a thin stroke beside each fold in the piece's own colour: the only highlight transmitted light allows). Patterns (mail, scales, plates) are grisaille arcs and bands.
- **Faces**: almond eyes, heavy upper lid, big pupils, brow flowing into the nose. **Expression lives in brows and mouth**; change it by swapping the face piece, never morphing.
- **Backgrounds**: hand-cut **lozenge quarries** (jittered lattice, shared vertices, a painted motif each); borders of a coloured band with pearls and a white fillet with a running vine.
- **Room**: warm grey ashlar, a lighter embrasure, mullions; floor of **large irregular flagstones**, never the wall's running bond.
- **Floor projection**: blurred, ~20 % desaturated, modulated by the stone, haloed, mirrored (the window's top lands farthest into the room). **Shafts**: additive with dust, near 0 at the window, peaking ~40 % down.

COLOUR LOGIC
- Two hues **carry the picture** (one deep, one hot); three or four more only as **small touches**. Example (Chartres logic): cobalt ground, ruby, touches of gold, green, murrey.
- Greenish white glass sparingly (faces, fillets, sky). **Flesh darker than you think**, or backlit faces burn white.
- A story light (candle, lamp, screen) may take a colour no glass uses, so it reads as a source.
- **The sun's colour is the clock**: blue-white dawn, near-white brightest noon, gold afternoon, deep orange dusk (raise intensity rather than redness), weak blue moonlight. Patch length follows sun height.
- Stone and lead take colour only from spilled or projected light.

TYPE & SUBTITLES
- **Subtitles**: a parchment **banderole** near the bottom (curled ends, gentle sag, thin rule), IM Fell English ~44 px in dark ink. It unfurls from the centre; text fades in only once it is ≥ 75 % open; pad the ribbon well past the text. Hold ≥ max(1.8 s, speech + 0.6 s).
- **Titles**: incised Roman capitals (Cinzel 600) in the stone, dark cut, light lower lip, gilding, revealed by a moving band of light; small lines ≥ 30 px on screen. In the glass, text only as grisaille on a white-glass scroll.

MOTION QUALITY
- **Figures step at 8 fps**; light, camera, shafts and dust move on ones (24 fps): stepped light reads as flicker.
- Rigid pieces on 2D FK (`setTransform(base · T(part))`, free draw order); long bodies are spine chains of banded pieces.
- **Freeze rule**: a pane's figures use `min(t, t_lightLeft)`. Actions land on the beat.
- **Crack**: jagged lines race from the impact in ~¼ s, bright with light, a short white flash and a 2-frame shake.
- **Re-leading**: piece groups slide one beat each with a small outward bump along the lead, each ending in a weld spark; cracks become mending leads.
- Glass never bends or morphs; things appear by being lit, not faded in.

CAMERA GRAMMAR
A vocabulary, not a route. Opening and ending come from the topic.
Framing: subjects big (close-ups ~1.3–2.3× a lancet width); in a narrow lancet stack figures diagonally; a revealed thing near centre, ≥ ~1/8 frame width with glow. Transitions: moving light, shafts, a crack, darkness; never a dissolve, wipe or flip.

SOUND PALETTE
- **Instruments**: organ (pedal drone, soft stops, full), harp, hand chimes, tubular bells, recorder or a chant-like voice, water-glass notes, timpani; also hurdy-gurdy drone, psaltery, portative organ. No generic piano or strings.
- **Mode families**: church modes (Dorian, Mixolydian, Aeolian, Lydian), open fifths, drones, parallel organum.
- **Options**: free unmetred chant for darkness; one note meaning "the light" each time a pane wakes; an organ pedal that holds and then cuts out; a rising scale, one step per lit piece.
- **Foley, material first** (synthesizable): struck glass = inharmonic partials ~1 : 2.32 : 4.25 : 6.63 : 9.38; crack = snap + pane thump + rain of tinkles; lead = stick-slip creak; glass sliding in a came = gritty band noise + faint squeal; solder = hiss + tick; air on light moves; stone-hall ambience; birds in the vault; iron ticking as it cools.
- **Silence**: a hard stop, reverb tails included, only the glass's own ring or creak inside; a single long-reverb bell can close it.
- **Mix**: big and reverberant; heavy hits avoid word onsets; music ducked ~9 dB under voice; −14 LUFS. **Voice**: a gentle, even storyteller, few short lines.

NATIVE MOVES
A menu: use the ones your story needs.
- **Light is time, and the camera.** Only the lit pane is alive; figures freeze when the light leaves. *Fits content like:* the steps of a recipe; a factory's shifts; four seasons of one farm.
- **Glass projects onto stone.** A key scene plays in the floor patch, which stretches as the sun sinks. *Fits content like:* a memory seen larger than life; a sports play replayed as shadows; a data trend drawn by lengthening light.
- **Glass cracks, lead mends.** Pieces slide along the leads into a new picture, leaving scars. *Fits content like:* a company's reinvention; an injury and recovery; an old building restored.
- **A window holds its own light.** At night one pane glows from a source inside the story. *Fits content like:* a lighthouse keeper; a night-shift nurse; a server that never sleeps.
- **The window as a diagram.** Lancets as columns, a rose as a wheel or dial, the border as a timeline. *Fits content like:* a product lineup; the parts of a cycle; a family tree.

PITFALLS OF THE MEDIUM
- Hidden leads show through front pieces → erase the surface under each piece before stroking its lead.
- Pastel glass → hue-preserving tone mapping; darker flesh. Red dusk × blue glass = mud → keep blue alive, raise intensity.
- Voronoi reads as a filter; flat fills read as clip-art → hand-cut lozenges, texture on every piece, heavy grisaille.
- A sharp floor patch looks like a decal → blur, desaturate, stone-modulate, halo. Shafts at full strength at the window wash out the pane.
- Top-down projection is flipped → put the wall at the frame bottom so figures stand upright (mirrored, correct).
- Patch-map canvas size mismatched with the code draws into a corner of the texture → one constant size.
- Carved text in a dark wide is unreadable → push in, park a light on it.
```
