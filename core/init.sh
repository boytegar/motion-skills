#!/usr/bin/env bash
# motion init – interactive setup: pick a default style and engine, persist to config.
#
#   ./core/init.sh
#   $MOTION_LIB/core/init.sh
#
# Writes MOTION_STYLE and MOTION_ENGINE into
#   ~/.config/motion-explainer/config.env   (via motion_config_set from env.sh)
#
# Then generates two reference documents in the series folder (or $PWD):
#   design.md   – the chosen style's visual DNA, palette, component catalogue reference
#   agents.md   – agent defaults: engine, style, preview command and skill call reference
#
# Options:
#   --series DIR   generate design.md / agents.md in DIR instead of $PWD
#   --yes, -y      use the first style/engine without prompting (CI/non-interactive)
#   --dry-run      print what would happen, change nothing
set -euo pipefail

L="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
. "$L/env.sh"

SERIES="${PWD}"; YES=0; DRY=0
while [ $# -gt 0 ]; do
  case "$1" in
    --series) SERIES="${2:?--series needs a directory}"; shift 2 ;;
    --yes|-y) YES=1; shift ;;
    --dry-run) DRY=1; shift ;;
    -h|--help) sed -n '2,16p' "$0"; exit 0 ;;
    *) echo "unknown option: $1 (see --help)" >&2; exit 2 ;;
  esac
done

[ -t 1 ] && { B=$'\033[1m'; G=$'\033[32m'; Y=$'\033[33m'; C=$'\033[36m'; N=$'\033[0m'; } || { B=""; G=""; Y=""; C=""; N=""; }
say()  { printf '%s==>%s %s\n' "$B" "$N" "$*"; }
ok()   { printf '  %s[ok]%s %s\n' "$G" "$N" "$*"; }
info() { printf '  %s...%s %s\n' "$C" "$N" "$*"; }
run()  { [ "$DRY" = 1 ] && printf '  (dry-run) %s\n' "$*" || "$@"; }
ask()  { [ "$YES" = 1 ] && return 0; printf '%s ' "$1"; read -r REPLY; }

# ------------------------------------------------------------------ style
say "motion init"
echo

STYLES=(); while IFS= read -r s; do STYLES+=("$s"); done < <(motion_styles)
if [ "${#STYLES[@]}" -eq 0 ]; then
  echo "no styles found under $MOTION_STYLES" >&2; exit 2
fi

STYLE_CHOICE="${MOTION_STYLE:-}"

if [ -z "$STYLE_CHOICE" ] || [ "$YES" = 0 ]; then
  # Renderable styles first, then the docs-only imports, so a long tagline and an
  # added "[docs only]" note never push the readable list out of shape.
  REND=(); DOCS=()
  for s in "${STYLES[@]}"; do if motion_renderable "$s"; then REND+=("$s"); else DOCS+=("$s"); fi; done

  echo "${B}Available styles:${N}"
  idx=1
  for s in "${REND[@]}"; do
    tl="$(python3 -c "import json,sys; print(json.load(open(sys.argv[1])).get('tagline','')[:84])" "$MOTION_STYLES/$s/style.json" 2>/dev/null || echo "")"
    printf '  %2d) %-22s %s\n' "$idx" "$s" "$tl"
    idx=$((idx+1))
  done
  if [ "${#DOCS[@]}" -gt 0 ]; then
    echo "${B}  docs-only imports (rules and prompts; script and storyboard, no render):${N}"
    for s in "${DOCS[@]}"; do
      tl="$(python3 -c "import json,sys; print(json.load(open(sys.argv[1])).get('tagline','')[:84])" "$MOTION_STYLES/$s/style.json" 2>/dev/null || echo "")"
      printf '  %2d) %-22s %s\n' "$idx" "$s" "$tl"
      idx=$((idx+1))
    done
  fi
  echo
  if [ "$YES" = 1 ]; then
    # never auto-select something that cannot render
    STYLE_CHOICE="${REND[0]}"
    printf '  auto-selected: %s\n' "$STYLE_CHOICE"
  else
    ORDERED=("${REND[@]}" "${DOCS[@]}")   # the numbered order printed above
    default_idx=1
    for i in "${!ORDERED[@]}"; do [ "${ORDERED[$i]}" = "${MOTION_STYLE:-}" ] && default_idx=$((i+1)) && break; done
    printf 'Choose style [%d]: ' "$default_idx"
    read -r c
    c="${c:-$default_idx}"
    if [[ "$c" =~ ^[0-9]+$ ]] && [ "$c" -ge 1 ] && [ "$c" -le "${#ORDERED[@]}" ]; then
      STYLE_CHOICE="${ORDERED[$((c-1))]}"
    else
      echo "invalid choice: $c" >&2; exit 2
    fi
  fi
