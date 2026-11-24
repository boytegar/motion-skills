Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Firefly Pointillism" style.

MATERIALS & RENDERING
- **Splatting into five blur levels**: σ ≈ 1.0 / 2.1 / 4.8 / 10.5 / 23 px, rendered at 1 / 1 / ½ / ¼ / ⅛ resolution; each particle carries a fractional blur level and is split between the two neighbouring levels, so focus can be pulled continuously. Energy gain per level `[1, 1.55, 2.7, 5.2, 10] / res²`, so large bokeh is bigger and fainter at once.
- **Halo**: each point puts ~16 % of its energy two levels up, so bright points glow.
- **Exposure and tone**: linear light × exposure, `1 − exp(−x)`, gamma 2.2. Point cores burn towards white and the edges keep their hue.
- **Motion blur**: a 180° shutter; each point is drawn as samples between its positions at the two ends of the shutter, so fast flight turns into light streaks by itself. Points that wrap around the frame (rain, flows) must not draw a streak across the screen.
- **Grain**: very light, darker areas slightly more, plus 1–2 LSB dither against banding. Heavy grain multiplies the file size.
- **Characters**: a 2D bone rig (tapered capsules and polygons). Points are bound to normalised local coordinates on each part, so poses, scale and mirroring never resample. Outline points that fall inside another part on the same or a nearer layer are dimmed to a few percent (union silhouette); without this the figure reads as a wireframe.
- **Positions are functions of time**: every group has a state function per scene and transitions blend per particle with staggered delays. Any frame renders alone, in any order (parallel and reproducible). The particle count must be identical at every time.

COLOUR LOGIC
Linear colours times brightness; the screen shows a mid tone and a burnt core for each.
- One warm yellow-green family. **Each character owns one colour and keeps it through every transformation**: after a scatter and re-gathering, the audience recognises who is who by colour alone. Colours are never swapped.
- Cold colour only for stars and rain, under 5 % of the frame. No pure white, no saturated red or blue.

TYPE & SUBTITLES
- **Titles and credits are made of points**: the text is rendered to a mask and sampled on a jittered grid (not randomly), one particle per sample. A 140–170 px CJK title needs about 3,400 points, a 30–40 px Latin line about 800; sparser and the strokes stop reading. Once landed, the text holds absolutely still (sub-pixel shimmer only), with a short flash on landing. Place it in empty sky, left-aligned, never centred over a character, never on a plate.
- **Subtitles** are plain type burned in (legibility first), a soft brush-like CJK face such as LXGW WenKai (OFL): Chinese ≈ 40 px in warm cream, English ≈ 28 px a little dimmer, centred at the bottom, no outline, no plate, only a faint glow of the same colour. Keep every subject above the subtitle band.

MOTION QUALITY
- 60 fps, every frame (no stepping). Particle paths are always curves: straight flight gets a rotation or scatter added.
- **Breathing**: character points pulse slowly (± 22 %), and a few points are much brighter than the rest (pointillist sparkle). Fireflies are mostly dark: a narrow pulse, so only about a quarter are lit at any moment.
- **Drift instead of physics**: a sum of slow sines (0.3–1.4 rad/s) for floating points; easing replaces damping (ease-out to land, smootherstep elsewhere).
- **Performance**: big silhouette actions only (look up, reach, lower the head, jump, run, fall, kneel, stand, raise an arm). Blinks and fingers cannot be drawn. A large figure keeps its full silhouette while it acts: when it bows or leaps, move the head, neck and legs and keep the back line and every limb readable.
- **Three speeds**: fast (transitions, climaxes, points streaking), medium (performance, one pose change per beat), still (reading, emotional pauses: positions frozen ≥ 1.5 s, brightness breathing only).

CAMERA GRAMMAR
A 2D camera on the particle world, with parallax by depth layer (screen offset × (1 + 0.38 × depth)).
**Transitions happen only inside the medium**: a flow pours in, a swirl gathers, a gust blows the scene away and it re-forms elsewhere, lights go out, the world re-forms upward. No hard cuts, dissolves or page turns.

