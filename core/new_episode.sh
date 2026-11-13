#!/usr/bin/env bash
# Scaffold a new episode from the shared library, in the style and engine you name.
#   usage: new_episode.sh <series dir> <NN-slug> <CompositionId> [out-name] [--style NAME] [--engine remotion|fframes]
#   e.g.   new_episode.sh ~/videos/my-series 04-mcp McpPapercut mcp-papercut --style papercut --engine remotion
#
# Creates <series>/episodes/<NN-slug>/{research,narration/voice,video}. An existing episode folder is fine (the
# earlier stages write research/, narration/script.json and video/STORYBOARD.md there); files already present are
# never overwritten. engine + style are written into video/episode.json.
#
# The project is the engine's template plus the style's assets:
#   remotion: the template project + the style kit synced into src/lib/; node_modules from $DONOR, else the newest
#             sibling project's node_modules, else `npm i`.
#   fframes:  a Cargo crate whose src/ is generated from plan.json at build time (see the style's fframes/README.md);
#             `cargo build --release` is left to the first render.
set -euo pipefail

rp() { python3 -c 'import os,sys;print(os.path.realpath(sys.argv[1]))' "$1"; }

usage() { echo "usage: $0 <series dir> <NN-slug> <CompositionId> [out-name] [--style NAME] [--engine remotion|fframes]" >&2; exit 2; }

POS=(); STYLE=""; ENGINE=""
while [ $# -gt 0 ]; do
  case "$1" in
    --style) STYLE=${2:?--style needs a style folder name}; shift 2 ;;
    --engine) ENGINE=${2:?--engine needs remotion or fframes}; shift 2 ;;
    -h|--help) sed -n '2,16p' "$0"; exit 0 ;;
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
[ -f "$MOTION_LIB/style/$STYLE/style.json" ] || { echo "unknown style '$STYLE' (have: $(motion_styles | tr '\n' ' '))" >&2; exit 2; }
S=$MOTION_LIB/style/$STYLE

# --- engine: explicit, else the style's first engine ---
if [ -z "$ENGINE" ]; then ENGINE=$(python3 -c "import json,sys;e=json.load(open(sys.argv[1])).get('engines',['remotion']);print(e[0])" "$S/style.json"); fi
case "$ENGINE" in remotion|fframes) ;; *) echo "--engine must be remotion or fframes (got '$ENGINE')" >&2; exit 2 ;; esac
[ -d "$S/$ENGINE" ] || { echo "style '$STYLE' has no $ENGINE engine (have: $(ls "$S" | tr '\n' ' '))" >&2; exit 2; }

E=$CH/episodes/$SLUG; P=$E/video
mkdir -p "$E/research" "$E/narration/voice" "$P"

scaffold_remotion() {
  # SYNCED.txt is written last (by sync_kit.sh), so a run that died halfway can be repeated
  if [ -e "$P/src/lib/SYNCED.txt" ]; then echo "already scaffolded: $P" >&2; exit 1; fi
  rsync -a --ignore-existing "$S/remotion/template/" "$P/"   # never overwrites; rsync is a required dependency
  local DONOR=${DONOR:-}
  if [ -z "$DONOR" ]; then
    DONOR=$(ls -td "$CH"/episodes/*/video/node_modules "$CH"/episodes/*/papercut/node_modules 2>/dev/null | head -1 || true)
  fi
  if [ -d "$P/node_modules" ]; then
    :   # left by an earlier run that stopped after the install
  elif [ -n "$DONOR" ] && [ -d "$DONOR" ]; then
    cp -cR "$DONOR" "$P/node_modules" 2>/dev/null || { rm -rf "$P/node_modules"; cp -R "$DONOR" "$P/node_modules"; }
  else
    (cd "$P" && npm i --no-audit --no-fund)
  fi
  if [ -d "$CH/video-assets" ]; then "$S/remotion/sync_kit.sh" "$P" "$CH/video-assets"
  elif [ -d "$CH/papercut-assets" ]; then "$S/remotion/sync_kit.sh" "$P" "$CH/papercut-assets"
  else "$S/remotion/sync_kit.sh" "$P"; fi
}

scaffold_fframes() {
  if [ -e "$P/Cargo.toml" ]; then echo "already scaffolded: $P" >&2; exit 1; fi
  rsync -a --ignore-existing "$S/fframes/template/" "$P/"
  mkdir -p "$P/media"
  # the style's media (fonts, grain, backdrops, textures) is linked, not copied: a style update reaches every episode
  [ -d "$S/remotion/public/fonts" ] && rsync -a "$S/remotion/public/fonts/" "$P/media/fonts/"
  [ -f "$S/remotion/public/bg/grain.png" ] && cp "$S/remotion/public/bg/grain.png" "$P/media/grain.png"
  [ -d "$S/remotion/public/bg/backdrops" ] && mkdir -p "$P/media/backdrops" && rsync -a --exclude '_sheet.jpg' "$S/remotion/public/bg/backdrops/" "$P/media/backdrops/"
  [ -d "$S/remotion/public/bg/textures" ] && mkdir -p "$P/media/textures" && rsync -a "$S/remotion/public/bg/textures/" "$P/media/textures/"
  [ -d "$CH/video-assets" ] && rsync -a "$CH/video-assets/" "$P/media/"
  [ -d "$CH/papercut-assets" ] && rsync -a "$CH/papercut-assets/" "$P/media/"
  printf '%s\n' "synced from style/$STYLE/fframes on $(date '+%Y-%m-%d %H:%M'); do not edit here, edit the style and re-sync" > "$P/media/SYNCED.txt"
  return 0
}

"scaffold_$ENGINE"

python3 - "$P/episode.json" "$COMP" "$OUTN" "$STYLE" "$ENGINE" <<'PY'
import json, sys, os
p, comp, out, style, engine = sys.argv[1:6]
e = json.load(open(p)) if os.path.exists(p) else {}
e.update({"comp": comp, "out": out, "style": style, "engine": engine})
json.dump(e, open(p, "w"), indent=1)
PY
[ -e "$E/narration/build_timing.py" ] || cp "$L/voice/build_timing.py" "$E/narration/"
[ -e "$E/narration/script.json" ] || echo "[]" > "$E/narration/script.json"
echo "new episode: $E  (style $STYLE, engine $ENGINE, comp $COMP -> out/$OUTN.mp4)"
echo "next: research/facts.md -> narration/script.json -> voice/narrate.sh -> voice/finalize.sh -> episode.json -> build steps of $ENGINE"
echo "      (the two voice steps are optional; any TTS works if it produces the files core/voice/README.md lists)"
