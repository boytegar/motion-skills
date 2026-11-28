#!/usr/bin/env bash
# Scaffold a new episode folder in the series, for the style you name.
#   usage: new_episode.sh <series dir> <NN-slug> <CompositionId> [out-name] [--style NAME]
#   e.g.   new_episode.sh ~/videos/my-series 04-mcp McpPapercut mcp-papercut --style papercut
#
# Creates <series>/episodes/<NN-slug>/{research,narration/voice,video}. An existing episode folder is fine (the
# earlier stages write research/, narration/script.json and video/STORYBOARD.md there); files already present are
# never overwritten. The style is written into video/episode.json.
#
# This script scaffolds the PLAN folder only: no build project, no dependency install, no scene code.
# Building the folder belongs to whatever the user builds with, recorded in video/HANDOFF.md.
set -euo pipefail

rp() { python3 -c 'import os,sys;print(os.path.realpath(sys.argv[1]))' "$1"; }

usage() { echo "usage: $0 <series dir> <NN-slug> <CompositionId> [out-name] [--style NAME]" >&2; exit 2; }

POS=(); STYLE=""
while [ $# -gt 0 ]; do
  case "$1" in
    --style) STYLE=${2:?--style needs a style folder name}; shift 2 ;;
    -h|--help) sed -n '2,13p' "$0"; exit 0 ;;
    -*) echo "unknown option: $1" >&2; usage ;;
    *) POS+=("$1"); shift ;;
  esac
done
[ ${#POS[@]} -ge 3 ] || usage

L=$(dirname "$(rp "${BASH_SOURCE[0]}")")
. "$L/env.sh"
CH=$(rp "${POS[0]}"); SLUG=${POS[1]}; COMP=${POS[2]}
OUTN=${POS[3]:-${SLUG#*-}-video}

# --- style: always explicit or from the series, never defaulted silently ---
if [ -z "$STYLE" ]; then
  STYLE=$(motion_style_of "$CH" 2>/dev/null || true)
fi
if [ -z "$STYLE" ]; then
  echo "no style given. available: $(motion_styles | tr '\n' ' ')" >&2
  echo "pass --style <name> (or write {\"style\": \"<name>\"} into the series SERIES.md/episode.json)" >&2
  exit 2
fi
[ -f "$MOTION_STYLES/$STYLE/style.json" ] || { echo "unknown style '$STYLE' (have: $(motion_styles | tr '\n' ' '))" >&2; exit 2; }

E=$CH/episodes/$SLUG; P=$E/video
mkdir -p "$E/research" "$E/narration/voice" "$E/video/out" "$E/video/tools"

python3 - "$P/episode.json" "$COMP" "$OUTN" "$STYLE" <<'PY'
import json, sys, os
p, comp, out, style = sys.argv[1:5]
e = json.load(open(p)) if os.path.exists(p) else {}
e.update({"comp": comp, "out": out, "style": style})
e.pop("engine", None)          # the plan is build-agnostic; the choice lives in HANDOFF.md
json.dump(e, open(p, "w"), indent=1)
PY

# plan files the later stages fill, created empty so a half-run resumes cleanly
[ -e "$E/narration/script.json" ] || echo "[]" > "$E/narration/script.json"
[ -e "$P/tools/cues.json" ] || echo '{"cues": []}' > "$P/tools/cues.json"

echo "new episode: $E  (style $STYLE, comp $COMP -> out/$OUTN.mp4)"
echo "next: motion-strategy (BRIEF) -> motion-hook -> motion-script -> motion-storyboard -> motion-voice -> motion-build"
