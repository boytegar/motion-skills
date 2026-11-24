Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Pop Art" style.

MATERIALS & RENDERING
- **Three layers, one print pass** (`demo/engine/pop.js`): a plates canvas whose R, G, B channels hold the density of the red, yellow and blue plates (1 = solid, 0.05–0.95 = dots, 0 = paper), a black key canvas (alpha), and a spot canvas for the rare unscreened colour. A WebGL2 shader turns densities into round Ben-Day dots (dots up to 50 %, round holes above), multiplies the inks over paper and lays the key on top.
- **Opaque fills knock out what is beneath** (plates, key and spot), so painter's order holds; `over` adds ink to what is already printed (red dots over yellow = orange shading, red and blue dots growing over anything = the print darkening).
- **Dot screen in page space**, converted from screen pixels by the camera's inverse. Below about 2–4 px per cell on screen the dots fade into flat tint, so a wide shot never moirés.
- **Misregistration**: each colour plate is offset from the key by a few page units (clamped to a few pixels on screen), so a white sliver shows beside the outline. Per copy, offset the fill from its key line by a different small amount: repeated prints are never identical.
- **Key lines** are variable-width ribbons, heavier on the side away from the light (upper left), tapered at open ends; the line width compensates for local scale so small prints keep their outline.
- **Paper**: warm off-white with light fibre and ink unevenness; age and roughness stay low (clean print). Raise them only to date a print (an older edition: more age, wider misregistration).
- Grain in the mux at 1; nothing blurred.

COLOUR LOGIC
- Three process colours plus black and paper. Every other colour is an overprint in dots: orange = yellow + red dots, green = yellow + blue, purple = red + blue, dark = red and blue growing over everything.
- Primaries are used flat and big: one ground colour per poster, one subject colour, black for drawing, paper for breathing.
- Never put a colour on itself (red type on a red ground, a yellow subject on a yellow ground without a key line or a paper disc between).
- Darkness is never a black fill: it is dots of red and blue growing until the image drowns. Black is reserved for the key (lines, type outlines).
- Example (our demo): red #E4232B, yellow #FFD21F, blue #1E62B0, key #141414, paper #F8F2E2.

TYPE & SUBTITLES
- **Headlines**: a tall condensed grotesque (e.g. Anton, OFL), 100–260 px; colour fill, black outline, an offset shadow in another plate. They are printed in: colour lands from an offset, the key snaps on.
- **Labels**: a paper slip with a black frame and a plate-coloured offset block behind it, tilted ±2°, at a fixed corner of the frame. Headline line (condensed or a heavy grotesque such as Archivo Black, 90–130 px) over one short Chinese line (a heavy CJK sans such as Noto Sans SC Black, subset to the characters used, 46–56 px). Mixed Latin and Chinese lines use a font stack (Latin face first) so Latin never falls back to a serif.
- **Subtitles** are the same paper slip at the bottom centre: Chinese 40 px over English 28 px (Archivo Black), one line each, never wrapped; hold ≥ max(1.8 s, speech + 0.6 s).
- Labels enter with a fast print (0.5 s) where paper and words appear together, and leave by sliding out; nothing fades.
- Page-space type (a word printed on the sheet beside the subject) may be covered by the next copies or carried away by the camera: that is the medium's own exit.
- One headline and one label at a time. No page headers, no centred stacks of text, no bullet lists.
- Reading time: every label line ≥ characters ÷ 15 (Latin) + characters ÷ 4.5 (CJK) + 1.5 s, checked with `readcheck.mjs`.

MOTION QUALITY
- Prints arrive in one of two ways: **stamped** (a short scale from 1.1 to 1 with the colour plates sliding into register, on the beat) or **printed plate by plate** (yellow, red, blue, black, each landing with a thunk).
- Camera moves are long, eased, often logarithmic in zoom (a pull from dots to wall reads as one gesture).
- Copies accelerate as they multiply: the interval between stamps shrinks.
- Flips are card turns (scale X through zero), and the new print shows from the turn's midpoint.
- Spreading things (a rot, a colour, a signal) move as a front with a gradient of dot sizes behind it.
- Never moves: the dot screen relative to the paper; the paper itself.
- **Frame rate and grain**: 24 fps.

