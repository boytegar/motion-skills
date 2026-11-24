Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Exploded Teardown" style.

MATERIALS & RENDERING
- Real-time PBR (three.js `MeshPhysicalMaterial`) at 2× and downsampled, ambient occlusion, a depth of field with one aperture for wide and macro (blur follows 1/focus − 1/z, so it grows as the camera comes close), a high-threshold bloom for emitters only, a vignette.
- Each material keeps its character: plastics with soft sheen (roughness 0.55–0.7), boards with a thin clearcoat and traces as a raised, lighter relief under the mask, metallic pads, tight highlights on machined metal, matte rubber.
- **Boards are painted from their layout files**: copper pours, every trace, via, pad and drill, silkscreen, on both sides, at 14–20 px per millimetre, with roughness, metalness and bump maps from the same drawing. Components stand at their footprint positions with their package sizes.
- Light: a warm key from above front, a cool rectangular rim behind that follows the camera so edges separate from the ground, a soft top light, a low HDRI for reflections; a practical fill only where the camera goes inside.
- Background: a dark graphite seamless sweep (screen-space radial gradient), floor visible only as soft shadow.
- Effects anchored to the world (leaders, dimension lines, glows) are projected from 3D points every frame; only subtitles and title cards live in screen space.
**Where the geometry comes from.** For the user's own product, ask for its STEP / IGES / KiCad / other CAD exports, BOM and spec sheet before anything else. For a public product, look for open hardware (GitHub, GitLab, OSHWA certification list, Hackaday.io, Printables/Thingiverse sources with the original CAD) and component datasheets from the maker. CC0, public domain, CC BY and MIT are fine with credit; CERN-OHL (-P, -W, -S), CC BY-SA, GPL hardware and "no known restrictions" are not the same thing: name the terms to the user and ask before using them. Without files, build only from published dimensions (datasheet, spec page), say on screen or in the voice that it is a model, and show no number you could not source. Never stand in a generated image for the real part.
Keep a FACTS.md in the project folder (claim, source URL, quoted wording, or the measurement method for anything measured from the CAD) and deliver it with the film.

COLOUR LOGIC
- The product brings its own colours; the frame around it is neutral and dark, so the product is always the brightest, most saturated thing.
- Interface ink is one warm white at two strengths (names at full, specs at about 60 %), plus **one accent** reserved for measured things: dimension lines, label indices and one matching detail on the product itself.
- Values are ordered: ground darkest, product mid to light, labels lightest. Never put text on a busy, bright part without a dark backing.
- Example (our demo): ground #34363a → #0d0e10, ink #f1efe9, accent #ff6b3d.

TYPE & SUBTITLES
- Names: a clean sans at medium weight in the film's script (28–32 px; Noto Sans SC 500 in our demo). Specs and numbers: a monospaced face (15–22 px; JetBrains Mono) so digits line up and read as measurement. A neutral grotesque (Inter) for Latin running text.
- Label structure is fixed: a two-digit index in the accent, the name, one spec line. Units always written (mm, MB, ×n for counts). A label carries at most one fact the voice does not say.
- Dimension text sits on a small dark plate, follows the line's angle and is never upside down.
- Subtitles: a centred strip at the bottom with a soft shadow and no box; if the film is bilingual, the film's language on top. Hold every cue ≥ max(1.8 s, speech + 0.6 s); every label stays fully in frame for CJK ÷ 4.5 + other ÷ 15 + 1.5 s.

MOTION QUALITY
- 24 fps on ones; everything is smooth and deterministic.
- **Separation**: smooth start, fast middle, long settle (ease-in-out cubed); layers leave top first, 0.15–0.2 s apart; repeated parts (keys, screws) fan out in a wave ordered by distance from a corner.
- **Closing**: bottom first, quicker (about 1 s per layer), the last layer lands with a real mechanical response (a key travels its stroke and returns; nothing bounces like rubber unless it is rubber).
- While held apart, layers may float a millimetre or two with offset phases; during macro shots they hold perfectly still.
- Labels draw in (anchor dot, leader growing, text sliding 10 px) and fade out before the parts move again.
- Never moves: the floor, the light rig's relation to the camera, the label column's x position.

