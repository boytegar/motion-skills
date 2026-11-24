Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Spreadsheet Film" style.

MATERIALS & RENDERING
- **Canvas 2D, one world → screen function.** The sheet is a grid in world units (column widths × row height); the camera is a centre and a zoom. Everything that belongs to the sheet (fills, values, merged cells, reference boxes, the selection) goes through that function; the shell is drawn in screen space on top.
- **Variable column widths** are part of the model: a hidden column has width 0 and every x position is a prefix sum of widths, so opening or closing columns reflows the whole sheet and the camera rides along.
- **Grid lines**: white cells get light grey lines (≈ `#dad9d3`); painted cells get thin dark lines (≈ rgba(6,4,18,0.42)), as if "all borders" were applied.
- **Cell values** at 11 px × zoom; below about 6.5 px they are not drawn, so the far view becomes pure colour instead of grey noise.
- **A painted cell flashes white** for ~0.12 s when its rule fires. That flash is the style's only "light".
- **Merged cells are the layout**: labels, results, titles and the end card are merged ranges with a cell border, never floating text.
- Nothing outside what a sheet can make: no gradients, glows, shadows, blur, depth of field or translucent panels. Text has no outline or drop shadow.
- **The sheet cannot scroll above row 1 or left of column A**: clamp the camera, and compose around that edge.

COLOUR LOGIC
- **The sheet is white; colour means a rule fired.** Every fill on a cell is a conditional format with a meaning the viewer can learn in one look (success / failure, a category, a scale step).
- **At most five working hues** plus the shell's warm greys: two for the main binary or categories, one for a second variable, one for the hero (the selected cell, a winning result), and ink. Office-style conditional-format colours (lime, red, indigo, amber, sky) read as "spreadsheet" at once.
- **A colour scale is a payoff, not wallpaper.** A banded scale (6–8 steps radiating from one cell, neighbouring bands alternating light and dark) can paint the empty background once the argument is made; until then the background stays white.
- Reference boxes take their colour from the formula: each referenced name in the formula bar and the box around its range share one hue.
- Example palette (the demo): lime `#b8f25a`, red `#e8382d`, indigo `#2a2f6c`, amber `#ffb21f`, ink `#17120a`, shell `#dcdbd5` / `#f1f0eb`.

TYPE & SUBTITLES
- **One grotesque for the whole interface** (Inter, OFL) and a matching CJK sans (Noto Sans SC, OFL), self-hosted as subsets.
- Roles: cell values 11 px × zoom (texture), merged-cell labels 14–26 px × zoom, results 26–32 px × zoom (they pop 30 % on Enter), formula bar 26 px fixed (the line of the film), shell 12–13 px fixed. On-screen Chinese ≥ 34 px at the zoom it is read at.
- **Subtitles are a cell comment**: a pale yellow note with an ink border and a red corner triangle, docked at the lower left, one Chinese line over one English line, never wrapped. Frame the subject above it (the camera's centre sits about 48 px above the viewport's centre).
- Formulas use the real syntax of a spreadsheet (`=AVERAGE(range)`, `=COUNTIF(range, "L")`), names for ranges, and colour the referenced names.
- Hold every subtitle ≥ max(1.8 s, speech + 0.6 s); a result that a sentence depends on stays in frame until the sentence has used it.

MOTION QUALITY
- 24 fps; the camera (zoom, scroll) is smooth every frame. Cells themselves move **on the grid**: positions snap to whole cells, so a moving cell hops.
- Paint happens in **rings or sweeps from one cell**, a few tens of milliseconds per cell, with the status bar counting cells.
- Typing is even within a line, about 14–30 characters per second: the longer the formula, the faster, so that one formula is typed in about 0.8–1.4 s; the caret blinks at 4 Hz. Enter is a hard cut of state: the edit box closes, the value pops.
- Column-width changes ease in and out (~0.5 s per column) and ripple across neighbouring columns.
- The shell never moves; only its state changes (pressed buttons, the name box, the zoom slider, status messages).
- **Frame rate and grain**: no film grain (mux grain 0).

