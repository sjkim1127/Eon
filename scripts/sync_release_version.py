#!/usr/bin/env python3
"""Keep workspace package and internal dependency versions in lockstep."""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
ROOT_MANIFEST = ROOT / "Cargo.toml"

PUBLISHABLE_CRATES = (
    "eon-core",
    "eon-astro",
    "eon-data",
    "eon-saju",
    "eon-vedic",
    "eon-zwds",
    "eon-western",
    "eon-human-design",
    "eon-qimen",
    "eon-ai",
    "eon-service",
)

SEMVER_RE = re.compile(
    r"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)"
    r"(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?$"
)


def workspace_version(text: str) -> str:
    section = re.search(
        r"(?ms)^\[workspace\.package\]\s*$\n(?P<body>.*?)(?=^\[|\Z)", text
    )
    if section is None:
        raise ValueError("missing [workspace.package] section")

    match = re.search(r'(?m)^version\s*=\s*"([^"]+)"\s*$', section.group("body"))
    if match is None:
        raise ValueError("missing workspace package version")
    return match.group(1)


def replace_workspace_version(text: str, version: str) -> str:
    section = re.search(
        r"(?ms)^\[workspace\.package\]\s*$\n(?P<body>.*?)(?=^\[|\Z)", text
    )
    if section is None:
        raise ValueError("missing [workspace.package] section")

    body = section.group("body")
    updated_body, count = re.subn(
        r'(?m)^(version\s*=\s*")[^"]+("\s*)$',
        rf"\g<1>{version}\g<2>",
        body,
        count=1,
    )
    if count != 1:
        raise ValueError("could not update workspace package version")

    return text[: section.start("body")] + updated_body + text[section.end("body") :]


def replace_internal_versions(text: str, version: str) -> str:
    updated = text
    requirement = f"={version}"

    for crate in PUBLISHABLE_CRATES:
        pattern = re.compile(
            rf'(?m)^({re.escape(crate)}\s*=\s*\{{[^\n}}]*\bversion\s*=\s*")'
            rf'[^"]+("[^\n}}]*\}}\s*)$'
        )
        updated, count = pattern.subn(rf"\g<1>{requirement}\g<2>", updated, count=1)
        if count != 1:
            raise ValueError(
                f"missing versioned workspace dependency for {crate!r} in Cargo.toml"
            )

    return updated


def validate_member_manifests() -> None:
    for crate in PUBLISHABLE_CRATES:
        manifest = ROOT / "crates" / crate / "Cargo.toml"
        if not manifest.exists():
            raise ValueError(f"missing manifest: {manifest.relative_to(ROOT)}")
        text = manifest.read_text(encoding="utf-8")
        if not re.search(r"(?m)^version\.workspace\s*=\s*true\s*$", text):
            raise ValueError(
                f"{manifest.relative_to(ROOT)} must use version.workspace = true"
            )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "version",
        nargs="?",
        help="release version without the leading v; defaults to workspace version",
    )
    parser.add_argument(
        "--check",
        action="store_true",
        help="verify synchronization without modifying Cargo.toml",
    )
    args = parser.parse_args()

    original = ROOT_MANIFEST.read_text(encoding="utf-8")
    version = args.version or workspace_version(original)

    if SEMVER_RE.fullmatch(version) is None:
        parser.error(f"invalid semantic version: {version!r}")

    validate_member_manifests()
    updated = replace_workspace_version(original, version)
    updated = replace_internal_versions(updated, version)

    if args.check:
        if updated != original:
            print(
                "Cargo.toml versions are not synchronized. "
                f"Run: python3 scripts/sync_release_version.py {version}",
                file=sys.stderr,
            )
            return 1
        print(f"Workspace release versions are synchronized at {version}.")
        return 0

    if updated != original:
        ROOT_MANIFEST.write_text(updated, encoding="utf-8")
        print(f"Updated workspace and internal dependency versions to {version}.")
    else:
        print(f"Workspace versions already synchronized at {version}.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