fi

STYLE_JSON="$MOTION_STYLES/$STYLE_CHOICE/style.json"
[ -f "$STYLE_JSON" ] || { echo "unknown style '$STYLE_CHOICE'" >&2; exit 2; }

# A docs-only pack (Lemo-Opuscar import) has rules but no engine: it can be scripted and
# storyboarded, it cannot be rendered. init.sh writes agents.md/design.md naming an engine,
# so it must stop here rather than emit a config that cannot build.
if ! motion_renderable "$STYLE_CHOICE"; then
  echo "style '$STYLE_CHOICE' is docs-only: it ships rules but no engine, so it cannot be rendered." >&2
  echo "  usable for scripting: motion-strategy, motion-hook, motion-script, motion-storyboard" >&2
  echo "  read:  $MOTION_STYLES/$STYLE_CHOICE/docs/style-guide.md" >&2
  echo "  renderable styles: $(motion_styles_renderable | tr '\n' ' ')" >&2
  echo "  to render it, port an engine into $MOTION_STYLES/$STYLE_CHOICE and add it to \"engines\" in its style.json" >&2
  exit 2
fi

# ------------------------------------------------------------------ engine
ENGINES=(); while IFS= read -r e; do ENGINES+=("$e"); done < <(
  python3 -c "import json,sys; [print(e) for e in json.load(open(sys.argv[1])).get('engines',['remotion'])]" "$STYLE_JSON"
)

ENGINE_CHOICE="${MOTION_ENGINE:-}"

if [ -z "$ENGINE_CHOICE" ] || [ "$YES" = 0 ]; then
  echo
  echo "${B}Available engines for ${STYLE_CHOICE}:${N}"
  idx=1
  for e in "${ENGINES[@]}"; do
    case "$e" in
      remotion) desc="React (Remotion) – TypeScript/TSX scenes, npx remotion studio" ;;
      fframes)  desc="Rust crate (fframes) – cargo build, cargo run --release -- preview" ;;
      kyfram)   desc="Logos DSL (.lgv) – kyfram view <scene.lgv> after render" ;;
      *)        desc="" ;;
    esac
    printf '  %2d) %-10s %s\n' "$idx" "$e" "$desc"
    idx=$((idx+1))
  done
  echo
  if [ "$YES" = 1 ]; then
    ENGINE_CHOICE="${ENGINES[0]}"
    printf '  auto-selected: %s\n' "$ENGINE_CHOICE"
  else
    default_eidx=1
    for i in "${!ENGINES[@]}"; do [ "${ENGINES[$i]}" = "${MOTION_ENGINE:-}" ] && default_eidx=$((i+1)) && break; done
    printf 'Choose engine [%d]: ' "$default_eidx"
    read -r c
    c="${c:-$default_eidx}"
    if [[ "$c" =~ ^[0-9]+$ ]] && [ "$c" -ge 1 ] && [ "$c" -le "${#ENGINES[@]}" ]; then
      ENGINE_CHOICE="${ENGINES[$((c-1))]}"
    else
      echo "invalid choice: $c" >&2; exit 2
    fi
  fi
fi

# Validate engine is available for style
VALID_ENGINES="$(python3 -c "import json,sys; print(' '.join(json.load(open(sys.argv[1])).get('engines',['remotion'])))" "$STYLE_JSON")"
case " $VALID_ENGINES " in
  *" $ENGINE_CHOICE "*) ;;
  *) echo "engine '$ENGINE_CHOICE' is not available for style '$STYLE_CHOICE' (available: $VALID_ENGINES)" >&2; exit 2 ;;
