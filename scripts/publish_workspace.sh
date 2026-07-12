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

# Return codes: 0 = exists, 1 = does not exist, 2 = registry/API error.
crate_exists() {
  local crate="$1"
  local code

  if ! code=$(curl \
    --silent \
    --show-error \
    --output /dev/null \
    --write-out "%{http_code}" \
    --retry 3 \
    --retry-delay 2 \
    --retry-all-errors \
    --user-agent "$USER_AGENT" \
    "$CRATES_IO_API/$crate/$VERSION"); then
    echo "Failed to query crates.io for $crate $VERSION." >&2
    return 2
  fi

  case "$code" in
    200)
      return 0
      ;;
    404)
      return 1
      ;;
    *)
      echo "Unexpected crates.io API status $code for $crate $VERSION." >&2
      return 2
      ;;
  esac
}

wait_for_crate() {
  local crate="$1"
  local max_attempts=60
  local status

  for ((attempt = 1; attempt <= max_attempts; attempt++)); do
    status=0
    crate_exists "$crate" || status=$?

    if [[ "$status" -eq 0 ]]; then
      echo "$crate $VERSION is available on crates.io."
      return 0
    fi

    if [[ "$status" -eq 2 ]]; then
      echo "Registry check failed while waiting for $crate ($attempt/$max_attempts); retrying." >&2
    else
      echo "Waiting for $crate $VERSION to appear on crates.io ($attempt/$max_attempts)..."
    fi
    sleep 10
  done

  echo "Timed out waiting for $crate $VERSION on crates.io." >&2
  return 1
}

publish_crate() {
  local crate="$1"
  local status=0

  crate_exists "$crate" || status=$?
  case "$status" in
    0)
      echo "Skipping $crate $VERSION: version already exists on crates.io."
      return 0
      ;;
    1)
      ;;
    *)
      echo "Cannot safely determine whether $crate $VERSION exists; aborting." >&2
      return 1
      ;;
  esac

  for attempt in 1 2 3; do
    echo "Publishing $crate $VERSION (attempt $attempt/3)..."
    if cargo publish -p "$crate" --locked --allow-dirty; then
      wait_for_crate "$crate"
      return 0
    fi

    status=0
    crate_exists "$crate" || status=$?
    if [[ "$status" -eq 0 ]]; then
      echo "$crate $VERSION was published despite the client error."
      return 0
    fi
    if [[ "$status" -eq 2 ]]; then
      echo "Could not verify publish state for $crate because crates.io is unavailable." >&2
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
