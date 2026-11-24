Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Microgame Frenzy" style.

MATERIALS & RENDERING
- **Home stage**: flat toon, thick near-black outline (~7 px), flat fills, **one hard-edged shadow** (fill in the shade colour, clip to the shape, refill shifted up-left). Fixed furniture: a frame-within-the-frame where games play (a TV, an arcade cabinet, a phone, a window), the lives board, a stage badge, a trigger prop (button, lever, bell).
- **Each medium is a simplified re-drawing, 100–200 lines of code**, built around one channel trick that makes it read at a glance:
  - drawn media store **pressure or wetness in a channel** (crayon: alpha = pressure against paper tooth; ink: wet and dry ink channels);
  - print media store **plate densities** (riso: a channel per plate, additive overprint, halftone, misregistration);
  - screen media are **low-res buffers** scaled up (pixel: indexed buffer, auto outline, dither; terminal: hand-authored glyph art, phosphor shader);
  - technical media are **line systems** (blueprint: line weights and dimensions; Swiss: a strict grid, one sans, one signal colour).
- **Hero translation**: the hero is redrawn from scratch in each medium, but the same marks always survive (e.g. a hat shape, one coloured dot, a number on the chest). Make a model sheet of the hero in every medium before animating.
- **Mixed-media frames**: alpha-coverage media over a base, paper media multiplied, screen media flat and clipped to a panel. WebGL2 material shaders turn the channel buffers into each look.

COLOUR LOGIC
- **The home stage owns a saturated party palette**: a deep ground behind a rotating sunburst, two or three loud accents, plus a fixed success colour and a fixed fail colour used by stamps and the lives board only.
- **Each medium brings its own palette** from its tradition (wax colours, one-ink-plus-vermilion, amber phosphor, three riso plates, a limited pixel palette, white-on-blue, black-white-red). Never tint a medium with the home palette.
- **One accent follows the hero across media** where the medium allows (a single vermilion dot in ink, an inverse glyph in a terminal, a signal-red circle in Swiss type): it is how the eye finds the hero after a switch.
- Example home palettes: violet `#2a0f5c` with magenta `#ff2e88`, gold `#ffc928`, cyan `#1fd1d1`; teal ground with coral and lemon; black stage with hot orange and electric blue.

TYPE & SUBTITLES
- **The command word is the subtitle**: a heavy display face (e.g. Titan One), white with a thick ink outline, a coloured drop and a burst behind it, skewed a few degrees. It slams in over ~3 frames (overshoot → settle), holds, then **shrinks into a corner tag before the first action**, so the result is never covered. One-off banners fly out instead of tagging, so tags never pile up.
- **Other lines**: a rounded card in the home palette with the home outline and hard shadow, a friendly rounded face (e.g. Lilita One ~50 px), a small speaker icon; placed above the timer in games. Hold ≥ max(1.8 s, speech + 0.6 s), never under a big banner.
- Inside a game, any text (replay captions, labels) uses **that medium's own typography**.
- The command word appears in the .srt only, never burned twice.

MOTION QUALITY
- **The frame rate is part of the ladder**: characters step on twos or at 8 fps early, faster later, on ones at the climax. Camera moves, transitions and the timer spark always run on ones.
- **Five things accelerate together** at each speed-up: tempo, game length, stage dwell, command hold, character frame rate.
- **The timer is visible and on the beat**: e.g. a fuse with one knot per beat, the spark passing each knot exactly on it. It freezes in a replay.
- **Unified transition through the frame-within-the-frame**: push into the screen to start a game (~0.25 s), shrink the game back into it to return (~0.33 s) with a ✓ / ✗ stamp. No fades, no blank frames.
- **Failure is performed**: the action overshoots, splats, bonks or rolls off; the medium's own physics makes the joke.
- **Instant replay** for a failure that must be understood: freeze, half speed, push-in, letterbox, a caption in the medium's type.
- The home host has a tick on every beat; stopping it dead is the strongest emotional tool.

CAMERA GRAMMAR
A vocabulary, not a route. Opening and ending come from the topic.
Framing: the game screen is a fixed fraction of the frame (around 40 % of the width); in a mixed-media frame keep **one focal chain** (A leads to B leads to C) and push info panels into a corner.

SOUND PALETTE
- **Music first**: a tempo grid and a cue table before any animation; every picture hit lands on a listed cue.
- **A constant groove + one colour instrument per medium.** The groove (kit with ghost notes, slap or synth bass, brass stabs, keys) runs throughout; each medium adds its own timbre: toy piano for crayon, shakuhachi or guqin for ink, square beeps for terminals, vibes for print, pulse arpeggios for pixel, a typewriter for drafting, claves for Swiss, kalimba for clay, koto for woodcut.
- **Foley follows the medium**: wax squeaks, wet splats, teletype relays, paper crunch, bit-crushed noise, ratchets and stamps, hollow bonks. Home stage: a spring trigger click, glass crack for a lost life, card flips, a reel whirr, sparse claps.
- **Acceleration options** (use what fits): a rising brass run into each speed-up; the key rising a step per round; halving note values instead of changing tempo; dropping the groove to a heartbeat on the last life.
- **Real silences**: at least one before the biggest hit (a heartbeat or one pitched note may remain). The loudest moment comes right after the longest silence.
- **Voices**: a host who shouts every command a hair *after* the stab so brass doesn't mask it; a hero with very few words (reactions work best). Duck music ~11 dB and foley ~6 dB under voice. −14 LUFS.

NATIVE MOVES
A menu: use the ones your story needs.
- **Style switch per game.** Each task is drawn in the medium that suits it. *Fits content like:* the steps of baking bread (flour in crayon, oven dial in blueprint, crust in woodcut); a day of a nurse; a heist's roles.
- **Command word.** One verb is script, subtitle and downbeat. *Fits content like:* SCAN! at a checkout; BREATHE! in a yoga class; SIGN! at a contract signing.
- **Lives as UI.** The counter carries emotion without narration; 3 → 2 → 1 (blinking) builds tension alone. *Fits content like:* a phone battery; retries on a driving test; chances left on a quiz.
- **SPEED UP.** Everything steps up together. *Fits content like:* rush hour; the last week before a deadline; a kitchen during dinner service.
- **Failure becomes the solution.** The material of an earlier failure returns in the final game as the answer. *Fits content like:* a spilled coffee that later stains the winning map; a bug that becomes a feature; a missed train that leads to the right meeting.
- **Boss mixes the rules.** The final game is built from pieces of earlier media. *Fits content like:* a final exam; launch day; a festival's closing night.
- **Instant replay.** The joke explained in slow motion. *Fits content like:* a goal-line decision; a typo that shipped; a cat's leap that missed.

PITFALLS OF THE MEDIUM
- **A luminance-ramp ASCII figure is mush** → hand-authored glyph art with structural glyphs, an inverse glyph for the accent.
- **Ink shapes read as smoke** unless their silhouette is designed (a short round blot phase, then a clear structure with seams and light edges).
- **A failure at high tempo goes unread** → make the action overshoot and hit something, or give it an instant replay.
- **Alpha-coverage media show the background through gaps** → a full-pressure base where it must be solid.
- **Drawing helpers with a global context** draw on the wrong canvas when scenes render into offscreen buffers (game screen, reels, tiles) → set and restore the context.
- **Command tags pile up** → secondary commands fly out.
- **A 100 % white flash** reads as a blank frame → cap at ~60 %.
- **TTS single-word commands** gain a vowel tail ("Pump!" → "Pompey") → generate a two-word phrase and cut after the first word using ASR timestamps; verify each clip, since ASR mishears 0.2 s words in a dense mix.
```
