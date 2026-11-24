Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Art Deco" style.

MATERIALS & RENDERING
- **Ground**: warm lacquer black, never neutral grey-black; a night sky may drift toward a deep emerald near the horizon.
- **Gold is a banded metal gradient** (dark – light – hot – light – dark); animate its `sheen` offset for a light sweep.
- **The signature stroke** (`gline`): a dark engraved underline ~1.6 px wider than the line, the gold gradient line, and a hairline highlight on top. Optional glow and a parallel twin (`double`). Nothing gets a black cartoon outline.
- **Volumes**: hard edge + airbrush (`airbrush`): a directional gradient from shade to light inside the shape, one consistent light direction (upper left by default), a thin gold rim on the lit side.
- **Motifs** (sunburst, stepped arch / ziggurat, fish-scale, chevrons, fan, four-point sparkle, speed lines, clock face, dial) are structural (a background, a frame, a transition), not stickers.
- **Perspective is real**: true one-point (mode-7 floors) and two-point perspective through a pinhole camera, so dolly, tilt and pan change perspective instead of scaling flats.
- **Characters are deco too**: about 7 heads tall, tapered streamline bodies, faceted lacquer planes (Lempicka), almond or arc eyes, a one-stroke nose, airbrush + gold rim. Give each lead one or two silhouette marks readable at a quarter of frame height.

COLOUR LOGIC
- Three constants: **warm black ground, gold line and metal, ivory** for paper, gloves, light and type.
- **One saturated accent reserved for the protagonist** (or the one object the story follows). It is the only large saturated area, so the eye finds it in any wide shot. A cooler jewel tone may carry secondary surfaces, never competing in area.
- **The one-colour exception**: any single element can be drawn in its own colour through the same fill + keyline + glow (`color` option) while everything else stays gold.
- Always expand hex to six digits in code (three-digit hex breaks colour mixing).
- Examples of the triad (accent / jewel / metal): burgundy / emerald / gold; coral / sapphire / pale gold; jade / plum / rose gold. Our demo's actual values are in DEMO.md.

TYPE & SUBTITLES
- **Type (OFL)**: Limelight (titles), Poiret One (numbers, sign skeletons, plaques), Josefin Sans (subtitles, labels), Italiana (numerals).
- **Titles are architecture**: a stepped gold title bar over an arch or sunburst, optional badge and wings; drawn on in gold, then a sheen sweep.
- **Subtitle card** (`subtitleCard`): a lacquer bar (~88 % black) with stepped ends like the title bar, double gold keyline, Josefin Sans SemiBold ~40–42 px ivory, an optional small speaker icon per voice. It **unfolds from the centre** like every other title, so titles, subtitles and transitions are one system. Hold ≥ max(1.8 s, speech + 0.6 s).
- Numbers carry plot without narration: dials, clock faces, floor medallions, badges.

MOTION QUALITY
- **Characters step on twos (12 fps)**; camera, light, bulbs, gold draw-on and sheen run on ones (24 fps). A stepped camera reads as stutter.
- **Everything unfolds from the centre**: a point → a hairline → bars open symmetrically → text rises → wings and sparkle.
- **Draw-on follows the music**: a gold line grows at the speed of the phrase under it and lands on the hit.
- **Bulbs have an ignition overshoot** (~1.6× flash settling in ~0.18 s) and a relay "chunk"; once a sign is lit, a fast chase may run across it.
- **Mechanical easing**: doors, dials and hands move with firm ease-in-out and a small settle, like machinery; nothing floats.
- **Performance beats get anticipation** and escalation in threes (once, twice, mash).
- What never moves: the centre axis of a composition. Symmetry may rotate or open, but it never drifts off-centre.

CAMERA GRAMMAR
A vocabulary, not a route. Opening and ending come from the topic.
Framing: the centre axis is sacred; an asymmetric frame must resolve to it within the shot. At an emotional peak the protagonist fills ≥ ~1/3 of frame height (smaller, the face reads as a prop in the ornament). Transitions: splits, revolves, arch openings, shape matches; never a generic dissolve or slide.

SOUND PALETTE
- **Symphonic jazz of the 1920s–30s, not a 60s big band.** Options: stride or concert piano, clarinet (trill; a glissando by time-varying resampling of a long note), strings (tremolo, rushes, pizzicato), muted trumpet commentary, trombone and sax section, walking pizz bass, brushes or sticks, tubular bells, harp glissandi, banjo, xylophone runs, celesta.
- Rhythm families to choose from: foxtrot, Charleston, tango, a slow torch-song ballad, a fanfare in the manner of a newsreel.
- **Techniques (options)**: one hit per unit of counted light; accelerate by dividing the beat; plant a theme in fragments so its full statement is a payoff; low-pass a band that is physically far away and open it as we approach; move a period narrator through acoustic spaces (horn, radio, PA, live mic) to make distance audible.
- **Foley by material**: brass clicks, bronze clang + chain, iron treads, marble heels, silver trays, wooden doors + spring twang, paper flutter, knife-switch "chunk" + filament hum, relay ticks, elevator ding, champagne cork.
- **Silence as a tool**: before the most important sound, drop to one small diegetic sound (a clock tick, wind) and cut back in with that sound alone.
- **J/L-cuts** carry architecture: the next room's sound enters a bar before its door opens.
- Mix: duck music ~−9 dB under voice (less under a period announcer who is part of the texture), beds −6 dB, −14 LUFS, light film grain.

NATIVE MOVES
A menu: use the ones your story needs.
- **Counted light.** A bulb sign lights one letter per beat or per event, so the audience can count. *Fits content like:* the countdown to a product launch; the letters of a city name lighting as a train arrives; votes tallied into a winner.
- **Threshold doors.** Every scene change is a door split along the seam. *Fits content like:* the departments of a company; the rooms of a museum; stages of a career.
- **Machine kaleidoscope.** People or objects become a mirrored, rotating pattern that locks into a symbol. *Fits content like:* a team becoming a logo; gears of a supply chain; a choir becoming a clock.
- **Dials carry the plot.** A floor indicator, a clock or a gauge moves instead of narration. *Fits content like:* a funding thermometer; the altitude of a climb; a stock ticker at the opening bell.
- **The architectural cutaway.** Pull out until a building is a section and the protagonist a coloured dot with speed lines. *Fits content like:* data moving through a server stack; a letter through a post office; a skier down a resort.
- **Sheen sweep / draw-on.** Light crosses a gold object on a hit; a gold line engraves itself at the speed of a glissando. *Fits content like:* unveiling a trophy; a map route; a signature on a treaty.
- **The one colour.** One element keeps its own colour in a gold world. *Fits content like:* the one product among competitors; the patient among doctors; the new idea in an old firm.

PITFALLS OF THE MEDIUM
- Bulb signs from glyph skeletons break on curves (G, S, R): Zhang–Suen staircase pixels fool a neighbour-count junction test → use the **crossing number** and weld chain ends within ~14 px.
- A generic cartoon character looks pasted in → characters are deco (§2).
- Fish-scale reads as brickwork → lower half-circles row by row, each overlapping the previous row, plus an inner arc.
- Door leaves over a flat "interior" read as a blank frame → draw only the leaves, closing over the live shot.
- A gold point on pure black trips `blackdetect` → lift the background wedges and glow slightly.
- Period loudspeaker distortion is unintelligible under music → gentle drive; whisper-check every line on the final mix.
```
