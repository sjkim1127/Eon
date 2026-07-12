#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CONFIG_FILE="$(mktemp)"
trap 'rm -f "$CONFIG_FILE"' EXIT

CRATES=(
  eon-core
  eon-astro
  eon-data
  eon-saju
  eon-vedic
  eon-zwds
  eon-western
  eon-human-design
  eon-qimen
  eon-ai
  eon-service
)

cat > "$CONFIG_FILE" <<EOF
[patch.crates-io]
eon-core = { path = "$ROOT_DIR/crates/eon-core" }
eon-astro = { path = "$ROOT_DIR/crates/eon-astro" }
eon-data = { path = "$ROOT_DIR/crates/eon-data" }
eon-saju = { path = "$ROOT_DIR/crates/eon-saju" }
eon-vedic = { path = "$ROOT_DIR/crates/eon-vedic" }
eon-zwds = { path = "$ROOT_DIR/crates/eon-zwds" }
eon-western = { path = "$ROOT_DIR/crates/eon-western" }
eon-human-design = { path = "$ROOT_DIR/crates/eon-human-design" }
eon-qimen = { path = "$ROOT_DIR/crates/eon-qimen" }
eon-ai = { path = "$ROOT_DIR/crates/eon-ai" }
EOF

cd "$ROOT_DIR"

for crate in "${CRATES[@]}"; do
  echo "Packaging $crate with local workspace dependency patches..."
  cargo --config "$CONFIG_FILE" package \
    -p "$crate" \
    --locked \
    --no-verify \
    "$@"
done
