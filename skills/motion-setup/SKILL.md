---
name: motion-setup
description: Install, check or upgrade the planning toolchain. Runs the doctor, picks the components the plan pipeline needs, runs install.sh with flags and fixes dependency errors. Use for "set up motion", "check my install", "set up the voice", "why won't the plan build".
---

# Motion setup

A guided wrapper around `install.sh`, scoped to this skill set: strategy, hook, script, storyboard, voice, plan and handoff. It checks what is installed, asks what the user wants, runs the installer non-interactively and reports. Use it whenever another skill finds something missing.

This skill installs **no render engine**. Engines (and their scene code, preview commands and renderers) belong to their own skill; a planning stage never installs one.

## Preflight

- Find the repo root (where `install.sh` lives): `readlink` on the installed skill folder ends in `<root>/skills/motion-setup`. Look in each harness's skills folder in turn (the table is `harnesses.sh`; typically `~/.claude/skills`, `~/.agents/skills`, `~/.config/opencode/skills`, or the same under `<project>/`). Failing that, `MOTION_HOME=` in `${XDG_CONFIG_HOME:-~/.config}/motion-explainer/config.env`. If neither exists, the skills probably came from `npx skills add` (skills only, no library). Ask where to clone the repo (default `~/motion-explainer`), run `git clone https://github.com/0xS4N4TG/motion-explainer <dir>`, and use that as the root. The installer then replaces the copied skill folders with links into the checkout (it backs up real folders first). Never guess a path.
- Run every `install.sh` command from that root. It needs bash, and macOS or Linux (Windows: use WSL).
- The installer's own prompts need a terminal, which the agent's shell tool lacks. Always pass `--yes` and the answers as flags. Tell the user they can run `./install.sh` themselves for the interactive menu.

## 1. Doctor

```
./install.sh --doctor
```

Add `--project DIR`, `--skills-dir DIR` or `--target <names>` if the skills went somewhere other than the detected defaults; the doctor only looks where you point it, and with no `--target` one complete install location is enough. It prints `[ok]`, `[--]` (missing) or `[!!]` (wrong). Summarise it in plain words, and only for what this skill set uses:
- skills installed, config file and `MOTION_LIB`
- ffmpeg, python3
- Python tools (numpy, Pillow, soundfile)
- voice: `qwen_tts` importable, a CustomVoice model in the Hugging Face cache
- library files present

Say which planning mode that adds up to (see `motion-explainer`, Planning modes).

Ignore the doctor's engine lines (`node`, `npm`, `rsync`, a renderer binary, Chrome, a style's engine shim). They belong to an engine's own skill. If one is `[--]`, say in one line that it is not needed for planning and leave it.

## 2. Ask

