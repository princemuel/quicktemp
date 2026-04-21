#!/usr/bin/env bash
set -Eeuo pipefail

# ── Resolve repo root  ─────────────────────────────────────────────────────
REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd -P)"
PKG_DIR="$REPO_ROOT/pkg"
ASSETS_PKG_DIR="$REPO_ROOT/client/src/lib"

cd "$REPO_ROOT"

# ── Flags ───────────────────────────────────────────────────────────────────
OPTIMISE=0

usage() {
  echo "Usage: $(basename "$0") [-o]"
  echo "  -o   Run wasm-opt after building"
  exit 1
}

while getopts ':o' flag; do
  case "$flag" in
  o) OPTIMISE=1 ;;
  *) usage ;;
  esac
done

# ── ERR trap ─────────────────────────────────────────────────────────────────
trap 'echo "❌ build-wasm failed — line $LINENO: $BASH_COMMAND"' ERR

# ── Steps ────────────────────────────────────────────────────────────────────
build() {
  wasm-pack build --target web --out-name index
}

optimise() {
  wasm-opt -Os --enable-bulk-memory \
    "$PKG_DIR/quicktemp_bg.wasm" \
    -o "$PKG_DIR/quicktemp_bg.wasm"
}

copy_artifacts() {
  rm -rf "${ASSETS_PKG_DIR:?}"
  mkdir -p "$ASSETS_PKG_DIR"
  find "$PKG_DIR" -maxdepth 1 -type f \( -name '*.js' -o -name '*.ts' -o -name '*.wasm' \) \
    -exec cp {} "$ASSETS_PKG_DIR/" \;
  rm -rf "${PKG_DIR:?}"
}

# ── Main ─────────────────────────────────────────────────────────────────────
build
[[ "$OPTIMISE" -eq 1 ]] && optimise
copy_artifacts
echo "✅ wasm build done. saved artifacts at client/src/lib"
