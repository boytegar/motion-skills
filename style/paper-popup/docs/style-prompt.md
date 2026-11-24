Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Paper Pop-up Book" style.

MATERIALS & RENDERING
- **Real units** (metres): page 0.3–0.5 m wide, actors 3–5 cm, near plane a few mm, so HDRI, shadows and depth of field behave like a real macro shot.
- **Cut-paper finish**: Canvas2D art with a dark ink outline; its silhouette dilated (drawn at many angles on several rings, recoloured `source-in`) into a grey cardboard edge offset slightly down (thickness) and a wide off-white border; a shared paper grain on top (`source-atop`).
- **Drawing**: one dark-brown ink (never black), thick outline + thinner detail line, round joins; shading = one hard **crescent** (fill with shade, refill shifted with base); wobbly shapes (low-harmonic noise), never perfect geometry; simple faces.
- **Pop-up pieces**: planes with art on the front and plain paper on the back (same alpha); `alphaTest` + `alphaToCoverage`; a custom depth material with the alpha map so **the shadow is the cut-out silhouette**. Anything "3D" is several flat cards (front/back layers, triangles hinged on a keel).
- **The book** is modelled: cloth cover with foil title (roughness/metalness map), page block with edge lines, a curling leaf.
- **Real world**: HDRI environment and softly blurred background, real wood or fabric, a few CC0 props. **Neutral tone mapping** (ACES shifts paper colours).
- **Depth of field by hand**: CoC = aperture × (1/focus − 1/z), spiral gather that keeps far samples off a sharp foreground (stock bokeh looks fake at this scale); then light bloom, vignette, warmth.

COLOUR LOGIC
- Saturated children's-book colour, **never neon**; the real world stays natural and slightly warm so paper pops against it.
- **One dominant hue family per spread**, so a page turn is also a colour change; a few accents (a red, a yellow, a gold) recur across spreads to stitch the book together.
- Ink dark brown, border off-white, card backs plain paper.
- Unfinished or "outside the story" pages lose colour: graphite on cream. Night is a change of the paper art; the real lamp keeps its warmth.
Examples of spread families: green meadow under sky blue; pink-and-lilac market with mint accents; ochre desert with a turquoise oasis.

TYPE & SUBTITLES
- Rounded OFL faces: a rounded sans for subtitles, bubbles, signs (e.g. Fredoka); a chunky display face for cover, banners, action words (e.g. Lilita One); a storybook serif for taglines (e.g. IM Fell English); a playful Chinese face (e.g. ZCOOL KuaiLe) with the Latin face in its stack.
- **Narration subtitles**: bottom centre, ~40 px, cream with a thick dark-brown outline and soft shadow, second language smaller below. Hold ≥ max(1.8 s, speech + 0.6 s).
- **Speech bubbles** live in the paper world: white card, ink outline, tail projected each frame to the speaker; pop in with overshoot, type on with blips.
- **Titles are paper objects**: a cardboard sign dropping on strings and swinging; the cover itself is the film title. Action words (italic display, gradient, double stroke, rays) sparingly, like a game rating.
- Subset Chinese fonts to the characters used; re-subset after every text change.

MOTION QUALITY
- On ones (60 fps suits the smooth macro camera), all a pure function of `t`. Paper moves stiffly; life comes from easing: `back()` overshoot for pop-ups, springs for things on strings, squash and stretch on characters.
- **Pop-ups rise back to front** (delay grows with distance from the reader), far pieces lying back, near ones folding forward face-down; they fold down in the same order.
- Plants and light props sway a few hundredths of a radian around their base.
- **Characters are flat cards redrawn per quantized pose** (walk phase, arms, eyes, mouth, look, blink); walk phase follows distance walked, so feet don't slide.
- **Turning around = a `scale.x` flip** between front and back drawings; never `rotation.y`. Jumps: parabolic arcs, squash at both ends, a paper puff on landing.
- **Page turn**: a many-segment strip whose bend is integrated along its length, so it curls like paper.
- Never moves: the room.

CAMERA GRAMMAR
One virtual macro camera keyed as position / target / fov / aperture / focus with monotone-cubic interpolation; a new key list = a hard cut. A vocabulary, not a route. Opening and ending come from the topic.
Actors in focus, room objects as soft foreground or background; keep near pop-ups out of the lens path. Transitions: page turns, a book closing and reopening, a hard cut on a paper sound; never a video dissolve inside the book.

SOUND PALETTE
- **Paper foley is the signature**: page swish, cardboard thump, pop-up snap (short sine sweep + crinkle), fold and crease, pull-tab slide, footsteps as tiny taps with crinkle, slat flaps, string-drop tinks; the book's spine creak and closing thump. All synthesizable from the page's event list.
- **Character voices as typing blips**: one waveform and pitch range per character, a blip every other letter.
- **Narration**: one warm storyteller, picture-book sentences; pronunciations kept apart from displayed text.
- **Music**: whimsical, acoustic or toy-like: music box, pizzicato strings, clarinet, glockenspiel, ukulele, toy piano, brushes. Options: a warmer cue for anything in the real room; a bar-aligned internal edit of a library track so its ending lands on a picture event; a pull-tab's clicks setting a cue's pulse.
- **Ambience**: room tone and a clock for the room; one bed per spread for the paper world.
- **Silence** = the book held still with only room tone; before a turn that matters, or a hesitation.
- **Mix**: voice high-passed, compressed, well above music; music ducked under speech with smooth release; limit SFX per channel; −14 LUFS.

NATIVE MOVES
A menu: use the ones your story needs.
- **The book opens**: creak, the board swings up, the world pops up back to front. *Fits content like:* a project launch; a first school day; a recipe book.
- **Page turn = scene change**: old pieces fold down as new ones spring up. *Fits content like:* stages of a process; a farm's seasons; chapters of a biography.
- **Pull-tab mechanisms**: flipping slats, a sun on a stick, sliding strips, a hinged door, pieces on strings. *Fits content like:* a forecast; old way/new way; a lock opening.
- **Paper as character**: crumpled, smoothed, refolded, torn, taped. *Fits content like:* a mistake becoming a lesson; recycling; a rejected draft becoming the design.
- **The edge of the page**: crossing between paper and the room, either way. *Fits content like:* a character stepping into a phone on the desk; a real object entering the story; a drawing coming true.
- **The real object plays a part**: a lamp as a sun, a mug as a tower. *Fits content like:* a product at home; the reader's presence; real clock time.
- **Typed bubbles and ratings**. *Fits content like:* a tutorial; a quiz; a small victory.

PITFALLS OF THE MEDIUM
- `rotation.y` shows a card edge-on, then its blank back → `scale.x` flip. Negative scale also swaps front/back materials → draw mirrored props instead.
- A one-plane container hides its contents → back card + front card, contents between.
- Near-lens foreground pop-ups fill the frame with blur → move them or close the aperture.
- Rectangle shadows kill the illusion → alpha-mapped depth material on every card; tight shadow bias and a big shadow map against acne on thin cards.
- A turning page goes black facing away from lights → emissive map = colour map.
- ACES shifts paper colours → Neutral.
- Headless Chrome falls back to software WebGL → GPU ANGLE flags; ES modules need HTTP.
- Redrawing character canvases each frame → redraw only on pose change.
- Chinese subsets miss new characters and Latin glyphs → re-subset; Latin face in the stack.
```