esac

# ------------------------------------------------------------------ persist
echo
say "Saving defaults"
run motion_config_set MOTION_STYLE "$STYLE_CHOICE"
run motion_config_set MOTION_ENGINE "$ENGINE_CHOICE"
[ "$DRY" = 1 ] || { MOTION_STYLE="$STYLE_CHOICE"; MOTION_ENGINE="$ENGINE_CHOICE"; }
ok "MOTION_STYLE=$STYLE_CHOICE  MOTION_ENGINE=$ENGINE_CHOICE  → $_pc_cfg"

# ------------------------------------------------------------------ design.md
SERIES="$(cd "$SERIES" && pwd)"
DESIGN_MD="$SERIES/design.md"

STYLE_TITLE="$(python3 -c "import json,sys; print(json.load(open(sys.argv[1])).get('title','$STYLE_CHOICE'))" "$STYLE_JSON")"
STYLE_TAGLINE="$(python3 -c "import json,sys; print(json.load(open(sys.argv[1])).get('tagline',''))" "$STYLE_JSON")"
STYLE_DESC="$(python3 -c "import json,sys; print(json.load(open(sys.argv[1])).get('description',''))" "$STYLE_JSON")"

GUIDE_REL="$MOTION_STYLES/$STYLE_CHOICE/docs/style-guide.md"
CATALOG_REL="$MOTION_STYLES/$STYLE_CHOICE/remotion/CATALOG.md"
FFRAMES_README="$MOTION_STYLES/$STYLE_CHOICE/fframes/README.md"
KYFRAM_PRELUDE="$MOTION_STYLES/$STYLE_CHOICE/kyfram/PRELUDE.lgv"

say "Generating design.md"
run cat > "$DESIGN_MD" <<DESIGN_EOF
# Design Reference — ${STYLE_TITLE}

> ${STYLE_TAGLINE}

