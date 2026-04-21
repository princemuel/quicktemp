#!/usr/bin/env bash
set -Eeuo pipefail

trap 'echo "❌ trace-wasm failed on line $LINENO"' ERR

trace() {
  wasm2wat components_bg.wasm >components_bg.wat
}

cd dist
trace
echo "✅ trace done → dist/components_bg.wat"
