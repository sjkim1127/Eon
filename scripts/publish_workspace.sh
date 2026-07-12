#!/usr/bin/env bash
set -euo pipefail

: "${VERSION:?VERSION must be set to the release version without a leading v}"
: "${CARGO_REGISTRY_TOKEN:?CARGO_REGISTRY_TOKEN must be set}"

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

CRATES_IO_API="https://crates.io/api/v1/crates"
USER_AGENT="eon-release-workflow/1.0 (https://github.com/sjkim1127/Eon)"

crate_exists() {
  local crate="$1"
  curl \
    --silent \
    --show-error \
    --fail \
    --retry 3 \
    --retry-delay 2 \
    --user-agent "$USER_AGENT" \
    "$CRATES_IO_API/$crate/$VERSION" \
    >/dev/null 2>&1
}

wait_for_crate() {
  local crate="$1"
  local max_attempts=60

  for attempt in $(seq 1 "$max_attempts"); do
    if crate_exists "$crate"; then
      echo "$crate $VERSION is available on crates.io."
      return 0
    fi

    echo "Waiting for $crate $VERSION to appear on crates.io ($attempt/$max_attempts)..."
    sleep 10
  done

  echo "Timed out waiting for $crate $VERSION on crates.io." >&2
  return 1
}

publish_crate() {
  local crate="$1"

  if crate_exists "$crate"; then
    echo "Skipping $crate $VERSION: version already exists on crates.io."
    return 0
  fi

  for attempt in 1 2 3; do
    echo "Publishing $crate $VERSION (attempt $attempt/3)..."
    if cargo publish -p "$crate" --locked --allow-dirty; then
      wait_for_crate "$crate"
      return 0
    fi

    if crate_exists "$crate"; then
      echo "$crate $VERSION was published despite the client error."
      return 0
    fi

    if [[ "$attempt" -lt 3 ]]; then
      echo "Publish attempt failed; refreshing the registry before retrying..."
      cargo search "$crate" --limit 1 >/dev/null 2>&1 || true
      sleep $((attempt * 10))
    fi
  done

  echo "Failed to publish $crate $VERSION after 3 attempts." >&2
  return 1
}

for crate in "${CRATES[@]}"; do
  publish_crate "$crate"
done
