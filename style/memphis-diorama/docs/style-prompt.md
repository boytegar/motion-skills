Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Memphis Diorama" style.

MATERIALS & RENDERING
- **Three tones only**: the top face is the brightest, sun-facing sides keep the base colour, the faces turned away drop to about ×0.5–0.95. Hard edges between faces, never smooth shading.
- **Hard shadows**, never blurred, tinted cool purple (shadow ≈ base × (0.70, 0.58, 0.90)); the sun direction never changes within a film.
- **Halftone in the shade**: inside shadows, a staggered grid of lighter dots in world space, like a print. It turns dark areas into texture instead of dirt.
- **Outline as a post pass** from normals and depth: about 2 px deep indigo `#25214A` at 1080p. Light cones, decals, particles (rain, snow, petals, smoke) and thin stems carry no outline; a thin object that is all outline reads as an ink scribble.
- **Patterns belong to the world**: dots and stripes are computed from world coordinates and stay on the object as it moves. Pattern scale grows with distance so far surfaces do not buzz.
- **Detail is built, not textured**: a house is a plinth, plank-striped walls, white corner trims, two roof slabs and a ridge, a gable with a round window, framed windows with shutters and flower boxes, a door with a step, awning and lamp. A tree is a trunk with a branch and a cluster of lobes, plus lighter lobes on the sun side. Water is layered: bank, shallows, a deeper channel, white foam lines at the edges, small white squiggles drifting with the current.
- **Screen space holds only three things**: the grainy frame, the bottom type, explainer callouts.
- **Never**: depth of field, real reflections, bloom, motion blur, volumetric light. A lit lamp is a flat warm colour plus a hard-edged pool of warm light on the ground.
- **Hero objects do not receive shadows** when they pass through shade, or they turn dirty.

COLOUR LOGIC
- The ground outside the frame is always warm white `#FFFCF6`; nothing else is drawn there.
- **5–7 saturated hues + 2–3 pastels** carry the picture. Ink `#25214A` is only for outlines, type, tyres and window frames.
- **Cluster colour by area**: neighbouring blocks differ in hue; one colour's share is controlled (a field type at about a third of the ground, not sprinkled at random). Leave large calm areas of one colour (water, a field, a square) so the picture can breathe.
- **Day is bright and warm.** When it gets dark, darken by the scene: a night scene may be dark, but never grey all over.
- **Night is a luminance shift toward violet**, not a general darkening, so yellows don't go olive; every light is the same warm yellow (about `#FFD36E`). **Dusk** adds an orange-pink layer (about `#FF9E7A`) on the way to night.
- **Time and season change the large areas only** (ground, crowns, roofs, water), never the whole palette. Each changing material carries one colour per state; a wave (§8) swaps them.
- **Night and day by area is also a wave.** When the dark falls or lifts street by street (dawn spreading from one landmark, a district losing power), night and day are two states like two seasons: each changing material carries its night colour (the violet shift above) and its day colour, and the ring swaps them. Lights belong to the night state: a lit window or lamp head is warm yellow on the night side and plain glass or metal on the day side; a lamp's pool is a flat disc on the ground, warm on the night side and the ground's own colour on the day side, so it goes out exactly as the ring passes. Keep the rest of the picture's light unchanged during such a wave: a global darkening on top of it turns the day side grey.
- Winter does not mean all white: crowns go to pastel lilac, ice blue and mint with white dots, paths stay swept, houses keep their colours.
- Example palette (an example only): yellow `#FFD234`, pink `#FF7EB3`, sky `#5CC8FF`, mint `#5EDFB0`, lilac `#B9A0FF`, peach `#FF9C6B`, cobalt `#2F5BFF`, coral `#FF5A5F`, green `#2FCB7A`.

TYPE & SUBTITLES
- **Three centred lines under the frame**: line 1, the title, Chinese and English, weight 600, about 25 px, dark; line 2, one sentence, weight 400, about 18 px, grey; line 3, small data (time, solar term, place, coordinates) in a monospace, about 15 px, light grey. Line 3 may change with the picture.
- **Line 3 is optional, and so is line 2.** An empty line leaves its place empty; the others never move up to fill it. With a voice, or when the film's words are bilingual, the type is **one Chinese line and one English line**: line 1 in Chinese (weight 600, about 25 px, dark) and line 2 in English (weight 400, about 18 px, grey), each on one line, never wrapped; line 3 stays empty unless there is small data to show. In this layout both lines change together, as one pair.
- The lines fade in and out with the frame and never move. When a line changes, the old one fades away and the new one fades in; line 1 stays (in the bilingual layout, both lines cross-fade, about 0.4 s out and 0.4 s in). The three marks stay at full strength through every change: only the words fade.
- Typefaces: Inter Tight for Latin, Noto Sans SC for Chinese, IBM Plex Mono for data (all OFL).
- Usually no voice: the type is the narration. With a voice, use plain bottom subtitles, one Chinese and one English line, no styled caption boxes.
- Hold every line ≥ max(1.8 s, reading time + 0.6 s).
- **Callouts** for explainers: a rounded pill with a leader line and a dot, Chinese main and English small, popping out of the model with a back-ease and fading when done.

