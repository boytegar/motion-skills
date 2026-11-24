Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Engineering Briefing Board" style.

MATERIALS & RENDERING
- **Board (z 0)**: procedural enamel, wiped ghost strokes, smudges, a window falloff, a sliding diagonal reflection; the frame shadows a warm grey wall.
- **Ink (multiply)**: flat colour at about 90 %, lighter core, round caps, a wet-tip glint on the stroke being drawn. Lettering: a marker font revealed glyph by glyph with seeded tilt and baseline jitter. Erasing leaves a blurred ghost (about 7 %).
- **Prints (z ≈ 6–14)**: photographs cut out at full resolution (a matting model, or a hand polygon / mask), clean edges, one shared grade (lower saturation, a touch warmer, lifted blacks). Thin structure (cables, wires, masts, railings) is measured in the photo and redrawn as clean lines in one colour. Shadow = blurred silhouette along one light vector, softer the higher the object. **No frame upsamples a photo**: ≥ 1 source px per screen px at its largest, measured; cap the zoom or find a bigger source.
- **Secondary layer** (only where the camera never goes before the end): the photos as pinned prints, a source card, sums on sticky notes, a small sketch. Texture and proof for the final wall, lighter than the main line.
- **Props (z ≈ 8–26)**: magnetic tags, magnets, a steel ruler, printed cards, a pitted stamp impression; the tray (z 70) nearest the lens.
- **Camera**: parallax by height, sub-frame motion blur (180° shutter), light grain. Only subtitles are screen-space.

COLOUR LOGIC
- Ground: the enamel and its light falloff; the wall around is neutral warm grey.
- Ink: graphite = names, rules, facts; blue = water and measurement (dimensions, projection lines, measured numbers); red = the decision (a rejection, the key word of a verdict, the one mark to notice); grey for drafting only (extension lines, hatching).
- Red is rare: two or three red marks per frame at most, never on a red object. Full colour belongs to the photographs; props stay neutral; a red magnet is a verdict.

TYPE & SUBTITLES
- **Marker lettering**: a single-weight marker face covering every script the film uses (ours: LXGW Marker Gothic, OFL, subset — re-run when words change). Content 100–160 board px; a verdict 230–400.
- **Printed labels** (tags, cards, stamps): a sturdy sans; credit lines in a monospace. A tag holds one to three words.
- **Subtitles**: a frosted acrylic strip at the bottom centre with a blue marker tick. If the film is bilingual, the film's language sits on top. **One line per language**: `nowrap`, measured in the page against a 1,500 px safe width; a long line becomes two cues. Key content stays above the strip (bottom 15 %).
- **Reading time**: subtitles ≥ max(1.8 s, speech + 0.6 s); board text stays fully in frame for CJK ÷ 4.5 + other ÷ 15 + 1.5 s from when it appears. In the final wall, text meant to be read is ≥ 20 px on screen.

MOTION QUALITY
- 24 fps on ones; nothing is stepped.
- Camera: overlapping eased moves, velocity ∝ u^a (1−u)^b (a, b ≈ 1.5–4), so speed and acceleration never jump; b > a gives inertia. A hand-held breath (≤ 3 px) underneath; shake only on an impact.
- Weight: prints are lowered from the camera side, land, bounce and settle as the shadow tightens; magnets clack on to pin them; tags swing; removed magnets pop toward the lens; struck things jolt and spring back. The camera may meet a landing close, briefly, on the object's most telling detail, then pulls out.
- Blurred in motion, sharp where words are read. Every 1–2 beats something lands, draws or swings; only silences and judgement holds go still.
- Ink draws at the speed of a hand; dimension lines land with an arrowhead tap.
- Never: dissolves, scale-pops on text, bouncy UI easing, text fading in by itself.

CAMERA GRAMMAR
Framing: off-centre, cropped, close. The first frame holds a readable subject; the first strong action hits within 0.5 s. Items tested against a limit each get a judgement frame (wider each time if they grow); the verdict lands after the camera settles. Before the reveal: ≤ about 60 % of the board's width, never both side edges, never the reveal's zone. Transitions are moves; no cuts, fades or zoom-to-fit.

SOUND PALETTE
- Room: an air-conditioning bed and traffic through the window, about 20 dB under the voice — audible, not a hiss; silences keep only this.
- Foley: felt-tip squeaks scaled to stroke length, nib taps, magnet clacks (a pitch per magnet in a row), suction pops, print landings (board thump, paper slap), a ruler's ring, eraser felt, a stamp's thud.
- The subject's own sound arrives early as a J-cut; the first sound after a silence is an impact or the subject's sound.
- Music: a small rhythmic ensemble, walking to brisk — bass, a light kit, wooden percussion, low brass stabs for verdicts (or mallets); no piano-and-strings pad. Landings, stamps and arrows sit on the beat. Voice calm, briefing-room confident, about 8–10 dB over the music; −14 LUFS.

NATIVE MOVES
- **The whole-wall reveal.** Design the wall as a poster first (hierarchy, reading path, filling the frame), then plan every shot as a crop of it; keep one zone (secondary layer, verdict, title card) for the reveal alone. *Fits:* a product roadmap; a household budget.
- **The subject passes through the answer.** The real thing moves through the chosen solution, its sound first. *Fits:* a parcel along the chosen route; a train over the new viaduct.
- **Ride the dimension line.** A quantity drawn as a real length to one scale; the camera rides the tip. *Fits:* battery ranges of four cars.
- **Projection lines.** Dashed lines carry an edge from one view to another so two drawings share a scale. *Fits:* a trail map above its elevation profile.
- **Magnets as countable things.** Units clack on in rhythm, pop off when a rule removes them. *Fits:* seats in a vote.
- **Erase to a ghost.** A rejected idea wiped but faintly visible. *Fits:* last year's plan under this year's.
- **The stamp.** A verdict lands with a thud and a jolt. *Fits:* passed / failed tests.
- **The physical answer.** A real prop laid down as the solution. *Fits:* one cable instead of five.
- **Leader to the verdict.** An arrow from the deciding number to the conclusion. *Fits:* the bottleneck in a process.

PITFALLS OF THE MEDIUM
- **Text glimpsed in passing** fails the reading time. Fix: words where the camera rests; frame each new item with the last result.
- **Only close-ups.** The viewer never sees the whole object against the limit. Fix: detail → judgement frame → hold → verdict → move on.
- **Photos that don't belong together.** Fix: one grade, one light vector, one viewpoint family.
- **Thin parts lost or stair-stepped** by the matte. Fix: key them against the clean background, redraw clean lines at the measured positions.
- **Upsampled photos** go soft in close-ups. Fix: full-resolution cuts, large sources, a per-frame check.
- **Lettering in photos** (signs, names, logos). Fix: paint out, cut, or choose another photo.
- **The wall seen too early** spends the ending; **a sparse or tiny final wall** wastes it. Fix: measure every frame's visible share; size the hierarchy for the poster; a quiet secondary layer, no empty middle.
```
