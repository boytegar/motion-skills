# Shared environment for the motion library scripts. Source it; do not execute it.
#   . "$MOTION_LIB/core/env.sh"      (the installer writes MOTION_LIB into the config file)
#
# Reads the installer's config file, then fills defaults. Variables already set in the
# environment win over the file. Bash 3.2 compatible (macOS default).
#
#   MOTION_LIB        this checkout's root (style/, core/, skills/, examples/)
#   MOTION_PY         python with numpy + Pillow + soundfile (timing, QA sheets, mix)  default: python3
#   QWEN_TTS_PY       python with qwen-tts installed (core/voice/narrate)              default: python3
#   ASR_PY            python with an ASR backend (core/voice/asr)                      default: python3
#   MOTION_STYLE      default style folder name under style/ (none by default: ask the user)
#   MOTION_CONFIG     config file path   default: ${XDG_CONFIG_HOME:-~/.config}/motion-explainer/config.env
#
# Installers before the rename wrote papercut-explainer/config.env with PAPERCUT_* keys. That file is
# still read, and its keys are mirrored to the MOTION_* names below.

_pc_cfg="${MOTION_CONFIG:-${XDG_CONFIG_HOME:-$HOME/.config}/motion-explainer/config.env}"
_pc_legacy_cfg="${XDG_CONFIG_HOME:-$HOME/.config}/papercut-explainer/config.env"

_pc_alias() { case "$1" in
  PAPERCUT_HOME) echo MOTION_HOME ;; PAPERCUT_LIB) echo MOTION_LIB ;;
  PAPERCUT_PY) echo MOTION_PY ;; PAPERCUT_STYLE) echo MOTION_STYLE ;;
  PAPERCUT_VOICE_VENV) echo MOTION_VOICE_VENV ;;
  *) echo "$1" ;;
esac; }

_pc_read_cfg() {   # fill unset variables from a key=value file
  [ -f "$1" ] || return 0
  local _k _v _cur
  while IFS='=' read -r _k _v || [ -n "$_k" ]; do
    case "$_k" in ''|'#'*|*[!A-Za-z0-9_]*) continue ;; esac
    _k="$(_pc_alias "$_k")"
    eval "_cur=\"\${$_k:-}\""   # indirect lookup that works in bash 3.2 and zsh
    [ -n "$_cur" ] || export "$_k=$_v"
  done < "$1"
}
_pc_read_cfg "$_pc_legacy_cfg"
_pc_read_cfg "$_pc_cfg"

# Voice engine and sync choices, all optional (see voice/README.md).
#   TTS_PROVIDER  unset + no TTS_URL -> local Qwen3-TTS; openai | elevenlabs | custom -> core/voice/tts_provider.py
#   TTS_URL       the endpoint to POST to (also selects the provider when TTS_PROVIDER is empty):
#                 with openai/elevenlabs it replaces the vendor URL, with custom it is required
#   TTS_API_KEY   key for the hosted provider (or TTS_<PROVIDER>_KEY)
#   TTS_MODEL, TTS_VOICE, TTS_SPEED, TTS_LANG, TTS_FORMAT, TTS_AUTH, TTS_PAYLOAD, TTS_TIMEOUT
#   PACE_TARGET   words per second the pace report (core/tools/pace.py) judges lines against
: "${PACE_TARGET:=3.3}"
export PACE_TARGET
motion_tts_hosted() { [ -n "${TTS_PROVIDER:-}" ] || [ -n "${TTS_URL:-}" ]; }   # true when narrate.sh should use the API

# a MOTION_LIB from the config file that no longer exists (moved or deleted checkout) is ignored
if [ -n "${MOTION_LIB:-}" ] && [ ! -d "$MOTION_LIB" ]; then unset MOTION_LIB; fi
if [ -z "${MOTION_LIB:-}" ] && [ -n "${BASH_SOURCE[0]:-}" ]; then
  MOTION_LIB="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # env.sh lives in <root>/core/
fi
export MOTION_LIB

: "${MOTION_PY:=python3}"; : "${QWEN_TTS_PY:=python3}"; : "${ASR_PY:=python3}"
export MOTION_PY QWEN_TTS_PY ASR_PY
# names older scripts and older skill text still read
export MOTION_LIB="$MOTION_LIB" MOTION_PY="$MOTION_PY" MOTION_CONFIG="$_pc_cfg"

# motion_config_set KEY VALUE: write or replace one line in the config file.
motion_config_set() {
  mkdir -p "$(dirname "$_pc_cfg")"; touch "$_pc_cfg"
  { grep -v "^$1=" "$_pc_cfg" || true; printf '%s=%s\n' "$1" "$2"; } > "$_pc_cfg.tmp"
  mv "$_pc_cfg.tmp" "$_pc_cfg"
}

# motion_styles: every style under style/ that ships a style.json, one name per line.
# MOTION_LIB normally names the repository root; older installers stored core/ instead.
# Support both layouts without silently searching elsewhere.
if [ -d "${MOTION_STYLES:-$MOTION_LIB/style}" ]; then
  MOTION_STYLES="${MOTION_STYLES:-$MOTION_LIB/style}"
else
  MOTION_STYLES="${MOTION_STYLES:-$MOTION_LIB/../style}"
fi
export MOTION_STYLES
motion_styles() { local d; for d in "$MOTION_STYLES"/*/; do [ -f "$d/style.json" ] && basename "$d"; done; }

# motion_style_of EPISODE_DIR [STYLE]: print the style name the episode uses: an explicit name,
# else episode.json "style", else $MOTION_STYLE. Fail 1 if none is set, 2 if it does not exist.
motion_style_of() {
  local ep="$1/episode.json" want="${2:-}"
  [ -n "$want" ] || { [ -f "$ep" ] && want="$(python3 -c "import json,sys;print(json.load(open(sys.argv[1])).get('style',''))" "$ep" 2>/dev/null)"; }
  [ -n "$want" ] || want="${MOTION_STYLE:-}"
  [ -n "$want" ] || return 1
  [ -f "$MOTION_STYLES/$want/style.json" ] || { echo "unknown style '$want' (have: $(motion_styles | tr '\n' ' '))" >&2; return 2; }
  echo "$want"
}
