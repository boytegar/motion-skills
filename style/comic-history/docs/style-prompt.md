Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Comic History Cartoon" style.

MATERIALS & RENDERING
- **Printed map** (rule): equirectangular projection with longitude compressed by cos(centre latitude); about six engraved ripple lines outside each coast; dashed graticule; thin ink borders; flat fills with a lighter rim about 20 % wide inside each shape, as if the plate inked thin at the edge; compass rose and a kilometre scale bar. A procedural paper texture (mottle, fibres, fine grain) overlays everything, about 55 % overlay.
- **Borders**: Natural Earth for modern lines, except where China is in frame (DIRECTOR.md §13); historical borders are pieced from modern ones and hand-traced where the difference shows in frame. Name the error margin in the credits.
- **Puppets** (rule): green-screen part sheets keyed and cut into parts; each part gets a cream card edge (dilated a few px) so it reads as cut paper. Parts rotate about their pins (2D forward kinematics); arms reach targets with two-bone IK. Small part-on-part shadows (about 3–4 px) and one larger cast shadow on the map (about 13–19 px).
- **Paper patches**: opaque card with a dashed line printed inside the edge and a thin cream rim; torn edges are jagged with white fibres; a peeled corner is a fold transform with the back of the paper showing.
- **Height is shadow** (rule): a patch held in the air throws a large soft shadow offset 30–45 px; once down it keeps only a 3–4 px paper-edge shadow.
- **The page**: panels drawn into their own canvases at 1:1 so a panel pushed to full screen stays sharp; ink panel frames, cream gutters, a warm grey textured table that only shows when the camera pulls back.
- Nothing is translucent: patches, bubbles, tags and caption boxes are opaque paper.

COLOUR LOGIC
- Two protagonists, two colours, kept for the whole film (for example a printed blue and a printed grey). Neighbours are low-saturation print colours (sand, sage, brick, ochre) and never compete.
- **A new patch is one step away from the printed colour** of its owner (lighter or deeper), so it reads as another piece of paper laid on the print.
- Ink is one warm near-black for all type and lines; paper is a warm cream. One accent red is reserved for scissors handles and the Chinese line in speech bubbles.
- A serious panel drops saturation and value (darker, colder) instead of adding new colours.
- Example (our demo): paper #F1E8D2, ink #2B2219, side A #6E93D6, side B #5D6672, sea #A6CCD3, red #C8463A, brass #D9B158, caption card #F5E9C4, table #7B6A57.

TYPE & SUBTITLES
All licence-free (OFL). Roles:
- **Place names**: an engraved small-caps serif (e.g. IM FELL English SC), tracked out, pure ink. No outline, glow or shadow.
- **Small print** (tag captions, treaty slips, title card): the same family in roman and italic.
- **Every number** (rule): a serif with lining figures (e.g. Old Standard TT). Old-style figures turn 1975 into "I975" and drop the 7 below the baseline. Year tags are bold lining figures with a thin printed rule under them; numbers inside other text switch to the lining face at the surrounding weight, on the same baseline, at about 0.9 of the surrounding size.
- **Shouts** in speech bubbles: a round heavy display face (e.g. Lilita One), one English line; the Chinese line under it in a rounded hand-drawn face (e.g. ZCOOL KuaiLe) in the accent red.
- **Where text lives** (rule): speech bubbles hold only what characters say, short, in the upper half above the speaker, tail growing from the bubble edge to the mouth. Narration goes into comic caption boxes (pale yellow card, ink border) at the bottom centre, Chinese above English, one line each, never wrapped; feet, place names and scale bars stay clear of that band. The year is a printed tag pasted at the top left of the panel; extra facts are slips pasted under it. Words on props (a passport, a pin, a title card) enter as paper: held up, stamped, dropped, slapped on.
- On-screen Chinese at least about 34 px at 1080p. Hold each caption ≥ max(1.8 s, speech + 0.6 s).

