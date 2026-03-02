# 🌌 Eon: The Destiny Reversing Engine

> **"Fate is not a prophecy; it is an Executable Binary."**

`Eon`은 사주 명리학과 베딕 점성학을 **시스템 공학(System Engineering)** 및 **리버스 엔지니어링(Reverse Engineering)** 의 관점으로 재해석한 차세대 운명 분석 플랫폼입니다. 인생의 데이터를 소스 코드로 간주하고, 이를 실행·디버깅·최적화하는 강력한 툴체인을 제공합니다.

**라이브 데모:** [Vercel 배포](https://eon-sage.vercel.app) | **데스크탑:** Tauri v2 (`npm run tauri dev`)

---

## 🏗️ 아키텍처

```text
Eon/
├── app/                        # 프론트엔드 (React + Vite + TailwindCSS)
│   ├── src/
│   │   ├── App.tsx             # 단일 SPA — 5개 탭 (overview / saju / vedic_charts / strength / transit)
│   │   └── lib/api.ts          # Tauri IPC ↔ WASM 자동 분기 API 레이어
│   └── src-tauri/src/lib.rs    # Tauri 커맨드 (get_vedic_analysis, get_saju_analysis, get_transit_analysis, get_ai_audit)
│
├── crates/
│   ├── eon-core/               # 공통 타입 (HeavenlyStem, EarthlyBranch, BirthInfo, Location, DST 보정)
│   ├── eon-data/               # 만세력(萬歲曆) 바이너리 캐시
│   ├── eon-astro/              # Swiss Ephemeris C API 바인딩 — 초정밀 행성 위치 연산
│   ├── eon-saju/               # 사주 핵심 엔진 (아래 ⚙️ 섹션 참조)
│   ├── eon-vedic/              # 베딕 점성학 엔진 (아래 🌌 섹션 참조)
│   ├── eon-ai/                 # LLM 컨텍스트 생성기 (DestinyAIAuditor)
│   └── eon-wasm/               # WASM 번들 — Vercel 웹 배포용 (get_saju_analysis, get_transit_analysis, get_vedic_analysis)
```

**Tauri 환경** (데스크탑): `invoke()` → Rust 네이티브 백엔드  
**Web 환경** (Vercel): `import("eon-wasm")` → WASM 번들 자동 폴백 (동일 API)

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
| **Golden Time Finder** | 슬라이딩 윈도우 | 100년 시뮬레이션 프레임에서 최고 점수 구간(황금기) 자동 추출 |
| **Cyclomatic Complexity** | 소프트웨어 복잡도 | 사주의 Cyclomatic M 및 Stability Grade 계산 |
| **CompatibilityAuditor** | IPC/프로세스 간 통신 | 두 사주의 synergy/conflict/deadlock 분석. `sync_score` 0~100 |

### 분석 모듈

- **FourPillars**: 사주팔자(四柱八字) 계산 — Swiss Ephemeris 기반 대운(大運) 정밀 산출
- **StrengthAnalysis**: 신강/신약 4득(得令·得地·得時·得勢) 분석
- **YongshinAnalysis**: 용신(用神) 및 보조 용신 추천
- **StructureAnalysis**: 격국(格局) 탐지
- **SpiritMarkers**: 12신살(神煞) 분석
- **VoidAnalysis**: 공망(空亡) 분석
- **TwelveStages**: 12운성(運星) 계산
- **YearlyLuck / MonthlyLuck**: 세운(歲運) / 월운(月運) — 원국과의 합충형해 관계 포함

---

## 🌌 eon-vedic — 베딕 점성학 엔진 (BPHS 표준)

BPHS(Brihat Parashara Hora Shastra) 표준에 맞춰 구현한 초정밀 베딕 점성학 엔진.

### 차트 계산

- **Rasi 기본 차트 (D1) ~ D144** 16가지 분할 차트(Varga) — 행성 배치, 역행(Retrograde), 연소(Combust) 상태 포함
- **Jaimini Chara Karakas**: Atmakaraka(영혼) / Amatyakaraka(직업) / Darakaraka(파트너) 산출

### 강도 분석

- **Shadbala**: 행성 전쟁(Yuddha Bala) + 특수 시각 선형 보간으로 Graha Bala 정밀 계산
- **Bhava Bala**: 하우스 주인(Lord) + 방위(Dig) + Drishti 가중치 결합
- **Ashtakavarga**: BPHS 표준 bindu 감쇄 동기화 — `Sarvashtakavarga` 12하우스 포인트
- **Vimshopaka**: 행성별 16 Varga 점수 합산

### 예측 / 관계 분석

- **Vimshottari Dasha**: 다샤 타임라인 (현재 대운 포커스 포함)
- **Yoga Engine**: Raj Yoga, Dhana Yoga, Parivartana Yoga 등 자동 탐지
- **Panchanga**: Tithi / Nakshatra / Yoga / Karana (판창가 5요소)
- **Ashta Kuta 궁합**: 8항목 36점 체계 (Nadi~Varna)
- **Sade Sati**: 토성 7.5년 주기 경고

---

## 🖥️ 프론트엔드 화면 구성

> 웹(Vercel) + 데스크탑(Tauri) 동일 UI. 이중 경로 자동 분기.

| 탭 | 표시 데이터 |
|----|------------|
| **대시보드** | Atmakaraka · 현재 Dasha · 전체 차트 강도(Shadbala) · 낙샤트라 · 사데사티 · 12하우스 Bhava Bala |
| **사주 분석** | 사주팔자 8글자 · 신강/신약 4득 분석 · 용신/격국 · 신살 · 대운 10주기 · 골든타임 |
| **베딕 차트** | D1/D9/D10/D30/D60/D144 분할 차트 2×3 그리드 (Sign별 행성 배치) |
| **역량 및 기운** | Qi Topology 5원소 네트워크 · Entropy/DIE 등급 · Fuzzer 취약점 수 · Destiny Linter 진단 · KarmaLoadBalancer 상위 8건 |
| **현재 운세** | 세운/월운 간지 카드 · LifeFrame Score/Tags/ESIL 트레이스 · 전후 시기 부하 진단 |

---

## 🚀 시작하기

### 필수 조건

- [Rust](https://www.rust-lang.org/) (stable)
- [Node.js](https://nodejs.org/) 18+
- [wasm-pack](https://rustwasm.github.io/wasm-pack/) (웹 빌드 시)

### 웹 개발 서버 (WASM)

```bash
# 1. WASM 패키지 빌드
cd crates/eon-wasm
wasm-pack build --target web --out-dir pkg

# 2. 프론트엔드 실행
cd ../../app
npm install
npm run dev
```

### 데스크탑 앱 (Tauri)

```bash
cd app
npm install
npm run tauri dev
```

### Cargo 예제 실행

```bash
# 사주 종합 분석 예제
cargo run --package eon-saju --example verify_user

# 베딕 점성학 검증
cargo run --package eon-vedic --example verify_vedic

# AI 감사 데모
cargo run --package eon-ai --example ai_audit_demo
```

---

## 🗺️ 구현 현황 및 로드맵

### ✅ 완료

- [x] Saju-VM 100년 생애 시뮬레이션 + ESIL 트레이스
- [x] Destiny TTD Backtrace / Life Diff
- [x] DIE (Shannon Entropy + Packer 탐지)
- [x] Destiny Fuzzer (대운×세운 전수 조사)
- [x] Qi Topology + KarmaLoadBalancer
- [x] Destiny Linter (ERROR/WARN/INFO)
- [x] Golden Time Finder
- [x] 대운 정밀 계산 (Swiss Ephemeris)
- [x] 세운/월운 + 원국 합충 관계
- [x] DST / 진태양시 자동 보정 (BirthInfo)
- [x] Vedic Chart D1~D144 전 계층
- [x] Shadbala / Bhava Bala / Ashtakavarga / Vimshopaka
- [x] Vimshottari Dasha / Yoga Engine / Panchanga
- [x] Ashta Kuta 베딕 궁합 (백엔드)
- [x] 사주 CompatibilityAuditor IPC (백엔드)
- [x] AI 컨텍스트 생성 (DestinyAIAuditor + get_ai_audit 커맨드)
- [x] Vercel WASM 배포 (get_saju_analysis, get_vedic_analysis, get_transit_analysis)
- [x] Tauri 데스크탑 빌드

### 🔧 진행 중 / 버그 수정 예정

- [ ] `strength` 탭 `entropy.score` 필드명 버그 수정 (`shannon_entropy` → `score`)
- [ ] `strength` 탭 Topology 병목 배지 버그 수정 (`node.is_bottleneck` → 최상위 `qi_topology.bottleneck` 비교)
- [ ] `overview` 탭 Amatyakaraka / Darakaraka 카드 추가
- [ ] `saju` 탭 생애 시뮬레이션 타임라인 차트 (`simulation_frames` 시각화)
- [ ] `transit` 탭 세운/월운 합충 뱃지 + 12운성 표시
- [ ] `strength` 탭 ComplexityAnalysis / DieAnalysis 카드 추가 (API 응답 확장 필요)
- [ ] `vedic_charts` 탭 역행/연소 배지 + Panchanga 섹션

### 📋 예정

- [ ] **궁합 탭** — 두 번째 사람 입력 폼 + 사주 CompatibilityAuditor + Ashta Kuta 36점 UI
- [ ] **AI Audit 탭** — `get_ai_audit` api.ts 노출 + 코어덤프 형식 UI
- [ ] **Yoga 섹션** — `VedicAnalysisReport`에 `yogas` 필드 추가 + UI 카드
- [ ] **Ashtakavarga 그리드** — `chart.sav` 12하우스 히트맵
- [ ] **Human Design 바디그래프** 연동
- [ ] **주역 64괘 → 6비트 논리 게이트** 구현

---

## 🧩 크레이트 의존성 그래프

```
eon-core ◄─────── eon-data
    ▲                  │
    │            (만세력 캐시)
eon-astro ◄────────────┘
    ▲
eon-saju ──────────────────► eon-ai
eon-vedic                         │
    │                             │
    └──────► eon-wasm ◄───────────┘
                 │
             (WASM pkg)
                 │
    app/src-tauri ◄──── eon-saju
         │              eon-vedic
         │              eon-ai
         ▼
     App.tsx (React)
```

---

## 📜 라이선스

MIT License — [LICENSE](LICENSE) 참조

---

> **"Debug your fate. Optimize your life."**