SOUND PALETTE
- Real sampled instruments (`core/audio/sampler.py`): harp (glissandi for flows, arpeggios for gathering, chords for landing), glockenspiel, hand chimes, glass harp and vibraphone for single lights, touches and flashes, string pads and tremolo, flute melody, timpani and suspended cymbal for forming and wind. Major with a Lydian ♯4 colour: bright and magical, not sad; a minor turn is available for loss.
- **Particle action ↔ sound**: gathering = rising arpeggio or glissando plus cymbal swell, landing exactly on the chord; scattering = wind and a falling glockenspiel run; a landing = a harp pluck or a hand chime; a synchronous flash = a fast harp sweep.
- Procedural ambience: crickets (short 4–5 kHz pulse trains), wind, rain; footsteps of animals from the gait phase.
- A light can carry a motif: a two-note rising figure for a "question", a falling figure to the tonic for an "answer".

NATIVE MOVES
- **The pour-in**: a stream of light enters from off-frame and lands, point by point, as the world and its characters.
- **The guiding light**: the single brightest point in the film (strong halo) leads the eye and can become a character's eye.
- **Synchronous flash**: a ripple delayed by distance (`1 + 2.6·exp(−((t − t0 − d/v)/0.08)²)`), or the whole field at once. *For*: an answer, a connection, "the world heard".
- **Swirl gathering**: points rotate around the new form's centre while they fly and focus. *For*: a summoning, a birth, many becoming one; the same move scales from a life-size animal to one taller than the frame.
- **Gust change of scene**: delay by screen x (one side leaves first), a wind vector, scatter; streaks come from motion blur. Wind direction is story direction (following wind = travel, head wind = storm).
- **Going out**: blown apart and not re-formed, brightness down to 1–2 %, embers left on the ground. *For*: loss.
- **Identity swap**: the same points change role without a break in the picture (a sun → a clock face, sparks → a flock of birds → the title).
- **Arc jump**: one rig's points fly from one pose and place to another (a leap across a gap, a dive); a facing flip can hide inside it.
- **Into a constellation**: a whole character leaps with its rig intact, shrinks, turns sharp and cool, and keeps walking in the sky as stars.
Our demo opens on *the pour-in*, uses *going out* followed by a *synchronous flash* across the horizon as its turn and ends *into a constellation*, the title gathered by an *identity swap*. A new film opens and ends another way: *the guiding light* alone in the dark, a focus pull from bokeh to a sharp form, an ending on *going out* with only embers left on the ground, or a *gust change of scene* into a locked wide shot.

PITFALLS OF THE MEDIUM
- **Green fog**: background bokeh, out-of-focus grass and fireflies add up to a haze and the black is lost. Keep fireflies mostly dark, background bokeh at 0.1–0.38, near blurred grass around 0.2.
- **Walls of light**: distant points on one horizontal line read as a lit wall; give them height above ground and random depth, and soften ripple fronts.
- **Hard distribution edges**: a uniform random band draws a visible line; use power-law vertical distributions.
- **Unreadable characters**: fill points alone are a cloud; outlines need about 0.7 points per pixel and occlusion culling. A standing animal seen from the side hides its far legs behind the near ones: stagger the feet so all four read.
- **Shapes dissolving at the peak**: a per-particle scatter on a big character turns it into a column of dots; for a hero action, move the rig and keep the shape.
- **Unreadable text**: random sampling or moving after landing; sample on a jittered grid and freeze.
- **Low-resolution levels**: energy splatted at ⅛ resolution must be divided by res², or the frame turns white.
- **Variable counts**: optional flashes must be fixed slots with zero brightness, or the two shutter evaluations differ in length.
- **Delays from in-flight positions**: compute a transition's delays from settled positions, not from the moving ones, or the points jitter.
- **Grain = bitrate**: per-frame random grain at 1 % makes the file ten times larger.
```
