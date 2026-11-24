Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Chalkboard" style.

MATERIALS & RENDERING
- Canvas 2D for everything visible; WebGL2 only for the post pass (camera motion blur, a light vignette, grade, dither).
- Each stroke is baked once into a board-space coverage sprite (coverage, arc length, lateral position). A partial stroke reveals pixels up to the current arc length with a round leading edge; the board's grain decides which grains take the chalk (deposit with a 2×2 supersample and a faint powder halo).
- Chalk edges are firm; loose powder only at the lift-off of a stroke (two to five grains). Never sugar the board.
- Writing strokes carry a width profile per stroke type (start bump, corner bumps, taper for 撇 / 提, foot for 捺, quick lift for hooks) and a pressure profile (heavier at both ends, lighter in the middle). Straight strokes bow slightly; horizontals rise 2–3° to the right; 横, 竖 and 折 enter with a short pressed tick from the upper left.
- Board, panels, frame, tray, clock and wall are baked once in world units; the slate grain is a periodic tile drawn in world space (soft-light) only when the camera is close.
- Erasing is a blurred destination-out band along one smooth path on a layer that holds only the strokes being wiped; it leaves a very thin, even haze. One wipe per film is plenty.
- A wet streak (a board just wiped with a damp cloth) is a soft mask with a wide blur at its drying edge, fading at top and bottom, that darkens the slate slightly; never a hard rim.
- Light: a gentle multiply falloff away from the windows and at most a faint warm window patch. Light never draws shapes across the writing in close shots.

COLOUR LOGIC
- Ground: deep ink green slate, wood frame, warm white wall and a coloured dado if the room is seen.
- White chalk carries every structure: characters, lines, labels, arrows.
- The accent chalk (example: orange) is reserved for the one component the lesson tracks; it never decorates and never draws arrows or underlines that are not that component.
- A pale second chalk (example: yellow) marks another person's handwriting.
- No rainbow, no gradients in chalk, no glow. Painted grids are dull and thin so the chalk always wins.

TYPE & SUBTITLES
- Chinese is hand-authored stroke data (centre lines + stroke type per stroke) drawn by the chalk engine, never a font's outline. Every character the topic needs is added to the data and checked against an OFL kai font behind it before shooting (how: §10, *New characters*).
- English on the board is a single-line OFL handwriting font (EMS family) drawn by the same engine; symbols missing from the font (arrows, plus) are drawn as strokes.
- On the board: keywords only, one line of Chinese, the next line English, a size smaller.
- Sizes at 1080p: big characters 300–450 px; labels ≥ 55 px CJK and ≥ 30 px Latin cap height; small writing gets heavier chalk so no stroke is thinner than 3 px.
- Reading time: every block stays fully in frame ≥ CJK ÷ 4.5 + other ÷ 15 + 1.5 s, counted from the frame it is wholly visible (readcheck). A block is fully written before it is read.
### Voice and subtitles
- When the style tells a story it is narrated: one calm teacher's voice in short spoken sentences, said as the matching chalk appears; what the voice names is being written or shown at that moment.
- A quiet subtitle plate sits above the bottom safe area: the spoken line in Chinese and a free English translation, one line each, a plain kai face (an OFL subset) on a soft dark plate, in and out with the voice. Long lines are split at sentence ends into several cues.
- Nothing the viewer must read on the board sits behind the plate; frames held for the board's words are composed above it.
- Pauses between voice lines are short (0.25–0.4 s); one longer hold (about 1.5 s) is kept for the verdict. Board action (a panel sliding, a wipe) runs under the voice instead of waiting for it.

