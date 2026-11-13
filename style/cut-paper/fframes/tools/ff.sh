#!/usr/bin/env bash
set -euo pipefail
exec "${CARGO:-cargo}" run --release -- "$@"
