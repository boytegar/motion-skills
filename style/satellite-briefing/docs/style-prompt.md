Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Satellite Briefing" style.

MATERIALS & RENDERING
- **Plates**: georeferenced rasters with a lon/lat box, sampled in a shader where the camera ray hits the ground, nested by resolution (world → region → detail) with feathered edges so finer plates dissolve in as the camera descends.
- **Globe**: ray–sphere from a perspective camera, a thin atmosphere rim, limb shading at orbit only; compute |C|² − R² in double precision (float32 fails below a few tens of km).
- **Time**: dated plates share one grid. A soft scan front (a satellite pass, never a hairline) swaps them: north → south forward, south → north for a rewind. Growth can run outward from the old edge, ordered by a distance map.
- **The analyst's grade** (one uniform from colour to grey): luminance with local contrast and clarity, a shoulder so salt, snow and cloud stay below white, toned blue-black → cool silver up to ~65 % grey, a fifth of the chroma. White belongs to the marks.
- **Marks**: a 2D overlay through the same world → screen function: hairlines 1.2–2.6 px with a dark halo, dashes for history and claims, brackets, ticks, leader lines. Lines draw on; outlines print behind the scan.
- **Screen space only for instruments** (altitude, scale bar, imagery credit) and the transcript strip. Everything else is anchored to the ground.
- **The claim layer** is a printed map drawn in the shader from a distance field of the old outline: paper grain a few pixels wide at any zoom, halftone ink, coast and waterlines, graticule, the serif name.
- **Post**: light grain in ffmpeg, a vignette, directional blur only on whip pans.

COLOUR LOGIC
- Neutral ground: grey, roughly 3–65 % in value. White means *measured*.
- One accent (warm, e.g. amber) for the single reading on screen, never decoration.
- A second hue may mark the **claim** the imagery tests (a map, a plan, a promise): flat, cartographic, never used for anything measured.
- True colour is evidence: at orbit, and again only where the story turns (the field that greened, the water that returned).
- No false-colour ramps over the imagery.
Example (our demo): ground #070B0E → #A1A7A9, marks #F2F5F6, reading #FFBE55, map blue rgb(46, 112, 206).

TYPE & SUBTITLES
- **Roles**: a light condensed grotesk for the reading (96–130 px, e.g. Barlow Condensed); a monospace for instruments and dates (20–34 px, e.g. IBM Plex Mono); a sans for labels in the film's language (24–42 px); a serif only for the claim layer and the title. OFL, self-hosted, CJK subset to the characters used.
- **Labels attach to the ground** with a leader line. The reading sits in the right third beside what it measures, label above, source note below ("Landsat, measured").
- **Subtitles are a transcript strip** at the bottom left: a 2 px white rule, a dark plate fading right, left-aligned, each line on one line (measure with the real font, `nowrap`; split a long sentence into two cues). If the film is bilingual, the film's language on top.
- Nothing below 20 px at 1080p, the imagery credit included.
- Hold subtitles ≥ max(1.8 s, speech + 0.6 s); a rolling counter reports its landing value; labels stay ≥ characters ÷ 4.5 + 1.5 s (CJK).

MOTION QUALITY
- The camera never stops: distance interpolates in log space, segments ease, holds keep a 2–5 % creep or a slow rotation. No frozen frame longer than a second.
- Marks arrive by being made: a line grows, an outline prints behind the scan, a counter rolls and lands with a small pop, a reticle snaps with overshoot.
- Time changes are scans, never dissolves; their spacing tells the speed, and the camera moves with them (accelerating as the steps shorten, braking on the last).
- Never moves: the ground under its annotations, the instrument positions.

CAMERA GRAMMAR
At key moments the subject fills a third of the frame height; reading right, subtitles bottom left, instruments in the corners. Transitions: scan-line swaps, a map layer switched off, zooming through plates, whip pans, the reticle opening into a title card. No crossfades between unrelated places, no hard cuts inside a zoom.

