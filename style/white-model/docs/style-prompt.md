Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Projection-Mapped Model" style.

MATERIALS & RENDERING
- **Model**: matte white (≈ #F3F3F1, roughness ≈ 0.9), crisp edges; water a darker satin inlay in the base plate; bridges low white slabs; a thin base plate on a dark satin plinth.
- **Light model**: house light = one soft key + a neutral studio environment + AO on close shots. Projection = a term added to the lit surface in the shader: `colour × (0.42 + 0.58 · max(N·toProjector, 0)) × throw mask`, scaled by albedo, reduced on water. Never a texture on the mesh.
- **Building data is the material.** First look for the place's own open 3D or footprint-with-height data: 3DBAG (Netherlands, CC BY 4.0), PLATEAU (Japan, CC BY 4.0-compatible), city open-data portals (footprints with heights or floors, zoning, plans). Fine: CC0, public domain, CC BY (credited in CREDITS and on screen). OpenStreetMap and Overture (ODbL), CC BY-SA and "no known restrictions" are not CC0: name the terms to the user and ask first. For their own site, ask the user for their data (plan, city model, survey heights).
- **Layers depend on what exists.** Construction years per building are a Dutch luxury (BAG); elsewhere they are often missing. Substitute what the data does hold and the medium can still project: heights or floor counts, footprint area, zoning or plan status, a boundary, sun and shadow (always computable). Never invent a year layer.
- **A layer is state, not a picture**: per-building attributes (year, height, rank, flags) live in vertex data; the shader decides each frame what colour the projector gives them.
- **The sun is a real directional light** with shadow maps, computed from the place's latitude and a real date and time (NOAA/Meeus solar position). The projector can trace what the sun does (a shadow footprint) but never fakes a shadow.
- **Post**: physical depth of field (stronger on close shots), bloom only above the projection highlights, vignette, light grain in ffmpeg. GTAO only on close shots: on wide grazing views it bands.
- **Screen space only for the 2D layer**: readouts, pins, measured lines and subtitles go through the camera's world → screen function so they stay pinned to the model.
- **Keep a FACTS.md in the project folder** (claim, source URL, quoted wording, or the measurement method) and deliver it with the film; every number on screen is computed by a script from the same data as the model.

COLOUR LOGIC
- Neutral ground: white model, dark room (near black), dark plinth. Projector black level is a cool near-black on the plate.
- **One hue family per layer**, chosen for the data: a single categorical accent for "the old/the exception" (e.g. warm amber), a sequential ramp for continuous data (blue → cyan → white for time, violet → magenta → peach for height).
- **A colour may return** only with the same meaning (the "before" colour reused later marks the same set of buildings).
- The sun is warm, its shade is a projected deep blue; data colours are put away when the sun arrives.
- Never colour the model in the house light; never use more than two layer palettes on screen at once.

TYPE & SUBTITLES
- **Roles**: a light grotesk for the figure (Inter Tight 200, 120–140 px), a sans for labels in the film's language (Noto Sans SC 300/500, 20–36 px), a monospace for instruments, years and coordinates (IBM Plex Mono, 18–22 px). OFL, self-hosted, CJK subset.
- **Readout in the left third**, one per layer: index + layer name, one figure, one label, one source line, a legend bar.
- **Pins**: a hairline from a roof or a ground point up to a tag on a dark plate (title 26 px + one sub line 20 px).
- **Identity label** top-left, quiet: place, scale, data sources.
- **Subtitles** centred at the bottom on a soft dark gradient, no box, measured with the real fonts; if the film is bilingual, the film's language on top, each language on a single line, split into two cues rather than wrap.
- Nothing below 18 px at 1080p. Hold subtitles ≥ max(1.8 s, speech + 0.6 s); labels ≥ characters ÷ 4.5 + 1.5 s (CJK), letters ÷ 15 + 1.5 s (Latin).

MOTION QUALITY
- The camera moves continuously and slowly; it changes speed with the layers, never stops dead.
- **Light is the actor.** Each layer arrives by a soft-edged gesture of light: a wash spreading from the projector axis, buildings switching on one by one with a short flash, a level climbing every facade, the sun's clock running. No hard line sweeping the table.
- House lights cut fast (a relay, 0.25 s) and return slowly (1 s); the projector strikes with a brief flicker.
- Counters roll while the light moves and land when it lands; measured lines grow from A to B with ticks.
- What never moves: the model, the plinth, the pins' anchors on the model.

CAMERA GRAMMAR
At key moments the subject fills a third of the frame height; readouts left third, subtitles bottom centre, pins never over the readout. Transitions are light changes on one take (lights out, a layer replacing another, the projector going dark), not cuts, fades or wipes.

SOUND PALETTE
- **The room**: room tone, the projector fan (hum + filtered air) rising when the lamp strikes, relays for the house lights, a lamp strike (zap + thump), the air of a light wash (sliding band-passed noise).
- **Light as sound**: mallets and glass for things switching on, bowed vibraphone for held light, a rising mallet line and air riser for a climbing level, one glass note per highlighted building.
- **Weight**: timpani, gran cassa, gong, low brass for a heavy fact; warm low brass for a turn.
- **Floor**: contrabass and cellos only as a floor under the chords; no generic piano and strings melody.
- **Silence**: the projector goes dark and everything stops but a thin fan hum before the turn; a short suck-out before the heaviest hit. The first sound after the silence is the turn.
- **Mix**: voice about 10 dB over the music; the two designed impacts are the loudest moments; −14 LUFS, true peak ≤ −1 dBTP. Voice: calm, short declarative lines, numbers spoken plainly.

NATIVE MOVES
- **Lights out, lamp on.** House light, a relay, darkness, the lamp strikes, the first layer arrives. *Fits content like:* a site before and after; a district before a plan is shown; any "now look closer".
- **The hole.** Light only the set that answers a question (built before a date, owned by one party, inside a zone) and let the absence draw the shape. *Fits content like:* a war-damaged centre; a flood plain with no housing; land a company owns; a protected zone.
- **Layers by time.** Buildings switch on in the decade they were built, each layer faster than the last, a year counter rolling. *Fits content like:* a new town's growth; a campus master plan by phase; a port's expansion; a developer's portfolio by year.
- **The rising level.** A soft level of light climbs every facade at once; low buildings finish first, the tallest last, the reading rides up with it. *Fits content like:* building heights; sea-level or flood depth; floor counts; a height limit being tested.
- **The sun clock.** A real sun at a real date sweeps the shadows; the projector then traces one shadow and measures it. *Fits content like:* a new tower's shadow on a park; winter light in a courtyard; solar panels' best roofs; a street's daylight hours.
- **The pick-out.** One building or set lights alone while the rest stay dark, with a pin. *Fits content like:* a listed building; the plot for sale; the hospital in an evacuation plan.
- **Lights up.** The data stops, the white model comes back. An ending or a reset.

PITFALLS OF THE MEDIUM
- **Wall normals** wound the wrong way show inside faces (walls look hollow): derive the outward normal from ring orientation. **Shared walls** between parts z-fight: skip the overlap below the lower neighbour.
- **Warm HDRIs turn the white model beige**: use a neutral studio environment.
- **GTAO on wide grazing views** produces wedge-shaped dark bands; keep it on close shots only.
- **A low sun lights facades, not roofs**: shoot from the sunlit side or side-on. **Narrow shadows vanish** among all the others: trace the one that matters (fill + edge) and measure it.
- **Hairline scan lines read as laser scanning**: give every projection front a wide soft edge or switch buildings individually.
- **Floating labels**: anchor pins on the tallest part of a building, not on its footprint centroid.
- **Numbers from mixed sources** (official height vs measured roof): say which on screen ("roof", "measured").
```
