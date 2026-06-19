# 🌌 Eon: The Destiny Reversing Engine

> **"당신의 운명은 정해진 결말이 아니라, 새롭게 쓰일 수 있는 지도입니다."**

`Eon`은 사주 명리학과 베딕 점성학을 **시스템 공학(System Engineering)** 및 **리버스 엔지니어링(Reverse Engineering)** 의 관점으로 재해석한 차세대 운명 분석 플랫폼입니다. 인생의 데이터를 소스 코드로 간주하고, 이를 실행·디버깅·최적화하는 강력한 툴체인을 제공합니다.

**라이브 데모:** [Vercel 배포](https://eon-sage.vercel.app)

---

## 🏗️ 아키텍처 (100% Rust 기반)

기존 React/Tauri 체제를 모두 걷어내고, 프론트엔드부터 백엔드 엔진까지 **100% Rust 언어(Dioxus Web)** 로 통합 구축되었습니다. 브라우저에서 WASM으로 컴파일되어 구동되며, Vercel을 통해 정적 사이트로 배포됩니다.

```text
Eon/
├── crates/
│   ├── eon-ui/                 # Dioxus 기반 프론트엔드 (UI & 상태 관리)
│   │   ├── src/components/     # UI 컴포넌트 (SajuTab, VedicTab, AiTab 등)
│   │   ├── src/store/          # Dioxus Signal 및 IndexedDB 로컬 저장소 연동
│   │   ├── src/worker/         # Web Worker (gloo-worker) 백그라운드 연산 분리
│   │   └── public/             # CSS 자산 및 vercel.json
│   ├── eon-core/               # 공통 타입 (HeavenlyStem, EarthlyBranch, BirthInfo)
│   ├── eon-data/               # 만세력(萬歲曆) 바이너리 데이터
│   ├── eon-astro/              # Swiss Ephemeris C API 바인딩
│   ├── eon-saju/               # 사주 핵심 엔진 (VM, Fuzzer, Topology 등)
│   ├── eon-vedic/              # 베딕 점성학 엔진 (Shadbala, Dasha 등)
│   ├── eon-ai/                 # Groq API 기반 LLM 연동 모듈
│   └── eon-service/            # 도메인 파사드(Façade) 및 DTO
├── .github/workflows/          # Vercel CI/CD 배포 파이프라인 (GitHub Actions)
└── DOCS/                       # 시스템 명세 및 분석 문서
```

---

## ⚙️ eon-saju — 사주 리버싱 툴킷

사주 팔자를 OS + 프로세스로 모델링하여 시스템 공학 관점에서 분석합니다.

### 핵심 엔진

| 엔진 | 영감 | 기능 |
|------|------|------|
| **Saju-VM** | 가상 머신 | 사주를 레지스터(R0=木~R4=水)와 인스트럭션으로 변환. 100년 인생 경로를 ~0.01초에 시뮬레이션, 매년 `LifeFrame` 스냅샷 생성 |
| **Destiny TTD** | WinDbg TTD | 위기 지점에서 역방향 Backtrace로 운의 Root Cause 탐색. 평행 인생 경로 Life Diff 비교 |
| **DIE** (Destiny It Easy) | Detect It Easy | Shannon Entropy로 오행 무질서도 정량화. Packer(에너지 응축) 탐지 및 언패커 오행 제시 |
| **Destiny Fuzzer** | 보안 퍼저 | 대운×세운 60×60 전수 조합 무차별 대입 → 흉운 취약점(`kernel_panic`) 사전 탐지 |
| **Qi Topology** | 네트워크 분석 | 오행 에너지 흐름을 트래픽으로 모델링. Throughput / Bottleneck 구간 감지 |
| **KarmaLoadBalancer** | 부하 분산기 | 운의 급격한 변화(Traffic Spike) 시 시스템 과부하 진단 및 제어 전략 제안 |
| **Destiny Linter** | 코드 린터 | 사주 구조적 결함을 ERROR/WARN/INFO 레벨로 진단. `SajuLint { code, message, advice }` |

---

## 🌌 eon-vedic — 베딕 점성학 엔진 (BPHS 표준)

BPHS(Brihat Parashara Hora Shastra) 표준에 맞춰 구현한 초정밀 베딕 점성학 엔진.

### 차트 계산 및 강도 분석
- **Rasi 기본 차트 (D1) ~ D144** 16가지 분할 차트(Varga) 산출
- **Shadbala**: 행성 전쟁(Yuddha Bala)을 포함한 6가지 범주 점수 정밀 계산
- **Bhava Bala & Ashtakavarga**: 하우스 파워 분석 및 Bindu 히트맵

### 예측 / 관계 분석
- **Vimshottari Dasha**: 마하다샤(Mahadasha) 타임라인
- **Yoga Engine**: Raj Yoga, Dhana Yoga 등 자동 탐지
- **Panchanga**: 판창가 5요소 및 Sade Sati 분석

---

## 🖥️ 프론트엔드 기능

Dioxus Web으로 구현된 UI는 성능 최적화와 사용자 편의성을 위해 **Web Worker**와 **IndexedDB**를 적극적으로 활용합니다.

| 탭 | 표시 데이터 및 주요 기능 |
|----|------------|
| **Saju Tab (사주 분석)** | 원국 8글자, 격국/용신, 신강약 분석 패널 |
| **Vedic Tab (베딕 차트)** | 통합 대시보드, D1~D144 차트 분할 |
| **Tier Tab (운명의 티어)** | S+ ~ D 등급의 종합 인생 뱃지 시각화 |
| **Strength Tab (오행 세력)** | 사주 및 베딕의 오행 세력을 막대그래프로 비교 |
| **Transit Tab (대운 흐름)** | 대운 / 세운 / 마하다샤를 타임라인 테이블로 조회 |
| **Simulation Tab (생애)** | 0~100세 인생 흐름 및 길흉 그래프 렌더링 |
| **AI Tab (챗봇)** | GPT/Claude 벤치마킹 프리미엄 대화형 UI 지원 |

> 💡 **특장점**: 사주/베딕 분석과 같은 무거운 연산은 `gloo-worker`를 통해 백그라운드 스레드로 위임되며, 여러 지인들의 생년월일 프로필은 브라우저 내 `IndexedDB`에 무제한으로 저장하여 손쉽게 재로드할 수 있습니다.

---

## 🚀 시작하기

### 필수 조건
- [Rust](https://www.rust-lang.org/) (stable)
- **`dioxus-cli`** (`dx` 명령어 설치 필요)

```bash
# Dioxus CLI 설치
cargo install dioxus-cli --locked
# 혹은 더 빠른 설치 (cargo-binstall)
cargo binstall dioxus-cli -y
```

### 웹 로컬 개발 서버 실행

```bash
cd crates/eon-ui
dx serve
```
*브라우저가 열리고 핫 리로드(Hot Reload)가 활성화된 개발 서버가 실행됩니다.*

### 배포용 WebAssembly 빌드

```bash
cd crates/eon-ui
dx build --release
```
빌드된 정적 리소스(HTML, JS, WASM)는 `target/dx/eon-ui/release/web/public` 폴더에 생성됩니다. GitHub Actions 파이프라인(`.github/workflows/deploy.yml`)이 이 폴더를 Vercel로 자동 배포합니다.

---

## 🧩 모듈 의존성 트래픽

```text
eon-core ◄─────── eon-data
    ▲                  │
    │            (만세력 캐시)
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

## 📜 라이선스

MIT License — [LICENSE](LICENSE) 참조

---

> **"Eon과 함께 당신만의 고유한 삶의 나침반을 발견하세요."**
