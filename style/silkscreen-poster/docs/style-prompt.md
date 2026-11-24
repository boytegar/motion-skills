Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Silkscreen Travel Poster" style.

MATERIALS & RENDERING
- **One `Path2D` per ink layer**, filled in the current transform's units; knock-outs are holes in the path. Any shape can be printed.
- **Pull reveal**: a ragged squeegee front as a clip (a ~30 u drag tail); blade, bead and a sliver of handle ride on it.
- **Ink bead**: a round tube with lit half, dark underside, rolling glints and viscous strands; a split fountain puts several colours in one bead.
- **Screen**: a hinged frame with mesh, emulsion and tape; its shadow is a ring (even-odd), never a full rectangle.
- **Shapes**: ridges with a jagged lit facet, pines as one outline of drooping tiers, water as a flat body with glint bars. Hard edges everywhere.
- **Poster templates**: a portrait sheet (1000×1500 u: paper border, art area, a dark **info band** at the foot holding a name, a number, a rule and a row of stats; columns sized by content), or a **full-bleed print** (16:9, art to every edge, the name printed into the sky, a dark info panel such as a ticket stub printed last). In a full-bleed film the camera stays inside the print.
- **Texture budget**: fine paper tile, mottle multiply ~55 %, pinholes ~75 %, light encode grain. More reads as risograph.

COLOUR LOGIC
- **One palette per poster, ≤ 6 inks**, assigned by role: light sky, deep sky / bands, far, mid, near (darkest; also the info band), and **one bright accent** (sun, route, highlights, key number). An optional lit-face ink.
- **Values step from far (light) to near (dark)**; depth is value, never blur.
- **The accent is rare**: the sun or light source, the one route or product, the most important number.
- **Paper is a colour**: an unprinted area is the brightest thing on the sheet (water, snow, a beam of light).
- **A series = one template, a new palette per item** (time of day, season, colourway, neighbourhood). Re-inking the same geometry in another palette is a statement.
- **Text inks are chosen by contrast** against the ink underneath (WCAG), never by a fixed key; a night palette must not swallow a header.
- Examples (sky1 → near, accent): dawn `#F6CD98 #EE9A73 #9A7FA8 #3E7079 #1E3B3F #FBE6B4`; harbour noon `#E8F0F2 #7FB3C8 #D9C7A3 #2F6E8E #173247 #E8553D`; winter `#EEF1F4 #B8C9D9 #8B9BB0 #4A5D78 #1F2A3C #F2B134`.

TYPE & SUBTITLES
- **Condensed display** for names and values (e.g. Big Shoulders Display, League Gothic), auto-fit; a geometric sans for letter-spaced labels (e.g. Outfit 600).
- Headers are **printed into the sky**, tracked out; the info band is printed last.
- **The poster text is the subtitle.** No narration is needed; if there is a voice, subtitles are a printed strip (a band pull), never a floating caption.
- **Reading rule**: each text stays ≥ chars/12 + 1 s (min 1.5 s) after its **last** item lands, and ≥ max(1.8 s, speech + 0.6 s) with a voice. Compute it from the content, round up to the beat.
- On a wall shot, labels stay ≥ ~18 px and values ≥ ~30 px at 1080p; close-ups on a band keep the whole band in frame with ≥ 5 % side margins.

MOTION QUALITY
- **Pulls are linear and steady** (a squeegee does not ease). Accent inks start half a beat after the main ink of their layer.
- **Info items** print left to right on subdivisions of the beat. **There is no fade anywhere.**
- **Screen lift**: the frame hinges up (vertical squash + growing shadow) in ~0.4 s.
- **Palette re-ink**: a wide squeegee travels diagonally over the sheet in ~0.35 s; geometry identical on both sides of the blade, only the inks change. Show only the blade, bead and a sliver of handle.
- **Loose layers**: in a multi-plane move, planes parallax by depth with paper gaps and thickness shadows, then **snap into register** on one hit.

CAMERA GRAMMAR
A vocabulary, not a route. Opening and ending come from the topic.
Close-ups on a portrait sheet keep its edge in frame (a full-bleed print has no edge to show); a squeegee across a close-up is tilted with the handle hidden, so it doesn't split the frame. Layers change by squeegee, posters through foreground silhouettes, a diagonal sweep that prints the next poster behind the blade, or a table move. No dissolves, no bare cuts.

SOUND PALETTE
- **Acoustic, hand-played instruments**: strummed guitar, slide guitar, harmonica, upright bass, cajon or brushes, banjo, fiddle, accordion, harmonium, ukulele. Choose the family from the place: folk, surf, bossa or brass band all fit.
- **One pull, one hit**: each ink lands on a musical event (a strum, a hit, a chord change); accelerate by subdividing, not by changing tempo. A registration snap is a loud downbeat.
- **Foley by material**: squeegee on mesh (band-passed noise with a ~220–280 Hz mesh grain; faster = brighter), wet ink squelch, screen clack and hinge creak, paper stamps, ticks for dashed routes, paper hiss, a wood-and-metal clamp, drying-rack clips.
- **Ambience from inside the print**: a finished scene may "wake up" (surf, street, crowd), J-cut in and L-cut out. Room tone under the table.
- **Silence as a tool**: the music drops out for a beat before the key facts; what remains is one ambience, and the first sound after is a decisive gesture (a clamp, a long slide up).
- Music ducks a few dB under foley. −14 LUFS.

NATIVE MOVES
Each serves information order. A menu: use the ones your film needs.
- **One ink, one beat.** Facts print last, when they are read. *Fits content like:* museum hours; a ferry timetable; tasting notes.
- **Split fountain.** Several inks in one pull fill the frame with colour. *Fits content like:* a festival's night sky; a sunset beach bar; a rainbow of colourways.
- **Palette re-ink.** Same poster, another palette = time passing, a season, a variant. *Fits content like:* summer vs winter hours; three jacket colourways; day and night at a market.
- **Registration slip → snap.** Loose layers clamp into register with one hit, before the key facts. *Fits content like:* the date of a race; a launch price; a ticket sale opening.
- **Paper as a colour.** The unprinted sheet is the brightest element. *Fits content like:* a ski slope; a lighthouse beam; fresh snow on a roof.
- **The route as a printed layer.** A dashed accent line stamped segment by segment. *Fits content like:* a bus line; a marathon course; a delivery path.
- **The series wall.** Every print ends up hung together as the final lockup. *Fits content like:* a city's four districts; a band's tour dates; a product family.

PITFALLS OF THE MEDIUM
- Stroke edge effects draw internal lines on unioned paths → fills only (fill, then fill shifted inside a clip).
- A full-rectangle screen shadow muddies the print → ring shadow.
- Cool shadow facets next to water read as water → shadows in the near ink.
- Jagged ink beads read as paper edges → a round bead.
- Parallax alone reads as "the mountains are moving" → paper gaps and thickness while loose, snapped to zero with a sound.
- Silhouette wipes hold a dark blank → centre on the switch so full cover lasts ~1 frame.
- Very wide banners are slivers in 16:9 → about 4:1.
- Random coloured dots on the table read as confetti → a real workbench (planks, grain, muted stains, tape).
- Too many items at honest reading times overrun the length → print items side by side in the same pulls, or let secondary ones be read on the final wall.
```