CAMERA GRAMMAR
The camera is the sheet's zoom and scroll. The column header grows a little with zoom (26 → ~38 px), as real software does.
Framing: a judged comparison is wholly in frame and held ≥ 2.5 s; at key moments the subject fills ≥ 1/3 of the viewport height; keep key text above the subtitle note. Transitions are made by the sheet (a hop, a zoom, a column opening, cells flying, a colour scale spreading, a merged cell expanding); no cross-fades, wipes or page turns.

SOUND PALETTE
- **The sheet's foley**: key clicks for every typed character, a clack + low thump on Enter, a short tick per hop of the selected cell, one tiny sine blip per painted cell (pitch rising with progress), a ratchet click per opened column, a soft blip per landing cell (pitch by height), mouse clicks, a small pop for menus and results, a ding for a winning value, air whooshes on big zooms. Drive all of it from the page's event list so each sound sits on its frame.
- **Music**: office-pop instruments (kit, electric bass, piano stabs, marimba, xylophone, glockenspiel). Accelerate by thickening the pattern (half-time → four on the floor, sixteenth mallets), not by changing tempo.
- **Silence** is a real cut of the music (foley may stay) right before a clue or a turn; the first sound after it should be a key moment.
- Mix: voice compressed and about 10 dB over the music by RMS, music carved 300 Hz–4 kHz under speech, −14 LUFS, true peak ≤ −1 dB. Voice: a calm explainer.

NATIVE MOVES
- **Pull out from a formula cell.** Type in one cell, then zoom out until the sheet is a picture made of values. *Fits content like:* a budget that becomes a portrait of a city; survey answers that become a map; one sale among a year of sales.
- **Conditional format sweep.** A rule fires and paints outward from one cell; the status bar counts. *Fits content like:* turning raw numbers into a heat map; flagging outliers; a season of temperatures.
- **Hidden columns.** Letters skip in the header and a double line marks the fold; "Unhide" opens the columns, which show `#`, `##`, then values while they widen. *Fits content like:* a confounder; a missing cost; the fine print of a contract.
- **Cells that move.** Cells lift and hop to new positions (rows first, then columns) to sort, filter or pivot; bars of cells grow as they land. *Fits content like:* a ranking that reshuffles; a cohort split by age; votes regrouped by region.
- **A count drawn as cells.** A COUNTIF or SUM shown as a row of cells filled in proportion, not just a number. *Fits content like:* market share; a quota; "3 of 10 households".
- **Reference boxes as pointers.** A formula's names colour boxes around ranges anywhere in the sheet. *Fits content like:* a weighted average; a dependency between two tables; where a total comes from.
- **Merged cell as stage.** A range merges and expands into a title or end card; typing fills it. *Fits content like:* chapter titles; a verdict; a credit.
- **`####` gag.** A column too narrow for its number shows hashes; widening it reveals the value. *Fits content like:* a number too big to believe; a surprise price; a hidden score.
Our demo opens on a *pull out from a formula cell* into a *conditional format sweep*, uses *hidden columns* as its turn and *cells that move* as its climax, and ends on a colour scale flooding the sheet and a *merged cell as stage*. A new film opens and ends another way: an error value (`#DIV/0!`) filling the screen, an extreme far zoom on a finished picture pushing into one cell, a *`####` gag* as the turn, an ending where the sheet is filtered down to one row, or ctrl-Z rewinding the whole film to a blank sheet.

PITFALLS OF THE MEDIUM
- **Grey mush when zoomed out**: values drawn at 3–5 px turn the picture to noise. Stop drawing values below ~6.5 px.
- **Text unreadable at the zoom it is shown**: size merged-cell text for the zoom of its shot; a number the voice uses must be ≥ 26 px on screen.
- **Wide-and-short compositions**: a sheet that is wider than tall forces a low zoom; add a meaningful row of cells (a count, a total) below, or crop to the part being discussed.
- **Selection box drawn over the value**: a thick border on a small cell hides its content; select merged ranges for results, single cells only when zoomed in.
- **Brand look-alike**: a green ribbon, a familiar grid font, a product name; keep the shell generic and drawn by hand.
- **Moving cells smearing**: interpolated sub-cell positions read as sliding stickers; snap to whole cells.
- **Too many formulas**: one formula per beat; a formula bar that changes every second is noise.
```
