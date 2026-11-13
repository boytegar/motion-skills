#!/usr/bin/env bash
# One-shot QA sheet: stills across a range at low scale, then a labelled contact sheet.
#   usage (from an episode project dir): $MOTION_LIB/core/tools/qa.sh NAME [FROM-TO] [NTH=12] [SCALE=0.3] [COLS=9]
#   -> out/qa/NAME/ (frames) and out/qa/NAME_0.jpg, _1.jpg ... (<= 45 frames per sheet)
#   the engine comes from the project's episode.json ("engine": remotion | fframes)
#   env: MOTION_PY (python with Pillow, default python3), CONC (render tabs, default 2)
set -euo pipefail

rp() { python3 -c 'import os,sys;print(os.path.realpath(sys.argv[1]))' "$1"; }

LIB=$(dirname "$(rp "${BASH_SOURCE[0]}")")
if [ -f "$LIB/../env.sh" ]; then . "$LIB/../env.sh"; fi   # fills MOTION_PY and resolves the style from the config file, if any
PY=${MOTION_PY:-python3}
STYLE=$(motion_style_of ".")             # episode.json "style"; fails if the episode names none
ENGINE=$(motion_engine_of ".")           # episode.json "engine"; remotion when absent
NAME=${1:?usage: qa.sh NAME [FROM-TO] [NTH=12] [SCALE=0.3] [COLS=9]}
case "$NAME" in */*|.*) echo "qa.sh: NAME must be a plain folder name, got '$NAME'" >&2; exit 2 ;; esac
RANGE=${2:-}; NTH=${3:-12}; SCALE=${4:-0.3}; COLS=${5:-9}
rm -rf "out/qa/$NAME"; mkdir -p out/qa
echo "qa: style $STYLE, engine $ENGINE" >&2

if [ "$ENGINE" = "fframes" ]; then
  # the fframes engine writes labelled strips itself (style/<style>/fframes/tools/ff.sh)
  R=$(python3 - "$RANGE" <<'PY'
import sys
r = sys.argv[1]
print(r.replace('-', '..') if r else 'all')
PY
)
  "$MOTION_LIB/style/$STYLE/fframes/tools/ff.sh" strip "$R" -n "$NTH" --columns "$COLS" -o "out/qa/$NAME" >/dev/null
else
  COMP=$(python3 -c "import json;print(json.load(open('episode.json'))['comp'])")
  FRAMES=(); [ -n "$RANGE" ] && FRAMES=("--frames=$RANGE")
  npx remotion render "$COMP" "out/qa/$NAME" --sequence ${FRAMES[@]+"${FRAMES[@]}"} --every-nth-frame="$NTH" --scale="$SCALE" --gl=angle \
    --concurrency="${CONC:-2}" --muted --log=error --image-format=jpeg 2>&1 | grep -iv deprecat || true
fi
OFF=${RANGE%%-*}
"$PY" "$LIB/sheet.py" "out/qa/$NAME" "out/qa/$NAME" "$NTH" 45 "$COLS" 200 "${OFF:-0}"
