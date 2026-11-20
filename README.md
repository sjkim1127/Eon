<p align="center">
  <picture>
    <source media="(prefers-color-scheme: light)" srcset="assets/eon-logo-dark.svg">
    <img src="assets/eon-logo.svg" alt="Eon logo" width="180">
  </picture>
</p>

# Eon

**Eon** is a high-performance, multi-dimensional destiny intelligence platform and metaphysical calculation engine written in 100% native Rust. It synthesizes 8 ancient and modern metaphysical systems through rigorous systems engineering, reproducible rule engines, and deterministic calculations.

Engineered specifically for **Large Language Models (LLMs) and autonomous AI Agents**, Eon provides an ultra-fast native CLI (`eon`), formatted context prompt generators, and a stdio **Model Context Protocol (MCP)** server.

[![CI](https://github.com/sjkim1127/Eon/actions/workflows/ci.yml/badge.svg)](https://github.com/sjkim1127/Eon/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](Cargo.toml)

---

## Table of Contents

- [Key Capabilities](#key-capabilities)
- [Architecture](#architecture)
- [Workspace Crates](#workspace-crates)
- [Quick Start](#quick-start)
- [CLI Command Reference](#cli-command-reference)
- [AI Agent & LLM Integration](#ai-agent--llm-integration)
- [Model Context Protocol (MCP) Setup](#model-context-protocol-mcp-setup)
- [Rust SDK & Service Façade](#rust-sdk--service-façade)
- [Development & Verification](#development--verification)
- [Disclaimer & License](#disclaimer--license)

---

## Key Capabilities

- **8 Multi-Tradition Calculation Engines**:
  1. **Saju (Four Pillars of Destiny / BaZi)**: Stems, Branches, Hidden Stems, Strength diagnostics, Gyeokguk, Yongshin, Daewun (10-year luck), Shinsal, and Five Elements throughput.
  2. **Vedic Astrology (Jyotish)**: Sidereal planetary positions, Varga charts (D1..D60), Shadbala, Ashtakavarga, Vimshottari Dasha, Sade Sati, and 100+ classical Yogas.
  3. **Western Astrology**: Tropical positions, Placidus & Whole Sign houses, major and minor aspects, Arabian parts, and complex aspect patterns (Grand Trine, T-Square, Yod, etc.).
  4. **Zi Wei Dou Shu (Purple Star Astrology)**: 12 Palaces, 14 Major Stars, Flying SiHua (Mutagenic Stars), and classic destiny configurations.
  5. **Human Design System**: Type, Profile, Inner Authority, 9 Energy Centers, 36 Channels, and 64 Gates.
  6. **QiMen DunJia**: Yin/Yang Dun 18 Ju formations, 9 Palaces, 8 Doors, 9 Stars, and 8 Deities with Value Chief and Envoy Door tracking.
  7. **Numerology (Pythagorean / Chaldean)**: Core Numbers (Life Path, Expression, Soul Urge, Personality, Birthday, Maturity), karmic debts (13/14/16/19), hidden passion, balance, cornerstone/capstone, missing numbers, 4 Pinnacle Cycles, and Personal Year / Month / Day vibrations.
  8. **Omni Destiny Tier 7.0 Synthesis**: Multi-engine quantum cross-synergy matrix scoring potential, resilience, growth gap, and 15 weighted component factors.
- **Ultra-Fast Native Performance**: Full 8-engine synthesis executes in **< 50 milliseconds**.
- **Agent-First Output Formats**:
  - `agent`: High-density markdown prompt context optimized for LLM reasoning.
  - `json`: Fully typed, structured JSON serialization of DTO contracts.
  - `markdown`: Clean human-readable reports with structured markdown tables.
  - `text`: Terminal summary view.
- **Native MCP Server**: Ready-to-use Model Context Protocol server over stdio for Claude Desktop, Cursor, Antigravity, and AI coding assistants.

---

## Architecture

Eon uses a modular Rust monorepo where all analysis engines feed into a single unified façade (`eon-service`) that serves the native CLI and MCP layer (`eon-cli`):

```text
eon-core (Shared Primitives, Birth Info, Locations)
  ├── eon-data (Calendar Data & Binary Manseryuk Cache)
  └── eon-astro (Pure-Rust Swiss Ephemeris Engine)
        │
        ├── eon-saju (Four Pillars of Destiny Engine)
        ├── eon-vedic (Vedic Astrology / Jyotish Engine)
        ├── eon-western (Western Astrology Engine)
        ├── eon-zwds (Zi Wei Dou Shu Engine)
        ├── eon-human-design (Human Design Engine)
        ├── eon-qimen (QiMen DunJia Engine)
        └── eon-numerology (Pythagorean Numerology Engine)
              │
              ▼
          eon-service (Unified Façade & Contract DTO Boundary)
              │
              ▼
          eon-cli (Native CLI `eon`, LLM Prompts, stdio MCP Server)
```

---

## Workspace Crates

| Crate | Role | Description |
|---|---|---|
| [`crates/eon-cli`](file:///Users/sjkim1127/Eon/crates/eon-cli) | User & Agent Interface | High-performance CLI binary (`eon`), prompt generator, and JSON-RPC 2.0 MCP server. |
| [`crates/eon-service`](file:///Users/sjkim1127/Eon/crates/eon-service) | Service Façade (SSOT) | Unified DTOs, constructors, and façade methods integrating all 8 engines. |
| [`crates/eon-core`](file:///Users/sjkim1127/Eon/crates/eon-core) | Primitives | Core types (`HeavenlyStem`, `EarthlyBranch`, `BirthInfo`, timezone, coordinates). |
| [`crates/eon-data`](file:///Users/sjkim1127/Eon/crates/eon-data) | Calendar & Cache | Binary manseryuk data and lunar-solar conversion cache. |
| [`crates/eon-astro`](file:///Users/sjkim1127/Eon/crates/eon-astro) | Astronomical Engine | Pure-Rust Swiss Ephemeris (`swisseph-rs`) planetary and house calculations. |
| [`crates/eon-saju`](file:///Users/sjkim1127/Eon/crates/eon-saju) | Saju Engine | Four pillars, strength analysis, Gyeokguk, Yongshin, and Shinsal. |
| [`crates/eon-vedic`](file:///Users/sjkim1127/Eon/crates/eon-vedic) | Vedic Engine | Sidereal rasis, nakshatras, Shadbala, Ashtakavarga, and Yogas. |
| [`crates/eon-western`](file:///Users/sjkim1127/Eon/crates/eon-western) | Western Engine | Tropical planets, houses (Placidus/Koch/Equal), and aspect configurations. |
| [`crates/eon-zwds`](file:///Users/sjkim1127/Eon/crates/eon-zwds) | ZWDS Engine | Zi Wei Dou Shu 12 palaces, 14 major stars, SiHua, and destiny patterns. |
| [`crates/eon-human-design`](file:///Users/sjkim1127/Eon/crates/eon-human-design) | Human Design Engine | Type, authority, 9 centers, 36 channels, and 64 gates. |
| [`crates/eon-qimen`](file:///Users/sjkim1127/Eon/crates/eon-qimen) | QiMen Engine | 9 Palaces, 8 Doors, 9 Stars, 8 Deities, and Value Chief/Envoy. |
| [`crates/eon-numerology`](file:///Users/sjkim1127/Eon/crates/eon-numerology) | Numerology Engine | Life Path, Expression, Soul Urge, Birthday, Maturity, karmic debts, name analysis (Pythagorean + Chaldean), Pinnacle cycles, personal cycles and a heuristic two-person compatibility. |

---

## Quick Start

### Installation

Install the native `eon` binary locally via Cargo:

```bash
cargo install --path crates/eon-cli
```

Verify installation:
```bash
eon --version
eon --help
```

---

## CLI Command Reference

### 1. Full 8-Engine Synthesis (`analyze`)
Runs all 8 engines and calculates the Destiny Tier 7.0 synthesis matrix.

```bash
# High-density LLM Agent prompt context (default)
eon analyze --birth "1990-05-15 10:30" --gender m -f agent

# Fully typed structured JSON output
eon analyze --birth "1990-05-15 10:30" --gender m -f json

# Clean Markdown report
eon analyze --birth "1990-05-15 10:30" --gender m -f markdown

# Concise summary text
eon analyze --birth "1990-05-15 10:30" --gender m -f text
```

### 2. LLM System Prompt Generator (`prompt`)
Formats birth data into an optimized system/user prompt ready for immediate injection into AI chat completions:

```bash
eon prompt --birth "1990-05-15 10:30" --gender m --system
```

### 3. Individual Engine Commands

```bash
# Four Pillars of Destiny (Saju)
eon saju --birth "1990-05-15 10:30" --gender m -f text

# Vedic Astrology (Jyotish)
eon vedic --birth "1990-05-15 10:30" -f text

# Western Astrology
eon western --birth "1990-05-15 10:30" --house-system Placidus -f text

# Zi Wei Dou Shu
eon zwds --birth "1990-05-15 10:30" --gender m -f text

# Human Design
eon hd --birth "1990-05-15 10:30" -f text

# QiMen DunJia
eon qimen --birth "1990-05-15 10:30" --gender m -f text

# Numerology (Pythagorean by default; --system chaldean, --target-date for Personal Month/Day)
eon numerology --birth "1990-05-15" --name "Sujin Kim" -f text
eon numerology --birth "1990-05-15" --name "Sujin Kim" --system chaldean --target-date 2026-11-20 -f text

# Destiny Tier 7.0 Synthesis
eon tier --birth "1990-05-15 10:30" --gender m -f text
```

### 4. Options That Go Beyond The Natal Chart

```bash
# Vedic: choose the ayanamsa (Lahiri default, FaganBradley, Raman, Krishnamurti)
# and the year of the annual (Varshaphala / Tajika) chart
eon vedic --birth "1990-05-15 10:30" --ayanamsa Raman --target-year 2027 -f text

# Western: any of 10 house systems, and secondary progression to a date
eon western --birth "1990-05-15 10:30" --house-system Porphyry -f text
eon western --birth "1990-05-15 10:30" --progress-to 2027-03-01 -f text

# Zi Wei Dou Shu: annual palace and four transformations of a chosen year
eon zwds --birth "1990-05-15 10:30" --target-year 2027 -f text

# Human Design: sky of a date on the natal chart, or a planetary return
eon hd --birth "1990-05-15 10:30" --transit-at "2027-03-01 10:00" -f text
eon hd --birth "1990-05-15 10:30" --return solar --year 2027 -f text
eon hd --birth "1990-05-15 10:30" --return saturn -f text

# Saju luck cycles (yearly / monthly / daily / hourly) at a moment (default: now)
eon transit --birth "1990-05-15 10:30" --gender m --at "2027-03-01 10:00" -f text

# Themed Saju report: wealth | love | health
eon report --birth "1990-05-15 10:30" --gender m --theme wealth -f text

# Human Design Penta for 3 to 5 people ('@city' is optional per member)
eon penta --member "1990-05-15 10:30" --member "1988-01-02 08:00@Busan" --member "1995-09-30 21:15" -f text
```

### 5. Two-Person Compatibility (`compat`)

```bash
# Saju + Vedic (Ashtakoota, 36 points) + Western (synastry & composite) + Human Design (connection)
eon compat --birth "1990-05-15 10:30" --gender m --city seoul \
           --partner-birth "1992-08-20 18:45" --partner-gender f --partner-city busan -f text

# One system only
eon compat --system vedic --birth "1990-05-15 10:30" --partner-birth "1992-08-20 18:45" -f text

# Numerology pairs (Life Path, plus Expression / Soul Urge when both names are given)
eon compat --system numerology --birth "1990-05-15" --name "Sujin Kim" \
           --partner-birth "1992-08-20" --partner-name "Minho Park" -f text
```

`--system numerology` is a heuristic (temperament groups 1-5-7 / 2-4-8 / 3-6-9, weights
Life Path 50 / Expression 30 / Soul Urge 20) and is not part of `--system all`.

The partner takes the same options as the first person with a `--partner-` prefix
(`--partner-birth`, `--partner-gender`, `--partner-city`, `--partner-lat`, `--partner-lon`,
`--partner-tz`, `--partner-lunar`, `--partner-leap`, `--partner-name`). With `--system all` (default) a system that
cannot run, for example one that needs a birth time when only a date was given, is reported as
`skipped` instead of failing the whole command.

### Common Flags & Options
- `--birth <YYYY-MM-DD [HH:MM]>`: Birth date and optional time (default time: `12:00`).
- `-c, --city <NAME>`: Birth city for 100% offline resolution of coordinates and timezone (e.g. `seoul`, `busan`, `tokyo`, `new york`, `london`, `paris`).
- `--gender <m|f>`: Gender (`m` for male, `f` for female; default: `m`).
- `--lat <f64>`: Latitude (default: `37.5665` for Seoul).
- `--lon <f64>`: Longitude (default: `126.9780` for Seoul).
- `--tz <STRING>`: IANA timezone string (default: `Asia/Seoul`).
- `--lunar`: Interpret birth date using the Lunar calendar.
- `--leap`: Specify that the lunar month is a leap month.
- `-f, --format`: Output format. `analyze` accepts `agent` (default; `text` is an alias), `json` and `markdown`. Single-engine commands (`saju`, `vedic`, ...) accept `json` (default) and `text` (compact summary; `agent` and `markdown` are aliases of `text`).

---

## AI Agent & LLM Integration

Eon provides a ready-made pipeline for feeding rich astrological and metaphysical intelligence directly into LLM prompts.

```bash
# Example: Pipe Eon context directly into an LLM CLI or Python script
eon prompt --birth "1990-05-15 10:30" --system | llm -m gemini-2.0-pro "Synthesize the client's career prospects."
```

Sample output snippet from `eon prompt`:
```markdown
# [DESTINY INTELLIGENCE PROFILE FOR AI AGENT]

> **Birth**: 1990-05-15 10:30 (Asia/Seoul) | Location: Busan (Lat: 35.1796, Lon: 129.0756) | True Solar Time: 1990-05-15 10:06 (-24m)

## 1. Omni Destiny Tier 7.0
- **Overall Destiny Tier**: **S** (Birth Chart Raw: 95.0pt / Total Score: 96.0pt / Potential: **A** 78.0pt)
- **Profile Type**: growth | **Growth Gap**: +-18.0pt
- **8D Quantum Cross Synergies (Cross-Engine Resonance)**:
  * [Geometric Resonance (Western Aspects & ZWDS Formations)] +15pt — Western planetary aspect geometry resonates directly with Zi Wei Dou Shu primary palace formations.
  * [Quantum Energy Alignment (Human Design Centers & QiMen 9 Palaces)] +14pt — Human Design defined center flows align with QiMen DunJia Nine Palaces auspicious door layouts.

## 2. Four Pillars of Destiny (Saju / BaZi)
- **Four Pillars**: Year: 庚午 (Geng-Wu) | Month: 辛巳 (Xin-Si) | Day: 庚辰 (Geng-Chen) | Hour: 辛巳 (Xin-Si)
- **Day Master Strength**: Strong (Score: 39.7)
- **Structure & Yongshin**: Structure: PianGuan, Primary Yongshin: Water
...
```

---

## Model Context Protocol (MCP) Setup

Eon includes a stdio MCP server implementing JSON-RPC 2.0. AI coding environments (Claude Desktop, Cursor, Antigravity) can call `eon_analyze` for the full synthesis, or any single-engine tool below.

### Claude Desktop Configuration
Add Eon to your `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "eon": {
      "command": "eon",
      "args": ["mcp"]
    }
  }
}
```

### Available MCP Tools
- **`eon_analyze`**: all-engine synthesis plus Destiny Tier 7.0.
  - Parameters:
    - `birth` (string, required): e.g. `"1990-05-15 10:30"`
    - `gender` (string, optional): `"m"` or `"f"` (default: `"m"`)
    - `name` (string, optional): English name for Numerology
    - `format` (string, optional): `"agent"` (default), `"json"`, or `"markdown"`

Single-engine and relationship tools. All take `birth` (plus the optional `gender`, `lunar`, `leap`,
`city`, `lat`, `lon`, `timezone`) and a `format` of `"text"` (default, compact) or `"json"` (full):

| Tool | What it returns | Extra parameters |
|---|---|---|
| `eon_saju` | Four Pillars, strength, structure, yongshin, major luck | `night_rat` |
| `eon_saju_luck` | Yearly/monthly/daily/hourly luck at a moment | `at`, `night_rat` |
| `eon_report` | Themed markdown report | `theme`: `wealth`, `love`, `health` |
| `eon_vedic` | Chart, dashas, yogas, Shadbala, Ashtakavarga, Bhava Bala, gochara, annual chart | `ayanamsa`, `target_year`, `year_type` |
| `eon_western` | Planets, houses, aspects, patterns; optional secondary progression | `house_system`, `progress_to` |
| `eon_zwds` | Palaces, stars, four transformations, annual palace | `target_year` |
| `eon_human_design` | Type, authority, centers, channels; optional transit or return overlay | `transit_at`, `return`, `year` |
| `eon_human_design_penta` | Penta group dynamics | `members` (3 to 5), `city` |
| `eon_qimen` | QiMen DunJia birth chart | |
| `eon_numerology` | Core numbers, karmic debts, name analysis, pinnacles, personal year/month/day | `name`, `target_year`, `target_date`, `system` (`pythagorean`/`chaldean`) |
| `eon_iching` | HeLuo hexagrams and lifetime cycles | |
| `eon_compatibility` | Saju, Vedic, Western and Human Design comparison of two people (`system: numerology` adds a heuristic numerology pair score) | `partner_birth` (required) and the other `partner_*` fields (`partner_name`), `system`, `house_system` |

Date parameters (`at`, `transit_at`, `progress_to`) are `YYYY-MM-DD[ HH:MM]` in the birth timezone,
or an RFC 3339 timestamp.

---

## Rust SDK & Service Façade

You can also use Eon as an in-process Rust library by importing `eon-service`:

```toml
[dependencies]
eon-service = { path = "crates/eon-service" }
```

```rust
use anyhow::Result;
use eon_service::dto::{AnalysisInput, SajuAnalysisInput};
use eon_service::facade::analyze_saju;

fn main() -> Result<()> {
    let base = AnalysisInput::new(1990, 5, 15, 10, 30, "Asia/Seoul".to_string(), 37.5665, 126.9780, true);
    let saju_in = SajuAnalysisInput::new(base, true, false, None);
    let output = analyze_saju(saju_in)?;

    println!("Four Pillars: {:?}", output.report.pillars);
    println!("Strength: {:?}", output.report.strength);
    Ok(())
}
```

---

## Development & Verification

Verify the entire workspace with formatting, clippy linter, and full test suite:

```bash
# Check code formatting
cargo fmt --all -- --check

# Run Clippy with zero tolerance for warnings
cargo clippy --workspace --all-targets -- -D warnings

# Execute all workspace tests
cargo test --workspace

# Validate version synchronization
python3 scripts/sync_release_version.py --check
```

---

## Disclaimer & License

### Disclaimer
Eon is an experimental engineering platform designed to study and calculate symbolic and metaphysical rule systems. The calculations and reports generated by Eon are deterministic algorithmic evaluations of traditional rule systems and do not constitute scientific, medical, legal, psychological, or financial advice.

### License
Dual-licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
