Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Colored-Pencil Science Illustration" style.

MATERIALS & RENDERING
- **Paper**: warm off-white with a tooth (fine grain, laid fibres, soft cockling). Pencil only lands on the raised tooth, which is where the colored-pencil grain comes from. The base mottling of blank paper is fixed to the screen.
- **Four pencil layers**, in this order, over every surface:
  1. **L0, side of the lead**: a soft broad fill in the local colour, tooth-limited.
  2. **L1, local colour hatching**: patches of short parallel strokes, each patch slightly re-angled; strokes follow the object (along stems, cables, trunks and blades; around stones and leaf clumps; a fixed diagonal for soil).
  3. **L2, cool cross-hatching**: in the shadows only, a darker and cooler version of the local colour, at another angle.
  4. **L3, accents**: deepest darks in sepia or indigo, a third angle.
- **Contour**: a graphite line from breaks in object id, depth and normal; the nearer surface owns the line; distant lines fade (aerial perspective).
- **Labels**: cream paper tags with a pencilled edge; the writing is graphite, written along the glyph skeletons stroke by stroke (never wiped on). Overlay marks share the paper tooth.
- **Pipeline (reference)**: three.js renders G-buffers (albedo + id, light + depth + normal, hatch direction, anchor coordinates + reveal time) at 2× and a post shader redraws them as pencil, then a 2× box downsample. See `demo/main.js`.

COLOUR LOGIC
- Warm, restrained, paper-led. Bedrock is blue-grey, never large areas of black.
- **Accents carry meaning**, at most three in a film, each always meaning the same thing (say amber for warmth, cobalt for groundwater, red for one burrowing animal).
- **White pencil over dark** is how fine lines, highlights and anything delicate read on soil.

TYPE & SUBTITLES
- One OFL handwriting face with a pencil feel (the demo uses LXGW WenKai, whose Latin comes from Klee One). English leads; one short Chinese line may follow.
- Labels in capitals on paper tags, at most one per shot; the Chinese at about 1.05× the English size, slightly lighter. A short pencil leader with a pale paper halo under it so it never reads as another line of the drawing.
- Titles are written in pencil on the sky, stroke by stroke, with a hand-drawn underline.
- Subtitles (when spoken): a cream paper slip, bottom left; Chinese over English, each one line; wiped on quickly (they are read, not drawn). Keep the subject out of that corner.

MOTION QUALITY
- **Strokes stick to what they draw.** Hatching, the tooth inside the pigment and the contour wobble are computed in anchor coordinates: each surface point projected through a fixed reference camera. Inside a shot group the camera only **translates** (crane, truck, push), so the anchor is exact and nothing swims under a fixed pattern.
- **Only the contour boils**: a new drawing every 2 frames, 3 drawings cycled, about 1 px. Hatching and colour fields do not boil.
- **Things that move do so on twos** (12 drawings a second: grass in the wind); the camera moves on ones.
- **Drawn in, not faded in**: a sweep colours a horizon left to right with a ragged front; growing things (stems, vines, cables) appear from their base outward, already carrying their base colour, and finish their hatching within a few frames; tips grow pointed.
- Particles (grains carried along a structure) are small 3D objects drawn by the same pencil, never glowing dots.

CAMERA GRAMMAR
- **Sink**: a vertical crane from the landscape into the cut face; the soil rises over the far view.
- **The magnifier**: a pencil circle drawn around a detail; inside it, the magnified view is drawn from blank paper outward from the centre; the circle grows to fill the frame. The reverse (the circle shrinking to a point in a wider view, then erased) returns. This is how the style changes scale, and it re-anchors the strokes at the new scale.
- **Focus is how fully something is drawn.** No optical blur: the subject gets all four layers and a firm line; what is behind keeps one light wash and a faint line. A focus pull moves "fully drawn" from one thing to another.
- **Inside the magnifier, draw only what the shot is about**; illustrators leave things out.
- **Follow**: a truck alongside a moving particle, which sits left of centre with space ahead.
- The orientation stays fixed inside a shot group; to turn, change scale or cut through a magnifier.

SOUND PALETTE
- Pencil foley driven by what is being drawn: graphite writing (high, fine), waxy colour pencil hatching back and forth (lower, panned with the sweep), long soft strokes for long growing forms, a dry crackle for spreading fine lines, a light tap for a paper tag, a wooden "tock" for the magnifier.
- Ambience follows the camera: wind and distant birds above ground, closed down by a low-pass as the camera sinks, a quiet low earth tone below.
- Score in acoustic, intimate colours (nylon guitar, flute, clarinet, pizzicato bass, harp, bowed vibraphone, marimba and kalimba); leave 400 Hz–2.5 kHz light for a narrator.

NATIVE MOVES
1. **The blank half**: a finished landscape whose underground is still blank paper, waiting to be drawn.
2. **The horizon sweep**: soil coloured in left to right as the camera sinks, the four layers arriving in order.
3. **Growth drawn outward** from its base, tips first.
4. **White over dark**: fine lines spreading across soil in white pencil, bridging structures.
5. **The magnifier** in and out (a pencilled circle as a transition and a change of scale).
6. **Focus by drawing**: wash ↔ fully drawn.
7. **A grain on a thread**: a coloured particle travelling along a structure, the camera following.
8. **The plate**: the final wide cutaway, everything drawn, the title pencilled in the sky.
Our demo opens on *the blank half* and the sink below the horizon, uses *the magnifier* (in, then out again) as its turn and ends on *the plate*. A new film opens and ends another way: already inside the magnifier, pulling back out to the landscape; *a grain on a thread* followed from the first frame; nested magnifiers down to a microscope inset; or an ending held close on one detail as *focus by drawing* passes to it.

PITFALLS OF THE MEDIUM
- Hatching fixed to the screen while things slide under it ("through glass"), or re-randomised every frame (the whole screen flickers).
- Rotating the camera inside a shot group: the anchored strokes stretch. Turn by changing scale or cutting.
- Square noise cells as soil grit; grains must be round.
- Blank paper left "waiting" for more than about a second, with no drawing front moving: it reads as a bug.
- Growing tips that start as white sticks on dark soil: grow them already coloured.
- Too many labels, rulers and arrows: the plate turns into a worksheet. One label per shot.
- 3D-render tells: translucency, glow, bloom, depth-of-field blur, drop shadows on type.
- Lines thinner than about 1.5 px in a wide shot shimmer: thicken them for wide views, keep the finest for close-ups.
```
