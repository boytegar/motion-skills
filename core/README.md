# Motion core

The scripts and data behind a plan-only motion library: 109 style packs in `../style/`, ten planning skills in `../skills/`, and the tools here that turn a voiceover into a timed, captioned, cued plan folder. Nothing here writes scene code and nothing here renders: building the folder belongs to whatever the user builds with, recorded in `video/HANDOFF.md`.

## Layout

```
core/
  README.md  HARVEST.md  CHANGELOG.md
  env.sh                       sourced by the scripts; fills the env vars below from ~/.config/motion-explainer/config.env
  init.sh                      pick a default style, write design.md + agents.md for a series
  new_episode.sh               scaffold <series>/episodes/NN-slug/ (plan folders only)
  audio/
    CATALOG.md                 every SFX and bed preset, one line each (generated)
    engine.js                  Web Audio kit (noise, tone, pop, chime, pluck...) and one parametric music bed
    sfx/<name>.js              one file per effect, first line "// @sfx name | tags | sec | description | origin"
    beds/<preset>.json         music beds as data (bpm, instrument, chords, pattern, percussion...)
  mold/                        template for a generated style pack (style_ref scaffold)
  tools/
    build_vo.py                narration lines + timing -> voiceover.wav, captions.json, src/plan.json
    cues.py                    per-scene SFX rows -> tools/cues.json
    timing_md.py               writes the "Timing" section of STORYBOARD.md from plan.json
    placeholder_vo.py          silent (or macOS `say`) narration, so a reel can be timed with no TTS
    dialogue_timeline.py       two-speaker dialogue -> timeline.json + vo.wav
    catalog.py                 regenerate the audio catalog
    sheet.py                   labelled contact sheets from stills a builder produced
    style_ref.py               ingest / lint / probe / scaffold for style packs
  voice/                       optional local voiceover pipeline, see voice/README.md
```

## Quick start

Prerequisites.

- Python 3.10 or newer with numpy and Pillow (contact sheets) and soundfile (audio).
- ffmpeg and ffprobe on PATH for timing and placeholder narration.
- Optional: Qwen3-TTS and a speech-to-text backend for the voice pipeline (`voice/README.md`). Without them you can use `placeholder_vo.py`, or record the narration yourself.

Setup.

```
git clone <this repo>
export MOTION_LIB="$PWD/core"
```

`../install.sh` at the repo root can do the same setup for you (skill links, Python tools, optionally the voice venv). `./install.sh --doctor` lists what it finds.

### Initialize defaults for a series

Run this once inside a series folder (or pass `--series`):

```sh
$MOTION_LIB/core/init.sh [--series ~/videos/my-series]
```

It asks for the default **style**, stores it as `MOTION_STYLE` in the motion config, then creates `design.md` (style DNA/reference paths) and `agents.md` (the defaults/workflow for coding agents). Future agents read `agents.md`; they should not ask again for the style.

Make a first episode folder.

```
$MOTION_LIB/core/new_episode.sh ~/videos/my-series 01-first Episode first-reel --style papercut
```

A series is a folder that holds `episodes/`, and optionally `voice/voice.json`, `video-assets/` (textures and props the plan can name) and `SERIES.md` (audience, voice and palette notes).

## An episode, step by step

The planning skills own these steps; run them from the series folder.

1. `$MOTION_LIB/core/new_episode.sh . NN-slug CompId out-name --style <name>`
2. Research into `episodes/NN-slug/research/facts.md`, then write `narration/script.json` as `[{"id": "l1_hook", "text": "..."}, ...]`.
3. Narration takes: `$MOTION_LIB/voice/narrate.sh voice/voice.json episodes/NN-slug/narration`. One TTS process at a time. Skip this and step 4 with `placeholder_vo.py` or your own recording.
4. Pick takes, fix pace and write timing: `$MOTION_LIB/voice/finalize.sh voice/voice.json episodes/NN-slug/narration episodes/NN-slug/narration/voice/takes/narrator [num.json]`.
5. From `video/`: `python3 $MOTION_LIB/tools/build_vo.py`, then `python3 $MOTION_LIB/tools/cues.py`, then `python3 $MOTION_LIB/tools/timing_md.py`. `build_vo.py` flags any scene under 1.2 s.
6. Write `video/HANDOFF.md`: the contract the builder reads, naming their build target and listing every file the folder carries.

## Token rules

These keep an agent's context small.

