Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Isometric People Scene" style.

MATERIALS & RENDERING
- True isometric (axes at 30°), Canvas 2D, painter's sort. Every solid and every person is sorted by its **floor footprint** (x + y at floor level), never by the centre of its box: a seat back sorted by its centre height ends up drawn over the passenger sitting behind it.
- Solids are flat-shaded in **three tones per colour**: top lifted toward warm paper, +y face the base colour, +x face dropped toward a warm umber. Curved bodies (a fuselage, a column) are faceted and quantised to the same tones plus one in-between, so they read as round without gradients. No outlines on solids, no shadows, no textures, no highlight spots (a round tree crown or a ball is a flat disc or a faceted solid in the same tones, never a glint).
- **New objects** (vehicles, machines, buildings, trees) follow the same model: built from boxes, prisms and faceted solids at real size in metres, three tones from the kit's `tri()`, recognisable from their silhouette at a glance, simpler than the people. Vehicles read from the body (cab, cargo box, a band of the subject hue); wheels are small, low and mostly under the body in one dark tone, never thick drums or tyre-and-hub detail.
- Ground surfaces carry only their functional markings (painted lanes, chevrons, lead-in lines, stop bars, seat squares) and a faint tile or slab grid.
- **Cut-aways** are part of the model: the near wall is cut low and the far wall keeps its windows; a lid is its own solid with a paper-coloured underside, hinged on the far edge so it opens away from the camera; glass is a pale tint with thin mullions and is cut to a parapet whenever it would stand between the camera and type or people.
- **People** are billboards anchored at their feet, about 1.6–1.8 m tall: a round hair-dark head with a sliver of skin at the ear, a two-tone torso, separate trouser legs, dark shoes, optional long hair or skirt, a rolling case. Seated people show only shoulders and the back of the head above a low seat back. Walking scissors the legs along the screen; arms counter-swing.
- Bodies obey space: people never pass through seats, walls or each other; a queue keeps about 0.75 m between feet; someone sitting down first steps back into the row's legroom, walks sideways to the seat and sits (never sinking into or floating above it); a lifted case travels up over the open bin and is lowered in, never into a wall or a seat.
- Everything is a function of time. A crowd that queues is simulated once at load (deterministic) and **sampled with interpolation**, so replaying it slowed down still moves every frame.

COLOUR LOGIC
- A warm paper neutral for every ground (outdoor concrete a touch cooler, indoor carpet a touch warmer), one dark ink, **one accent reserved for the rule** (the key character, the "stand here" outline, arrows, a lit sign, the winning number, a stretch of floor that matters), one **subject hue** carried by the place itself (seats, livery bands, chairs in a paler tint, sign trim), one marking colour (lead-in lines, a tug, a baggage tractor). People wear four muted tops, so a filled room reads as an even four-colour weave.
- The accent never decorates; it appears only where the rule is. It stays small because its rarity is what makes the eye go to it: many frames carry none at all. A path in the accent marks the stretch being explained now (it grows ahead of the walker or the vehicle and fades behind it), not a trail left across the whole film.
- The subject hue and the marking colour must be clearly apart, in hue and in value, from the four tops: people have to stand out from the place they move through (a blue-green place swallows fog-blue people).
- Shade toward warm umber and paper, never toward grey or violet: dirty shadows are the fastest way to lose this look.
- Example palette: paper #F1E8D8, concrete #EAE2D3, carpet #F2E5CE, ink #2A2E29, accent persimmon #D0573A, subject green #3D6B59 / #A7C2B2, marking ginger #E2AE3A; tops ochre, fog blue, grey violet, oat.

TYPE & SUBTITLES
- Chinese: a serif (Noto Serif SC, 700 for keywords, 500 for particles); Latin and numerals: a book serif (Newsreader). Open-licence fonts only, subset to the characters used.
- **Floor type**: big ≈ 1.5 m on the ground, small ≈ 0.8 m; read along the up-right axis (coordinates in §10). One phrase per beat; one line where the frame is tight. One accent character or number per phrase, never a single thin stroke (一 in the accent reads as a line, not a character). Characters land ≈ 0.14 s apart: fade and scale 1.22 → 1, pale ghost → ink. A painted title or a pair of numbers may scale up to ×1.6.
- Type sits on clear ground: nothing stands on it, nothing raised stands in front of it (see §9), at least 5 % from every frame edge, and clear of the subtitle bar.
- Signs in the world (gate signs, group signs, screens) face the camera side of the room at the iso angle, never a flat card facing the lens.
- Reading time: every piece of world text stays fully in frame ≥ CJK/4.5 + Latin/15 + 1.5 s.
### Narration & subtitles
When the film tells something, a narrator says it and the picture shows it at the same moment: one line per action or shot, each line starting as its action starts. A calm, quick reader in the film's language; the floor carries only that line's key word or number, never the sentence. Subtitles: one cue per narration line on a pale bar in the lower safe area, one Chinese line above (≈ 34 px) and one English line below (≈ 25 px, a translation of the whole sentence), switched hard, never two cues on screen. Numbers the viewer must remember: at most three, each said with what it compares to, and held on screen ≥ 2.5 s.

