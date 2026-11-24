Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Silhouette Rhythm" style.

MATERIALS & RENDERING
- **Three 2D layers composited in one fragment shader**: silhouettes, their reflections, water light (ripples and droplets). The shader paints the sky, the sun and the water, then lays the layers over it.
- **Sky**: two vertical colour ramps (gold-to-crimson and magenta-to-violet) blended by one story-phase value; perspective cloud streaks lit on their sun-facing edge; a sun disc with a warm glow and a horizontal flare band.
- **Water**: the sky reflected with wobble from two noise octaves anchored to the world, a glitter column under the sun, darker towards the camera. Reflections of figures are the same shapes flipped about the waterline, wobbled and darkened.
- **Light around the figures**: volumetric rays marched from each pixel toward the sun (56 steps) and occluded by the silhouettes; a thin rim on the sun-facing edge and a soft halo, stronger near the sun.
- **Ripples** have a bright crest (in the water-light layer, tinted by the sky) and a dark trough (in the reflection layer), so they read on bright and dark water alike. **Droplets** are bright beads, never black specks.
- **Camera**: a level pinhole over flat water (focal length, horizon height, camera height, lateral and depth offset, yaw). Long focal lengths make the sun huge; the horizon height frames the shot.
- **Screen space only for** the end card and a true directional motion blur on whip pans (many samples along the motion, at full resolution).
- Light film grain at the master; nothing else in post.

COLOUR LOGIC
- Two values carry the frame: black figure and bright ground. Everything else is the sky's gradient.
- The figure colour is fixed: near-black with a trace of violet. It never takes a hue.
- The sky moves through warm hues only (gold, orange, coral, crimson, magenta, violet). No green, no blue sky, no grey.
- Brightest values belong to the sun, the horizon band and the light on the water; type uses the sun's cream, never pure white.
- Colour change is time: plan the story's moments against the sinking sun (high, touching the horizon, half, a sliver).
Example (our demo): figure #090307, sun #FFF6DC, gold #FFB347, coral #FC4D2B, magenta #E0245C, violet #380A52 → #1A083D.

TYPE & SUBTITLES
- **Roles**: a heavy condensed sans for the title (120–150 px, e.g. Anton), a condensed sans for small lines (30–40 px, e.g. Barlow Condensed), a bold CJK sans for Chinese (≥ 34 px, e.g. Noto Sans SC). OFL, self-hosted, subset to the characters used.
- Cream on the dark lower water, left-aligned in the lower third, never inside the sun, never centred, no shadow or stroke.
- **Type lands on the beat**: one word per beat, each landing with a small ripple on the water under it. One word may land late or early on purpose.
- Subtitles, when the film speaks: one line per language on the water at the bottom left, the same cream, clear of the figures; hold ≥ max(1.8 s, speech + 0.6 s).

MOTION QUALITY
- 30 fps, everything on ones. Poses snap into place in ~0.13 s with a small overshoot (ease-out-back) and hold until the next beat.
- Feet stay planted in the world while walking or running (two-bone IK); never slide a standing figure.
- Wings, steps and head turns are scheduled on the beat grid: the top of a wingbeat, a footfall, a head flag on the beat; double-time subdivision for running.
- The freeze stops subject time and the shader's noise time together; only the sky phase and the sun height continue. A camera may move inside a frozen moment.
- Never moves: the horizon line within a shot (no dutch tilt), the sun's place in the world.

CAMERA GRAMMAR
Framing rules: at the key moments the subject fills ≥ 1/3 of the frame height; a wide shot keeps the subject on light (in the glitter column or against the horizon band). Transitions belong to the medium: a near silhouette walking across the lens (a black wipe), a match cut on the same pose, a whip pan, a cut straight into a freeze. No dissolves, no page turns.

SOUND PALETTE
- **Instruments**: tuned wood and metal percussion (marimba, kalimba, vibraphone, glockenspiel), plucked bass, a dry kit, claps and shakers; strings only for held chords under a freeze.
- **Character sounds are rhythm**: each kind of figure owns one percussive voice (a low log drum, a bright woodblock…) that lands where its gesture lands. Being on or off the beat is audible.
- **Foley**: water in all sizes (step, patter, splash, plop, drip), wing pushes, distant calls, wind. Footfalls pan with position.
- **Silence as a tool**: cut the music dead (a record stop, a half-beat drop) before the turn and before the climax; keep only water and wind. The first sound after the silence is the most important one.
- **The freeze**: one hit (kick, cymbal, wings) and then only the reverb tail and a held chord.
- **A technique**: build back a lost beat one instrument per step; divide the beat for acceleration instead of changing tempo.
- Mix: −14 LUFS, true peak ≤ −1 dB, leave 300 Hz–4 kHz for a voice if there is one.

NATIVE MOVES
- **The beat in unison.** A row or a flock changes pose on the same beat; one member doesn't. *Fits content like:* a team where one player finds their timing; a choir; a production line with one machine out of step.
- **The cascade.** The same action passes along a line, one figure per beat or half beat (take-off, a jump, a bow). *Fits content like:* a relay; a domino of decisions; a crowd standing up for an anthem.
- **The beat written on the water.** Footfalls leave ripples evenly spaced in time and space; they light up one per beat. *Fits content like:* a trail to follow; a training plan; a song's rhythm shown as a path.
- **The freeze with the sky still moving.** Everything stops on the hit, droplets in the air, while the colour keeps sinking; the camera may orbit. *Fits content like:* a record jump; a goal; a wedding kiss; the last note of a show.
- **The comic freeze.** The same device on a fall or a mistake, with a record stop. *Fits content like:* a blooper; a failed first attempt; a punchline.
- **The line across the sun.** A row of figures crosses the disc, doubled by its reflection. *Fits content like:* a caravan; a migration; a team walking out.
- **The black wipe.** A near figure crosses the lens and becomes the cut. *Fits content like:* a change of place; a jump in time; a reveal.
Our demo opens on *the beat in unison* in telephoto against the sun, uses *the beat written on the water* as its turn and *the freeze with the sky still moving* as its climax, and ends on *the line across the sun*. A new film opens and ends another way: hooves in telephoto against the disc, each footfall a kick; *the cascade* along a line in a side pan; an ending on the last figure stepping out of the disc and leaving it empty; or long shadows merging into one line across the water.

PITFALLS OF THE MEDIUM
- **Black on dark.** A silhouette over dark water or a violet zenith disappears. Put it in the glitter column, against the horizon band or in front of the sun.
- **Merged shapes.** Overlapping figures become one blob; at key moments leave air between contours and keep limbs in profile.
- **Unreadable spread shapes.** Open wings or limbs seen face-on lose the species or the person. Prefer profile; for birds, show flight from the side (neck forward, legs back), not a V above the body.
- **Puppet humans.** A naive forward-kinematics human reads as a marionette. Hand-key a small set of anatomically correct poses and blend them, or use animals.
- **Tiny subject in the wide.** If the subject must be small, make that the point (solitude) and keep it on light; otherwise cut closer.
- **Blocky blur.** Never fake motion blur by downscaling and upscaling the frame; sample along the motion at full resolution.
- **Cutting on every beat.** The beat lives in the poses; cuts follow sections.
```
