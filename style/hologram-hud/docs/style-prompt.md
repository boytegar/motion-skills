Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Sci-fi Hologram HUD" style.

MATERIALS & RENDERING
- **Stage**: near-black tinted by the hue, a radial lift behind the subject, a fading dot grid, a strong vignette.
- **Projector pad** in true perspective: concentric rings (one bright, one dashed), ticks, faint rays; it rotates slowly and spins with any turntable move.
- **Wireframe**: additive ~1.3 px strokes in a few alpha levels set by **depth fog** (near bright, far ~25 %), plus two low-res glow passes.
- **Hot edges** (near-white of the hue) only for what matters now: the scan front, inner pieces flagged `hot` when opened, a light wave, a part being pinged.
- **Real CAD topology**: tubes are rings plus longitudinal lines. **Every part is physically connected**: a floating piece is noticed first.
- **Solid hologram**: faces filled with a screen-space **Fresnel** × a scrolling **interlace**; lines thicken as it ignites; a light wave climbs the model. A **beam** (one path, nonzero winding) with faint rays and dust.
- **HUD furniture**: corner brackets, parallaxing rulers, subject // mode top left, a live status line top right, telemetry at the bottom.
- **Text never sits on the wireframe**: every text block has its own **dark plate** (translucent near-black, 1 px border, corner ticks); the camera still keeps the subject clear of text.

COLOUR LOGIC
- **One cold hue plus one warm accent**, both parameters (e.g. mint/orange, violet/lime, ice-blue/red; choose from the subject).
- The hue carries lines, HUD and fills. The **accent is spent on locked values only**: a number turns accent-coloured at the frame it becomes true. Nothing else ever uses it.
- White is heat: near-white edges and one-frame flashes mark what is active. Everything at rest sits in the hue's alpha levels.
- One object may keep its own colour (the single-colour exception, `keep`), for a brand mark, never for decoration.

TYPE & SUBTITLES
- Rajdhani, all caps: Light for big numbers and the product name (wide tracking), Medium/SemiBold for labels. Share Tech Mono for micro data (coordinates, counters, zoom).
- Numbers are **monospaced** so rolls never jitter; the decimal point is a small square (a glowing "." reads as a comma).
- Subtitles are HUD: a dark plate with corner brackets and "voiceprint" bars moving with the speech, near-white Rajdhani Medium, bottom centre; in just before the voice, hold ≥ max(1.8 s, speech + 0.6 s).
- The title is part of the scene (no title card): the name rolls in like a value, then code, category and tagline.

MOTION QUALITY
- **Everything on ones at 24 fps.** Only glyph scrambling steps, every frame.
- **Target lock**: oversized, rotated 45°, faint; snaps to fit in ~0.2 s (cubic ease-out), flashes white 3 frames on the downbeat.
- **Explode** along each piece's assembly axis: the shell slides *along its own axis*, the contents stay in place and glow. Open over a beat or two, close fast.
- **Leader**: anchor ring → diagonal → horizontal to the card in ~0.5 s. **Rolling value**: characters lock right-to-left and turn accent on the downbeat with a one-frame flash.
- **Chip dock**: a finished value slides into a side rail and stays until the end.
- **The camera follows the box**: cubic ease, starting *before* the next lock, **real motion blur** only inside the whip (sub-frames, 180° shutter); HUD stays sharp.
- **No cuts or fades inside the scene**: only the scan plane (scanning in or erasing) and a loupe iris. A **glitch** of three frames at most once, on the biggest change.

CAMERA GRAMMAR
An orbit camera (yaw, pitch, distance, narrow fov) with a screen position for the target, so the subject can sit beside text. A vocabulary, not a route. Opening and ending come from the topic.
Framing: the subject is ≥ ⅓ of frame height at every key moment. Cards on one side, the chip rail on the other, subtitles bottom centre, a loupe above them: never colliding. Shorten explode vectors rather than fly a piece into text.

SOUND PALETTE
- **Music first**, on a grid (a bar = one information beat); locks land on downbeats, checked against the score.
- **Synth palette**: saw/square arpeggio with ping-pong delay (scooped 1–4 kHz for the voice), square pulse, 8th bass, synth hats and claps, detuned saw pads.
- **Density follows information.** Tools to pick from, in any order (most films need two or three): half-time for reading time; one beat of **true digital silence** (nothing, not even UI blips); a drop (808-style glide + wide saw chord) for the biggest change; a layer added or removed per part; deceleration by longer note values; a key change; a single sustained tone under a long hold.
- **Foley follows the material**: light = sines and FM blips **in the key**; machines = servo slides, pneumatic puffs, hydraulic hiss, metal tinks (J-cuts); a lock = thump + clack + two-tone beep; rolls tick at frame rate. Ambience: projector hum (40/80/120 Hz) and room tone.
- **Voice**: calm, confident. Duck music *and* foley with a **held** envelope (no pumping between words); lock transients escape. −14 LUFS.

NATIVE MOVES
Each move serves information, not story. A menu: use the ones your film needs.
- **Scan growth = "this is real data."** *Fits content like:* a coffee machine; a satellite; an architectural model.
- **Target lock = "look here now."** The box is the camera. *Fits content like:* a car's sensors; a laptop's ports; a robot arm's joints.
- **Local explode = "why the number is true."** *Fits content like:* the burr set of a grinder; the layers of a running shoe sole; the elements of a camera lens.
- **Rolling digits = the payoff beat.** *Fits content like:* a brew temperature; a tolerance in microns; a price.
- **Chip accumulation = reading time.** *Fits content like:* 3–5 facts that must all be read.
- **Loupe = two scales at once.** *Fits content like:* a screw inside a watch; a chip on a board; a valve in an engine.
- **Solid hologram + beam = "the whole is ready."** Once, if at all. *Fits content like:* a launch reveal; a finished assembly; a v2 replacing v1.

PITFALLS OF THE MEDIUM
- Target boxes balloon → rest pose, skip `internal` pieces, long attachments `box:false`.
- Long diagonal leaders point at the wrong part → ring on the part body, flash the part.
- Two parallel boxes moving don't read as an explode → shell slides on its axis, contents stay and glow.
- A part built from generic boxes doesn't read as that part → give it its characteristic silhouette.
- Voice masked by foley, not music → duck foley too, hold the duck through word gaps.
- UI blips inside a silence break it → check cues against silence windows.
- A whip written inside the next segment plays as a hard cut → start it in the previous one.
```
