Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Under the Microscope" style.

MATERIALS & RENDERING
- **Plates**: the WSI pyramid levels as plates in one world frame (level-0 pixels), the coarse levels whole and alpha-feathered at the scan edge, the fine levels as crops around where the camera lands. Each frame draws the finest plate that is downsampled no more than ~1.45×, and blends the next level in between 2.2× and 1.45×: no seams and no visible upsampling, because every level is the same sample, already registered.
- **The naked-eye plate**: the scanner's own photo of the whole slide (many WSI files carry one), registered through the scan's area-of-interest metadata and checked against the tissue's centroid. Cut the glass out of the scanner tray, remove the line-sensor streaks, and fill any manufacturer's printed logo with the surrounding frosted glass. Below ~30 mm of field the photo would be upsampled: let the cover glass become clean glass (its dust leaves the focal plane).
- **One grade for every plate**: glass normalised to a warm white, a little density and chroma in the stain, nothing else. No false colour, no film emulation.
- **The bench**: graphite under a slow pool of light; a soft shadow under the slide.
- **Focus**: screen-space Gaussian blur on a frame rendered with a margin (so edges never go dark); the focal sweep is a tilted sharp front crossing a softened frame. A light illumination vignette, nothing more.
- **Marks** in screen space but anchored to the specimen through one world → screen function: hairlines 1.5–2.5 px with a dark or light halo, circles drawn on as arcs with cardinal ticks, dimension lines with end ticks, leader lines with one elbow. Counted pixels are shown as a mask revealed behind a scan front, then kept as a faint trace.
- **Frosted panels** (dark translucent glass over a blurred copy of the frame) for readings, instruments, subtitles and the credit: white type reads on glass and stain alike, nothing switches colour mid-shot.

COLOUR LOGIC
- Glass: warm paper white. Bench: graphite. The stain is evidence and keeps its true colour (e.g. trichrome red and blue, H&E pink and purple).
- One accent, cool and away from the stain's hues (mint against red and blue; amber against a green or purple stain), only for the measured reading and the counted pixels.
- Ink on glass is near-black; type on stain sits on a frosted panel in warm white.
Example (our demo): glass #F4F1EC, bench #0B0D0F → #2A3036, ink #181B1F, panel rgba(12,14,17,0.74), accent mint rgb(120,240,205) (on stain) / rgb(0,122,99) (on glass).

TYPE & SUBTITLES
- **Roles**: a light condensed grotesk for readings and the scale bar (40–72 px, e.g. Barlow Condensed); a monospace for instruments and pixel counts (20 px, e.g. IBM Plex Mono); a sans in the film's language for labels (20–34 px, e.g. Noto Sans SC); a serif only for the title. OFL, self-hosted, CJK subset.
- **Instruments** live in one frosted panel in a corner: scale bar with its value, field of view, magnification with a small nosepiece glyph that turns a quarter on every change. The source credit in a small panel in the opposite corner.
- **Readings** sit in a frosted panel beside what they measure, joined by a leader: a small label above ("collagen share of tissue area"), the number large in the accent, the method or the raw counts below.
- **Subtitles**: a frosted strip at the bottom; if the film is bilingual, one line per language with the film's language on top, never wrapped. Measure each line with the real font and split long lines into two cues.
- Nothing below 20 px at 1080p. Hold labels ≥ characters ÷ 4.5 + 1.5 s (CJK); a reading that rolls is reported by its landing value.

MOTION QUALITY
- The camera is a similarity transform (zoom, roll, position) interpolated as a log spiral between keys (M^u about the fixed point of the two frames), so every push is a true dive towards one point and the roll is part of it. Ease the moves, creep 2–5 % during holds; never a frozen frame longer than a second.
- Magnification changes happen where the plates actually change, each with a breath of focus, a turret click and the new magnification said once, large.
- Marks arrive by being made: a dimension line grows from its centre, a circle is drawn as an arc, a scan front counts pixels while the reading rolls, the reading lands with a small spring.
- Never moves: the specimen under its marks, the instrument positions.