MOTION QUALITY
- **Puppets on twos** (12 poses a second); camera and shadows move every frame.
- Keyframes with spring overshoot: hop (legs swing, squat on landing), wind-up (lean back, tremble), strike (lunge, every joint rebounds), settle (arms crossed with overshoot). A head swap pops the head 1.07–1.10× for a frame or two.
- **One in-between drawing** carries a big strike: the pose at the top holds and trembles, then a single frame with ink motion arcs lands it.
- Impact spreads: the camera shakes, paper scraps fly, ink lines burst, the other puppet's hat jumps off and lands back.
- Emotions are paper: jagged-edged card bubbles for shouts, two paper clouds of steam for anger.

CAMERA GRAMMAR
- Inside a panel the camera stays on the paper; when a panel is pushed up it fills the frame exactly (1:1). A narrow panel parks at one side of frame with the panel just played still visible beside it, never centred over bare table.
- Transitions are page moves only (slide, whip, drop, pull back); no dissolves or flips between panels.

SOUND PALETTE
- **Accents per side**: each country gets one instrument family (e.g. tuba and trombone oom-pah, accordion), so the audience hears who is winning. Xylophone steps for hops and snips, a music-box glockenspiel for a civilian moment, low strings and slow timpani for the serious panel with no comic sounds.
- **Paper foley**: slaps (a high crack, the paper body, a low thump), scissors (two metal clicks and a short ring), rubbing paper flat, tearing, crumpling, tape, a slide whistle for a flying hat, small clicks of the pins.
- A near-silence (a thin tremolo) before the biggest slap; the music can stop dead on a stamp.
- Voice: a dry, slightly amused narrator. Compress it and set it about 10 dB over the music; while it speaks, the music and foley dip about 3 dB and the music's 300 Hz–4 kHz band another 6–7 dB. Master at −14 LUFS, true peak ≤ −1 dB.

NATIVE MOVES
- **Cut along the dotted line.** A dashed cut line prints along the border, scissors snip along it, the cut land greys and hatches, a new patch drops with a big shadow and slaps down. *Fits content like:* a company buying a rival's market; a district redrawn for an election; a river changing course and moving a border.
- **Tear it back.** A patch is torn off with ragged fibres and leftover shreds underneath, and flung at the other puppet. *Fits content like:* a treaty revoked; a franchise reclaimed; a merger undone.
- **It won't lie flat.** A patch pressed on by force keeps a peeled corner; someone tugs it and it will not come off. *Fits content like:* an occupation; a policy nobody accepts; a rebrand that doesn't stick.
- **Layer on layer.** Instead of tearing, a new patch is pasted over the old ones, until the tilt-down shows a stack. *Fits content like:* a city's successive names; layers of regulation; a product's version history.
- **Stamp the papers.** A small civilian holds a document; a stamp drops in front of their face and changes one word. *Fits content like:* a citizenship change; a visa rule; a currency switch.
- **Pin on the map.** A labelled pin drops onto the real location to end a thread. *Fits content like:* where a headquarters moved; where a battle ended; where a person still lives.
Our demo opens on a puppet hopping in with a patch and *cut along the dotted line*, uses the tilt down through *layer on layer* as its climax and ends on *pin on the map*, then a pull back to the whole page. A new film opens and ends another way: scissors already cutting before anyone appears, the whole comic page on the table before a push into the first panel, *stamp the papers* in a narrow corner panel as the last word, or a whip across a gutter that lands the punchline.

PITFALLS OF THE MEDIUM
- Recolouring a country without a paper action: the style disappears.
- A new patch in exactly the printed colour of its owner: once it moves it reads as a white outline only.
- Outlines, glows or drop shadows on type; old-style figures in years; system fonts.
- Two speech bubbles on screen at once, bubbles over place names, tails pointing at hats instead of mouths.
- Characters' feet, names or the scale bar in the caption band.
- Smooth on-ones puppet motion or ease curves with no overshoot: it reads as software, not paper.
- Comic sound effects or slapstick on war deaths, massacres or persecution; those panels go darker and quieter.
- Real flags with political symbols, Nazi symbols, real people's likenesses. Uniforms personify a country; say in the narration who the actual party was when the costume simplifies it.
- Upscaling a panel past 1:1: render panels at their on-screen size.
```
