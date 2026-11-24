Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Color-Coded Mechanism" style.

MATERIALS & RENDERING
- **Geometry is engineered, not sculpted**: involute gears with real module and tooth counts, extruded profiles (wheels, bridges, levers), cylinders for arbors. No bevels (a bevel doubles the outline). Choose module and camera so one tooth pitch is ≥ 12 px at 1080p.
- **Soft cel shading** (`cel()` in `demo/film.js`): lit faces are the flat colour, shade faces × 0.80 and slightly cool, a third tone for faces turned away. No environment map, no cast shadows between parts.
- **Outlines** (`demo/film-outline.js`): a second pass of view normals and depth; an edge on the near side of a depth step or on a crease is drawn in the part's own colour × 0.52 mixed with 28 % ink. 2× supersampling; lines 1–1.3 px.
- **Section cuts**: clipping planes through an axis; the back faces seen through the cut are shaded as the cut face, with 45° hatching computed where the view ray meets the cut plane, in world units (it never swims when the camera moves). Cut only the housing, never the part it reveals.
- **Dye**: colour arrives as a spherical front spreading from the point where energy enters a part (a mesh point, an arbor, an impulse pin), with a thin pale wet edge. Never a global crossfade.
- **One soft contact shadow** under the base plate (a blurred silhouette, 20 % opacity), drawn in the colour pass only.
- **Text and leaders** live in an HTML/SVG layer on top, anchored to projected 3D points every frame.

COLOUR LOGIC
- A warm off-white ground, flat, no gradient or vignette.
- One light neutral for the frame, plates, bridges, housings and every part not being discussed.
- Three system hues, mid-value and slightly greyed, always in the same roles: **warm yellow = energy source**, **grey-green = transmission**, **grey-violet = control or measurement**. In a different machine the roles carry over (battery / gearbox / governor; lens / shutter / meter).
- **One accent** (a saturated red-orange), only for the protagonist part at its moment, and for the control that drives it. Never two accents.
- **Focus mode**: only cause and effect keep colour, everything else falls to the neutral.
- Ink is near-black, for text, leaders, hands and markers only.
Example (our demo): ground `#F3F0EA`, neutral `#DDD8CF`, ink `#25282D`, energy `#E6B94F`, transmission `#79B09F`, control `#8A97D4`, accent `#E8472A`.

TYPE & SUBTITLES
- **Roles**: a neutral grotesk for titles and labels (e.g. Inter, 600–650 weight, tight tracking); a monospace in caps with wide tracking for figure lines, readings and units (e.g. IBM Plex Mono); a CJK sans for Chinese (e.g. Noto Sans SC). All OFL, self-hosted.
- **Three levels only.** Primary: a headline (60–64 px) or the label of the part being explained (name ~30 px ink + one 18 px line). Secondary: what was already explained, name ≥ 22 px mid grey, its line greys and lingers before it goes. Tertiary: the figure line (`FIG. n · …`, 13 px mono grey) and small data tags.
- **Labels grow out of the part**: a leader draws from a dot on the part, a colour dot in the system hue pops, the text slides in. Re-project every frame; when a label would cover a part, pin it to empty space and keep the leader; when the part leaves the frame, the label goes with it.
- **Comparisons** (ratios, before/after): the number is the primary (≥ 120 px), the two ends that produce it are secondary with leaders, all held in frame ≥ 2.5 s.
- **Subtitles**: bottom left, no plate, Chinese ≥ 34 px over English ~22 px, one line each; compose the subject away from that corner. Chinese on screen only in subtitles and short ≥ 34 px lines; labels are English. Hold ≥ max(1.8 s, speech + 0.6 s).
- **The credit** belongs in the style as a drawing's title block: ink rules, mono field names, values in the grotesk.

MOTION QUALITY
- 24 fps on ones, smooth camera; no stepping.
- **Motion is driven by one master clock**: a smooth speed curve (real time, a fraction of it, deep slow motion, stopped) whose integral moves everything. Speed changes never jump.
- Ratios are exact: every wheel's angle comes from the driver through tooth counts; meshing stays meshed (a free-run must end on a whole number of teeth).
- Stepped motion (an escapement, a ratchet, an indexing table): the stepped part advances inside a short impulse window; the oscillator is a sine through its centre at each beat.
- Things appear by dye, cut, lift or draw-on; never by a fade-in of a whole object.
- What never moves: the ground, the palette roles, the light direction.
- **Frame rate and grain**: no film grain (mux grain 0).

