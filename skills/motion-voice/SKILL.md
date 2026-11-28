---
name: motion-voice
description: Voice a short video and time it to the picture. Three paths, local Qwen3-TTS, your own recording or another TTS, or a placeholder voice. Covers delivery rules, voice.json, captions and scene timing. Use for "set up the voice", "re-voice this", "time the script".
---

# Voice

Every later stage needs two things in the narration folder: `voice/final_narrator/<id>.wav` (one WAV per script line) and `timing_narrator.js` (a start and end for every word). This skill produces them by one of three paths, then builds the captions and the scene plan. The full reference for the scripts is `$MOTION_LIB/core/voice/README.md`.

## Preflight

- Library: `$MOTION_LIB`, else the `MOTION_LIB=` line in `~/.config/motion-explainer/config.env`, else ask once. Export it, and `. "$MOTION_LIB/env.sh"` loads `ASR_PY`, `QWEN_TTS_PY` and the other config values into the shell.
- An episode folder (`motion-strategy`, step 5) with `narration/script.json` and `video/episode.json`. Copy `build_timing.py` into `narration/` (`cp $MOTION_LIB/core/voice/build_timing.py narration/`).
- If the `.env` already names a `TTS_PROVIDER` (or the user has a hosted voice key), use path A. If the user already has a local TTS installed, use it as it is and don't rebuild it (path B if it is Qwen3-TTS, else C). Ask once which one and where it runs. Otherwise ask which path, and say what it needs:

| Path | Needs | Planning mode |
|---|---|---|
| A. Hosted TTS (`.env`) | `TTS_PROVIDER` and/or `TTS_URL`, plus `TTS_API_KEY`, in `~/.config/motion-explainer/config.env`; ffmpeg to decode the reply; no model download | plan + real voiceover |
| B. Local Qwen3-TTS | the voice venv from `$MOTION_LIB/core/voice/setup_tts.sh` (model download 2.3-4.2 GB) | plan + real voiceover |
| C. Your own recording or another TTS | an ASR backend (mlx-whisper or faster-whisper) and librosa + numpy for the pitch pass, in the Python that `ASR_PY` and `QWEN_TTS_PY` name | plan + real voiceover, or those packages by hand |
| D. Placeholder | ffmpeg; macOS for `--say` | plan + audio files |

If the path needs more than the install has, run `./install.sh --doctor` and hand over to `motion-setup`.

Timing is not gated on ASR any more: `finalize.sh` runs `core/tools/pace.py` on the final lines, which reads each WAV's energy envelope and writes `voice/pace_<ROLE>.json` — per-line words per second plus a per-word curve that keeps the pauses. `build_timing.py` prefers that file, then ASR, then the 2.75 wps estimate. Captions therefore follow the real delivery rate with no extra install.

## Path A: hosted TTS from the .env

Start from the template (it has every key, commented, one block per engine): `cp $MOTION_LIB/core/config.env.example ~/.config/motion-explainer/config.env`, then `chmod 600` it and uncomment one block.

```sh
# ~/.config/motion-explainer/config.env
TTS_PROVIDER=openai            # openai | elevenlabs | custom (any JSON endpoint)
TTS_URL=                       # optional: endpoint override, or the whole setting for a custom server
TTS_API_KEY=sk-...
TTS_MODEL=gpt-4o-mini-tts      # elevenlabs: eleven_multilingual_v2
TTS_VOICE=alloy                # elevenlabs: a voice id
TTS_SPEED=1.0                  # 0.25-4.0
TTS_LANG=en
PACE_TARGET=3.3                # wps the pace report judges against
```

`TTS_URL` is how a non-vendor endpoint gets used, and it also selects the provider when `TTS_PROVIDER` is empty:

| `.env` | Endpoint |
|---|---|
| `TTS_URL=...` + `TTS_PROVIDER=openai` | that URL instead of `api.openai.com/v1/audio/speech` (gateway, proxy, self-hosted) |
| `TTS_URL=...` + `TTS_PROVIDER=elevenlabs` | that URL instead of `api.elevenlabs.io/...`; `{voice}` is substituted |
| `TTS_URL=...` alone, or `TTS_PROVIDER=custom` + `TTS_URL` | POSTs `TTS_PAYLOAD` (JSON with `{text}` `{voice}` `{model}` `{lang}` `{speed}`) with auth from `TTS_AUTH` (`none`/`bearer`/header name); `TTS_FORMAT` names the reply encoding |
| neither | local Qwen3-TTS |

Read the `.env` first and ask only for what is missing; never print the key back to the user. Then:

```
$MOTION_LIB/core/voice/narrate.sh voice/voice.json episodes/NN-slug/narration
$MOTION_LIB/core/voice/finalize.sh voice/voice.json episodes/NN-slug/narration episodes/NN-slug/narration/voice/takes/narrator
```

`narrate.sh` execs `voice/tts_provider.py` when `TTS_PROVIDER` is set. A hosted voice returns one fixed take, so `finalize.sh` skips the ASR + pitch picker and goes straight to trim / loudness-normalise → pace → timing. Errors are one line (`HTTP 401 key rejected`, `HTTP 404 wrong URL, model or voice id`), not a traceback.

- **Sync loop.** `finalize.sh` prints each line's wps and flags lines more than 12% off `PACE_TARGET`. Slow lines: raise `TTS_SPEED` slightly or shorten the text; fast lines: lower it or split the line. Re-run both commands; `timing_narrator.js` and every caption and scene cue follow.
- Keep `TTS_SPEED` in the 0.9-1.1 band: the pace pass moves the captions with the audio, but a genuinely hurried voice still reads rushed.

## Path B: local Qwen3-TTS

From the series folder:

```
cp $MOTION_LIB/core/voice/voice.example.json voice/voice.json     # once per series; edit it
$MOTION_LIB/core/voice/narrate.sh voice/voice.json episodes/NN-slug/narration
$MOTION_LIB/core/voice/finalize.sh voice/voice.json episodes/NN-slug/narration episodes/NN-slug/narration/voice/takes/narrator [num.json]
```

- `voice.json`: `speaker` (CustomVoice: Aiden, Dylan, Eric, Ryan, Serena, Vivian, Sohee, Ono_anna, Uncle_fu) and `instruct`. Recommended: CustomVoice 1.7B, Ryan or Aiden, instruct *"friendly, natural, conversational, like telling a friend; clear pauses between sentences; stress the key word of each line"*. `takes_per_line` defaults to 3. `tempo` stays 1.0 (other values need `rubberband`). `qa_targets` set the pitch range and the ASR match (0.9).
- A script line can carry `tts` (respelled text for pronunciation; captions keep `text`) and its own `instruct`. `num.json` maps numerals to spoken words (`{"13,000": "thirteen thousand"}`) so ASR matching doesn't fail on digits.
- Run one TTS process at a time; the 1.7B model is memory-hungry.
- `finalize.sh` runs ASR and pitch on every take, picks one per line, trims, loudness-normalises to -16 LUFS, runs `tools/pace.py` on the finals and writes `timing_narrator.js` from the pace json. After listening, put corrections in `voice/picks_override.json` (`{"l3_name": 2}`) and rerun with `SKIP_PICK=1`.
- Voice Design (describing a voice in words) is a Qwen3-TTS mode that `narrate.py` doesn't drive. Generate with it elsewhere and use path C.

## Path C: your own recording or another TTS

- Put one WAV per line in `voice/final_narrator/<id>.wav`, or raw takes as `voice/takes/narrator/<id>_<k>.wav` and run `finalize.sh` as above.
- For finished lines, write `voice/chosen_narrator.json` as `{id: "voice/final_narrator/<id>.wav"}`, then from `narration/`:

```
python3 $MOTION_LIB/core/tools/pace.py --script script.json --out voice/pace_narrator.json
CHOSEN=voice/chosen_narrator.json TIMING_OUT=timing_narrator.js PACE=voice/pace_narrator.json \
  python3 build_timing.py            # ASR json is optional and no longer needed
```

With ASR installed, the old route still works: `$ASR_PY .../asr.py voice/final_narrator/*.wav > voice/asr_narrator_final.json`, then pass that file to `build_timing.py`. The pace json wins when both exist.

## Path D: placeholder

From the `narration/` folder:

```
python3 $MOTION_LIB/core/tools/placeholder_vo.py [--wps 3.0] [--say [VOICE]]
```

It writes silent WAVs (or the macOS `say` voice with `--say`) and `timing_narrator.js` from `script.json` alone, spreading word times by word length. `--wps 3.5` to `4` approximates the target pace (default 3.0). The video can be timed first and re-voiced later: swap the WAVs, rerun `finalize.sh`, rerun `build_vo.py`, and the word cues follow.

## Delivery rules

Delivery is human, unhurried and emphasised. The viewer must follow every line on the first listen.
- **Breathing room.** A real pause at every sentence end and between list beats ("Fine. Fine. Fine."). Start with 0.3-0.4 s between lines (`gap` in `episode.json`, default 0.22; `gaps` adds an extra beat after one named line) and judge by ear. Pauses go at sentence or clause boundaries only: a 0.2 s gap inside a phrase, such as "copied it, | by taking...", reads as lag. Use the 30 s ceiling for air instead of cutting pauses, and never slow the tempo to get it.
- **Emphasis.** The one or two words each line leans on (the number, the twist word, the verdict) come from the script's staging table. Pass them as a per-line `instruct`.
- **Tone.** Conversational and a little amused, not announcer energy. Questions sound like real questions, the catch line sounds curious, the CTA is warm. The narrator still doesn't joke; the picture does.
- **Clone model (no instructions).** With a clone voice (`ref_audio` + `ref_text`, Base model) the levers are the text and the take. Write the pauses into the punctuation (full stops between list beats, a short question as its own sentence). Generate several seeds of one continuous take (`SEED_BASE`, `takes_per_line`), pick the one with the most pitch movement on the target words and ASR 0.9 or better, and re-take a flat line alone (`ONLY=l4_peak`) and splice it at a sentence boundary. Only clone a voice you have the right to use.
- **Whole script at once** gives consistent prosody: generate one take, `split_take.py` cuts it per line, then set `"gap": 0` in `episode.json`. Per scene is fine if the user prefers.
- **Play the first three lines to the user before the full run**: `ONLY=l1_hook,l2_...,l3_... narrate.sh ...`.

## Captions and timing

From `video/`, once `episode.json` has `scenes`:

```
python3 $MOTION_LIB/core/tools/build_vo.py     # public/vo/voiceover.wav, public/vo/captions.json, src/plan.json
python3 $MOTION_LIB/core/tools/timing_md.py    # Timing section of STORYBOARD.md
```

`captions.json` is one entry per spoken word with millisecond start and end; scene boundaries come from the word timestamps; the video's length is `plan.json`'s `durationInFrames`. Both files are plain data, readable by any engine. `build_vo.py` flags any scene under 1.2 s. With no library, transcribe the WAV with any speech-to-text tool that returns word timestamps.

Captions are one tag per word, pages of 1-3 words, at 3.5-4 words a second (`$MOTION_LIB/style/$MOTION_STYLE/docs/style-guide.md`, sections 11 and 13). How a tag is drawn is the style's and the engine's business, not this stage's.

## Hand-off

Next is `motion-build`, which finalises `episode.json`, the cue list and `HANDOFF.md`. It reads `src/plan.json` and the Timing section of `video/STORYBOARD.md`. Any later voice change reruns `build_vo.py` and `timing_md.py`, and the plan — not any engine's scene code — is what moves.
