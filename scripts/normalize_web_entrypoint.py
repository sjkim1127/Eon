#!/usr/bin/env python3
"""Normalize the Dioxus-generated web entrypoint for static hosting."""

from __future__ import annotations

import argparse
import re
from pathlib import Path

WASM_JS_URL = "/wasm/eon-ui.js"
WASM_BINARY_URL = "/wasm/eon-ui_bg.wasm"


def point_entrypoint_at_complete_wasm_bundle(html: str) -> str:
    # `dx build` also emits a hashed JS bundle that can embed the absolute
    # wasm-bindgen output directory from the build runner. The deployment
    # preparation step regenerates a portable wasm-bindgen bundle under /wasm;
    # load that bundle so its snippets and worker module resolve relative to it.
    entry_js = re.compile(
        r'''(?P<quote>["'])(?:/|\./)?(?:assets/)?eon-ui(?:-[^/"']+)?\.js(?P=quote)'''
    )
    entry_wasm = re.compile(
        r'''(?P<quote>["'])(?:/|\./)?(?:assets/)?eon-ui_bg(?:-[^/"']+)?\.wasm(?P=quote)'''
    )

    def stable_url(match: re.Match[str], url: str) -> str:
        quote = match.group("quote")
        return f"{quote}{url}{quote}"

    html = entry_js.sub(lambda match: stable_url(match, WASM_JS_URL), html)
    html = entry_wasm.sub(lambda match: stable_url(match, WASM_BINARY_URL), html)

    # The generated wasm-bindgen initializer derives the binary URL from
    # import.meta.url when called without arguments. Passing a string is
    # deprecated by current wasm-bindgen output.
    html = re.sub(
        r'''\binit\(\s*["']/wasm/eon-ui_bg\.wasm["']\s*\)''',
        "init()",
        html,
    )
    return html


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("index", type=Path)
    args = parser.parse_args()

    index = args.index.resolve()
    html = index.read_text(encoding="utf-8")
    wasm_js = index.parent / "wasm" / "eon-ui.js"
    wasm_binary = index.parent / "wasm" / "eon-ui_bg.wasm"
    if not wasm_js.is_file() or not wasm_binary.is_file():
        raise SystemExit(
            "portable WebAssembly bundle is missing: "
            f"expected {wasm_js} and {wasm_binary}"
        )

    html = point_entrypoint_at_complete_wasm_bundle(html)

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
    if f'href="{WASM_JS_URL}"' not in html or f'href="{WASM_BINARY_URL}"' not in html:
        raise SystemExit("failed to point the preloads at the portable WebAssembly bundle")
    if 'import("/wasm/eon-ui.js")' not in html or "init()" not in html:
        raise SystemExit("failed to point the entrypoint at the portable WebAssembly bundle")

    print(f"Normalized generated web entrypoint: {index}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