CAMERA GRAMMAR
At key moments the measured structure fills a third of the frame height or more; instruments in a corner, subtitles bottom centre, readings beside their marks. Transitions are scale changes, trucks and focus, never cuts or crossfades between samples.

SOUND PALETTE
- **Instrument foley**: the objective turret (a click, a detent, a small metal ring), a focus servo, pen on glass for each line drawn, a band-passed scan travelling left to right with counter ticks that speed up, a soft filtered sweep for the focal plane.
- **Music**: hard-mallet vibraphone in 8ths and 16ths, marimba ticks, bowed vibraphone and water glasses for pads and the motif, glockenspiel for each reading that lands, a pulse bass with 2nd/3rd harmonics, timpani and bass drum under the landings. Harmony stays open and light; the score is the user's to shape.
- **Beds**: a quiet lab room (fan, a little mains hum).
- **Silence** before the verdict and before the climb; the first sound after it is the reading or the turn.
- **Mix**: voice ~10 dB over music, music ducked under it; −14 LUFS. Voice: a calm, exact narrator; numbers spelled out for the TTS, digits on screen.

NATIVE MOVES
- **The dive.** From the slide in the hand to a cell in one camera, the turret clicking at each real change of level. *Fits content like:* a biopsy to its cells; a coated surface to its crystals; a hair or skin section to its layers.
- **Counting the colour.** Draw a circle, let a scan front cross it, light up every counted pixel and roll the share up to its landing value; show the raw counts underneath. *Fits content like:* collagen in tissue; porosity in a ceramic; coverage of a coating; pigment in a sample.
- **Two circles, one verdict.** Measure one place, truck to another at the same scale, then frame both and say what differs. *Fits content like:* treated against untreated; edge against centre; before against after.
- **The focal sweep and the ring.** Land, sweep the focal plane across the frame, ring one structure and give its size. *Fits content like:* a nucleus; a grain; a fibre's diameter.
- **The climb back.** Pull out to the eye's scale with the scale bar jumping backwards; a title card written on the glass.

PITFALLS OF THE MEDIUM
- **Licences**: most microscopy on the web is CC BY-NC or all rights reserved. Usable: CC0 whole-slide scans (e.g. the OpenSlide test data: check each file's licence line), public-domain government micrographs (NIH, CDC PHIL, NASA, USGS), CC BY collections (Cell Image Library, Wellcome Collection), with every file listed in `CREDITS`. Not usable: CC BY-SA without agreement, NC images, journal figures, AI images.
- **Finding material for a new topic**: search by organ, stain or material plus "whole slide" in the OpenSlide test data, NCI Imaging Data Commons (TCGA/CPTAC slides, licence per collection), the Cell Image Library, Wellcome Collection, Wikimedia Commons (filter by licence) and NIH/CDC galleries; prefer a file whose metadata carries its pixel size. CC BY-SA, ODbL or "no known restrictions" are not CC0: name the terms to the user and ask before using them. For a product, a lab or a brand, ask the user for their own scans (SVS/NDPI/MRXS, SEM TIFFs with a scale) first. Never stand in a generated image for a real specimen.
- **Different samples never zoom into each other**: two micrographs of "the same thing" at two scales are two specimens. Use one pyramid; if you must join sources, cut on a focus breath and say so.
- **Pixel size**: read it from the file (XMP `ScanRes`, TIFF resolution, OME `PhysicalSizeX`); a slide-photo's resolution tag can be trusted only after checking against the scan.
- **Upsampling** shows as mush beyond ~1.6×: end the dive at the scan's real resolution.
- **Tile seams**: some scanners overlap tiles; check the joint info, frame out visible seams.
- **Labels and logos** on the slide: barcodes and IDs are part of the object; manufacturer logos are not — fill or crop them.
- **Claims**: a stain's colour means what the stain protocol says, nothing more; measured shares are of the drawn region and the stated threshold, never a clinical score.
```