CAMERA GRAMMAR
Framing: the subject is a whole print or a whole wall; at key moments one print fills at least a third of the frame height. Labels live in the top-left corner, subtitles at the bottom centre; keep subjects out of both. Transitions come from the print (stamp, push through a shape, flip, plate-by-plate print, sliding label); no cross-dissolves, no fades to black, no page turns.

SOUND PALETTE
- The print shop is the foley: squeegee drags (band-passed noise), stamps (low thump plus a paper slap), paper whooshes for camera moves, a card flick for every flip, a single thunk per plate, a low granular crackle under anything that spreads.
- Music in a sixties register: vibraphone, marimba, upright or jazz bass, conga, shaker, hand claps, muted and staccato trumpet; exotica, lounge or surf colours rather than orchestral pads. Darker passages strip the groove to a bass pedal and bowed vibraphone.
- One-note-per-copy: a repeated visual event (each flip, each stamp) can carry a pitched note that climbs a scale.
- Silence: before a turn, drop to a held low note for under a second; the first sound after it is a flip or a stamp.
- Voice: a warm, slightly radio-announcer narrator; compressed, set about 10 dB over the music by RMS, the music giving way in 300 Hz–4 kHz. Master −14 LUFS, true peak ≤ −1 dB (limit transients from stamps and claps before the master).

NATIVE MOVES
- **Dot pull.** Start inside the screen (one shade area at zoom ×15–20, the dots as big as coins) and pull logarithmically to the whole print. *Fits content like:* the pixels of a famous photograph; a skin tone that turns out to be a face; a crowd that turns out to be one person.
- **Stamped copies.** One print stamped again and again in rings, faster and faster, until a wall. *Fits content like:* factory output; viral copies of a post; identical houses in a suburb.
- **Plate-by-plate print.** Hide the plates and bring them back one by one (`present({mask})`), sliding into register. *Fits content like:* a product launch; a new version; a portrait assembled from its parts.
- **Dots as data.** A front of growing dots crosses prints; dot size encodes amount. *Fits content like:* a drought across farmland; inflation across a shopping list; a rumour across a city.
- **Colour wave.** One tile flips to a new colourway, then a wave flips the whole wall. *Fits content like:* diversity; a team; a market opening up.
- **Through a letterform.** Cut a counter (the hole of a 0, an O, a P) out of the type sheet and push through it into the next print. *Fits content like:* a year; a score; a price.
- **Mosaic echo.** Pull back until each tile is one pixel of a bigger image whose colours were set by the tiles' grounds; draw the big key line along the tile edges. *Fits content like:* many faces making one face; many cities making a country; many small donations making one building.
Our demo opens on the *dot pull*, uses the *colour wave* as its turn and ends on the *mosaic echo*. A new film opens and ends another way: an empty sheet receiving its first stamp, a lateral track along a long print, an ending on a single tile left in a different colour on a wall of copies, or the last plate (the key) landing and the camera pulling out to the whole print.

PITFALLS OF THE MEDIUM
- **It turns into a comic.** Panels, balloons and sound words make it a comic strip; keep labels as printed slips and give the story to the voice.
- **Dots in screen space.** A screen-fixed dot pattern looks like a filter and swims under camera moves; always compute dots in page space.
- **Moiré in wide shots.** Fade dots to flat tint below a few pixels per cell.
- **Dirty paper.** Too much age, roughness or blotchy ink reads as a bad scan; keep it clean and use age only to date a print.
- **Same-colour collisions.** Red on red, yellow on yellow: put a key line or a paper disc between.
- **Blank print moments.** Printing plate by plate from an empty sheet leaves frames of bare paper; start with the first plate already down, and print a label's words together with its paper.
- **A key line that does not match its fill.** When an image is made of tiles, the outline must follow the tile edges; a smooth curve over square cells looks drawn wrong, not misregistered.
- **Too many words.** One label at a time, short Chinese lines, otherwise the reading-time rule cannot be met.
- **Hot transients.** Stamps and claps push the true peak over the limit; limit the mix before the master.
```