- Read `style/<name>/SKILL.md` and `docs/style-guide.md`, and `audio/CATALOG.md` (about 9 KB), not any build source.
- Reuse before writing: name an existing camera move, transition, prop or set instead of inventing one.
- Pick SFX by name. A new sound is a new `sfx/<name>.js` file with the header line, never an edit inside a large score file.
- A music bed is a JSON preset. A new mood is a new preset.
- Beds with `"style": "night"` (`beds/night-rain-piano-F.json`) replace the plucked loop with rain on glass, room tone, a sparse felt piano and a soft pad through a convolution reverb. They also take `thinFrom`, `thinTo` and `resolveAt`. Beds without `style` render the plain loop.
- Contact sheets of at most 45 frames at 200 px keep image tokens low.

## Environment variables

The scripts read these. `env.sh` supplies the defaults and also loads `KEY=value` lines from `${MOTION_CONFIG:-~/.config/motion-explainer/config.env}`; anything already set in your shell wins. A commented template for that file is `core/config.env.example` (copy it, `chmod 600` it, uncomment one provider block).

| Variable | Meaning | Default |
|---|---|---|
| `MOTION_LIB` | Absolute path to this folder. The skills use it. | the folder `env.sh` sits in |
| `MOTION_PY` | Python with numpy, Pillow and soundfile. Used by `sheet.py` and the timing tools. | `python3` |
| `MOTION_STYLE` | Default style folder name under `style/`. | unset (ask the user) |
| `QWEN_TTS_PY` | Python with `qwen-tts` and torch installed. Used by `voice/narrate.sh`, and by `finalize.sh` for the pitch pass (needs librosa). | `python3` |
| `QWEN_TTS_HOME` | Folder to add to `PYTHONPATH`, only if you run `qwen_tts` from a source checkout instead of pip. | unset |
| `QWEN_TTS_MODEL` | Model repo for custom-voice mode when `voice.json` has no `model` key. | `Qwen/Qwen3-TTS-12Hz-1.7B-CustomVoice` |
| `ASR_PY` | Python with `mlx-whisper` or `faster-whisper`, for `voice/asr.py`. | `python3` |
| `FASTER_WHISPER_MODEL` | Model name when `asr.py` uses faster-whisper. | `large-v3-turbo` |
| `MOTION_CONFIG` | Path of the config file (the `.env` the voice settings live in). | `~/.config/motion-explainer/config.env` |
| `TTS_PROVIDER` | `openai`, `elevenlabs` or `custom` makes `voice/narrate.sh` call `voice/tts_provider.py` instead of the local model. Unset picks `custom` when `TTS_URL` is set, else local Qwen3-TTS. | unset |
| `TTS_URL` | Endpoint to POST to. Also selects the provider when `TTS_PROVIDER` is empty. With `openai`/`elevenlabs` it replaces the vendor URL; `custom` requires it. `{voice}` and `{model}` in the URL are substituted. | unset (vendor URL) |
| `TTS_API_KEY` | Key for the hosted provider (`TTS_<PROVIDER>_KEY` also works). | unset |
| `TTS_MODEL`, `TTS_VOICE`, `TTS_SPEED`, `TTS_LANG` | Hosted provider model, voice, pace (0.25-4.0) and language. | provider defaults, `1.0` |
| `TTS_AUTH`, `TTS_PAYLOAD`, `TTS_FORMAT`, `TTS_TIMEOUT` | `custom` provider: auth (`none`, `bearer`, or a header name), JSON body template with `{text}` `{voice}` `{model}` `{lang}` `{speed}`, response format, timeout. | `none`, `{"text": "{text}", "voice": "{voice}", "model": "{model}"}`, `mp3`, `90` |
| `PACE_TARGET` | Words per second the pace report (`tools/pace.py`) judges each line against. | `3.3` |

The voice scripts take more (`ROLE`, `SCRIPT`, `SKIP_PICK`, `ONLY`, `TAKES`, `SEED_BASE`, and the usual Hugging Face variables). They are listed in `voice/README.md`.

## Status

The scripts were written on macOS (Apple Silicon) for one author's workflow and have only been run end to end there. The shell scripts are bash and pass `bash -n`, the Python passes `py_compile`. Linux should work, Windows probably needs WSL, and neither has been tried.

Things to know before you rely on it.

- The voice pipeline is optional and the least tested part. It was built around Qwen3-TTS and mlx-whisper. The CUDA path in `narrate.py` and the faster-whisper path in `asr.py` were written afterwards and not run on real hardware by the author.
- `dialogue_timeline.py`, the `SPEAKER` and `ROLE` options in the voice scripts support two-speaker videos.
- The audio catalog is generated. Run `tools/catalog.py` after you add or change anything in the audio folder, or agents will read a stale one.
