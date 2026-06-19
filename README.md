# 🌌 Eon: The Destiny Reversing Engine

> **"Your destiny is not a predetermined conclusion, but a map waiting to be rewritten."**

`Eon` is a next-generation destiny analysis platform that reinterprets Korean Saju (Four Pillars of Destiny) and Vedic Astrology under the lens of **Systems Engineering** and **Reverse Engineering**. It treats life data as source code, providing a powerful toolchain to execute, debug, and optimize it.

**Live Demo:** [Vercel Deployment](https://eon-sage.vercel.app)

---

## 🏗️ Architecture (100% Pure Rust)

By replacing the legacy React/Tauri framework, Eon is fully consolidated into a **100% Rust codebase (Dioxus Web)**. The entire application compiles to WebAssembly (WASM) to run directly inside the browser client and is hosted statically on Vercel.

```text
Eon/
├── crates/
│   ├── eon-ui/                 # Dioxus frontend (UI & State Management)
│   │   ├── src/components/     # UI Components (SajuTab, VedicTab, AiTab, etc.)
│   │   ├── src/store/          # Dioxus Signal & IndexedDB local storage binding
│   │   ├── src/worker/         # Web Worker (gloo-worker) background computations
│   │   └── public/             # CSS assets & vercel.json configuration
│   ├── eon-core/               # Shared types (HeavenlyStem, EarthlyBranch, BirthInfo)
│   ├── eon-data/               # Saju calendar binary database cache
│   ├── eon-astro/              # Swiss Ephemeris C API FFI Bindings
│   ├── eon-saju/               # Core Saju Engine (VM, Fuzzer, Topology, etc.)
│   ├── eon-vedic/              # Core Vedic Astrology Engine (Shadbala, Dasha, etc.)
│   ├── eon-ai/                 # Groq API LLM integration module
│   └── eon-service/            # Integration Façade & DTOs (SSOT)
├── .github/workflows/          # Vercel CI/CD Deployment Pipeline (GitHub Actions)
└── DOCS/                       # System specifications & domain documentation
```

---

## ⚙️ eon-saju — Saju Reversing Toolkit

Eon models Saju (Four Pillars) as an Operating System and processes, analyzing them from a systems engineering perspective.

### Core Engines

| Engine | Inspired by | Features |
|------|------|------|
| **Saju-VM** | Virtual Machine | Compiles Saju pillars into register states ($R_0=\text{Wood} \sim R_4=\text{Water}$) and instructions. Simulates a 100-year life path in ~0.01s, generating annual `LifeFrame` snapshots. |
| **Destiny TTD** | WinDbg TTD | Time Travel Debugging. Backtraces critical life turning points to locate the root causes of luck. Enables diff-comparisons of parallel life paths. |
| **DIE** (Destiny It Easy) | Detect It Easy | Uses Shannon Entropy to quantify the disorder of Five Elements. Detects Packers (energy condensation) and proposes unpacking elements. |
| **Destiny Fuzzer** | Security Fuzzer | Bruteforces $60 \times 60$ full combinations of Daeun × Sewun to detect vulnerabilities (`kernel_panic`) in future luck cycles. |
| **Qi Topology** | Network Analysis | Models Five Element flow as network traffic. Detects Throughput and Bottleneck paths. |
| **KarmaLoadBalancer** | Load Balancer | Identifies system overload during rapid luck changes (Traffic Spikes) and suggests flow distribution and control strategies. |
| **Destiny Linter** | Static Code Linter | Diagnoses structural flaws in Saju at ERROR, WARN, and INFO levels. Outputs `SajuLint { code, message, advice }`. |

---

## 🌌 eon-vedic — Vedic Astrology Engine (BPHS Standard)

A high-precision Vedic Astrology engine built strictly according to BPHS (Brihat Parashara Hora Shastra) standards.

### Chart Computation & Strength Analysis
- **Rasi (D1) to D144**: Computes 16 divisional charts (Vargas) with high precision.
- **Shadbala**: Calculates 6 sources of planetary strengths, including planetary wars (Yuddha Bala).
- **Bhava Bala & Ashtakavarga**: Houses strength analysis and Bindu heatmaps with triangular (Trikona) and dual-lord (Ekadhipatya) reductions.

### Prediction & Compatibility
- **Dasha Systems**: Mahadasha timeline via Vimshottari Dasha, and a 36-year predictive cycle via Yogini Dasha.
- **Yoga Engine**: Auto-detects complex planetary combinations (Raj Yoga, Dhana Yoga, etc.).
- **Panchanga**: 5 traditional limbs (Vara, Tithi, Nakshatra, Yoga, Karana) and Sade Sati transit analysis.

---

## 🖥️ Frontend Features

The Dioxus Web UI leverages **Web Workers** and **IndexedDB** for optimal performance and local persistence.

| Tab | Display Data & Key Features |
|----|------------|
| **Saju Tab** | Saju 8 characters, **Structure & Pattern** (Stems projection path & reasons), **Yongshin Recommendations** (element-by-element list of core elements), strength analysis panel. |
| **Vedic Tab** | **Dashboard (Sub-tab 0)**: High-precision SVG-based **Visual Birth Chart** (supporting South Indian perimeter style & North Indian diamond style with live switcher), Lagna badge with Ayanamsa degrees, **Vedic Destiny Profile** card, **Panchanga Limbs** card (with timezone-corrected sunrise/sunset times), **Arudha Padas** (A1-A12) Grid, **Planets Table** (with Drishti aspects and Vimshopaka scores), **House Ratings** (with Lord, Dig, and Drishti score breakdowns), and **Analysis Metadata** card.<br>**Dasha (Sub-tab 2)**: Toggle between Vimshottari and **Yogini Dasha** timeline tables.<br>**Tajika (Sub-tab 5)**: Annual chart calculations with the **Tajika Annual Planets Table** (coordinates, house index, retrograde/combust status).<br>**Varga Charts (Sub-tab 6)**: Dynamic rendering of D1 to D144 divisional charts with South/North Indian style switchers and planetary details table.<br>**Varga Interpretations (Sub-tab 7)**: Navamsa D9 marriage/relationships overview, Dasamsa D10 career path overview, and **Varga Interpretations** (Vargottama, Pushkar Navamsa status badges for Sun-Saturn).<br>**Ashtakavarga (Sub-tab 8)**: SAV 12-house grid and interactive BAV reduction matrix with Trikona/Ekadhipatya reduction switchers. |
| **Tier Tab** | Qualitative lifecycle rating badges (S+ to D). |
| **Strength Tab** | Multi-dimensional comparison of Saju and Vedic element strengths via bar charts. |
| **Transit Tab** | Unified timeline of Daeun, Sewun, and Mahadasha cycles. |
| **Simulation Tab** | Lifecycle trajectory visualization (0–100 years luck curve SVG chart), and **Golden Time** highlight. |
| **AI Tab** | Interactive LLM chat interface configured with Groq API. |

> 💡 **Core Advantage**: Computational overhead (such as Saju/Vedic chart analysis) is delegated to background threads using `gloo-worker`. Client data is stored locally in `IndexedDB`, allowing users to save and reload birth profiles instantly.

---

## 🚀 Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/) (stable)
- **`dioxus-cli`** (`dx` command)

```bash
# Install Dioxus CLI
cargo install dioxus-cli --locked
# Or speed up installation using cargo-binstall
cargo binstall dioxus-cli -y
```

### Local Development Server

```bash
cd crates/eon-ui
dx serve
```
*This command runs a local dev server with hot reloading enabled and automatically opens it in your default browser.*

### Production WASM Compilation

```bash
cd crates/eon-ui
dx build --release
```
The compiled static assets (HTML, JS, and WASM) are generated under `target/dx/eon-ui/release/web/public`. The GitHub Actions CI/CD pipeline (`.github/workflows/deploy.yml`) automatically deploys this directory to Vercel on commits to the main branch.

---

## 🧩 Dependency Graph

```text
eon-core ◄─────── eon-data
    ▲                  │
    │            (Calendar Cache)
eon-astro ◄────────────┘
    ▲
eon-saju ──────────────────► eon-ai
eon-vedic                         │
    │                             │
    └──────► eon-service ◄────────┘
                 │
              eon-ui (Dioxus Web)
                 │
            (WASM / HTML)
                 ▼
          Vercel Static Hosting
```

---

## 📜 License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.

---

> **"Navigate your life with Eon - the compiler for your destiny."**