CAMERA GRAMMAR
Framing: the subject fills ≥ 60 % of the frame width in explanatory shots; text sits in the clear third, never on parts; nothing important in the subtitle corner. Transitions are made inside the object: a cut opening, a lid lifting, layers exploding or landing, a match cut on a mechanical event. No dissolves, page turns or slides.

SOUND PALETTE
- **Foley from the mechanism**: metal ticks built from three micro-clicks (unlock, impulse, drop) panned to the part; a thin metallic swish for a section cut; brass clacks for parts landing; a tooth-rate gear buzz that follows the speed curve; air for lifts.
- **Instruments**: plucked and struck pitched percussion over a light bass (harp, marimba, glockenspiel, vibraphone, cello pizzicato); tremolo strings only for tension. A part taking its colour may sound one note, panned to it.
- **Grid**: the mechanism's real beat sits on the music grid (e.g. a press at 120 strokes a minute = quarter notes at 120 bpm); slowed time keeps the tick on the grid.
- **Silence** before the key mechanical event; the first sound after it is that event.
- **Voice**: an even, measured narrator. Music ducks under it and gives way in 300 Hz–4 kHz; voice ~10 dB over music by RMS; −14 LUFS, true peak ≤ −1 dB.

NATIVE MOVES
- **Dye by system.** A grey machine takes its colours in the order energy flows. *Fits content like:* the power path in a drone; the signal chain in a radio; the blood route in a pump.
- **Section with hatching.** A wedge or plane cuts the housing; the cut faces hatch in world space. *Fits content like:* a valve body; a camera lens barrel; a lock cylinder.
- **Explode by system, remove one layer.** Layers lift apart; pulling one out shows what it was doing (a motor over-revs without its governor). *Fits content like:* a brake without its caliper; a router without its clock; a dam without its gate.
- **Follow the force.** A low tracking shot from source to sink while the dye travels with it. *Fits content like:* a bicycle drivetrain; a heat path in a laptop; water through a turbine.
- **True slow motion of one contact.** Speed curve down to deep slow motion into a single interaction, silence, then its sound. *Fits content like:* a shutter curtain; a ratchet pawl; a relay closing.
- **Parameter drag.** A control in the accent changes one input; readouts and a small curve recompute from the model (style frame 3 in `demo/`). *Fits content like:* aperture vs depth of field; gear ratio vs speed; tension vs amplitude.
- **Whole object, still cut open.** The housing returns with the cut kept, the coloured systems running inside. *Fits content like:* an engine in its car; a heart in its chest; a server in its rack.
Our demo opens on a fast pull-back from a detail to the whole object, uses *explode by system, remove one layer* as its turn and *true slow motion of one contact* as its climax, and ends on the *whole object, still cut open*. A new film opens and ends another way: the machine already exploded, assembling itself; the outside of a product with a *section* plane sweeping in; an ending on a *parameter drag* left at its best value with the verdict held; or the camera rising to the whole system the machine is part of.

PITFALLS OF THE MEDIUM
- **Outline flicker on fine teeth** → keep tooth pitch ≥ 12 px, render at 2×, no bevels, do not spin fine teeth fast (≤ 45° per frame for a part with 4-fold symmetry; slow the time instead).
- **Wagon-wheel aliasing** of fast oscillators → show them slowed and say so.
- **Rainbow parts** → colour by system, never by part.
- **Plastic or metal look** → no reflections, no transparency, no specular.
- **Labels everywhere at equal weight** → one primary; the rest secondary or gone.
- **Labels on parts or under subtitles** → pin to clear space with a leader; keep the subtitle corner empty.
- **A housing that moves through the parts** (a case lifting through the movement) → move each piece the way it would come apart (dial up, case down).
- **Meshing that drifts after a free run** → make the extra rotation a whole number of teeth.
- **Hatching in screen space** swims with the camera → compute it on the cut plane in world units.
```
