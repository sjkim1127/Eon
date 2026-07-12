#!/usr/bin/env python3
"""Validate that local assets referenced by the generated web entrypoint exist."""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path
from urllib.parse import urlsplit

ASSET_RE = re.compile(
    r'''["'](?P<url>(?:/|\./|\.\./)?[^"'<>\s]+\.(?:js|mjs|wasm|css)(?:\?[^"']*)?)["']''',
    re.IGNORECASE,
)


def local_asset_path(dist: Path, raw_url: str) -> Path | None:
    parsed = urlsplit(raw_url)
    if parsed.scheme or parsed.netloc or raw_url.startswith(("data:", "blob:")):
        return None

    path = parsed.path.lstrip("/")
    return (dist / path).resolve()


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("dist", type=Path, help="generated static distribution directory")
    args = parser.parse_args()

    dist = args.dist.resolve()
    index = dist / "index.html"
    if not index.is_file():
        print(f"missing generated entrypoint: {index}", file=sys.stderr)
        return 1

    html = index.read_text(encoding="utf-8")
    referenced = sorted({match.group("url") for match in ASSET_RE.finditer(html)})
    if not referenced:
        print(f"no JS/WASM/CSS asset references found in {index}", file=sys.stderr)
        return 1

    missing: list[tuple[str, Path]] = []
    for raw_url in referenced:
        asset = local_asset_path(dist, raw_url)
        if asset is None:
            continue

        try:
            asset.relative_to(dist)
            is_valid = asset.is_file()
        except ValueError:
            is_valid = False

        if not is_valid:
            missing.append((raw_url, asset))

    print(f"Validated {len(referenced)} generated asset reference(s) in {index}.")
    for raw_url in referenced:
        print(f"  {raw_url}")

    if missing:
        print("Generated entrypoint references missing or unsafe local assets:", file=sys.stderr)
        for raw_url, asset in missing:
            print(f"  {raw_url} -> {asset}", file=sys.stderr)
        return 1

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
