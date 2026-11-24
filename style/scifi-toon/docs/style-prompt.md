Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Sci-Fi Sitcom Toon" style.

MATERIALS & RENDERING
- **Line**: near-black (a slightly purple black reads best), ~7 px for characters and props, 4–6 px for background detail, round joins. **Line width is constant in screen space**: close-ups are "redrawn" at the same weight, never scaled up (transform points to screen, then stroke with an identity transform).
- **Line boil**: resample every outline in screen space (~7 px steps) and displace along the normal with low-frequency noise, amplitude ~1.7 px; cycle **3 boil drawings at 12 fps**. Everything boils, including held poses and backgrounds: a still frame is never dead.
- **Fill**: flat colours only. Shadows are hard-edged darker shapes clipped inside the fill (one per form, away from the light). Glows are 1–2 flat translucent rings, not gradients. Skies are **flat colour bands**.
- **Faces**: big white eyes (touching in 3/4 view), 4–5 px pupils, heavy lids for deadpan, eye bags. Parametric mouths (open, width, curl, skew) with interior, teeth, tongue. Brows carry the emotion.
- **Cast design**: silhouettes must read in solid black. Give each lead one absurd costume idea. Avoid white lab coat + spiky hair.
- **Creatures**: every world may get one creature that is disgusting *and* adorable; keep it non-verbal (sound effects) so the film stays a dialogue between the leads. Check that anything behind furniture still shows face and upper body.
- **The portal** (or whatever device moves between worlds): the only thing that glows. A lumpy rim with a thick outline, rotating spiral arms, a pale core, orbiting sparks, drips, a flat translucent wash over the scene while it's open. Open with a gentle overshoot (~12 %, back-ease ~0.3 s); a springy overshoot covers the actors.
- **Screen graphics**: a retro-terminal tag (VT323) typed in, coloured per world.

COLOUR LOGIC
- **One palette per world**, 3–5 colours, maximally different from the previous world in hue *and* value, so the audience reads "we jumped" from colour alone.
- Line colour and eye white never change between worlds; everything else may.
- Glow colour belongs to the travel device only: nothing else in any world may use it.
- A deliberately bland palette (beige, brown) is itself a joke: use it for the world that is "suspiciously normal".
- Examples of a sequence of worlds: sage lab → magenta and lime jelly → orange and teal; or grey office → ultraviolet and gold → mint and coral → beige. Our demo's values are in DEMO.md.

TYPE & SUBTITLES
- **Subtitles**: bottom centre, a rounded heavy sans (Baloo 2 ExtraBold ~50 px), white with a ~11 px black outline, so they look like the cartoon's own captions, with a small **speaker pill** in the character's colour (boiling outline) on the left. Lines over ~1300 px wrap to two lines. Overlapping lines stack; the older one dims to ~70 %.
- **Timing**: hold ≥ max(1.8 s, speech + 0.6 s) for narration; for rapid dialogue, `max(audio + 0.35 s, 0.9 s + chars / 17)`. Clip at world changes and **before a silent beat**, so the silence plays on a clean screen.
- **Title**: chunky display lettering popped in on 12 fps steps on a musical hit. Choose a period look that is not the source genre's logo: **avoid acid green, slime drips and wobbly bubble lettering** together, which reads as a specific show.
- Keep faces out of the subtitle band (bottom ~170 px) and the world tag clear of faces.

MOTION QUALITY
- **Characters on twos (12 fps)**: poses, mouths, blinks, walk cycles and boil step at 12 fps. **Camera moves, portal swirl, flying props and wipes on ones (24 fps)**.
- **Acting over moving**: held poses with small changes (eye twitch, pupils sliding, a gulp, sweat drops). Idle life: breathing (±1 % squash), a nod driven by speech loudness.
- **Anticipation → action** before every gesture.
- **Squash & stretch** on landings (decaying cosine squash ~22 %), stretch while flying out of a portal; "sucked in" = scale toward the portal centre while stretching.
- **Lip-sync from audio**: per line, RMS (open) and spectral centroid (wide vs round) at 24 Hz, sampled at the 12 fps drawing time. Exaggerate for screams (mouth ×1.5–1.9).
- **Nervous jitter**: a 12 fps random offset of 3–6 px plus trembling pupils; amplitude grows with panic.
- What never moves in a cold pause: everything but the boil.

