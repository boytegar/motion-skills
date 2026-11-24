Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Ukiyo-e" style.

MATERIALS & RENDERING
- **The sheet is the frame**: the print fills the screen at zoom 1, inside a thick + thin ink border, with a paper margin beyond. Sheets can sit on a long mounting scroll (darker tan paper, a roller at one end), so the camera can travel from print to print.
- **Blocks**: draw each colour group into its own offscreen canvas, cached once (key lines; blues; greens / secondary; reds / figures). Composite each with a small fixed registration offset (1.5–2.5 px, a different direction per block).
- **Texture every colour block**: woodgrain (`source-atop`, strongest in sky and water) → baren streaks → goma-zuri specks (both `destination-out`); the key block gets specks only; light paper fibres over the finished sheet so paper shows through the ink. Big dynamic fills get the same textures inside their clip. Woodgrain = iso-lines of a warped noise field; baren = blurred circular arcs + zigzag streaks (recipe and alphas in [DEMO.md](DEMO.md#palette--props)).
- **Bokashi** is generated per pixel: a gradient whose top edge wanders by low-frequency noise per column (±20–30 px). Never strips of `fillRect` (vertical seams).
- **Line**: key lines 2.4–3 px; large contours are variable-width filled strokes, tapered at both ends, with ±20–35 % low-frequency width jitter. Distant things get a grey key line or none.
- **Motifs of the medium**: mist as long rounded flat bands (suyari-gasumi) fading at both ends; pines with flat-topped **needle pads**; fields as high, flat, parallel layers whose cross lines are **parallel and slanted**, never converging; rain as two sets of fine straight lines at slightly different angles under a sumi bokashi sky; water as banded blues with bokashi between bands, foam and spray left as paper.
- **Seal**: characters cut out of red, chipped edges. **Figures** are small in the landscape (roughly 80–130 px at full sheet), flat, with a clear silhouette; faces in a few strokes.

COLOUR LOGIC
- **A pigment set, not a colour wheel**: 6–9 flat hues, each printed from its own block, over the paper ground. No gradients except bokashi, no shading on forms.
- **Sumi key line** over everything; **paper is the white** (no white pigment for foam, snow, mist, blank sky).
- **One dominant hue family** carries sky and water (in the 19th-century prints, Prussian blue); **red is rare**: seals, a sun, one garment or tab.
- Far layers pale and low-contrast, near layers saturated with dark key lines.
- Examples, one per film: an indigo-and-beni landscape (Prussian blue, pale blue, beni red, ochre, greens, sumi); a bijin-ga interior (pale pink, mauve, grey-green, lacquer black, a mica-grey ground); an autumn surimono (persimmon, ochre, grey-green, deep brown, one blue).

TYPE & SUBTITLES
- **Subtitle = a horizontal cartouche**: a cream slip with a thick + thin ink border and a small red seal at the left; Shippori Mincho (500) in sumi, centred near the bottom. It "prints" in with the reveal mask and fades out. Hold ≥ max(1.8 s, speech + 0.6 s). It covers the bottom ~150 px: keep ground lines and figures above it.
- **View cartouche**: vertical cream slip, thick + thin border, a pale tab with the series title, the view title below in a brush font (e.g. Yuji Syuku), a seal beside: the "full stop" of each view. Titles live on the print as cartouches and seals, never as floating type.

MOTION QUALITY
- Prints are still. **Most of the frame does not move**; each view has one or two small actions (steam, rain, a lantern, a falling leaf).
- **Figures and nature step at 8 fps**; rain and fast water at **12 fps**; **the camera moves on ones (24 fps)**. A stepped camera judders.
- **Printing reveal**: each block appears over ~0.3 s with a diagonal baren sweep with a noisy edge, sliding 5–6 px into registration as it lands (pure noise looks like mould).
- Cartouches drop in (fade + a short slide); a seal *stamps* (scale ~1.35 → 1 in 0.2 s).
- A native move needs a **held beat of about a second** to be read; a peak flashed in under a second reads as noise.

CAMERA GRAMMAR
The camera moves *over* prints and between them; it never rotates in 3D space inside one. Opening and ending come from the topic.
Framing: bold cropping and extreme foreground elements (a branch, a post, a lantern crossing the frame) are native; depth by stacking, never by perspective. Allowed transitions: scroll slides, printing reveals, a wash back to paper, a cut to blank. No cross-dissolves, no 3D flips.

SOUND PALETTE
- **Japanese instruments, not piano and pads**: shamisen (`core/audio/pluck.py shamisen`, with sawari buzz), shakuhachi, koto, taiko and shime-daiko, kotsuzumi / ōtsuzumi calls, hyōshigi (wooden clappers), a small bell.
- **Modes**: miyako-bushi (in scale), yō, ritsu; long breathy tones with a slide into each note and *meri* drops at phrase ends suit the shakuhachi; the shamisen can hold a pulse or an ostinato.
- **Techniques, as options**: absolute silence as *ma* (cut reverb tails too); a single sawari note left ringing; free-rhythm breath phrases against a steady pulse; clappers that accelerate (the kabuki curtain call); a taiko heartbeat that tightens into a roll; a koto glissando for a turn of season; a drum call (*kakegoe*) before an entrance.
- **Foley follows the material**: paper (baren rub = band-passed noise with a slow circular modulation; scroll slide; seal = low thump + tacky peel), wood (block set down, hollow steps), glass (wind chime), water, birds and insects of the season.
- **Voice**: calm, few short lines, like a poem written beside the print. Duck music ~8 dB under voice, ambience ~4 dB. Loudness −14 LUFS; `mux.sh` grain **0** (the paper is the grain).

NATIVE MOVES
A menu: use the ones your story needs.
- **A series of views.** One constant subject in different compositions; the story is told by *where it sits in the frame* each time. *Fits content like:* one building through a century; a product in five homes; the same street in four seasons.
- **The handscroll.** Scenes don't cut; the camera slides right to left and the next print arrives. *Fits content like:* a supply chain; the stations of a pilgrimage; the steps of a recipe.
- **Printing, block by block.** Key line first, then each colour lands slightly off and snaps into register. *Fits content like:* a building going up; an idea taking shape; a character being "made" for the story.
- **Paper is the white.** Foam, snow, mist and blank sky are unprinted paper; white can flood the frame and wash the print back to a blank sheet. *Fits content like:* a snowfall that erases a town; forgetting; a clean start.
- **The frame is an object.** Border, cartouche and seal are physical; something big enough breaks out of the border into the mount. *Fits content like:* a festival that can't be contained; a legend; a runaway success.
- **Alternate impressions.** The same key block reprinted with different colour blocks (day → night, spring → winter). *Fits content like:* before and after; a city lit at night; two lives in one house.
- **The triptych.** A scene split across three sheets that join. *Fits content like:* three generations; a panorama of a harbour; a debate between three voices.

PITFALLS OF THE MEDIUM
- Bokashi from `fillRect` strips leaves vertical seams → build it per pixel.
- Too much speckle or fibre makes dark areas look like sandpaper or scratched film → specks α ≈ 0.3, fibres α ≈ 0.09.
- A perspective grid for fields looks digital → parallel, slanted, flat layering.
- Pine clusters drawn as half-discs look like parasols → flat pads with needle fans.
- Offset curves self-intersect at tight bends → build inner lines from the gentle side of the contour; force control points x-monotonic.
- Breaking out of the print must escape **both** the sheet clip and the frame clip.
- A seal drawn with `multiply` vanishes on dark ground → `source-over` there.
```
