Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Scratchboard Long Take" style.

MATERIALS & RENDERING
- **Render model:** three.js scene at 2× supersampling, box-filtered down in a final print pass. Every material is a small shader that decides "ink or paper" per pixel; there is no lighting except one direction used to choose lit and shadow faces.
- **Two kinds of field.** Every surface is either a paper field carrying ink cuts (lit faces, snow, sand, cloud, foam, steam) or an ink field carrying paper cuts (shadow faces, sky, water, forest, night). A cloud is paper with a few horizontal ink cuts on its underside, not a reversed sky as densely cut. Tone inside a field comes only from the width and density of the cuts.
- **Faceted terrain:** mountains and volcanoes are low-poly cones with flat facets. Lit facets are paper with a few fine ink cuts near the terminator; shadow facets are ink with paper cuts running down the slope. Cut spacing is fixed in world units so lines never swim when the camera moves.
- **Sky in direction space:** a sphere that travels with the camera; cuts are rows of constant elevation, so they stay horizontal and never slide with translation. Night = sparse wavy wind cuts and square stab stars; day = dense parallel cuts, thicker near the horizon. Words in the sky are plates hung in the same direction space.
- **Billboard atlases drawn on canvas:** trees (pines, frosted deciduous trees), animals, steam. Cylindrical billboards for anything standing, spherical with a screen rotation for anything flying. Sample the atlas with a negative mip bias and alpha-test so thin shapes survive at distance.
- **Water** is a ribbon cut into the snow (the ground is discarded inside a river mask), with bank walls and a paper snow lip. Ripples are short cuts across the flow, faded to solid ink where they would turn grey with distance.
- **Print pass (screen space):** paper fibre, ink mottling in the darks, soft grey blotches near the top of the sheet, a two-frame flicker, a light vignette. Nothing else lives in screen space except transitions made of paper (a whiteout, a sheet being peeled).
- **Props with words** (plates, boards, notices) are canvases on meshes: carved letters in paper on an ink ground, or ink on a pasted paper label. They sit at a 3/4 angle with thickness, a snow cap or a rim.

COLOUR LOGIC
- Two values carry everything: ink (a warm black) and paper (a warm off-white). Never pure black or pure white, never a third grey fill.
- **One red, reserved for one subject**, kept small (a crown, a lamp, a flag) and never smaller than about 3 px at 1080p; enlarge it a little rather than lose it. It may carry into the subject's own trace (its ripple, its trail), never onto another object. A flat stab: no outline, highlight or gradient, a thin paper ring only where it sits on ink. When the take follows the red through steam or a whiteout, the red stays on top of the paper.
- A warm yellow is allowed for light itself (sun disc, its road on water, warm cuts in a dawn sky, a lamp) and one material at most. It never fills a large area except that one material.
- Value order by depth: far mountains paper with ink shadow facets; forest bands solid ink; the middle ground paper; foreground ink silhouettes. Keep the subject on the opposite value of what lies behind it.
- Example palette: ink `#16171b`, paper `#ede5cd`, red `#c4332b`, yellow `#dfa443`.

TYPE & SUBTITLES
- One serif typeface, bold (e.g. Noto Serif SC, OFL), subset to the characters used. Chinese and English each on their own line.
- **Type lives in the world:** carved into a board, cast on a plate, printed on a pasted notice, or cut as a plate of type into the sky. Words in the world carry only keywords and numbers; no centred title cards, no headers.
- Sizes at 1080p: at most three sizes per frame; nothing smaller than 28 px for Chinese or 22 px for Latin; every word at least 5% inside the frame. Left-aligned blocks, aligned by optical edge.
- Words appear the way the medium makes them: cut left to right with chips at the knife, pressed by a seal, revealed under a peeled sheet.
- Reading time: hold every piece ≥ max(1.8 s, characters ÷ 4.5 for CJK + characters ÷ 15 for Latin + 1.5 s); judged comparisons hold still ≥ 2.5 s before the verdict lands.
### Narration & subtitles
When the film tells a story with a narrator:
- **Speech and picture move together.** Each line is spoken while the shot shows what it names; the long take keeps going under the voice, and the silent stretches (a hold to read, a pass through a structure, a flock taking off) are left without words.
- **Words in the world carry only keywords and numbers** (a count on a board, a date on a seal). The sentence is in the voice and the subtitle, not on the props.
- **One subtitle bar per spoken line**, at the bottom inside the 5% safe area: one Chinese line above one English line, never wrapped, in the film's serif, paper letters on an ink band about 80% opaque (thinner bands vanish over paper). It appears with the line and leaves with it; keep the subject of each shot above it.
- A second-language version swaps only the voice: the same picture, the same bilingual bar, each line starting at the same moment, finished before the next one starts.

MOTION QUALITY
- **Output 30 fps** (`--fps 30` for `video.mjs` and `mux.sh`). The camera and the world move on every output frame. Separately, the print pass steps its paper flicker and blotches at 12 Hz inside the shader (`floor(t*12)/12`), whatever the output rate, so the image breathes like a worn sheet.
- Easing: long monotone curves; the camera keeps momentum through key positions and decelerates into every hold. No bounce, no shake except one frame for an impact.
- Animals are flat silhouettes with three or four poses per action (sleep, wake, call, peck, walk, run, three wing frames, land, leap, bow). Wings cycle on fixed frames; never morph between poses.
- Steam, snow and splashes are paper marks: horizontal cut wisps, square stabs, short arcs.
- Mountains, horizon and sky texture never move; trees never sway.