CAMERA GRAMMAR
- At key moments the subject fills at least a third of the frame height. Labels live in a column on the free side (shift the projection with a view offset rather than moving the object), subtitles in the bottom 12 %.
- Transitions are camera moves through the object itself. No cuts to black, no slide transitions, no split screens.
- The camera path is keyed in the product's own coordinates and interpolated without overshoot (monotone cubic), so a held macro really holds.

SOUND PALETTE
- Instruments: tuned mallets (marimba, vibraphone, glockenspiel, glass), a round bass, a short synthetic kick and small dry percussion. Precise and dry outside the object; sustained and airy inside it.
- Foley is the material: hollow plastic clacks for keycaps or housings, a switch click with a low thock, dry taps for boards, inharmonic rings for small machined metal, a soft thud for the enclosure, airy swooshes for layers lifting and for camera dives, a fine pen hiss when a dimension line draws, a tiny tick per label.
- Every separation and every landing has its sound, panned to where the part is.
- Silence: at least two near-silences (room tone only) before the big separations or the turn back; the first sound after each is the most important hit around it.
- Mix: voice clearly on top (music about 10 dB under it while it speaks), music bus ducked, sub-bass kept lean (20–60 Hz well under the full band), loudness −14 LUFS.
- Voice: calm, close, unhurried, short sentences; numbers spoken as words, written as digits.

NATIVE MOVES
- **The wave lift.** Repeated parts rise in a wave across the object. *Fits content like:* battery cells; solar tiles; the drawers of a modular shelf.
- **Inside the gap.** The camera enters the space between two lifted layers to see a hidden face. *Fits content like:* the back of a logic board; the inside of a drone arm; the underside of a sensor module.
- **The leader that lands on the part.** A macro label whose line ends on the component. *Fits content like:* a pressure sensor in a shoe; a gasket in a waterproof case; a motor driver.
- **Dimensions drawn on the thing.** After closing, the real outline gets dimension lines in perspective. *Fits content like:* the thickness of a laptop lid; the footprint of a router; the bore of a valve.
- **One real motion.** A part performs its true mechanical motion once (a press, a hinge, a latch). *Fits content like:* a camera shutter; a buckle; a folding hinge.
- **Lights on inside.** Emitters inside the product switch on while it is apart. *Fits content like:* status LEDs; a backlit display; an indicator ring.

PITFALLS OF THE MEDIUM
- **Invented numbers.** A size that no file gives. Fix: measure it from the CAD and say "measured" in the facts file, or leave it out.
- **Mixed-up revisions.** Plates, cases and boards from different versions of a project. Fix: check hole positions and stack heights across the files (they must coincide), and pin the commit.
- **Panelised boards.** Layout files may hold a panel with breakaway tabs. Fix: take the outline from a plate or case of the same revision, or flood-fill from an interior point.
- **Zero-length edges.** Duplicate outline points give zero normals → NaN → black squares after bloom. Fix: clean outlines, repair zero normals, scan frames for one-frame black tiles.
- **Overshooting camera splines.** Catmull-Rom over unevenly spaced keys flies past a macro stop. Fix: monotone interpolation per component.
- **Labels too short.** Macro stops last about three seconds. Fix: one fact per label, opened before the camera settles; check with `readcheck`.
- **Over-bright light sweeps.** Area lights blow plastics to white. Fix: sweeps at a fraction of the key, bloom threshold above the brightest diffuse surface.
- **Brand marks on parts.** Chip tops and connector shells carry trademarks. Fix: plain package tops; drop logo footprints unless the product is the user's own.
- **Crushed blacks.** Black plastics against a black ground disappear. Fix: lift them to a dark grey and let the rim light separate them.
```