CAMERA GRAMMAR
A vocabulary, not a route. Opening and ending come from the topic.
Rule: every absurd image is followed by a **reaction close-up**. Transitions go through the travel device (swirl wipes) or hard cuts; no dissolves.

SOUND PALETTE
- **Synthesized score, no samples needed.** A **theremin**-like lead (sine + a little 2nd/3rd harmonic, legato portamento ~70 ms, vibrato fading in ~150 ms after each onset, spring reverb) over analog bass, a square-wave arpeggio and a retro drum machine. Minor modes and chromatic lines suit the main theme.
- **Genre cues per world** (options): surf guitar; polka accordion; toy-piano lullaby; elevator bossa (Karplus–Strong nylon guitar); brushed swing with vibes; tuba + slide whistle; warm Rhodes + a vocal "aah"; tremolo strings for panic.
- **Techniques (options)**: a cue's first beat is the cut; a tempo that snaps cuts to a grid; rule of three becoming machine-gun (same stinger, shorter each time).
- **Silence is the punchline**: hard-cut the music, reverb tails included, on a reveal and for a cold pause. Leave only room tone (fluorescent hum, a clock tick, a fridge).
- **Foley (synthesized)**: portal open (sub boom + down-swept noise + rising swirl), hum, close "fwump", swirl whoosh per wipe, pop/boing, wet glorp/squish, slurp, chomp, blink "blip", click-beep, kazoo sting, squeaky toy.
- **Voices**: a two-hander with contrasting voices: low, flat and slow for the deadpan lead (light saturation + a ~180 Hz bump for gravel); higher and faster for the nervous one (presence boost). A tiny creature may say one or two words, pitched up. Compress, RMS-match, add short early reflections so voices sit in the room. Duck music ~−7 dB under dialogue. Master −14 LUFS.

NATIVE MOVES
A menu: use the ones your story needs.
- **Parallel worlds.** Each jump = new palette, new cue, new creature. *Fits content like:* versions of a product; careers you could have had; the same city in five eras.
- **Duo contrast.** Deadpan vs panic: the joke is in the difference between reactions. *Fits content like:* a veteran and a new hire; a parent and a teenager; an AI and its user.
- **Rule of three → machine-gun.** Two slow wrong answers, then one-second worlds cut on the beat with the same rejection word. *Fits content like:* failed prototypes; bad date ideas; tax forms.
- **The suspiciously normal world.** The pattern breaks; the calm is the joke. *Fits content like:* the one boring answer that works; a quiet office after chaos; the real customer.
- **Dead-air reversal.** A long silence after the twist, then one line revealing a character's values. *Fits content like:* a price reveal; a fine print clause; a gift that wasn't wanted.
- **Hardware as junk.** World-bending tech used for a petty chore. *Fits content like:* a quantum computer used for a grocery list; a rocket to deliver a sandwich; a time machine to fix a typo.
- **Callback button.** The last line repeats the first so the story loops. *Fits content like:* a catchphrase of a brand; a morning routine; a running complaint.

PITFALLS OF THE MEDIUM
- TTS reads hyphen stutters as words: write syllables ("buh, buh-broken"); respell words it mangles; pick names that survive a low voice.
- Whisper mis-hears very short clips unless padded with ~0.6 s of silence.
- A real interruption: generate the interrupted line *longer* than needed and truncate the audio where the other voice cuts in.
- A springy overshoot swallows the actors: gentle back-ease.
- Food and liquids need cues (surface detail, drips, steam) or they read as other objects.
- Square-wave synth cymbals alias into harsh 8–20 kHz fizz: band-passed noise plus a little FM.
- A sustained bass below ~40 Hz is rumble on laptop speakers: end an octave higher.
- A cut music cue without its reverb tail cut is not dead silence.
- `ffmpeg fps=1` contact sheets are offset up to 0.5 s: don't misdiagnose a shot boundary.
```