CAMERA GRAMMAR
Framing: the horizon sits on a third; the subject is never smaller than 1/20 of the frame height when it must be recognised; words stay inside the frame with a margin of 5% and never cross another object. Allowed transitions: through a structure, through steam or snow (under half a second of white paper), a match on a falling or rising subject, a sheet peeled from the corner, a seal. Forbidden: fades to black, cross-dissolves, slides, page flips, zoom cuts.
Fitting the high wide to the terrain (options, not a recipe). The sky is in frame only while the camera's pitch plus half its vertical field of view stays above the horizon, so a steep look down at a valley right below loses the sky. Ways round it: stand the camera farther back and higher on a longer lens, so the subject sits at a shallower angle below the horizon; take the printmaker's licence and stretch horizontal distances or compress heights, since each layer is a plane; break a far plane that fills the frame (a sea, a plain) into bands with headlands, islands, boats or tiers of cut density. When the sky is out of frame and words are needed, carve them into something in the world that faces the lens (rock, board, hull, a mark on the water), let the take tilt up into a hold where the sky returns, or keep the words for a later shot where the sky is in frame.

SOUND PALETTE
- Three layers. **The world:** the matter of the place (water under ice, frost cracking, wind in three weights, wings, steps on snow, small animal sounds). **The print:** knife through paper or wood, a block's thump, a sheet torn away, a seal pressed; used only on transitions and verdicts, never as a bed. **Music:** a small chamber group of real samples.
- Timbres that suit the medium: bowed vibraphone, harp, horn, low strings, pizzicato bass, xylophone, glockenspiel, timpani. Avoid synth pads and drum kits.
- A three-beat or slow pulse that can stop; silence before a verdict, then one heavy low stroke.
- A narrator, when there is one, sits on top: music ducks about 17 dB under speech and comes back up in the silent stretches; foley ducks about 7 dB.
- Animal calls are recordings with a CC0 or CC BY licence or are synthesised; never CC BY-NC.
- Mix: music sits under the world; the print sounds are the loudest events. Integrated −16 LUFS in the master, true peak under −3 dBTP (the final mux brings it to −14).

NATIVE MOVES
- **The sky carved open.** The sky is cut from the horizon up, one row at a time, the newest row still running across; dawn as a carving, not a gradient. *Fits content like:* a factory's first shift; the first day after a war; a city's lights coming on in reverse at dawn.
- **One red waking.** In a field of ink and paper, the single red appears point by point (heads lifting, windows lighting, flags raised) across the frame. *Fits content like:* poppies opening on a hillside; lanterns along a river; the first lit windows of a rebuilt street.
- **Through the steam.** The take passes through a cloud of paper cuts that eats the frame for under half a second; the world on the other side is a different scale or place. *Fits content like:* a ship into fog and out at a harbour; a train through a tunnel's smoke; a balloon through cloud.
- **The same place printed again.** A locked frame of one place; the top sheet peels from the corner to show the next year's print beneath, with ink that has not quite taken. *Fits content like:* a coastline retreating; a forest replanted; a village growing into a town.
- **Words cut into the world.** A count or a date carved into a board, cast in a plate or stamped by a seal while the camera holds. *Fits content like:* a flood mark on a wall; a ship's name on the hull; a record on a mountain hut.
- **A plate of type in the sky.** The title or the sign-off is cut into the sky as an ink block with paper letters, part of the same direction-space sky. *Fits content like:* a title over a skyline; chapter names over a sea; a dedication over a valley.

PITFALLS OF THE MEDIUM
- **Close-ups expose the flat shading.** 3D props pushed close to the lens look like toys. Keep living subjects to mid shot as flat silhouettes; give close props real materials (carved wood, concrete formwork, cast iron).
- **Empty paper.** A field of snow without layers goes blank. Keep three layers in frame; add drift strokes, reed ticks, thickets, belts.
- **Grey sludge.** Fine cuts at distance average to grey; fade them to solid ink or paper with distance and use negative mip bias for thin sprites.
- **Sliding lines.** Any cut spaced in screen space swims as the camera moves; anchor cuts to world space (or direction space for the sky).
- **The road look.** A dark river with a pale bank lip and regular dashes reads as asphalt. Break the rhythm of bank marks and keep the lip narrow.
- **Small subjects vanish.** Real-scale animals disappear in a wide frame; draw distant ones up to 3× larger (a printmaker's scale), keep the red at its minimum size.
- **Camera through solids.** Clear trees and posts from the camera path before placing scenery; check every low pass frame by frame. The same holds for every small scatter (reeds, stones, fence posts, grass): one within a metre or two of the lens fills the frame with ink. Sample the camera path and clear every instance inside a radius of it, give near sprites a dithered fade, then step stills through the low passes (`still.mjs --range a:b:0.1` and a contact sheet) and look for frames that are nearly all ink or all paper.
- **Empty paper close up.** A lit facet near the lens becomes a blank sheet. Make facets smaller near the camera path and give near lit faces their few ink cuts, grit or tufts.
- **Words on a moving take.** `readcheck` counts only the first stretch in which a piece is fully in frame, and knows nothing of occlusion or size. Report each piece in `TEXTS(t)` only inside its planned hold, when it is in frame, unobstructed and large enough to read; ease the camera into a hold or slow the clock for it.
- **Too many greys from post.** Vignette and blotches darken the paper and make the print look dirty; keep them light.
```