MOTION QUALITY
- 30 fps, continuous (render with `--fps 30`: `core/render/video.mjs` defaults to 24).
- Chalk speed, measured on our demo: a big character takes about 0.1 s per stroke including the air between strokes (a ten-stroke character about 1 s), labels about 0.04 s per stroke. Writing speeds up rather than making the voice wait, but a big character written much faster than about 0.07 s per stroke stops looking like a hand.
- When reading time and chalk speed pinch in a short film, change the words, not the hand. Ways that work (choose per film): fewer and shorter blocks on the board; big characters only for what the lesson tracks, everything else at label size; start a block while the voice is still on the previous line, so it is finished when it is named; write the next block while the last one is still being read in the same frame; hold the frame instead of moving on; make the film longer.
- Camera eases in and out (cubic), never bounces except a whip landing.
- Things appear only by being written, wiped, revealed by a sliding panel or by drying. Nothing fades in.

CAMERA GRAMMAR
The camera always faces the board flat on (no perspective tilt).
The room is part of the style: at least once the frame opens wide enough to read board, frame, tray and wall as a classroom (where, and what else the room holds, is the film's choice). Close shots may show only slate and frame.
Rules: the subject is never smaller than a third of the frame in close shots; text blocks are never cut by the frame edge while they are read; no empty board for more than half a second. Transitions are hard cuts, a sliding panel or a line across a seam; no dissolves or page flips.

SOUND PALETTE
- Voice first: it is the loudest element; music sits 15–18 dB under it and comes up only in the pauses.
- Foley: real chalk recordings cut into grains, one grain chain per stroke, level from stroke size; a light knock on landings; the accent chalk slightly softer and duller. A felt pass for the wipe, the low roll of a sliding panel ending on a stop, room tone very low; school sounds (a distant bell) only as punctuation. Where the recordings come from and how to cut them: §10, *Sound*.
- Music: sparse plucked strings (a guqin model or nylon guitar) with marimba and pizzicato bass; a plucked note can be "born" from something drawn. Silence while a question or a verdict is read.
- Mix: about −16 LUFS integrated before mastering, true peak ≤ −1.5 dBFS.

NATIVE MOVES
- **Write, wipe, rewrite in place.** The wrong reading is written under its object, wiped with one pass, and the right one written in the same spot from the same camera position. *Fits content like:* a common misconception in physics; a misremembered date; a sum everybody gets wrong.
- **Tracked component in accent chalk.** One part keeps its colour and its height across several forms. *Fits content like:* a root through a word family; a variable through an equation's rearrangements; a gene through generations.
- **Sliding panel reveal.** The front panel slides away to show a clean back board, and can slide back to close the lesson. *Fits content like:* "but before that…"; the hidden proof; the answer kept back.
- **A line that sews two boards.** One accent line runs across the seam to join two places. *Fits content like:* the same constant in two formulas; a quote and its source; cause and effect.
- **Drying wet streak.** The freshly wiped board dries back and reveals what was left untouched. *Fits content like:* yesterday's question; a note left by a student; a reminder at the corner.
- **Second hand.** A different, childlike chalk colour and hand on the same board. *Fits content like:* a pupil's question; a visitor's comment; a wrong attempt.
- **Chalk as a measuring tool.** An accent object laid along a white line and ticked off, so a comparison is shown, not told. *Fits content like:* a unit's origin; a scale; how many of one thing fit another.

PITFALLS OF THE MEDIUM
- Dirty boards (old swirls, ghosts, floating dust) read as "messy", not "real". Keep the slate clean and let the grain carry the realism.
- Equal-width polylines look like a plotter and make characters misread. Use stroke-type width and pressure profiles.
- Character data that is structurally wrong is caught at once by native readers: check every character against a reference before shooting.
- Small writing at 1080p disappears with the same chalk width as large writing; give small text heavier chalk.
- Text written while the camera moves away fails reading time; hold the frame or keep the block in view.
- Accent colour spread over arrows and decorations stops meaning anything.
- A soft halo at the chalk's front reads as glow. The demo's `drawFront` (`props.js`) puts a faint radial gradient under its grains: in your copy, drop the gradient and keep the grains.
- A wet streak with a hard edge looks like a cut-out.
- Board-speak in the voice ("A plus B", component names with no meaning) is not understood: every component is said with what it means the first time it appears.
```