SOUND PALETTE
- **Telemetry**: FM pings (a lock-on ping worth bringing back), dry ticks on the grid that divide to accelerate, noise scans (rising for a rewind, falling forward), servo whines, a ratchet for a range line.
- **Weight**: timpani, bass drum, gran cassa, bowed cymbal, a tam-tam reversed when time runs backwards. Measurements land on a pitched hit.
- **Synthesis**: a pulse bass with audible 2nd and 3rd harmonics, detuned-saw pads under a low-pass, risers for dives.
- **Beds**: orbit hum, radio hiss, data chatter; the place's own ambience (wind, water, a far horn), sparingly, often as J- or L-cuts.
- **One option for the shape**: keep the palette dry (electronics, skins, metal) and let one wet timbre (glass, water, bowed metal) mark the turn.
- **Silence**: music and foley stop, a thin bed stays, impacts may decay into it; the first sound after it is the turn. A short suck-out (0.2–0.45 s, music and foley down ~25 dB) before the heaviest impacts makes them land.
- **Mix**: voice ~10 dB over music; the two or three designed impacts are the loudest moments (out of the bus compressor, with mid-range body, e.g. a low-brass stab); 20–60 Hz about 15 dB under the full band (high-pass ~40 Hz, impact weight at 60–120 Hz); −14 LUFS, true peak ≤ −1 dBTP. Voice: a calm analyst, short declarative lines.

NATIVE MOVES
- **The map against the photograph.** The camera falls into the printed map of the claim; a scanner bar tears it away and the ground is underneath; the name outlives the page. *Fits content like:* a lake still on maps but gone; a cleared concession forest; a planned city never built.
- **Time rings in a locked frame.** Dated plates scan in one by one; each leaves its outline, the band lost between two rings flashes. *Fits content like:* a glacier's retreat; a city's sprawl; a fire scar season after season.
- **The range line.** Mark the destination, grow a ticked line from A to B, chase the tip while the reading counts up, land on the number, pull back to show the whole distance. *Fits content like:* how far a shore moved; a village to its well; a camp to the border.
- **The dive and the climb.** One zoom from orbit to the ground, or back. *Fits content like:* locating anything; a scale reveal; an ending that returns one place to the planet.
- **Colour returns where the story turns.** Only the subject regains true colour inside the grey. *Fits content like:* an irrigated field; a refilled reservoir; a regrown forest.
- **The reticle becomes the frame**: the lock-on brackets open out to frame a title card or the last reading.

PITFALLS OF THE MEDIUM
- **Licences**: not usable: Google Earth/Maps, Bing, Esri World Imagery, Mapbox Satellite, Planet, Maxar, news photos, AI-generated imagery. EOX Sentinel-2 cloudless is CC BY 4.0 only for 2016 (2018 onward is NC). Raw Copernicus Sentinel data need "Contains modified Copernicus Sentinel data [year]"; check the user's rules. Public domain or free with credit: USGS Landsat (1972 on, worldwide), USGS NAIP (US, ~1 m), NASA Blue Marble, MODIS, VIIRS. Burn a readable credit into the picture and list every asset in `CREDITS`.
- **Finding imagery for a new place**: search by bounding box and date in the Planetary Computer STAC (Landsat C2, Sentinel-2 L2A; no account) or USGS EarthExplorer; MODIS/VIIRS and Blue Marble from NASA GIBS; coastlines, lakes and rivers from Natural Earth (borders and China: DIRECTOR.md §13). Log every request. OSM (ODbL), CC BY-SA and "no known restrictions" are not CC0: name the terms and ask the user first. Ask for the user's own surveys or GIS layers when the place is theirs. Never fill a gap with generated imagery presented as evidence.
- **Seams**: paths imaged on other days disagree. One date per path, each scene gain-matched on stable land, feathered overlaps, holes filled from the same year only.
- **Old sensors**: Landsat MSS (to 1992) has no blue or SWIR and drops scan lines: nearest-neighbour resampling, near-zero rows as nodata filled from same-year scenes, scan-angle streaks cleaned on land only. Inspect every year at full contrast.
- **Plumes and haze** cross water as bright diagonal bands: swap the scene for another date of the same year rather than painting it out.
- **Edges**: resampling against nodata darkens scene rims (erode the mask); drop contour pieces along a plate border. Real straight edges (dikes, polders) can read as seams: frame them out or label them.
- **Grey mush at close range**: raise local contrast for that altitude only and dim outside the measured corridor.
- **Water and cloud**: MNDWI where shortwave infrared exists, only the basin you measure; clouds read as features in grey, so use the dry season.
- **Facts**: keep a FACTS.md in the project folder (claim, source URL, quoted wording, or the measurement method with scene IDs and dates) and deliver it with the film. Measure on the date you show.
```