Ask only what is still open, in one call (your agent's question tool, for example AskUserQuestion in Claude Code). Run `./install.sh --dry-run --mode custom --components skills --yes` first if you need to know which harnesses it detected (`--target auto`, the default):

| Question | Options |
|---|---|
| How much? | **Plan only**: skills, seconds. **Plan + audio files**: adds ffmpeg and the Python tools (caption track, cue list, placeholder voice), ~0.7 GB and a few minutes. **Plan + real voiceover**: adds local TTS, +4-6 GB (a 1.5 GB venv plus a 2.3 or 4.2 GB model) and a 2-minute smoke test. **Custom**: pick components. |
| Which agents? | **Detected on this machine** (default, `--target auto`; Claude Code if none), a list such as `--target claude,codex,gemini`, or `--target all`. Names are in `harnesses.sh`; the README's *Works with* table says which are tested. |
| Where do the skills go? | User-wide (the default) or one project (`--project DIR`). |
| Copy or link? | Symlink (default, updates with `git pull`) or `--copy` (a snapshot). |
| TTS model size (real voiceover) | 1.7B, better prosody, ~4.2 GB (default). 0.6B, lighter and faster, ~2.3 GB. |
| Voice-clone model (real voiceover) | No (default), or `--with-clone`: the Base model of the same size again. |

Confirm before any multi-GB download. On a machine with no GPU (not Apple Silicon, no NVIDIA), TTS generation takes minutes per line; suggest recording the voiceover or the placeholder voice instead.

Skip the scene-agent model questions unless the user asks: those flags (`--claude-model`, `--opencode-model`, `--agent-model`) tune a scene-building subagent, which belongs to an engine skill.

## 3. Install

```
./install.sh --mode custom --components skills,deps,pytools,verify --yes [--target auto|all|claude,codex,...] [--skills-dir DIR | --project DIR] [--copy]
./install.sh --mode custom --components skills,deps,pytools,tts,verify --yes [--tts-model 1.7B|0.6B] [--with-clone]
```

- Add `--dry-run` first if the user wants to see the plan; it changes nothing.
- Always spell out `--components`. The named modes (`--mode standard`, `--mode full`) also install a specific engine's agent skills, which this skill set never uses; do not reach for them here.
- Skills stay symlinks (or copies with `--copy`). The installer writes `MOTION_HOME`, `MOTION_LIB`, `MOTION_PY`, `QWEN_TTS_PY` and `ASR_PY` into `~/.config/motion-explainer/config.env`. An existing skill folder that differs from this checkout is backed up to `~/.config/motion-explainer/backups/` (not into the skills folder, so it is not loaded twice); an identical copy, such as one from `npx skills add`, is just replaced.
- What each component does: `skills` links this set into each selected harness. `deps` checks the shared tools (ffmpeg, python3). `pytools` builds a venv with numpy, Pillow and soundfile, used for captions, cue lists and contact sheets. `tts` adds `$MOTION_LIB/core/voice/setup_tts.sh`: a voice venv with Qwen3-TTS, an ASR backend (mlx-whisper on Apple Silicon, faster-whisper elsewhere), the model download and a one-line smoke test. `verify` runs the self-test.
- Minutes-long steps (venvs, model download) run in the background; check the output, do not hold a foreground call.

## 4. Verify and report

Run `./install.sh --doctor` again with the same `--project`, `--skills-dir` or `--target`. Report what is installed, what failed (the installer prints `needs attention:` with component names), and the fix. A new Claude Code, Codex or Gemini CLI session picks up newly linked skills (restart it); opencode reloads configuration automatically. Then suggest "make an explainer about <link>", or one stage at a time ("write a hook for...", "storyboard this script", "set up the voice").

## Troubleshooting

| Symptom | Fix |
|---|---|
| `ffmpeg` missing | `brew install ffmpeg` or `apt install ffmpeg`, then rerun with `--components deps,verify`. |
| Voice venv: "need Python 3.10-3.12" | Qwen3-TTS has no wheels for newer Python. Install `uv` (https://docs.astral.sh/uv/) or `brew install python@3.12`, rerun the `tts` component. |
| Hugging Face download stalls or fails | Rerun; finished files are kept and the script retries 3 times. It sets `HF_HUB_DISABLE_XET=1`; set it yourself for manual downloads. |
| Self-test fails | Run `$MOTION_LIB/core/tools/selftest.sh` directly to see the failing step. |
| Skills don't show | Restart Claude Code, Codex or Gemini CLI (opencode reloads). Check the harness's folder (`harnesses.sh`; `~/.agents/skills` for most) and run `./install.sh --doctor`. |
| opencode: "Configuration is invalid ... tools" | An old copy of a Claude agent file sits in an opencode agents folder. Rerun `./install.sh --target opencode`. |
| Add the Python tools later | `./install.sh --mode custom --components pytools,verify --yes` |
| Add the voice later | `./install.sh --mode custom --components tts --yes [--tts-model 0.6B] [--with-clone]` |
| Remove it | `./install.sh --uninstall` removes the links it recorded. It leaves `config.env`, the venvs under `~/.local/share/motion-explainer` and the Hugging Face cache; delete those by hand for the space. |
| Windows | Not supported directly. Use WSL. |

## Hand-off

Return to the skill that sent you here, or to `motion-explainer` for the stage table. Files hold the state: `config.env` carries the library path, so the other skills resolve it without asking again.

If the user asked about rendering, preview or a renderer's own dependencies, that is an engine's skill, not this one. Say so in one line and stop.
