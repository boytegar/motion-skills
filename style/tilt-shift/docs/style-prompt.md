Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Tilt-Shift Miniature" style.

MATERIALS & RENDERING
- **Vantage**: always high. Oblique views at roughly 45–55° pitch from 100–200 m with a moderately long lens (~27–30° vertical FOV); straight top-down for the most "board game" look. Real tilt-shift is never shot from street level, except for a deliberate intimate moment: a true medium close-up at the actors' height, a blurred foreground object framing sharp actors who fill ~1/4–1/3 of the frame height (a long lens from high up keeps them as dots).
- **Blur band** (`demo/post.js`): physical depth of field mixed with a screen-space band. CoC in pixels = `mix(aper·(1/focus − 1/z), sign(dy)·amp·(max(0,|dy|−w)/0.5)^1.2, mix)`, clamped (~30 px). Put the band on the subject; when the subject moves, project it to screen every frame and slide the band there. Narrow the band for intimacy. Gather at output resolution after a 2× supersampled render, with many spiral taps and per-pixel rotation; far samples must not bleed onto nearer sharp ones.
- **Grade**: neutral tone mapping, strong saturation boost, soft added contrast, slightly warm highlights, a vignette. Lower saturation at night.
- **Light**: a hard sun with sharp shadows (a big shadow map fitted to each shot), a low hemisphere fill with warm ground bounce so shadows stay blue but not murky, a sky-gradient environment map for glass and paint. **Cheat the sun azimuth per shot**: each time-lapse clip is its own shot, so front-light the facades you see; one fixed sun leaves half a film backlit and grey.
- **Model-like detail** (what separates "filmed miniature" from "game CG"): facades with several window types, 1.6–3 m bays, dark reveals, world-space grime, sky-reflective glass and walls darkening toward the ground; roof clutter, awnings and signs; worn markings and stains; vehicles with dark glass, a soft contact shadow and brake lights; ambient occlusion over everything.
- **People** are small colour capsules, like model-railway figures. Actors seen up close get organic bodies with a soft sheen material and are made 20–30 % larger than real so they read.

COLOUR LOGIC
- Saturated, sunny, "freshly painted model": a pale warm ground of buildings with a few candy accents, very saturated greens, dark ground surfaces so white markings and vehicles pop.
- Shadows stay blue and clean, never grey. Night and dawn are the only low-saturation states; dawn must read as early morning (a warm low fill from the sunrise side, few lit windows), not midnight.
- The subject you want found should be the one saturated thing in the sharp band, or move against the flow.
- Examples: a Mediterranean harbour (whitewash, cobalt shutters, terracotta, turquoise water); a snowy ski village (white, pine green, red gondolas, long blue shadows); a desert market (sand, saffron, indigo awnings, dusty olive).

TYPE & SUBTITLES
- Subtitles belong to the world's own signage: a panel in the graphic language of the place's signs (road sign, platform board, harbour notice…), an open sign-revival font (Overpass, OFL, is a Highway-Gothic revival), sitting inside the lower blur band. Hold ≥ max(1.8 s, speech + 0.6 s).
- A **time-lapse clock** with a speed readout (e.g. `×24 TIME-LAPSE` / `×1 REAL TIME`, monospace) is the audience's speedometer for the native trick; use it whenever speed changes carry meaning.
- Titles live in the world: letters on a roof, a sign or a field, placed where nothing blocks them, lit or revealed on the beat.

MOTION QUALITY
- **Time-lapse stutter is the soul of the style.** In fast-forward the whole world (vehicles, people, lights, sun) updates at a low rate and holds each position for several frames (8 Hz = 3 frames at 24 fps); choose the rate so every jump lands on the musical grid. **The camera stays smooth on every frame.** Step at a middle rate while the speed is moderate; below about ×2 the stutter disappears and motion becomes silky, which makes a drop to real time hit hard.
- Systems run on simple simulations pre-computed per clip (car-following, signal cycles, crowds gathering and surging), with cycle offsets chosen so changes land on bar lines.
- Hero actors get art-directed timing (scripted paths fitted through beat times), never a hope that the sim hits a beat.
- Close-up acting: anticipation squash → stretch → land squash → settle; a failed attempt before success is the comedy engine.

CAMERA GRAMMAR
A vocabulary, not a route. Opening and ending come from the topic.
Framing: stay high; the sharp band carries the subject; vehicles and people stay tiny except in a close-up. Transitions: cuts on the beat, speed ramps, dives and rises; avoid dissolves that reveal the fake lens.

SOUND PALETTE
- **Ensemble**: tuned percussion carries the style (marimba, glockenspiel, vibraphone, woodblocks, kalimba, celesta), pulse-based and minimalist (Reich grammar: phasing, voices added one at a time, pulsing chords), plus plucked strings or a small reed section. Not generic piano and strings.
- **Techniques, as options**: world events gate the notes of a fixed phrase skeleton (vehicles crossing a line, lights changing, crowds moving each trigger an instrument), with a base fill rate so the phrase stays audible; tuned horns or bells voiced as chords; a tape-stop (varispeed down, voices dropping out) into silence when time slows; single notes for single events in a rest; phasing two identical lines apart; a music-box version of the theme for the smallest scale.
- **Sound follows speed**: in time-lapse, music first with a faint granular "sped-up world" hiss, clacks and relay clicks. At real speed the world turns suddenly real: idling engines, distant traffic, birds, small voices, footsteps. On release, engines rev and wind rises.
- **Silence** is the moment real time arrives: cut the music, keep one or two real sounds.
- **Voice**: relaxed, few short lines, music ducked ~8 dB. Verify every line with whisper; write numbers as words and put digits in the `asr` field. Master to −14 LUFS.

NATIVE MOVES
A menu: use the ones your story needs.
- **The speed drop (×40 → ×1).** Time crashes to real speed on one detail. *Fits content like:* one worker finishing a shift; a child dropping an ice cream; a proposal on a crowded pier.
- **The band as pointer.** Slide or narrow the sharp strip to steer the eye. *Fits content like:* a courier through a market; a single wrong-way skier; one boat leaving port.
- **God's-eye system.** The place as a sequencer or machine. *Fits content like:* a port unloading ships; an airport's turnaround; a farm harvest.
- **Long-exposure light.** At dusk, lights leave trails that draw on the world. *Fits content like:* ferries crossing a bay; runners with headlamps; lanterns along a river.
- **The scale reveal.** Rise until the place becomes a pattern. *Fits content like:* a festival from setup to crowd; a school at recess; a city grid at night.
- **Cloud shadows sweeping.** Weather crossing the model. *Fits content like:* a season turning; a mood change; a news cycle.

PITFALLS OF THE MEDIUM
- Tall buildings plus shallow pitch hide the ground → low to mid-rise buildings, pitch 45°+.
- The subject hides in a canyon or behind a facade → route it on the camera side and let the band follow it.
- Instance caps silently drop actors → cull by distance from the camera (≈ 4× camera height) and raise the cap.
- Simulated vehicles stop short of lines (the jam gap) → subtract it where the story needs contact.
- Focus lags a frame → update actors before the camera.
- A top-down → oblique dive twists → rotate the top-down so its screen-up matches the target shot.
- A close-up from high up is too far → go low and close, small near plane for foreground objects.
- Clouds wash out a rise → unlit white, low opacity, nothing below the camera. Light pools read as bubbles → denser, larger, fainter pools.
- Scene backlit and grey → cheat the sun per shot. Dawn reads as midnight → warm low fill, fewer lit windows.
```
