Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Assembly-Line Diagram" style.

MATERIALS & RENDERING
- **All vector, drawn in code**: SVG built per frame from shapes and paths. Clean ink outlines with round caps and joins, flat fills, no gradients.
- **Print, lightly.** Every colour plate is shifted by its own 1–1.6 px misregistration, the same direction across the frame (ink, paper and walls stay put), and a faint paper grain and fibre sit on top (multiply). Nothing more: no aging, no torn edges, no yellow stains, no mottled ink.
- **Flat shadows.** A Holmes shadow is the same shape offset down-right in one pale flat colour (plaques 5–6 px, large shapes 14–18 px). No soft shadows, no blur except motion blur on whip moves.
- **The cut-away.** The silhouette is a dark ground with a thick body wall (a band about 56 px thick with ink on both sides); where goods enter (an intake, a hatch, a door) the wall breaks and both ends get an ink cap. Rooms have a coloured frame, a pale wall and a floor strip; floors between rooms are slabs with rivets; steel trestles and gantries hold the machines.
- **Pipes**: ink edge (width + 6 px), colour, then a highlight stripe (0.2 × width, offset up-left, masked inside the pipe); rounded elbows; flanges every so often. Width encodes importance (main duct, arteries, branches).
- **One printed sheet at every distance.** Line weight follows the camera (thinner when pulling back, roughly `z^-0.6`; only slightly heavier when pushing in), and below a set scale small workers, plaques and small text are not drawn while the big machines stay. The drawing never turns into a hairline mess or a blob of text.

COLOUR LOGIC
- **Paper + ink + one ground + four spot colours.** Paper outside the silhouette; one dark ground inside the cut; pale walls for rooms; a body colour for walls and slabs; then three or four spot colours that are the legend. No other hues.
- **Spot colours have one meaning each for the whole film** (in a recycling plant, say: ochre = paper, vermilion = plastic, teal = glass). Room frames may rotate through the spot colours by station; the cargo's own colours stay fixed.
- **Ground dark enough to frame, never black**: a deep indigo or wine reads as "inside" and lets the spot colours sit on it. Check that every spot colour separates from the ground (cobalt on indigo does not).
- Numbered station circles are one colour (ochre fill, ink edge); a circle is an outline until its station is reached, then fills.
- Example (our demo, palette A "Night Shift"): paper #F2ECE0, ink #1D2030, ground #24345A, wall #FBF7EE, body #EE9F87, vermilion #E04A2F, ochre #F3B23A, teal #2E8D87. Two alternatives live in `demo/lib.js` (`PALS.B`: a daylight chart with a pale body-colour ground; `PALS.C`: a wine-red ground).

