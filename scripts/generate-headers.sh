#!/usr/bin/env bash
#
# Regenerate cmake-produced headers (config.h, export.h) from the libversion
# submodule so that the normal build (which uses the cc crate) never needs
# cmake installed.
#
# Usage:  bash scripts/generate-headers.sh
#
# Run this script whenever the libversion submodule is updated.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
TMPDIR="$(mktemp -d)"

trap 'rm -rf "$TMPDIR"' EXIT

echo "Running cmake configure on libversion submodule..."
cmake -S "$PROJECT_ROOT/libversion" -B "$TMPDIR" >/dev/null 2>&1

mkdir -p "$PROJECT_ROOT/generated/libversion"
cp "$TMPDIR/libversion/config.h" "$PROJECT_ROOT/generated/libversion/config.h"
cp "$TMPDIR/libversion/export.h" "$PROJECT_ROOT/generated/libversion/export.h"

echo "Generated headers updated in generated/libversion/"
