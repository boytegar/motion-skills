#!/usr/bin/env bash
set -euo pipefail
rp() { python3 -c 'import os,sys;print(os.path.realpath(sys.argv[1]))' "$1"; }
if [ $# -lt 1 ]; then echo "usage: $0 <project dir> [assets dir]" >&2; exit 2; fi
L=$(dirname "$(rp "${BASH_SOURCE[0]}")"); P=$(rp "$1")
mkdir -p "$P/src/lib" "$P/public/bg" "$P/public/assets" "$P/public/fonts"
rsync -a --delete "$L/kit/" "$P/src/lib/"
printf '%s\n' "synced from sketchbook/kit on $(date '+%Y-%m-%d %H:%M'); do not edit here, edit the library and re-sync" > "$P/src/lib/SYNCED.txt"
if [ -d "$L/public/fonts" ]; then rsync -a "$L/public/fonts/" "$P/public/fonts/"; fi
if [ -d "$L/public/bg" ]; then rsync -a "$L/public/bg/" "$P/public/bg/"; fi
if [ -n "${2:-}" ]; then rsync -a "$(rp "$2")/" "$P/public/"; fi
echo "sketchbook kit -> $P/src/lib ($(find "$P/src/lib" -name '*.ts*' | wc -l | tr -d ' ') files)"
