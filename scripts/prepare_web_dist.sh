#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIST_DIR="${1:-$ROOT_DIR/target/dx/eon-ui/release/web/public}"
UI_DIR="$ROOT_DIR/crates/eon-ui"
TAILWIND_VERSION="3.4.17"
TAILWIND_BIN="${RUNNER_TEMP:-${TMPDIR:-/tmp}}/tailwindcss-${TAILWIND_VERSION}-linux-x64"

mkdir -p "$DIST_DIR"

bash "$ROOT_DIR/scripts/ensure_web_wasm_bundle.sh" "$DIST_DIR"

if [[ ! -x "$TAILWIND_BIN" ]]; then
  echo "Downloading Tailwind CSS $TAILWIND_VERSION..."
  curl \
    --fail \
    --location \
    --silent \
    --show-error \
    --retry 3 \
    --output "$TAILWIND_BIN" \
    "https://github.com/tailwindlabs/tailwindcss/releases/download/v${TAILWIND_VERSION}/tailwindcss-linux-x64"
  chmod +x "$TAILWIND_BIN"
fi

(
  cd "$UI_DIR"
  "$TAILWIND_BIN" \
    --config ./tailwind.config.js \
    --input ./input.css \
    --output "$DIST_DIR/tailwind.css" \
    --minify
)

cp "$UI_DIR/public/vercel.json" "$DIST_DIR/vercel.json"
python3 "$ROOT_DIR/scripts/normalize_web_entrypoint.py" "$DIST_DIR/index.html"
python3 "$ROOT_DIR/scripts/verify_web_dist.py" "$DIST_DIR"

echo "Prepared static web distribution:"
find "$DIST_DIR" -maxdepth 4 -type f -printf '%P %s bytes\n' | sort
