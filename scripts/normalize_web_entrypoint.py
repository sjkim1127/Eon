#!/usr/bin/env python3
"""Normalize the Dioxus-generated web entrypoint for static hosting."""

from __future__ import annotations

import argparse
import re
from pathlib import Path


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("index", type=Path)
    args = parser.parse_args()

    index = args.index.resolve()
    html = index.read_text(encoding="utf-8")

    # Dioxus 0.6 emits `/./wasm/...` when no base path is configured. It is
    # technically resolvable in a browser, but normalizing it avoids cache and
    # hosting discrepancies and gives stable URLs for validation.
    html = html.replace('href="/./', 'href="/')
    html = html.replace('import("/./', 'import("/')
    html = html.replace('init("/./', 'init("/')

    # Dynamic import uses module-fetch semantics. A plain script preload uses a
    # different credentials mode, so Chromium discards it. Dioxus may emit
    # either stable /wasm names or content-hashed /assets names; preserve the
    # generated URL while changing only the preload relation.
    def normalize_js_preload(match: re.Match[str]) -> str:
        href = re.search(r'\bhref="(?P<href>/[^" ]+eon-ui[^" ]*\.js)"', match.group(0))
        if href is None:
            return match.group(0)
        return f'<link rel="modulepreload" href="{href.group("href")}" crossorigin="anonymous">'

    html = re.sub(
        r'<link(?=[^>]*\brel="preload")(?=[^>]*\bas="script")[^>]*>',
        normalize_js_preload,
        html,
    )
    html = re.sub(
        r'(<link\s+rel="preload"\s+href="/wasm/eon-ui_bg\.wasm"\s+as="fetch"\s+type="application/wasm")\s+crossorigin(?:="[^"]*")?\s*>',
        r'\1 crossorigin="anonymous">',
        html,
    )

    index.write_text(html, encoding="utf-8")

    if "/./wasm/" in html:
        raise SystemExit("failed to normalize Dioxus WASM paths")
    if 'rel="modulepreload"' not in html or 'eon-ui' not in html:
        raise SystemExit("failed to normalize the JavaScript module preload")

    print(f"Normalized generated web entrypoint: {index}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
