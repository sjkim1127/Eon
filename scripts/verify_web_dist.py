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
JS_MODULE_RE = re.compile(
    r'''(?:\b(?:import|export)\b[^;\n]*?\bfrom\s*|\bimport\s*\(\s*|\burl\s*:\s*)["'](?P<url>(?:/|\./|\.\./)?[^"'<>\s]+\.(?:js|mjs)(?:\?[^"']*)?)["']''',
    re.IGNORECASE,
)


def local_asset_path(dist: Path, raw_url: str, base_dir: Path) -> Path | None:
    parsed = urlsplit(raw_url)
    if parsed.scheme or parsed.netloc or raw_url.startswith(("data:", "blob:")):
        return None

    if parsed.path.startswith("/"):
        return (dist / parsed.path.lstrip("/")).resolve()
    return (base_dir / parsed.path).resolve()


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("dist", type=Path, help="generated static distribution directory")
    args = parser.parse_args()

    dist = args.dist.resolve()
    index = dist / "index.html"
    if not index.is_file():
        print(f"missing generated entrypoint: {index}", file=sys.stderr)
        return 1

    pending: list[tuple[str, Path]] = [(index.read_text(encoding="utf-8"), index)]
    visited_sources: set[Path] = set()
    referenced: set[tuple[str, Path]] = set()
    missing: list[tuple[str, Path]] = []

    while pending:
        content, source = pending.pop()
        if source in visited_sources:
            continue
        visited_sources.add(source)

        for match in ASSET_RE.finditer(content):
            raw_url = match.group("url")
            asset = local_asset_path(dist, raw_url, source.parent)
            if asset is None:
                continue

            referenced.add((raw_url, source))
            try:
                asset.relative_to(dist)
                is_valid = asset.is_file()
            except ValueError:
                is_valid = False

            if not is_valid:
                missing.append((raw_url, asset))
                continue

            if asset.suffix.lower() in {".js", ".mjs"}:
                try:
                    module = asset.read_text(encoding="utf-8")
                    for module_match in JS_MODULE_RE.finditer(module):
                        module_url = module_match.group("url")
                        referenced.add((module_url, asset))
                        module_asset = local_asset_path(dist, module_url, asset.parent)
                        if module_asset is None:
                            continue

                        try:
                            module_asset.relative_to(dist)
                            module_is_valid = module_asset.is_file()
                        except ValueError:
                            module_is_valid = False

                        if not module_is_valid:
                            missing.append((module_url, module_asset))
                        elif module_asset.suffix.lower() in {".js", ".mjs"}:
                            pending.append((module_asset.read_text(encoding="utf-8"), module_asset))
                except UnicodeDecodeError:
                    missing.append((raw_url, asset))

    if not referenced:
        print(f"no JS/WASM/CSS asset references found in {index}", file=sys.stderr)
        return 1

    print(
        f"Validated {len(referenced)} generated asset reference(s) "
        f"across {len(visited_sources)} file(s) starting at {index}."
    )
    for raw_url, source in sorted(referenced):
        print(f"  {source.relative_to(dist)} -> {raw_url}")

    if missing:
        print("Generated entrypoint references missing or unsafe local assets:", file=sys.stderr)
        for raw_url, asset in sorted(set(missing)):
            print(f"  {raw_url} -> {asset}", file=sys.stderr)
        return 1

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