MOTION QUALITY
- **30 fps, on ones** (pass `--fps 30` to the render and to `mux.sh`): crowds and walks are judged frame by frame (§9), and the check assumes this rate.
- People move only when the rule says so, in order; the order is the point. Walk ≈ 1.5 m/s; nobody moves faster than about 3 m/s on screen, even in a time-lapse: when a process must be compressed, show it as places filling (people settling into their seats) rather than people zipping past.
- Everything eases; nothing pops. A beat = one floor phrase + one diagram mark (an outline traced, an arrow grown ahead of the walkers, a light run along a path) + one action.
- A rewind runs the same motion backwards, faster, with a counter running down.

CAMERA GRAMMAR
The angle never changes (iso is fixed); camera moves are translation and zoom. Every move is motivated by the action, and text and subject stay inside the frame at both ends of it. Transitions: a continuous move, a lid opening, a run back along the same path, or a cut on the beat at the same angle. No page turns, wipes, black or blank frames; a pull-out shows its next subject (a mark being drawn) within half a second. A lid is a reveal only while it is seen: it finishes turning, paper side up, inside the frame before it glides off away from the camera.

SOUND PALETTE
Soft upright piano and vibraphone; a vibraphone note per landing character (keywords louder, the accent doubled on glockenspiel); pizzicato and shaker when people walk. Repeated actions become rhythm: a stuck queue is broken pizzicato stabs, an orderly one is one chord per group. Quiet place beds (hall air, cabin air, distant engines) and a sustained low string root overlap every cut so no scene change drops out; functional sounds of the place (a home-made chime, wheels on carpet and on metal, bin lids, a tug). A rewind plays the last second of music backwards. With narration, the music ducks about 16 dB under the voice and lifts between lines; mix ≈ −16 LUFS before mux, true peak under −1.5 dBFS.

NATIVE MOVES
- **Floor type.** The key word lands on the ground beside the action. *Fits content like:* a hospital triage rule; a polling-station step; a library's quiet zone.
- **Domino stand-up.** The group called sits together and rises one by one from the near end into a painted lane. *Fits content like:* an exam hall emptying by row; a ferry deck released lane by lane; a classroom drill.
- **Lid off.** A roof flips open on its far hinge, paper side up, and glides away; the camera rises to see the whole inside. *Fits content like:* a bus at a depot; a lecture hall; a container being packed.
- **Order lighting.** Places light up one by one in the order they will be used, before anyone moves. *Fits content like:* seat assignments; a parking plan; a vaccination queue by number.
- **Order arrows.** A flat accent arrow grows ahead of the walkers and branches where each one turns. *Fits content like:* escalator lanes; recycling bins by type; a one-way museum route.
- **Weave fill.** Places fill in a pattern (every other, window first) so the room becomes a woven grid in four colours. *Fits content like:* a stadium by sections; a cinema with assigned seats; a car park by level.
- **Bars on the ground.** Two quantities painted as strips on the floor in proportion, the difference marked by a dashed bracket after a still hold. *Fits content like:* two queue times; two routes' lengths; before and after a change.
- **People as the sample.** The people of a study stand on the ground in the shape of the thing they tested. *Fits content like:* a 72-person boarding test; a class of 30 in their desk plan; a jury in their box.
- **Stand here.** An accent outline or a lit stretch of floor marks the spot that matters. *Fits content like:* where to wait for a lift; the jammed stretch of an aisle; the right queue at a counter.

PITFALLS OF THE MEDIUM
- Anything raised by H metres hides what lies behind it (toward −x behind a face that looks +x, toward −y behind a face that looks +y): floor up to H metres back from a long edge, and a person standing there loses everything below the edge's top, vanishing completely within H − 1.7 m. It holds for low objects too: a 0.8 m belt or counter covers floor type 0.8 m behind it, a 3.2 m truck side hides people's legs up to 3.2 m behind it and whole people within 1.5 m, a glass wall or a wing hides type metres away. Keep type and the people who matter on the camera side of tall objects, or cut those objects down; check every frame in which a vehicle passes.
- A path that zigzags (turning at every vehicle or doorway) reads as noise; routes run along the world axes with few right-angle turns, and the walker or vehicle on them heads away from the camera (see the back-view pitfall below).
- A crowd folded into a serpentine reads as a solid block; one straight line reads as order. Numbers on the floor beside each person need spacing wide enough for the label (about 1 m).
- People walking toward the camera show a back view that looks wrong; route walks up-left, up-right or across.
- Seated people disappear behind high seat backs: keep backs low enough that shoulders show.
- Sorting boxes by their centre and people by their feet puts furniture over people (see §2).
- Colours passed to the engine as a bare hex string are shaded by `engine.js`'s own `tri()`, which darkens toward violet; always pass `kit.js`'s `tri(colour)` (warm umber). `iso.sphere` adds a white glint unless given `{ hi: false }`.
- A simulated crowd sampled by rounding to frames steps on and off when replayed slowly; interpolate. Compressing a long process by moving people faster than ~6 m/s (0.2 m per frame) reads as teleporting.
- A camera move can drag a sign or a word across the frame edge, and background details (a far sign, a row of labels) can appear half-cut at the edge of another shot: anything meant to be read needs both ends of the move checked, and background text belongs only to its own shot.
- Check a crowd film frame by frame, not by eye: `demo/tools/motion_check.mjs` renders every frame at 30 fps and lists jumps, stop-start stepping and people standing closer than a body width, from positions the film reports through a one-line hook (documented in the tool); copying it is in §10.
```
