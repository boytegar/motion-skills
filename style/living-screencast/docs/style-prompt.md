Copy-paste blocks for using this style outside this skill: another coding agent, p5/Canvas, After Effects notes, image or video generators.

Imported from Lemo-Opuscar by Lemomo (LemoLab) (https://github.com/lemomo-ai/lemo-opuscar), CC BY-NC 4.0.

---

```
Make this in the "Mascot Screencast" style.

MATERIALS & RENDERING
- **Real UI, redrawn**: every label, mode name and shortcut matches the real product; no screenshots, no lorem ipsum. Demo content (code, data, test output) is coherent, and the app under test really changes.
- **Pixel grid is sacred**: the mascot, its props (pencil, "!", sparkles, heart, dust) and trails share one square-pixel grid with crisp edges. It scales with the camera; its position snaps to half-pixels, so its motion feels stepped.
- **Gravity = top edges of UI**: the mascot only stands on the top edge of a real element (input box, menu, list row, code line, button, status bar, the letters of a heading). Its position comes from **measuring the live DOM** every frame, never hand-placed coordinates.
- **Window on wallpaper**: the app floats on a desktop with a soft shadow; zoom never shows beyond the monitor.
- **Depth** only where it serves attention: rack focus (blur the pane that doesn't matter), a vignette spotlight. Never blur text the viewer must read.
- **Motion blur** is derived from camera speed and applied as an anisotropic blur on an **untransformed wrapper**, only on whips. Slow moves stay sharp.

COLOUR LOGIC
- The product's own interface colours rule: follow its real light and dark themes exactly.
- The mascot keeps the product's or brand's mascot colour, or one saturated hue that no UI element uses, so it finds the eye in any frame.
- Overlays (chapter pill, key HUD, captions) are neutral frosted glass; one accent at most, taken from the product.
- A theme switch can be a story event: the whole film may change state with it.

TYPE & SUBTITLES
- UI type follows the product (or licence-free look-alikes such as Inter, JetBrains Mono, a serif for editorial UIs).
- **Information layers only**: a chapter pill (01–04), a KeyCastr-style keystroke HUD, frosted-glass caption pills, an optional "▶▶ 4×" time-lapse tag. Nothing else overlays the screen.
- Captions sit in a pill at the bottom, never over the element being discussed and never under the mascot; hold ≥ max(1.8 s, speech + 0.6 s).
- A title, if any, is diegetic: it appears as part of the UI (an empty-state heading, a window title, a notification).

MOTION QUALITY
- 30 fps; UI on ones; mascot position stepped to its pixel grid.
- **Mascot acting**: jumps with anticipation crouch, stretch on take-off, tucked legs, landing squash with a damped bounce and dust. Idle breathes (a tiny squash) on the score's beat and blinks irregularly. Walks by leg alternation, flipping the sprite for direction.
- **Eyes carry the acting** (neutral / left / right / up / down / wide / happy / shut / blink): they look at what matters. Arms: down / up (cheer) / wave.
- **Cursor**: smoothed, spring-eased paths, small push on click, never teleports.
- **Cause and effect**: every mascot action triggers a real UI state change; every user action is a cursor move, click or keystroke shown on the HUD.
- UI animates like the real product (its own menus, popovers, diff reveals); don't invent flourishes the product doesn't have.

CAMERA GRAMMAR
A recording-software camera: one continuous take inside the screen, zoom ≥ 1, cuts only under chapter transitions. Opening and ending come from the topic.
Framing: the thing being discussed and its caption never overlap; the mascot never covers text to be read. Transitions: chapter pill wipes, pixel wipes, a theme reveal circle; never dissolves or screenshots.

SOUND PALETTE
- **Two resolutions in the music**: a hi-fi layer (piano, upright or electric bass, brushed or tight kit, glockenspiel, real samples) is the world; a chiptune square or pulse voice is the mascot. They can trade, duet or merge.
- Options: a low-passed "night" version of the band for a dark theme; a tom fill and crash at chapter seams; dropping the drums to build tension; the mascot's notes on words it lands on; a stereo split for split screens; a single held pad under a freeze.
- **Foley**: keyboard (a thock plus a click, human timing jitter), trackpad clicks, pane swishes, popup pops, notification chimes, CI ticks, a shutter for screenshots. Mascot steps, jumps and landings are 8-bit.
- **Silence**: a short drop to room tone before the key promise makes it land.
- **Voice**: calm, one feature per line; product names verified with whisper. Keep effects off word onsets. Duck music under voice; −14 LUFS.

NATIVE MOVES
A menu: use the ones your walkthrough needs.
- **Match-cut origin.** The mascot is born from something already in the UI (a logo's block characters, an icon, a loading spinner) and climbs out, leaving an outline. *Fits content like:* a chat bubble turning into a guide; an icon becoming an assistant; a spinner becoming a helper.
- **Diegetic title.** The name appears as a real UI element; the mascot lands on it on a downbeat. *Fits content like:* an empty state; a new-project dialog; a welcome email.
- **Moving floor.** The mascot rides a caret, a popup, a sent message, a progress bar. *Fits content like:* a message flying into a thread; an upload bar; a drag-and-drop.
- **Time-lapse with a tag.** Long work runs at N× with a flashing tag and rack focus. *Fits content like:* indexing a library; a data import; a build.
- **Spotlight freeze.** Push in and vignette the one badge that holds the promise. *Fits content like:* "Saved"; "Encrypted"; "No charges until…".
- **The film changes state.** A toggle's effect escapes the preview and takes over the whole film, and returns later. *Fits content like:* dark mode; a language switch; offline mode.
- **Cell division.** A new session or window splits the screen and the mascot splits with a pixel burst. *Fits content like:* parallel tasks; multiple accounts; team members.
- **Cursor and mascot as characters.** The cursor pets, pokes or drags the mascot, and it reacts. *Fits content like:* approving a suggestion; undoing; onboarding delight.
- **Karaoke-ball words.** The mascot hops from word to word as they are spoken. *Fits content like:* a slogan; a shortcut list; step names.

PITFALLS OF THE MEDIUM
- Invented features or renamed modes → check every button, mode and shortcut against the product's documentation; if it isn't in the docs, it isn't in the film.
- The mascot floats or drifts off its element → measure the DOM every frame, stand only on top edges.
- The mascot covers text or sits under a caption → reserve a text-free lane for it.
- Blur-filter "fake depth" on text you want read, or zooming out past the screen → rack focus only on panes not being read; zoom ≥ 1.
- A hand, arm or finger on screen → the cursor is the user.
- Motion blur applied to a transformed layer smears the pixel sprite wrong → blur an untransformed wrapper, and only on whips.
- A keypress or pop landing on a product name masks it → shift foley off word onsets; re-check with whisper on the final mix.
- TTS mispronounces the product name → try voices and spellings, verify with whisper.
```
