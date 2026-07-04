# Eon

Saju (Four Pillars of Destiny) and Vedic Astrology analysis platform built entirely in Rust.

**Live:** [eon-sage.vercel.app](https://eon-sage.vercel.app)

---

## Architecture

Eon is a **100% Rust / Dioxus Web** application compiled to WebAssembly and hosted statically on Vercel. No backend server — all analysis runs in-browser.

```text
Eon/
├── crates/
│   ├── eon-ui/           # Dioxus frontend — UI components & Dioxus Signal state
│   ├── eon-core/         # Shared primitive types (HeavenlyStem, EarthlyBranch, BirthInfo)
│   ├── eon-data/         # Saju calendar binary cache (만세력)
│   ├── eon-astro/        # Swiss Ephemeris C FFI bindings
│   ├── eon-saju/         # Saju analysis engine
│   ├── eon-vedic/        # Vedic astrology engine (BPHS standard)
│   └── eon-service/      # Integration façade & DTOs (single source of truth)
├── .github/workflows/    # CI/CD — auto-deploys to Vercel on main branch push
└── DOCS/                 # Domain reference documentation
```

---

## Saju Engine (`eon-saju`)

Analyzes the Four Pillars (사주 팔자) via a rule-based computation pipeline.

| Module | Role |
|---|---|
| **Pillar Calculator** | Derives Year / Month / Day / Hour pillars from birth date/time using the sexagenary cycle |
| **Ten Gods (십성)** | Identifies the relational archetype of each stem relative to the Day Master |
| **Yongshin (용신)** | Determines the favorable element based on structural balance |
| **Structure Analysis** | Classifies the chart's dominant pattern (격국) with supporting evidence |
| **Qi Topology** | Maps the Five Element energy flow — detects dominant paths, blocked circuits, and missing links |
| **Lifecycle Diagnostic** | Traces Daeun / Sewun cycles to surface high-impact periods and structural vulnerabilities |
| **Structural Linter** | Flags chart-level imbalances at ERROR / WARN / INFO severity with corrective suggestions |
| **Scenario Analysis** | Exhaustively evaluates Daeun × Sewun combinations to surface future risk windows |

---

## Vedic Engine (`eon-vedic`)

Built to BPHS (Brihat Parashara Hora Shastra) standards with Swiss Ephemeris precision.

### Chart Calculation & Strength
- **Divisional Charts**: D1 (Rasi) through D144 — 16 Vargas computed with full precision
- **Shadbala**: Six-factor planetary strength (Sthana, Dig, Kala, Cheshta, Naisargika, Drik) including Yuddha Bala
- **Bhava Bala & Ashtakavarga**: House strength scores and Bindu heatmaps with Trikona / Ekadhipatya reductions

### Prediction & Compatibility
- **Dasha Systems**: Vimshottari Mahadasha timeline; Yogini Dasha 36-year cycle
- **Yoga Detection**: Automated recognition of Raj Yoga, Dhana Yoga, and other combinations
- **Panchanga**: Five traditional limbs (Vara, Tithi, Nakshatra, Yoga, Karana) with Sade Sati transit
- **Guna Milan**: Ashtakoota compatibility scoring (36-point system)
- **KP System**: Sign / Star / Sub Lord cuspal analysis

---

## Frontend (`eon-ui`)

Dioxus Web SPA. Heavy computations run on a background Web Worker; birth profiles persist locally via IndexedDB.

| Tab | Contents |
|---|---|
| **Saju** | Four Pillars chart, Ten Gods, Yongshin, Structure & Pattern analysis, Qi Topology |
| **Vedic** | SVG birth chart (South / North Indian style), Lagna badge, Panchanga, Arudha Padas, Planets table, House ratings, Dasha timeline, Tajika annual chart, Varga charts (D1–D144), Ashtakavarga grid |
| **Tier** | Lifecycle quality rating (S+ to D) by period |
| **Strength** | Comparative Saju / Vedic elemental strength bar charts |
| **Transit** | Unified Daeun / Sewun / Mahadasha timeline |
| **Simulation** | 0–100 year luck curve, Golden Time window highlight |

---

## Getting Started

**Prerequisites:** Rust (stable) + Dioxus CLI

```bash
cargo install dioxus-cli --locked
```

**Development server (hot-reload):**

```bash
cd crates/eon-ui
dx serve
```

**Production WASM build:**

```bash
cd crates/eon-ui
dx build --release
# Output: target/dx/eon-ui/release/web/public
```

CI/CD (`.github/workflows/deploy.yml`) deploys the output directory to Vercel on every push to `main`.

---

## Dependency Graph

```text
eon-core ◄──── eon-data
    ▲               │
    │        (calendar cache)
eon-astro ◄────────┘
    ▲
eon-saju ──────────────┐
eon-vedic              │
    │                  │
    └──► eon-service ◄─┘
              │
           eon-ui  →  WASM  →  Vercel
```

---

## License

MIT — see [LICENSE](LICENSE).
