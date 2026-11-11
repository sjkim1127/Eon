# AGENTS.md — AI Agent Guide for the Eon Repository

This guide instructs AI agents (Antigravity, Cursor, GitHub Copilot, Claude Code, etc.) on how to accurately understand, safely modify, and effectively contribute to the Eon repository.

---

## 1. Project Overview

Eon is a next-generation multi-dimensional destiny intelligence and metaphysical calculation platform that reverse-engineers ancient and modern systems engineering traditions:
- **Saju (Four Pillars of Destiny / BaZi)**
- **Vedic Astrology (Jyotish)**
- **Western Astrology (Tropical / Placidus & Whole Sign)**
- **Zi Wei Dou Shu (Purple Star Astrology)**
- **Human Design System**
- **QiMen DunJia (Eight Gates & Nine Palaces)**
- **Pythagorean Numerology**
- **Destiny Tier 7.0 Synthesis** (15 weighted components & 8D Quantum Cross-Engine Matrix)

All legacy web UI frontend code (Dioxus/React) has been decommissioned. Eon is now a **100% Native Rust CLI & Model Context Protocol (MCP) server (`crates/eon-cli`)** engineered specifically for direct invocation, interpretation, and synthesis by LLMs and autonomous AI agents.

- **Primary Binary (CLI & MCP)**: `eon` (`crates/eon-cli`)
- **Core Domain Façade & Contract**: `crates/eon-service` (Single Source of Truth / SSOT for all engine DTOs and calculations)
- **8 Domain Analysis Engines**:
  1. Four Pillars of Destiny (`eon-saju`)
  2. Vedic Astrology (`eon-vedic`)
  3. Western Astrology (`eon-western`)
  4. Zi Wei Dou Shu (`eon-zwds`)
  5. Human Design (`eon-human-design`)
  6. QiMen DunJia (`eon-qimen`)
  7. Pythagorean Numerology (`eon-numerology`)
  8. Omni Destiny Tier 7.0 Matrix (`eon-service::facade::analyze_destiny_tier_omni`)

---

## 2. Directory Map

```text
Eon/
├── Cargo.toml                   # Workspace manifest
├── Cargo.lock
├── README.md                    # Primary repository documentation
├── AGENTS.md                    # AI agent instructions (this document)
├── crates/
│   ├── eon-cli/                 # Native CLI, LLM Prompt Generator, and JSON-RPC 2.0 MCP server
│   │   ├── src/
│   │   │   ├── commands/        # analyze, prompt, mcp, saju, vedic, western, zwds, hd, qimen, etc.
│   │   │   ├── format/          # agent (LLM prompt markdown), json, markdown, text formatters
│   │   │   └── main.rs          # Clap entrypoint
│   │   └── Cargo.toml
│   ├── eon-core/                # Shared primitives (HeavenlyStem, EarthlyBranch, BirthInfo)
│   ├── eon-data/                # Manseryuk calendar binary data and cache
│   ├── eon-astro/               # Pure-Rust Swiss Ephemeris astronomical algorithms (swisseph-rs)
│   ├── eon-saju/                # Four Pillars engine (Pillars, Shinsal, Gyeokguk, Yongshin, Daewun)
│   ├── eon-vedic/               # Vedic engine (Shadbala, Ashtakavarga, Dasha, Yogas)
│   ├── eon-western/             # Western astrology (Planets, Houses, Aspects, Patterns)
│   ├── eon-zwds/                # Zi Wei Dou Shu (12 Palaces, 14 Major Stars, SiHua, Destiny Patterns)
│   ├── eon-human-design/        # Human Design (Type, Authority, Centers, Channels, Gates)
│   ├── eon-qimen/               # QiMen DunJia (9 Palaces, 8 Doors, 9 Stars, 8 Deities)
│   ├── eon-numerology/          # Pythagorean Numerology (Life Path, Expression, Soul Urge, Pinnacles)
│   ├── eon-ai/                  # Heuristic linting, confidence auditing, and rule validation
│   └── eon-service/             # Unified façade and DTO contract boundary (SSOT)
│       ├── src/dto.rs           # AnalysisInput, OmniDestinyTierInput, output structures
│       └── src/facade.rs        # analyze_saju, analyze_all_synthesized, etc.
│
├── .github/workflows/           # CI/CD (formatting, clippy, test suite, binary release)
├── scripts/                     # Release packaging and version synchronization scripts
└── docs/                        # Domain analysis and architectural documentation
```

---

## 3. Core Architectural Principles

### 3-1. Input Construction via `new()` Constructors
Always construct engine inputs using the explicit constructor functions provided in `eon_service::dto`. Avoid direct struct literal initialization where constructors exist:

```rust
let input = AnalysisInput::new(
    birth.year,
    birth.month,
    birth.day,
    birth.hour,
    birth.minute,
    birth.timezone,
    birth.lat,
    birth.lon,
    birth.gender,
);
```

### 3-2. AI Agent Optimized Formats
The CLI natively supports high-density outputs tailored for LLM reasoning and agent tool pipelines:
- `--format agent` (or `eon prompt`): High-density markdown context ready for injection into an LLM's system or user prompt.
- `--format json`: Fully typed, structured JSON serialization of engine DTOs.
- `--format markdown`: Clean human-readable reports with formatted tables and bullet points.
- `--format text`: Ultra-compact summaries for terminal inspection.

### 3-3. Model Context Protocol (MCP) Standard
The `eon mcp` command runs an MCP server over stdio using JSON-RPC 2.0. Any MCP-compatible host (Claude Desktop, Cursor, Antigravity, etc.) can discover and call `eon_analyze` directly as a native agent tool.

---

## 4. Build, Verification & Testing Guide

### 4-1. Local Installation and Execution
```bash
# Build and install the eon binary locally
cargo install --path crates/eon-cli

# Run development build directly
cargo run --bin eon -- --help
cargo run --bin eon -- analyze --birth "1990-05-15 10:30" --gender m
cargo run --bin eon -- prompt --birth "1990-05-15 10:30" --system
```

### 4-2. Workspace Quality Checks
Before submitting any changes, verify all quality gates:
```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
python3 scripts/sync_release_version.py --check
```

---

## 5. Strict Prohibitions for AI Agents

1. **Never Revert to Web or GUI Stacks**: Do NOT re-introduce Dioxus, React, Tauri, Wasm web bundles, `package.json`, `app/`, `dx.toml`, or npm dependencies. Eon is strictly a native CLI and agent tool.
2. **Never Add Unsanctioned Heavy Runtimes**: Keep the codebase 100% pure Rust (utilizing `swisseph-rs` in-memory astronomical algorithms). Do not introduce extraneous C/C++ runtimes or heavy unvetted dynamic libraries.
3. **Preserve Deterministic Calculation Purity**: All metaphysical calculations must remain completely pure and deterministic. Given identical birth input parameters, every engine must consistently return identical results.
