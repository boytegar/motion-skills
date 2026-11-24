Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Transit Map" style.

MATERIALS & RENDERING
**The unit is the grid step, G.** Every coordinate and size on the sheet is given in G. Named stations sit on a 0.1 G lattice and at least 0.7 G apart along a line; a city network spans about 30 × 30 G and the sheet adds margins for the cartouche and any printed panel (about 45 × 47 G in our demo). The camera's zoom is read in screen pixels per G at the focus point (1080p).
- **Interchange shape follows the station.** Points side by side make a horizontal stadium, stacked points a vertical one; lines that cross at one point make a circle r 0.13 with the ink outline, which reads as an interchange next to the 0.085 stop because of its black edge. A capsule never rotates to 45°; a diagonal spread becomes a rounded square.
- **Two passes.** A 2D canvas inks only the region the camera sees into a texture (about 1.35× the screen density at the focus, capped at 4096², mipmapped, anisotropic filtering), limited in depth to a few camera distances so low views don't starve; a WebGL pass intersects each pixel's ray with the sheet plane, prints the ink onto procedural paper and computes depth of field from the plane's known depth.
- **Paper**: warm off-white, procedural and tileable, sampled in world coordinates so it never slides. Three layers: a cloud of value at about 9 G period with detail down to 0.04 G, a tooth at about 2.4 G period, and short fibres 0.02–0.1 G long. Paper value varies by no more than ±3 % (fibres darken up to 3 % more); ink is multiplied by the tooth so flat colour varies 7–10 % and is never flat. Stronger than this and the sheet reads as plaster.
- **Light**: one raking light from one corner of the sheet (north-west in our demo), about 33° above the paper, neutral in colour and fixed for the whole film; it lifts that corner about 7 % above the opposite one. A specular sheen (up to +10 %, narrow lobe) appears only when the camera tilts more than about 30° from straight down and is full at 65°. Warmth comes from the paper and the table, never from a tinted light.
- **The sheet edge** sits on a near-black warm table (about #1B1918) with a contact shadow that fades over about 0.2 G. Far away the table goes into a warm dark haze so the horizon never aliases.
- **Grade**: a light vignette (up to −24 % in the corners), a 1.02 gamma, a light film grain at the master; no colour cast, no bloom except the pen-tip halo.
- **Screen space only for the instrument** (the counter) and objects near the lens (a station plate standing on the sheet, a tunnel before the map). Everything else is anchored to the sheet.

COLOUR LOGIC
- Neutral ground: warm off-white paper; ink near-black for text and interchange outlines; secondary text in warm grey; water one pale blue with a darker edge, its labels in a mid blue.
- **Line colours carry identity and nothing else.** A real network uses its published colours; an invented one picks hues spaced around the wheel at similar weight, never two neighbours alike.
- **Mode is carried by line form, never by hue.** Each line keeps its colour whatever kind of vehicle runs on it:
  Dashes mean "below": a ferry is never dashed, a tunnel always is. The ferry's paper lane is what tells it from a tram at any distance (10 px of paper on blue in a wide shot); a tram never runs on water without a bridge, which is drawn as ordinary track.
- **Paper inside a line is never painted.** The paper of a bus core or a ferry lane is erased out of the ink layer (canvas `destination-out`), so the procedural paper of the print pass shows through with its own tooth and light; a paper-coloured stroke never matches it. Erasing removes every ink beneath, so order the ink: water → surface lines (bus, ferry) → rail lines → stops and capsules → labels. A bus core over water then shows paper (correct for a printed map), and a rail line crossing a bus line stays whole on top.
- **Red (or the oldest line's colour) is the story's accent**: the first section, the "you are here" dot, the stamp of a verdict. When the accent sits on a line of the same colour, the dot is r 0.17 G (more than twice the line's width) with a 0.04 G white outline and a pulsing ring, so it never melts into the line. A second, dark neutral (navy) stands for anything compared against.
- Light colours (yellow, lilac, aqua, light green, sand) carry ink inside bullets; deep colours carry white.
- A paper wash (veil, 60 % paper over the sheet) fades the network back when one part must lead; stations under it keep about 20 %; never a dark overlay.
Example (our demo): paper #F2EEE5, ink #1B1C1E, secondary text #4A4740, river #BCD6EA / #8FB5D6, river labels #5F88AD, accent #E3002B, comparison navy #24346E.

TYPE & SUBTITLES
- **Faces**: a neutral grotesk for Latin and figures (Inter in the demo) and a CJK sans of matching weight (Noto Sans SC, subset to the characters used). OFL, self-hosted. Native language above, second language below, one line each, never wrapped.
- **Sizes on the sheet**, in G (native / second language): hub label 0.30 / 0.17 bold; interchange 0.23 / 0.14 semibold; named stop 0.20 / 0.14 semibold; the second line in warm grey, 0.03 G below. Callout: date figures 0.58 heavy, native line 0.34, second line 0.24. Printed panel titles 0.58–0.60 / 0.36–0.38. Cartouche title 0.64 / 0.30, its note 0.24 in grey. Water names 0.22 / 0.17, set along the band.
- **Labels**: a paper-coloured halo (0.05 G stroke at 92 %) under every label. Placement is automatic: eight candidates around the station's edge (in order of preference: east, west, north-east and south-east, north and south, north-west and south-west), each scored against every line of the **finished** network and against labels already placed, hubs and interchanges first. It runs once, before the first frame, so a label never jumps while lines are drawn and lines that arrive later never cross it. A hand preference may override the score for a few stations.
- **Levels of detail by zoom**: minor stops fade in between 55 and 85 px per G, interchange names between 62 and 90, named-stop names between 100 and 130; hubs keep their names at every scale and grow when pulled out (× (85 / zoom)^0.6, at most 1.75), anchored to the station edge so they grow away from it without changing side. Which names show may also be chosen by shot, never flashed by time. While the camera rides a tip, names leave the frame before they can be read: show them in the held shots that follow, not during the ride.
- **Legibility is measured where the text lands**: font size on the sheet × the zoom at that point, foreshortened by the tilt. Text the viewer must read keeps the native line ≥ 20 px at 1080p and the second line ≥ 12 px (≥ 16 px on the counter); a two-line label counts only when its block is ≥ 26 px tall on screen. Below that, hide it by level of detail or move the camera.
- **Callouts** are printed on the paper: a bar 0.10 G wide in the accent (or the line being drawn), then the date in heavy figures, one native line, one second-language line; they wipe on from the bar in about 0.5 s. When a callout must sit over lines it gets a paper-coloured block with a soft shadow, no border.
- **Cartouche**: in one corner of the sheet, an ink bar, the map's name in both languages and a one-line note in grey. It is furniture: the camera finds it, it is not a title card.
- **The film's title is printed on the map.** At the start the title appears in the map's world: the cartouche (the map's name is the film's title), a line bullet, a station name plate by the lens, or a printed callout. Never a card or caption floating over the picture. It is bilingual like every printed text (native line above, second line below, one line each) and is held for its reading time where it lands (sizes and legibility above).
- **The film closes on the whole network.** The last shot shows the finished network entire, in top view or a gentle tilt, every line drawn; the end title and any credit are printed on the sheet (the cartouche, or a block in a margin, in the map's own type and ink), never laid over it. A detail the story ends on (a terminus, a first section) comes before this view, not instead of it.
- **The counter**: a dark enamel plate in the top-left corner (about 660 px wide, 86 px per row, rounded 14 px, four screws), rows with the native label above (24 px), the second label below (17 px), cream rolling drums (38 × 60 px, heavy figures), units at the right. Drums land only on sourced values; in between they spin blurred so no invented figure can be read.
- Reading time: hold every line ≥ (CJK characters ÷ 4.5 + other characters ÷ 15) + 1.5 s.

MOTION QUALITY
- 30 fps. Pen tips move on monotone keyframes; a tip may slow on a station and accelerate between them, never stop dead mid-segment.
- Stops pop from 0.6× with a small overshoot in about 0.28 s and a ring in the line colour that spreads from 0.10 to 0.42 G and fades in 0.6 s. Minor stops pop the same way at a smaller size.
- Interchanges scale down from 2× in 0.22 s with an ink ring spreading to 0.64 G. A major interchange falls from above in about 0.4 s (from 3.4× its size, its shadow offset and blurred, sharpening as it lands), then rings in ink and in the colours of the lines that met spread to 1.4–2.6 G in 0.9 s.
- Line bullets spring in with a back ease and carry the opening year; they fade when the story moves past their chapter.
- Vehicles (§2): white capsules at a constant speed, one per 4 G of track in each direction, only on finished track; boats slower than trains and buses.
- A line that starts from a station already on the map makes it an interchange the moment its tip leaves; start that tip at least 0.5 s after the first line has reached the station, so the capsule's fall reads as an arrival.
- What never moves: the geometry of anything already built, label positions, the paper and its light.
- A still of the style shows the medium in motion: at least one pen tip with its halo or trains on finished track, the counter landed, and a tilted view with depth of field.

CAMERA GRAMMAR
A pinhole camera with a 32° vertical field of view (about a 42 mm lens on full frame), aimed at a point on the sheet. **Tilt** is the angle from straight down (0° = top view); **heading** turns the view about the vertical; there is never any roll. Zoom is in px per G at the aim point; focus is always on the aim point. Depth of field is analytic: blur radius = aperture × |depth − focus depth| ÷ depth, in pixels at 1080p, capped at 26 px, so the far sheet blurs up to the aperture value and the near edge more.
When riding a tip, keep tip speed × zoom at or under about 500 px/s on screen (a long, fast line means a wider zoom or a slower tip). The camera is never lower than about 25° above the paper (tilt ≤ 65°): lower, and the sheet stops reading as a map. Top views carry no blur; any tilt above 10° carries some.
Framing: the subject sits off-centre with room ahead of a moving tip; the counter owns the top-left corner and nothing important goes under it. Transitions: match cuts on round shapes (a headlight, a lamp, a coin becomes a station dot), glides across the sheet, hard cuts to a new panel. No page turns, no fades to black mid-film.

SOUND PALETTE
- **Tuned percussion** for drawing and stops: glockenspiel, marimba, vibraphone; a stop is a pluck, an interchange a pop with a low thump.
- **A two-note station chime** as the motif, returning at the turns of the story.
- **Layering by line**: each new line adds one voice to a running ostinato (flute, spiccato strings, xylophone, horn staccato, harp, trumpet, oboe…), so the arrangement grows with the network. Accelerate by adding voices, not by changing tempo; a groove written in film time never stretches when picture time is slowed.
- **Matter**: a felt-tip glide under every drawing tip; rail clacks in pairs for trains; ratchet ticks for the counter; a filtered, submerged mix while a line runs under water; a descending whistle and a low hit for a capsule that falls.
- **Silence**: one beat of near-silence before a verdict; the end thins to a pad and the chime.
- Mix: music under the effects, sub-bass shelved (< 50 Hz well below the rest), true peak ≤ −3 dBTP measured oversampled (ebur128 with peak=true; a sample-peak limiter misses the inter-sample peaks of short clicks and plucks, so soft-clip those first), about −16 LUFS before the master's −14.

NATIVE MOVES
- **The pen tip.** A glowing tip draws a line at the camera's pace; stops light behind it. *Fits content like:* a product roadmap; a migration route; a career, job by job.
- **The interchange that falls.** When two lines meet for the first time (a second line arriving, or leaving from a station already drawn) the capsule drops onto the paper with its shadow, then shock rings in both colours. *Fits content like:* two teams merging; a treaty; two rivers joining.
- **The tunnel under the water.** Inside the water band a line turns dashed with a casing; the camera drops low to watch it cross. *Fits content like:* a cable under the sea; a deadline crossed; going public.
- **Time lines.** Two networks drawn as two lines over one year axis, stops at sourced milestones, brackets for the spans. *Fits content like:* two companies' growth; two countries' rollouts; a project against its plan.
- **Levels of detail.** Pulling back, minor stops and their names fade first, interchange names next, hubs stay. *Fits content like:* zooming from a street to a city; from a team to a company; from a gene to a genome map.
- **You are here.** The network fades under a paper veil, one segment stays lit, a red dot pulses. *Fits content like:* an origin story; a recap; a call to visit.
- **The odometer.** Rolling drums that land only on true values: a year, a running total, a time of day, a count. Each landed value must hold for its reading time (about 1.8 s for five characters), so plan about one landing per 2 s of film; more values than that go into printed callouts. *Fits content like:* kilometres, users, stores, sensors.
- **The plate by the lens.** A station name plate stands on the sheet near the camera, out of focus, as the low sweep passes it. *Fits content like:* a place that matters; a stop someone uses every day; a product's home.

PITFALLS OF THE MEDIUM
- **Off-grid geometry**: any segment not at 0/45/90° breaks the style. Route between points with an automatic bend (straight then diagonal, or the reverse) and validate every segment.
- **Labels over lines, or labels that jump**: give labels a paper halo and place them once against the finished network, before the first frame. Placing against what is drawn so far makes labels hop as lines arrive.
- **Parallel lines sharing a corridor** collide: offset each line by ±0.10 G and let one capsule enclose all points of the station.
- **Too much text in wide shots**: hide minor names by zoom, not by time.
- **A flat map**: a top view all the way through reads as a slideshow. Tilt, focus and the sheet edge are what make it an object; a still of the style should show at least one tilted view with depth of field.
- **Paper too loud**: tooth and fibres stronger than a few percent turn the sheet into plaster or canvas.
- **Mode by colour**: a ferry or bus line in a "ferry blue" breaks colour as identity. Use the line forms in §3.
- **Real networks**: published colours and station names are facts, logos and branded bullets are not; the film is an unofficial tribute and says so on screen. Check every date and length against official sources; encyclopaedia figures for opening sections are often for a different span, and stations are renamed (the data rule is in §11).
- **Counters that interpolate** invent numbers. Land only on sourced values; spin between them.
- **Texture resolution in tilted views**: limit the far distance of the inked region and let depth of field hide the softness.
- **Stale ink**: when the inked texture is reused, reset the 2D transform before clearing it, or last frame's ink ghosts every other frame.
```
