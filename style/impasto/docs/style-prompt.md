Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Impasto Oil Painting" style.

MATERIALS & RENDERING
- **Value first.** Design each set as a flat, value-planned illustration (3 values minimum), then convert it to strokes. If the reference doesn't read at 480 px wide, no brushwork will save it.
- **Stroke sizes by importance and depth.** Coarse to fine: large planes in sky and walls, a few px only at edges and faces (per shape: `maxR`, `detail`, `dir`).
- **Strokes never cross shape groups**, so silhouettes stay crisp. Lost edges are opt-in (put two shapes in the same `group`).
- **Thin underpainting** (the reference itself, blurred ~1.5 px) under the strokes, so the canvas never peeks through as outlines.
- **Knife stroke**: longer than wide (≈ 1.7–3.1 × radius long, 0.95–1.45 × radius wide), slanted ends, small angle and value jitter (≈ ±13°, ±6 %). Solid objects that must read as silhouettes (figures, props) get little or no ragged run-out.
- **Light** rakes over a height field; wet or glossy surfaces get slightly more specular.
- **Reflections** (wet ground, glass, water) are **explicit stacks of short horizontal knife dabs** under each light source, broken and fading; never a painted rectangle.
- Render supersampled (2×). **No film grain**: grain plus compression eats the knife edges.

COLOUR LOGIC
- Planes of mixed, slightly broken colour; each stroke may drag a second colour (`c2`) into the first. No gradients made by blending pixels: gradients are made by stroke-to-stroke steps.
- Warm light against cool shadow is the default temperature structure; the brightest, purest chroma is reserved for the story's subject (a prop, a crowd, a light source), not the background.
- **Colour can be an event.** Every stroke can hold a grey and a colour version; a monochrome world with one coloured exception, or a world re-laid from grey to colour, is native to the medium. The grey version is the luminance of the same strokes, slightly cool, with a small colour residue.
- Examples of palettes that fit: an after-rain sunset (cobalt → peach → gold sky, ochre facades, full-chroma props); a harvest field (straw yellow, burnt sienna earth, one viridian tractor); a night kitchen (umber shadows, sodium-lamp orange, one cold blue window). Choose from the topic.

TYPE & SUBTITLES
- Titles are **laid in with the knife** (a stroke-by-stroke `appear` sweep of about a second) and can be taken off by the world (washed by rain, scraped, painted over). They move with the world.
- Small text (credits, data, captions) stays crisp DOM text over the painting: knife strokes can't carry small letters.
- A serif display face (e.g. Cormorant, OFL). Subtitles, if there is a voice: plain serif, ≤ 2 lines, in a calm plane; hold ≥ max(1.8 s, speech + 0.6 s).

MOTION QUALITY
- Static paint is static. Only the things the story moves move (weather, figures, props, ribbons, light). Smooth 24 fps, no stepping: clarity is the brief.
- Moving figures are procedural strokes rebuilt each frame from **fixed seeds**, so their paint doesn't boil (walk cycles, IK arms, props that open and close).
- Actions sit on the music grid: a change of direction, a pop, a swell lands on a beat.
- Anticipation → action → follow-through on every gesture (lift before the strike, dip before the snap, swing after the stop).

CAMERA GRAMMAR
A vocabulary, not a route. Opening and ending come from the topic.
Framing: keep the camera **inside the painted plate** (`half = W/2/zoom`), or the canvas shows. The subject gets the hard edges and the smallest strokes; everything else stays in big calm planes. Transitions: paint-over sweeps and scrapes, hard cuts on beats; no dissolves (two layers of knife ridges turn to mud).

SOUND PALETTE
- **Foley follows the paint and the world**: palette-knife scrapes on every repaint or paint-over, wet dabs, the materials of the scene (wet stone, nylon, wood, fabric, glass).
- **Beds** in layers (a hiss, near grains, one distinct detail), filtered when the camera goes under cover.
- **Acoustic instruments with a hand-made grain**: solo strings, accordion, pizzicato, harp arpeggios, glockenspiel or celesta for pops, piano, guitar, woodwinds; a solo instrument for a lonely world, an ensemble for a shared one.
- Options: **one action = one sound = one cut** (music-locked edit); pops pitched as a rising arpeggio so a sequence of events climbs; an unresolved phrase that the film resolves later; a hard silence that cuts music and beds to digital zero, containing at most one sound; a J-cut of the next scene's foley into a silence.
- Mix: music and ambience duck under any voice (~8 dB); −14 LUFS. Music and foley can carry a film without narration.

NATIVE MOVES
A menu: use the ones your story needs.
- **Colour re-laid by the knife.** Every stroke keeps a grey and a colour version; a reveal wave re-paints them stroke by stroke (`reveal`, `revC`). *Fits content like:* a town waking up; a patient recovering; a brand relaunch; a season turning.
- **The one coloured thing.** The world drawn with `grey: 1`, one object with `grey: 0`. *Fits content like:* one person in a crowd; the fruit that ripened; a warning light.
- **Motion paints the picture.** Moving objects drip their colour as knife dabs; seen from above, their paths leave a painted pattern. *Fits content like:* a dance; delivery routes across a city; a football play; migration.
- **Raking light.** At the peak, drop the light to a grazing angle so every ridge catches it, then let it settle. *Fits content like:* a sunrise; a moment of recognition; unveiling a sculpture.
- **Paint-over transition.** The next shot's strokes are laid over the previous one in a sweep (`appear`), with a scrape on the soundtrack. *Fits content like:* a renovation; memory replacing the present; a timeline of versions.
- **Sound made visible.** A note or a voice becomes a paint ribbon that carries colour to what it touches. *Fits content like:* a song reaching listeners; a radio broadcast; a rumour spreading.

PITFALLS OF THE MEDIUM
- Confined strokes shrink near edges and leave canvas-coloured outlines → add the underpainting texture.
- Draw order in the reference matters: painting the ground after the buildings covers their feet.
- Plain rectangles read as posts or blocks → give silhouettes taper, a lit edge, and one or two defining dabs (shoulder, hair, handle).
- Proportions carry the object's identity at this stroke size: a prop drawn too wide reads as a different thing. Check silhouettes at thumbnail size.
- Film grain or heavy compression destroys the knife edges → no grain, supersample, high bitrate.
- Strokes that are re-seeded every frame boil → fixed seeds per stroke.
- Engine-level WebGL traps (reserved words, VAO binding) are listed in DEMO.md build notes.
```