TYPE & SUBTITLES
- **Three faces, three jobs** (all OFL): a condensed industrial display face for plaques, legend tags and the title (Big Shoulders Display 800–900, like letters painted on a factory wall); a grotesque for small labels, subtitles and credits (Archivo 600–800); a CJK sans for the one short Chinese line on each sign (Noto Sans SC 700, at least 34 px).
- **Plaques**: cream board, 2.6 px ink frame, 3 px corners, flat shadow; `[number circle] [ENGLISH METAPHOR | 中文名]`, stuck on the room's top edge or on the machine, never on a leader line.
- **Legend tags** hang above their line: a colour chip, then the English and Chinese name (`GLASS 玻璃`).
- **Leaders** (Holmes): dot → vertical → right angle → bold words, only for a moving hero, at most one per frame.
- **At most about seven pieces of text in a frame**, two to four words each.
- **The title is an object in the picture** (a gantry sign over the infeed, a machine's nameplate) hung on the thing it names, never in a page margin, never centred, never a header. The end card is the same object: a nameplate standing on the ground line.
- **Subtitles are work tickets**: a cream ticket with an ink frame, a stub on its left showing the current station number (a clock when there is none), Chinese 34 px over English 28–30 px, one line each, bottom left; move them bottom right in shots where the title or the end card holds the left. Compose every shot with that corner free.

MOTION QUALITY
- **The belt never stops moving**; cargo travels at constant speed and eases into stations.
- **A station starts on arrival.** Its machine starts on the beat the cargo lands (rollers turn, sprays fall, a press stamps), its worker does a two-frame action, its number circle fills and pops (scale 0.9 → 1 with overshoot).
- **Signs drop on chains** and bounce once; a second-language sign swings in half a beat later.
- **Repetition is the joke.** A gatekeeper raises STOP, lets one portion through, raises STOP again; machines repeat their cycle on the music's grid.
- **Time compresses on clocks and dials**: hands spin, lamps dim, workers doze; never a title card saying "later".
- **Big figures move by procedure, grounded**: a running figure plants its support foot on the ground line and moves with the ground, knees solved from two bones and always bending forward; nothing slides, floats or crosses scissor-wise on a held frame.
- **Frame rate and grain**: 24 fps.

CAMERA GRAMMAR
Always a flat front elevation: the camera pans, tilts, pushes and pulls in the plane of the drawing, never rotates into perspective.
Transitions happen inside the drawing: passing through a shape, floor slabs wiping the frame, a match cut on the same piece of cargo, a colour follow, a circular porthole mask, a whip along a pipe (directional motion blur from the camera's speed plus speed lines, rendered once, never stacked ghosts). No page turns, no fades to paper, no white flashes.

SOUND PALETTE
- **Score as a production line**: a march on a fixed figure (around 120 BPM) that the stations add to; each line or machine brings in its own voice (bassoon, clarinet, flute…); woodblock and bass drum on presses and stamps; glockenspiel or celesta when a station lights up.
- **The stop is a sound**: when a gate closes or an alarm goes, the whole band stops dead and restarts on the next release.
- **Foley from machines** on every visible action: hoppers, chains, sign bounce, roller shutters, presses, sprays, drums, whistles, ladles, clocks, alarm bells, ignition, doors. Procedural, short, on the frame.
- **Voice** (optional): a calm explainer, one idea per line, saying what the picture cannot (the reversal, what a part becomes, why it waits); leave arrivals, the big lighting-up and the end card without words. Compress it and set it about 9–10 dB over the music, carve the music's 300 Hz–4 kHz band under it, let the music rise between lines.
- **Mix**: −14 LUFS, true peak ≤ −1 dB; limit the transients of presses and ignitions.

NATIVE MOVES
- **Station starts.** Cargo lands, the machine starts, the worker acts, the number fills. *Fits content like:* each step of a manufacturing process; each stage of a data pipeline; each department a request passes through.
- **Taking it apart into colours.** One lump enters a sorting floor and leaves as three streams of differently coloured parts, each into its own pipe, with legend tags dropping over them. *Fits content like:* a mail sack into postcodes; crude oil into fractions; revenue into costs; ore into metals.
- **Gatekeeper.** A worker with a STOP sign lets one portion through at a time. *Fits content like:* a valve, a rate limit, a queue, a customs desk, a dam.
- **Warehouse and clock.** Goods are shelved; the clock spins; the lights dim; workers doze until an alarm. *Fits content like:* energy storage, inventory, a battery, savings.
- **Porthole alarm.** A circular POV on the outside world shows the event that sets the line running. *Fits content like:* demand arriving, an order, a sensor trigger.
- **Follow the colour.** The camera picks one pipe by colour and rides it to its destination while the others turn away. *Fits content like:* where one product goes; one signal's route; one customer's journey.
- **Light the whole line.** The great pull-back to the full silhouette; a glow runs from intake to output and the numbers pop in order. *Fits content like:* the summary of any process; a product's whole life; an ending.
Our demo opens on an ordinary factory with the title sign dropping over the infeed, uses *taking it apart into colours* (entered on a match cut) as its turn and ends on *light the whole line*. A new film opens and ends another way: the outside event seen through the porthole first, a truck with one cargo already riding the belt, an ending that follows one colour to its output in the outside world, or the factory going dark station by station.

PITFALLS OF THE MEDIUM
- **Process diagram, not a story.** A station-by-station walk is a flowchart. Give it a hook (a reversal, a deadline, a question) and a payoff picture; let each station say one thing.
- **Text creep.** Plaques, tags, leaders and subtitles add up fast; keep to about seven pieces per frame and let colours carry meaning.
- **Hairlines and blobs at distance.** Without camera-dependent line weight and level of detail, the pull-back turns into a grey mesh of strokes and unreadable labels.
- **The silhouette disappears** when rooms fill the cut edge to edge: keep at least 30 px of dark ground between rooms and the outer wall.
- **Spot colours that vanish on the ground.** Test each against the dark ground and against the walls before committing.
- **Bodies that look like barrels.** When a figure has to hold a lot of rooms, its torso grows and its head shrinks; cross-fade to a schematic figure with human proportions for the full-body shot rather than inflating the cut-away.
- **Feet.** Procedural legs that swing on sine curves float, sink and slide; solve them against the ground line, and hold the final frame on a readable phase (one leg reaching, not crossed).
- **Metaphors that lie.** Every simplification the metaphor makes (one input → one output, a valve where there is really a gradient) goes into FACTS.md; the voice should not claim what the drawing only implies.
- **Leaders everywhere.** One leader per frame at most; everything else is a plaque on its object.
```
