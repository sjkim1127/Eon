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


def set_link_attribute(tag: str, name: str, value: str) -> str:
    attribute = re.compile(
        rf'''\b{re.escape(name)}\b(?:\s*=\s*(?:"[^"]*"|'[^']*'|[^\s>]+))?''',
        re.IGNORECASE,
    )
    replacement = f'{name}="{value}"'
    if attribute.search(tag):
        return attribute.sub(replacement, tag, count=1)
    return f"{tag[:-1].rstrip()} {replacement}>"


def link_attribute(tag: str, name: str) -> str | None:
    match = re.search(
        rf'''\b{re.escape(name)}\s*=\s*(?:"(?P<double>[^"]*)"|'(?P<single>[^']*)')''',
        tag,
        re.IGNORECASE,
    )
    if match is None:
        return None
    return match.group("double") if match.group("double") is not None else match.group("single")


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

    # Keep the JavaScript preload's credentials mode aligned with dynamic
    # import, and preserve a fetch preload for the shared WebAssembly binary.
    def normalize_link(match: re.Match[str]) -> str:
        tag = match.group(0)
        href = link_attribute(tag, "href")
        if href == WASM_JS_URL:
            tag = set_link_attribute(tag, "rel", "modulepreload")
            return set_link_attribute(tag, "crossorigin", "anonymous")
        if href == WASM_BINARY_URL:
            tag = set_link_attribute(tag, "rel", "preload")
            tag = set_link_attribute(tag, "as", "fetch")
            tag = set_link_attribute(tag, "type", "application/wasm")
            return set_link_attribute(tag, "crossorigin", "anonymous")
        return tag

    html = re.sub(r"<link\b[^>]*>", normalize_link, html, flags=re.IGNORECASE)

    index.write_text(html, encoding="utf-8")

    print(f"Normalized generated web entrypoint: {index}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
