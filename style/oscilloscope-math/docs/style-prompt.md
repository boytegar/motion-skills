Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Oscilloscope Math" style.

MATERIALS & RENDERING
- **Vector beam on a phosphor screen.** Draw with constant-width lines in screen pixels (1.2–3.2 px), so a line stays a beam at any zoom. No scanlines (an analogue scope is a vector display).
- **Light channels.** Draw three layers of light (beam trace, graticule and text, white-hot cores) plus an optional second channel for measurements; a post pass gives each its phosphor colour and glow. Beam layers bloom strongly; text blooms faintly so it stays crisp.
- **Bloom**: several Gaussian blurs at 1/2 to 1/16 size added back; where light piles up it turns towards white, through a soft shoulder rather than clipping.
- **Afterglow is drawn, not accumulated.** Earlier instants of moving objects (8–10 ghosts a few milliseconds apart, fading by a power curve) and a short trail behind a moving point. Frame-to-frame decay strobes at 24 fps; do not use it for anything that moves fast.
- **A fast beam leaves less light.** Steep segments of a trace are drawn dimmer (about 55–60 % on a vertical edge); older parts of a scrolling trace fade with age.
- **Graticule**: dotted minor lines, a solid centre axis with 0.2-division ticks, refined in levels of 5 as the camera zooms (the finer level fades in).
- **The screen**: near-black with a faint green ambient falloff, a vignette, a little dither noise so the glow never bands in 8-bit video.
- Example (our demo): Manim (Cairo renderer) draws the channels into PNG frames; a numpy/OpenCV pass makes the phosphor.

COLOUR LOGIC
- One phosphor family; colour means role, not decoration.
- Beam green for anything the signal draws; paler green for graticule and text; white-hot for the beam's newest point, joints and arms, and the flash of anything just created.
- **Amber = the second channel**: measurement cursors, measured values, markers of a finding. Never used for decoration, never for the signal itself.
- No other hues. Grey cards only for unfinished placeholders.
- Example (our demo): screen #010402, ambient #0B2412, beam #4DFF88, text #8DFFB0, hot core #E6FFEC, amber #FFB547.

TYPE & SUBTITLES
- **Instrument voice**: a monospace (e.g. JetBrains Mono, OFL) for readouts, cursors, knob values, labels on objects, 19–30 px. Values are live (computed every frame); align the `=` signs in a column.
- **Formula**: TeX typography (MathJax SVG paths or LaTeX), top left, about 520–560 px wide; glyphs are lit one by one by the beam; the part that changes (an upper limit, a coefficient) flashes when it changes.
- **Titles**: a serif that matches TeX (e.g. STIX Two Text, OFL), 60–84 px, with the key word in italic; a Chinese line under it in a CJK serif (e.g. Noto Serif SC) ≥ 34 px. Written by the beam (outline traced, then filled, then a white-hot flash), never faded in.
- **Subtitles**: bottom left, fixed to the glass, a thin vertical light bar, Chinese serif 36 px over English STIX italic 29 px, one line each, a soft dark scrim behind them so a trace never runs through the words. Keep subjects and labels out of that corner.
- Holds: anything the viewer must read stays at least characters ÷ 15 (Latin) + characters ÷ 4.5 (CJK) + 1.5 s; a measured verdict holds ≥ 2.5 s.

MOTION QUALITY
- Rotation is uniform and real (the frequencies are the maths). Growth eases in: a new element grows from nothing at the point it attaches to in about 0.45 s, flashing white (in our demo, a new circle at its parent's tip).
- Additions accelerate rather than repeat: one, then a few, then too many to count.
- Live values update every frame; when a value is the point, it is shown at the precision that stays constant.
- STOP freezes the signal; the camera may keep moving through the frozen world.

CAMERA GRAMMAR
The camera lives in the world (centre and zoom); zoom is logarithmic, and the centre moves so the target stays put on screen while zooming (flying to a point). Line widths and HUD do not scale.
Transitions come from the medium: letters morphing into the next subject; a title flying down and straightening into the axis; STOP followed by a dive; a CRT switch-off (the picture collapses to a line, then to a dot). No crossfades, no slide-like page changes.

SOUND PALETTE
- **The signal is audible**: a quiet tone computed from the same numbers as the trace on screen (in our demo, the same partial sum as the picture: each new circle adds its harmonic).
- **Lead**: plucked synth tones whose timbre follows the signal on screen (in our demo, a pure sine for one term, richer with every term added); tuned bells (glockenspiel, bowed vibraphone, sine bells) for events; a soft synth bass with 2nd and 3rd harmonics.
- **Instrument foley**: relay clacks for RUN/STOP and power, knob detent clicks, short blips for readouts and cursors, a tick per glyph as the beam writes, filtered-noise sweeps for morphs and camera dives, a falling zap for power-off; a faint electrical hum bed.
- **Silence**: STOP cuts the music dead; the twist is heard over hum and a slow clock.
- **Mix**: music under the voice by about 10 dB RMS, its 300 Hz–4 kHz band carved under speech; −14 LUFS, true peak ≤ −1 dB.

NATIVE MOVES
- **Epicycle chain**: circles on circles whose tip draws a curve, with afterglow. *Fits content like:* Fourier series; phasors in AC circuits; planetary epicycles.
- **Live readouts**: values computed from the drawn state, updating every frame. *Fits content like:* a pendulum's energy; a filter's gain; a running sum.
- **Text into geometry**: a title written by the beam, its letters morphing into the next subject. *Fits content like:* naming a theorem before it appears; a chapter title becoming an axis.
- **Knob zoom**: a dive with refining graticule and changing scale readouts. *Fits content like:* the fine edge of a fractal; a discontinuity; numerical error near a limit.
- **Cursor measurement** (amber): two lines, an arrow, the difference and its share. *Fits content like:* a period; a rise time; a peak height.
- **STOP and power-off**: freezing the signal for a closer look; collapsing the picture to a dot at the end.
Our demo opens on the beam writing the answer before the question, uses *STOP* and a *knob zoom* into one corner as its turn and ends on a pull back over the whole curve, the name written by the beam and the *power-off*. A new film opens and ends another way: a formula whose letters become the first object (*text into geometry*), two signals compared on two channels, an ending on a locked frame with *cursor measurement* holding the verdict, or a match cut to another curve of the same shape.

PITFALLS OF THE MEDIUM
- **Numbers that don't match the picture.** Compute every readout from the drawn values; never type them.
- **Strobing afterglow.** Frame-accumulated decay at 24 fps leaves doubled curves; draw ghosts sub-frame.
- **Thin features vanish when sampled.** Sample curves at screen resolution and add dense samples around edges and spikes, or a 0.1 px spike disappears.
- **Crowded chains.** Small circles pile up at the tip; push in, or draw only the circles larger than about a pixel and limit ghosts to the first few.
- **Out of frame.** Size the chain so its full reach stays inside the graticule (a sum of radii can exceed the radius you expect).
- **Text in the glow.** Keep glow on beams, not on type; no outlines or drop shadows on text.
- **Readouts that flicker.** A value meant to be read is shown at a precision that holds still.
- **Facts.** Mathematical claims shown as numbers come with their source (FACTS.md).
```