MOTION QUALITY
- **60 fps, every frame moves**; no stepping, no jitter. The frame's grain boils at 12 fps, which is the only thing on twos.
- **Shapes may be simple; motion may not.** Everything eases. Vehicles accelerate, brake and nod; turns follow smooth curves, never polylines.
- All natural motion is procedural and looping: water, trees, cloud shadows, leaves, snow, smoke, flocks. Every object has its own phase; nothing marches in step.
- **People are distant decoration**: small, walking a cycle, flying a kite, rowing. No close-ups, no acting, no hand business.
- Things never pop into being: they grow from nothing or bounce in with a back-ease; a change of state is announced by an object **squashing down and springing back** (anticipation, then overshoot).
- What never moves: the frame's position, the type's position, the paper, the sun direction.

CAMERA GRAMMAR
A vocabulary, not a route. Opening and ending come from the topic.
- **Prefer one continuous take**, slow enough that the viewer can stop and look at any corner. When a long take must hold a composition, ease the camera speed down instead of letting the place drift out of frame.
- The frame is about 1520 × 744 px, a little above centre; the type sits under it and never covers the subject.
- **Transitions**: only three — the frame draws in and grows out again; a wave sweeps across (colour, light, season); the ground is cut open (the Cutaway of §8: the same move, whether the cut goes into the ground, a building or water). No white flash, spin or fast hard cut.
- The camera never passes through geometry; to get around something, move in two steps (rise, then travel).

SOUND PALETTE
- **Warm, gentle and continuous**: piano, Rhodes-like electric piano, marimba, celesta, music box, soft synth bass, a warm pad, brushes and shaker. A quiet stretch can be piano and pad alone.
- **One piece, not a medley**: when the place changes (a season, day to night), keep the same harmony and tempo and change only how thick the accompaniment is. A section that stops and a new one that starts sounds broken.
- **Foley is the sound of small things**, kept under the music: birds, ducks, water, a bicycle bell, raindrops, leaves, geese, wind, snow, the click or *ting* of a light coming on. Tuned foley (a light coming on) uses notes of the current chord or its pentatonic scale.
- **The same event always makes the same sound**: if every change of state starts with a squash, every squash gets the same thump, spring and rising harp.
- Silence is allowed, but keep a faint room tone under it.
- Loudness −14 LUFS, true peak ≤ −1 dBTP; fades follow the frame growing open and drawing in. No voice by default.

NATIVE MOVES
A menu: use the ones your story needs.
- **Wave.** A ring spreads from one point; everything it passes takes its next state (colours swap per pixel inside the radius; objects grow, open, fall or light up as the ring reaches them). *Fits content like:* night falling street by street; a change of season; a delivery or signal coverage map; power coming back after an outage. Night falling or day breaking by area follows §3 (night and day as two states); how to build it is in §10.
- **Squash and spring.** One landmark squats, springs back, and the change starts from it. Use it to say "this causes that" without a person. *Fits content like:* a fountain that starts the morning; a factory chimney that starts a shift; a station clock that starts the rush hour.
- **Cutaway.** Cut along a line; the near part sinks and slides away to show the section underground, inside a wall or under water. *Fits content like:* how a metro and its pipes work; a tree's roots; what each floor of a building is doing; fish under a pond. What is revealed is built, not implied: floors with their rooms and furniture, soil in layers, pipes and tunnels, water with what lives in it; every cut face is drawn as a section (flat colour with section hatching), never left hollow. The near part leaves in two steps (a lid rises, then slides back; a wall or a slab of ground sinks straight down), so nothing passes through anything, and it comes back the same way in reverse. The engine has no cutaway built in: §10 says how to make one.
- **Grow the frame.** The organic frame grows from the centre to open, draws back in to close, and can close and reopen as a chapter break. *Fits content like:* an opening; a change of chapter; an ending.
- **Living loop.** Every small motion is periodic, so a passage loops seamlessly. *Fits content like:* a website hero; a screensaver; a livestream background; a live wallpaper.
- **Callout.** A pill label with a leader line pops out of the model. *Fits content like:* explaining a mechanism; labelling data; naming places.

PITFALLS OF THE MEDIUM
```
