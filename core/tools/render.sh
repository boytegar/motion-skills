#!/usr/bin/env bash
# Render the episode in the current project dir -> out/<episode.json "out">.mp4
# The engine comes from episode.json ("engine": remotion | fframes | kyfram).
#   remotion: a wrapper around render.py (bundle once, per-scene ProRes cache, grouped parallel jobs, one final
#             x264 encode, audio mix in parallel). Flags: --workers N (default 3), --conc N (default $CONC or 4),
#             --force, --keep.
#   fframes:  `cargo run --release -- render` in the project (builds on first use; same flags as the fframes CLI).
#   kyfram:   render_kyfram.py (one .lgv per scene, cached per scene, concat + one x264 encode).
#             Flags: --workers N (default 3), --force, --keep, --voice (mux public/vo/voiceover.wav).
# env: MOTION_PY (python with numpy + soundfile for the audio mix, default python3), CARGO
set -euo pipefail
HERE=$(dirname "$(python3 -c 'import os,sys;print(os.path.realpath(sys.argv[1]))' "${BASH_SOURCE[0]}")")
if [ -f "$HERE/../env.sh" ]; then . "$HERE/../env.sh"; fi   # fills MOTION_PY and resolves the config, if any
ENGINE="remotion"
[ -f episode.json ] && ENGINE=$(motion_engine_of ".")
case "$ENGINE" in
  fframes) exec "${CARGO:-cargo}" run --release -- render "$@" ;;
  kyfram)  exec "${MOTION_PY:-python3}" -u "$HERE/render_kyfram.py" "$@" ;;
  *)       exec "${MOTION_PY:-python3}" -u "$HERE/render.py" "$@" ;;
esac
