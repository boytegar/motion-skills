Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Swiss Motion Graphics" style.

MATERIALS & RENDERING
- **Frame grid**: 1920×1080, 12 columns, ~96 px side margins, 24 px gutters, 24 px baseline. The frame can act as a **spread**: a left page (the programme: numeral, rule text, notes) and a right page (the artifact).
- **Artifact grid** derived from the content (Gerstner): what the columns and rows *mean* is a design decision. **Choose module sizes that give the elements mass**; too many thin columns make a layout look like a chart.
- **Surfaces**: page, paper, ink, signal colour, grid lines in two or three light greys (on page, while building, on paper). No gradients, no shadows, no particles, no grain (mux grain 0).
- **Bars and blocks** are flat rects; consecutive cells of one unit merge into one bar, overdrawn 1 px at seams (antialiasing shows hairlines, especially when rotated or scaled).
- **A secondary voice** can be light-grey elements under the black ones: depth without a second colour.
- **A 2D camera matrix** (centre, scale, rotation) for all moves.

COLOUR LOGIC
- **Paper, ink, greys, and one signal colour.** Only one thing carries the signal colour; that makes it the protagonist. It is the only thing allowed to be inexact.
- Overprint is allowed: black type on the signal colour.
- The signal colour can be red, orange, ultramarine or green; choose it for the topic, then never add a second.
- Examples: `#EAE9E5` page, `#FFFFFF` paper, `#111111` ink, signal `#E30613`; or `#F2F0EA` / `#141414` / signal `#FF5A00`; or `#E9ECEF` / `#0B0B0B` / signal `#1F3FFF`.

TYPE & SUBTITLES
- **One family for the whole film**, an OFL neo-grotesque (Archivo, Inter, Schibsted Grotesk, Instrument Sans…), several weights. Letter-spacing ~−2 % at ≥ 90 px. Headlines can be lowercase. Load every weight with `document.fonts.load` before drawing.
- **Flush left, ragged right**; everything on a grid line; big white space is part of the layout. Giant numerals are **optically** flush left (subtract the glyph's left bearing).
- **Subtitles are part of the layout**, not a bar at the bottom: each line sits at a column start in the current composition (a small grey lead-in + a large bold statement, or a caption like a museum label), each word clip-revealed on its whisper timestamp. Hold ≥ max(1.8 s, speech + 0.6 s); assert it in code.
- **Caption rows**: small (~19 px) medium-weight notes with a 1 px rule above; technical notes in a corner (module sizes, counts, a bar counter).

MOTION QUALITY
- **Two curves only**: a precise ease `cubic-bezier(.7,0,.2,1)` for snaps; linear for lines being drawn, playheads and the exception. No overshoot, no spring, no bounce.
- **Durations are note values** at the film's tempo. A snap starts one note value early and **lands on the beat**.
- **Entrances are wipes or clip-reveals, never fades**: words rise out of a clip rect from below the baseline and leave upward; lines grow along their length; paper appears by a wipe; big numerals climb in eased steps ("one module at a time").
- **Deletions are knife cuts**: a thin line crosses the element, then the element collapses to its midline.
- **Ripples are ordered by distance** from their cause, a couple of units per subdivision.
- **The exception moves differently**: linear, unsnapped, perhaps pausing mid-flight; a coordinate readout beside it can turn from "on grid" to "off grid".
- Smooth 24 fps, no stepping: digital precision, not hand-made.

CAMERA GRAMMAR
A vocabulary, not a route. The camera moves only with the precise ease and only on beats. Opening and ending come from the topic.
Framing: everything on the grid; the signal element may sit off-grid only when it is being the exception. Transitions are wipes, knife cuts, camera moves and re-flows; never dissolves.

SOUND PALETTE
- **Music with a grid of its own**: a steady pulse whose subdivisions give snap durations. Options: motorik / krautrock kit and bass, minimal techno, marimba or vibraphone patterns, a mechanical piano ostinato, a drum machine with a glockenspiel; one instrument added per new rule is a clear build.
- **Picture and music can share one data file** (a column → pitch, a row → onset), so a graphic can literally be the score.
- **Techniques (options)**: drop the drums for the one moment of the exception; a near-silent bar before a reveal; thin the band during a reveal so every note is audible.
- **Foley follows the materials of print**, dry and close, no reverb: letterpress clack, ruling pen "tss", guillotine snick, paper slap and slide, numeral thunk, block snap, felt thump for a landing, rubber stamp, typewriter carriage.
- **The exception can be the only element that sings** (a sine glide following its position).
- **Silence**: a hard stop of everything on a downbeat; the next snap reads as an event.
- **Mix**: voice compressed and ≥ 6 dB above music, music ducked ~10 dB under voice, −14 LUFS.
- **Voice**: cool, crisp, rule-like sentences; prefer voices with bright consonants.

NATIVE MOVES
A menu: use the ones your story needs.
- **The grid means something.** Columns and rows become a quantity (notes, hours, floors, years). *Fits content like:* a building's floors and rooms; a week's timetable; a city's districts.
- **Snap = beat.** Every arrival is a percussion hit. *Fits content like:* a product's feature list; the steps of a recipe; a countdown.
- **Programme.** Rules applied one by one to one artifact, then the same programme generating many. *Fits content like:* a brand identity; a typeface family; a set of stamps.
- **Extreme scale contrast.** A giant element as chapter card beside tiny notes. *Fits content like:* a record statistic; a year; a price.
- **The exception.** One element breaks the grid, linear and unsnapped. *Fits content like:* an inventor; an outlier in the data; a rebel employee.
- **Knife cut.** An element deleted by a line. *Fits content like:* budget cuts; editing a text; removing a feature.
- **Re-flow.** Everything else rearranges around a change, nearest first. *Fits content like:* a team reorganised; traffic after a road closes; a layout adapting to a phone.

PITFALLS OF THE MEDIUM
- Too many thin columns look like a chart → fewer, wider modules.
- A "better" composition that isn't obviously better → one strong diagonal with long/short rhythm, a larger signal element, bleeds, type over colour. Merely tilting is not enough.
- Rotating and labelling at once reads as noise → rotate, hold, then add labels.
- Merged bars show seams after rotation → overdraw 1 px.
- Elements that share the camera matrix leak into other shots → draw them only when on screen.
- Canvas `letterSpacing` affects `measureText` → measure after setting the font.
- Onsets that fall between frames flicker → put visual onsets on whole frames.
- Whisper splits or merges words and pads the first word → align by letters, clamp to 0.
```
