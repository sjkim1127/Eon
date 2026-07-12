#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIST_DIR="${1:-$ROOT_DIR/target/dx/eon-ui/release/web/public}"
WASM_DIR="$DIST_DIR/wasm"
JS_OUTPUT="$WASM_DIR/eon-ui.js"
WASM_OUTPUT="$WASM_DIR/eon-ui_bg.wasm"
DIAGNOSTIC_LOG="${WASM_BINDGEN_LOG:-$ROOT_DIR/wasm-bindgen.log}"

if [[ -s "$JS_OUTPUT" && -s "$WASM_OUTPUT" ]]; then
  echo "Dioxus wasm bundle already exists in $WASM_DIR."
  exit 0
fi

RAW_WASM=""
for candidate in \
  "$ROOT_DIR/target/wasm32-unknown-unknown/release/eon-ui.wasm" \
  "$ROOT_DIR/target/wasm32-unknown-unknown/release/eon_ui.wasm"; do
  if [[ -s "$candidate" ]]; then
    RAW_WASM="$candidate"
    break
  fi
done

if [[ -z "$RAW_WASM" ]]; then
  RAW_WASM="$(find "$ROOT_DIR/target/wasm32-unknown-unknown/release" \
    -maxdepth 1 -type f -name '*.wasm' -size +0c -print -quit 2>/dev/null || true)"
fi

if [[ -z "$RAW_WASM" ]]; then
  echo "Dioxus did not create a raw WebAssembly executable." | tee "$DIAGNOSTIC_LOG" >&2
  find "$ROOT_DIR/target/wasm32-unknown-unknown/release" -maxdepth 2 -type f \
    -printf '%p %s bytes\n' 2>/dev/null | sort | tee -a "$DIAGNOSTIC_LOG" >&2 || true
  exit 1
fi

DATA_HOME="${XDG_DATA_HOME:-$HOME/.local/share}"
BINDGEN_DIR="$DATA_HOME/dioxus/wasm-bindgen"
WASM_BINDGEN="$(find "$BINDGEN_DIR" -maxdepth 1 -type f -name 'wasm-bindgen-*' \
  -perm -u+x -print 2>/dev/null | sort -V | tail -n1 || true)"

if [[ -z "$WASM_BINDGEN" ]]; then
  echo "Dioxus wasm-bindgen executable was not found under $BINDGEN_DIR." | tee "$DIAGNOSTIC_LOG" >&2
  find "$DATA_HOME/dioxus" -maxdepth 3 -type f -printf '%p\n' 2>/dev/null | sort \
    | tee -a "$DIAGNOSTIC_LOG" >&2 || true
  exit 1
fi

{
  echo "Dioxus did not retain its wasm-bindgen output; regenerating it explicitly."
  echo "Raw WASM: $RAW_WASM"
  echo "Raw WASM size: $(stat -c '%s' "$RAW_WASM") bytes"
  echo "wasm-bindgen: $WASM_BINDGEN"
  "$WASM_BINDGEN" --version
} | tee "$DIAGNOSTIC_LOG"

rm -rf "$WASM_DIR"
mkdir -p "$WASM_DIR"

set +e
"$WASM_BINDGEN" \
  --target web \
  --keep-lld-exports \
  --no-demangle \
  --remove-name-section \
  --remove-producers-section \
  --out-name eon-ui \
  --out-dir "$WASM_DIR" \
  "$RAW_WASM" 2>&1 | tee -a "$DIAGNOSTIC_LOG"
status=${PIPESTATUS[0]}
set -e

if [[ "$status" -ne 0 ]]; then
  echo "wasm-bindgen failed with exit status $status." | tee -a "$DIAGNOSTIC_LOG" >&2
  exit "$status"
fi

if [[ ! -s "$JS_OUTPUT" || ! -s "$WASM_OUTPUT" ]]; then
  echo "wasm-bindgen completed without producing the expected bundle." | tee -a "$DIAGNOSTIC_LOG" >&2
  find "$WASM_DIR" -maxdepth 3 -printf '%y %p %s bytes\n' 2>/dev/null | sort \
    | tee -a "$DIAGNOSTIC_LOG" >&2 || true
  exit 1
fi

echo "Generated WebAssembly bundle:" | tee -a "$DIAGNOSTIC_LOG"
find "$WASM_DIR" -maxdepth 3 -type f -printf '%P %s bytes\n' | sort | tee -a "$DIAGNOSTIC_LOG"