**Style:** \`${STYLE_CHOICE}\`
**Default engine:** \`${ENGINE_CHOICE}\`

${STYLE_DESC}

## Style DNA

See the full style guide:
\`\`\`
${GUIDE_REL}
\`\`\`

## Engine catalogue

| Engine | Catalogue / entry |
|--------|-------------------|
| remotion | \`${CATALOG_REL}\` |
$([ -f "$FFRAMES_README" ] && echo "| fframes  | \`${FFRAMES_README}\` |")
$([ -f "$KYFRAM_PRELUDE" ] && echo "| kyfram   | \`${KYFRAM_PRELUDE}\` |")

## Palette (${STYLE_CHOICE})

$(python3 - "$GUIDE_REL" 2>/dev/null <<'PYEOF'
import sys, re
path = sys.argv[1]
try:
    text = open(path).read()
    palettes = re.findall(r'(?i)(palette.*?)\n(?=##|\Z)', text, re.S)
    if palettes:
        print(palettes[0][:600].strip())
    else:
        print("See style-guide.md for the full palette.")
except Exception:
    print("See style-guide.md for the full palette.")
PYEOF
)

## Component quick reference

For the full component catalogue, read:
- **remotion:** \`${CATALOG_REL}\`
$([ -f "$FFRAMES_README" ] && echo "- **fframes:** \`${FFRAMES_README}\`")
$([ -f "$KYFRAM_PRELUDE" ] && echo "- **kyfram:** \`${KYFRAM_PRELUDE}\`")

## Motion & timing

See \`${GUIDE_REL}\` section on camera, boil, and motion timings.

---
*Generated by \`core/init.sh\` on $(date '+%Y-%m-%d'). Re-run \`$MOTION_LIB/core/init.sh --series $SERIES\` to refresh.*
DESIGN_EOF

ok "design.md → $DESIGN_MD"

# ------------------------------------------------------------------ agents.md
AGENTS_MD="$SERIES/agents.md"

case "$ENGINE_CHOICE" in
  remotion) PREVIEW_CMD="cd \$EPISODE_DIR/video && npx remotion studio" ;;
  fframes)  PREVIEW_CMD="cd \$EPISODE_DIR/video && cargo run --release -- preview" ;;
  kyfram)   PREVIEW_CMD="kyfram view \$EPISODE_DIR/video/out/cache/kyfram/build/<Scene>.lgv" ;;
esac

say "Generating agents.md"
run cat > "$AGENTS_MD" <<AGENTS_EOF
# Agent Defaults — ${STYLE_TITLE}

This file is the single source of truth for agents operating in this series.
Agents read it at the start of each session and do **not** ask the user for style or engine.

## Defaults

| Key | Value |
|-----|-------|
| \`MOTION_STYLE\` | \`${STYLE_CHOICE}\` |
| \`MOTION_ENGINE\` | \`${ENGINE_CHOICE}\` |

> These are also persisted to \`${_pc_cfg}\` and exported as env-vars by \`core/env.sh\`.

## Engine — ${ENGINE_CHOICE}

| Item | Detail |
|------|--------|
| **Scaffold** | \`\$MOTION_LIB/core/new_episode.sh <series> NN-slug CompId [out] --style ${STYLE_CHOICE} --engine ${ENGINE_CHOICE}\` |
| **Preview** | \`${PREVIEW_CMD}\` |
$(case "$ENGINE_CHOICE" in
  remotion) echo "| **Scene files** | \`video/src/scenes/\*.tsx\` |"
            echo "| **Config** | \`video/episode.json\`, \`video/src/plan.json\` |" ;;
  fframes)  echo "| **Scene files** | \`video/src/\*.rs\` |"
            echo "| **Config** | \`video/episode.json\` |" ;;
  kyfram)   echo "| **Scene files** | \`video/scenes/\*.lgv\` |"
            echo "| **Config** | \`video/episode.json\` |" ;;
esac)

## Style — ${STYLE_CHOICE}

| Reference | Path |
|-----------|------|
| Style guide (DNA) | \`${GUIDE_REL}\` |
| Component catalogue | \`${CATALOG_REL}\` |
| Agent brief | \`$MOTION_STYLES/${STYLE_CHOICE}/remotion/AGENT_BRIEF.md\` |
| Design reference | \`${DESIGN_MD}\` |

## Workflow quick-reference for agents

Agents use these skills in order. No need to pass style or engine — they read this file.

| Stage | Skill | Writes |
|-------|-------|--------|
| Strategy & brief | \`motion-strategy\` | \`research/facts.md\`, episode scaffold |
| Hook | \`motion-hook\` | \`video/STORYBOARD.md\` HOOK block |
| Script | \`motion-script\` | \`narration/script.json\`, staging table |
| Storyboard | \`motion-storyboard\` | camera/cut table, scene blocks |
| Voice & timing | \`motion-voice\` | \`narration/\`, \`public/vo/captions.json\`, \`src/plan.json\` |
| Scenes & render | \`motion-build\` | \`video/src/scenes/\*\`, \`out/\*.mp4\` |
| QA | \`motion-qa\` | fixes, delivery summary |

## Calling skills

When an agent invokes a strategy or build skill, it passes no \`--style\` or \`--engine\` flags.
Both are resolved from \`MOTION_STYLE\` / \`MOTION_ENGINE\` (env-vars set via \`core/env.sh\`)
and from \`video/episode.json\` (written by \`new_episode.sh\`).

---
*Generated by \`core/init.sh\` on $(date '+%Y-%m-%d'). Re-run \`$MOTION_LIB/core/init.sh --series $SERIES\` to refresh.*
AGENTS_EOF

ok "agents.md → $AGENTS_MD"

# ------------------------------------------------------------------ done
echo
say "Done"
printf '  style   %s%s%s\n' "$C" "$STYLE_CHOICE" "$N"
printf '  engine  %s%s%s\n' "$C" "$ENGINE_CHOICE" "$N"
printf '  config  %s\n' "$_pc_cfg"
printf '  design  %s\n' "$DESIGN_MD"
printf '  agents  %s\n' "$AGENTS_MD"
echo
printf 'Next: start a new episode with\n'
printf '  %s\n' "\$MOTION_LIB/core/new_episode.sh <series-dir> 01-slug CompId out-name"
printf 'or run motion-strategy in your coding agent.\n'
