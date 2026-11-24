Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "1920s Silent Film" style.

MATERIALS & RENDERING
- **Tonal painting + Redraw.** Scenes are painted in grey values only (print densities roughly: ink 0.07, dark cloth 0.15–0.3, mid wood 0.45–0.6, skin 0.8, white linen 0.9–0.95), with soft light gradients and a thin separation line. A **Redraw** pass (WebGL2) inks the whole frame: difference-of-Gaussians contours on the dark side of every edge, luminance posterised into ~5 soft wash steps, and 45° hatching (then crossed) in the darks, re-drawn every two frames like hand-redrawn animation. Real video frames can go through the same pass so footage and drawing share one hand.
- **Print pass**: silver tone with a sepia amount between ~0.1 and ~0.36, print black ~0.055 and white ~0.93, S-curve, halation on highlights, uneven density, flicker, gate weave with occasional perforation jumps, broken vertical scratches (white = scraped emulsion, dark = dirt on the negative), brown burn creeping from the edges, vignette, two-octave silver grain; then dust, a hair in the gate, a rare blotch.
- **Frame**: 1440×1080 in the centre of 1920×1080, a rounded aperture with a soft dark rim; velvet drapes at the sides lit by the screen's spill and by footlights. Curtains can part and close.
- **Figures**: realistic proportions (7.5-head adults, ~5-head children) on a 2.5D rig (joints solved in 3D, projected with a yaw), silent-era make-up (pale skin, dark lids and lips); identity by silhouette marks (a hat, a garment, a tool).

COLOUR LOGIC
- **Monochrome silver**, tinted only by the print: colder and harsher (more damage, less sepia) when things go wrong, warmer and cleaner when the film turns tender. Tint and damage are mood controls, set per section.
- Values carry everything: dark clothes against pale walls, pale faces against dark interiors; check every shot as a thumbnail.
- **One colour, at most**: an element drawn into a separate colour mask keeps its hue through the print (still getting grain, flicker, vignette). Use it for the single thing the film is about, or not at all.
- Period tinting of whole scenes (amber for day interiors, blue for night, rose for romance) is a valid alternative to sepia; one tint per section.

TYPE & SUBTITLES
- **The intertitles are the subtitles**; the `.srt` lists their text. A card holds 2.5–3.75 s (≥ max(1.8 s, reading time + 0.6 s)).
- A black painted card with a deco border whose ornament follows the emotion: one heavy rule for a shout (the text shakes), a double rule with stepped corners and fans for narration, vines and small flowers for tenderness.
- Period faces (OFL): a small-caps display serif (e.g. Playfair Display SC) for titles and shouts, an italic old-style serif (e.g. Old Standard TT Italic) for narration and dialogue.
- A few words per card; three to five cards in a short film. A card never explains what the picture already shows.

MOTION QUALITY
- **Hand-cranked sampling**: character and camera motion sampled at 16 fps (18 or more to undercrank a fast scene); flicker, weave, grain and scratches change every output frame (24 fps). That mix makes it look projected rather than merely low-frame-rate.
- **Speed is an emotion**: undercrank chases and panic (faster motion, faster flicker); return to 16 fps and a calmer print for tender scenes.
- **Deadpan performance**: the face barely changes; the body acts (crouch before effort, held poses, the two-frame snap of a double take, hands on knees). Objects act too (a wobble before a fall, a ritardando to rest).
- **Keyframed, eased poses**; only deliberate gags snap.

CAMERA GRAMMAR
A vocabulary, not a route. The period camera is heavy: most shots are locked. Opening and ending come from the topic.
Framing: wide and medium shots; big close-ups of rigged faces look like mannequins, so keep any close-up small or inside an iris. Transitions: iris, fade to black, straight cuts, the curtain. No dissolves between shots of the same scene, no modern whip pans.

SOUND PALETTE
- **The instrument is the sound design**, as in a picture house: an upright piano (optionally a reed organ, theatre organ, violin or small ensemble) plays every event. Mickey-mousing vocabulary (options): chromatic creep in the bass (sneaking), grace note (a wink), forearm cluster (a crash), swelling diminished tremolo (a wait), low rumbling tremolo (a storm, a machine), "plink … plonk" (emptiness), diminished sforzando (a shock), rolled chord (a door, a reveal), descending glissando (a fall), low "boing" (a bounce), nervous trill (hesitation), minor second (something breaking).
- **Photoplay idiom**: stride left hand, music by mood section in the manner of cue sheets (Hurry, Misterioso, Agitato, Tenderly, Maestoso), a theme that returns changed. Original music only.
- **No foley, no voice.** **The projector is the ambience**: lamp click, motor spin-up, claw-and-motor purr, film flapping on the take-up reel.
- **Silence**: the music stops and the projector alone remains; it frames a held moment.
- **Mix**: tame piano hammer transients with light compression before loudness normalisation (−14 LUFS). Grain goes in the picture (mux grain 1).

NATIVE MOVES
A menu: use the ones your story needs.
- **The chain in one take.** A triggers B, B leaves the frame, the audience waits, B returns and hits C. *Fits content like:* a supply chain gone wrong; how a rumour spreads; a Rube Goldberg invention.
- **The near-miss.** A hand closes exactly where the object was a frame ago. *Fits content like:* catching a bus; a startup missing a deal; a goalkeeper.
- **The intertitle twist.** A card reframes the next shot in a few words. *Fits content like:* "Meanwhile, in the lab…"; "The machine had other ideas."; a statistic as a punchline.
- **Undercranked chaos.** Speed and damage rise together. *Fits content like:* rush hour; a kitchen at dinner service; a stock crash.
- **Iris on the look.** An iris stops half-way so a character can glance at the audience. *Fits content like:* a product winking; a cat that knows; a scientist's secret.
- **Redrawn footage.** Real video pushed through the ink pass becomes a period drawing. *Fits content like:* a company's archive; a family film; a sports clip.
- **The echo gesture.** A gesture from the start repeated by someone else with a new meaning. *Fits content like:* a teacher and a pupil; a handover; a tradition.

PITFALLS OF THE MEDIUM
- **Too much content kills a silent film**: a many-step chain at small figure scale is noise in a thumbnail. One legible chain with a pause in the middle beats three busy ones.
- **Puppet arms**: apply abduction before flexion, let the shoulder rise and swing forward as the arm goes up, slim the torso's shoulder corner on the raised side, hide the arm's contour where it lies over the torso. Raise arms sideways (or turn the figure) so they don't cross the face.
- **Pose pops**: drive every figure by eased keyframe lists, never by `if` branches.
- **Floating props**: hold a prop between the palms at the depth between the arms; tie a rolling object's rotation to distance (angle = distance / radius).
- Brow angles flip with facing direction: check expressions in a crop at final size.
- An iris must track its target in film coordinates after any camera zoom.
- A grab must put the hand exactly where the object is at the beat, or it doesn't read.
- Closed curtains lit only by a dark screen go pure black: add footlights.
- Heavy grain makes huge files → re-encode with `-tune grain` at a higher CRF.
```
