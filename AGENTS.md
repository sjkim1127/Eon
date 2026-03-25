# AGENTS.md — Eon 리포지토리 AI 에이전트 가이드

이 파일은 AI 에이전트(GitHub Copilot, Antigravity, Cursor 등)가 Eon 리포지토리를 정확하게 이해하고 안전하게 기여할 수 있도록 작성된 지침서입니다.

---

## 1. 프로젝트 개요

Eon은 사주 명리학과 베딕 점성학을 **시스템 공학/리버스 엔지니어링** 관점으로 재해석한 운명 분석 플랫폼입니다.

- **웹(Vercel)**: React + Vite SPA → WASM 백엔드 (`crates/eon-wasm`)
- **데스크탑(Tauri v2)**: 동일 프론트엔드 → Rust 네이티브 백엔드 (`app/src-tauri`)
- **공통 도메인 로직**: `crates/eon-service` façade를 통해 Tauri·WASM 양쪽에 공급

```
app/src (React)
  └─ lib/backend.ts  ←─ 환경 자동 분기(isTauri 플래그)
       ├─ WasmBackendClient  →  crates/eon-wasm/src/lib.rs
       └─ TauriBackendClient →  app/src-tauri/src/lib.rs
                                     ↓
                            eon-service::facade  (단일 소스)
```

---

## 2. 디렉토리 맵

```
Eon/
├── app/
│   ├── src/
│   │   ├── components/
│   │   │   ├── layout/          # AppLayout.tsx, Fallbacks.tsx
│   │   │   ├── sections/        # 재사용 섹션 컴포넌트 (VulnerabilitySection 등)
│   │   │   ├── shared/          # Sidebar, BirthDrawer, ExportActionButtons 등
│   │   │   └── tabs/            # 탭 뷰 (SajuTab, VedicChartsTab, …)
│   │   ├── hooks/               # useAstrologyAnalysis, useBirthForm, useTabPrefetcher
│   │   ├── router/              # AppRoutes.tsx — React Router 설정 분리
│   │   ├── store/               # useAppStore.ts (Zustand, 단일 analysisState)
│   │   ├── types/               # saju.ts, analysis.ts, vedic.ts, common.ts
│   │   └── utils/               # exportMarkdown.ts, tierScore.ts, analysis.ts, …
│   ├── src-tauri/src/
│   │   ├── lib.rs               # Tauri 커맨드 (DTO 생성자 사용)
│   │   └── tier.rs              # get_destiny_tier_analysis 커맨드
│   └── package.json
│
├── crates/
│   ├── eon-core/                # 공통 타입 (HeavenlyStem, EarthlyBranch, BirthInfo, Location)
│   ├── eon-data/                # 만세력 바이너리 캐시
│   ├── eon-astro/               # Swiss Ephemeris FFI 바인딩
│   ├── eon-saju/                # 사주 엔진 (VM, Fuzzer, Topology, Linter, …)
│   ├── eon-vedic/               # 베딕 엔진 (Shadbala, Ashtakavarga, Dasha, Yoga)
│   ├── eon-service/             # 통합 Façade + DTO (SSOT)
│   │   ├── src/dto.rs           # AnalysisInput / SajuAnalysisInput 등 new() 생성자 포함
│   │   └── src/facade.rs        # analyze_saju / analyze_vedic / analyze_transit
│   └── eon-wasm/src/lib.rs      # WASM 익스포트 (DTO 생성자 사용)
│
├── DOCS/
│   └── ANALYSIS.md              # 110점 가중치 등 도메인 분석 문서
└── scripts/                     # 보조 스크립트
```

---

## 3. 핵심 아키텍처 원칙

### 3-1. 상태 관리 — 단일 소스 (SSOT)

> **절대 규칙:** 모든 분석 결과는 `useAppStore`의 `analysisState`(타입: `AnalysisBundleState`)를 통해서만 읽고 씁니다.

```typescript
// ✅ 올바른 패턴
const sajuData = useAppStore(state => state.analysisState.saju.data);

// ❌ 금지 — 별도 레거시 필드 추가 금지
const sajuReport = useAppStore(state => state.sajuReport); // 존재하지 않음
```

`AnalysisBundleState` 키: `saju` | `vedic` | `transit` | `tier`  
각 키의 타입: `AnalysisTaskState<T>` = `{ status, data, error }`

### 3-2. DTO 조립 — 생성자 사용

`eon_service::dto`에 각 입력 모델의 `new()` 생성자가 구현되어 있습니다.  
Tauri/WASM 브릿지 양쪽 모두 이 생성자를 호출하여 중복 보일러플레이트를 방지합니다.

```rust
// ✅ 올바른 패턴 (app/src-tauri/src/lib.rs, crates/eon-wasm/src/lib.rs 동일)
let input = SajuAnalysisInput::new(year, month, day, …, unknown_time);

// ❌ 금지 — 구조체 리터럴 직접 조립 금지
let input = SajuAnalysisInput { base: AnalysisInput { year, … }, … };
```

### 3-3. 컴포넌트 계층

```
App.tsx (전역 상태 → props 전달)
├── AppLayout    (layout/AppLayout.tsx) — 공통 레이아웃·사이드바·Drawer
│     └── AppRoutes (router/AppRoutes.tsx) — React Router 설정
│           └── <SajuTab> / <VedicChartsTab> / … (tabs/)
│                  └── <VulnerabilitySection> / … (sections/)
└── <Analytics />, <SpeedInsights />
```

탭 컴포넌트는 `lazy()` + `<Suspense>`로 코드 분할되어 있습니다.

---

## 4. 빌드 시스템 & 검증

### 4-1. 프론트엔드 (TypeScript)

```bash
cd app
npm install
npm run build     # tsc && vite build — 에러 0개 유지 필수
npm run dev       # 개발 서버
```

**PR 전 반드시:** `npm run build` 통과 확인.

### 4-2. Rust (Tauri 브릿지)

```bash
cd app/src-tauri
cargo check       # 경고 0개 권장, 에러 0개 필수
```

### 4-3. Rust (WASM)

```bash
cd crates/eon-wasm
cargo check                       # 빠른 타입 검증
wasm-pack build --target web --out-dir pkg  # 실제 WASM 빌드
```

### 4-4. Rust (도메인 크레이트)

```bash
cargo check --workspace            # 전체 워크스페이스 검증
cargo test --package eon-saju      # 사주 엔진 단위 테스트
cargo test --package eon-vedic     # 베딕 엔진 단위 테스트
```

---

## 5. 커밋 컨벤션

[Conventional Commits](https://www.conventionalcommits.org/) 규칙을 따릅니다.

| 접두사 | 사용 상황 |
|--------|-----------|
| `feat` | 새 기능 추가 |
| `fix` | 버그 수정 |
| `refactor` | 기능 변화 없는 구조 개선 |
| `chore` | 빌드 설정, 의존성 변경 |
| `docs` | 문서 수정 |
| `test` | 테스트 추가·수정 |
| `style` | UI/CSS 수정 |

**스코프 예시:** `feat(ui)`, `refactor(api)`, `fix(wasm)`, `docs(eon-service)`

---

## 6. 에이전트가 하면 절대 안 되는 일

1. **`useAppStore`에 `analaysisState` 외 별도 분석 결과 필드 추가** — 상태 이중화 야기.
2. **Tauri 또는 WASM 브릿지에서 DTO를 구조체 리터럴로 직접 조립** — `new()` 생성자 사용 필수.
3. **`app/src-tauri/src/lib.rs`와 `crates/eon-wasm/src/lib.rs`의 함수 시그니처를 서로 다르게 변경** — API Drift 발생.
4. **빌드 에러(tsc, cargo)를 무시하고 커밋** — CI 파이프라인 깨짐.
5. **궁합(Compatibility) 또는 AI Audit 관련 코드를 프론트엔드에 재추가** — 의도적으로 제거된 기능 (엔진 내부에는 존재, UI는 비활성).

---

## 7. 도메인 필수 지식

### 7-1. 간지(干支) 표기 규약

- **한자 표기**: `ganziDisplay(ganzi)` → 예: `"甲子"`
- **한글 표기**: `ganziHangul(ganzi)` → 예: `"갑자"`
- 두 함수 모두 `app/src/utils/ganzi.ts`에서 임포트.

### 7-2. 탭 ID 목록

```typescript
type TabId = "saju" | "vedic_charts" | "strength" | "transit" | "simulation" | "destiny_tier";
```

라우트 경로: `/` (saju), `/vedic_charts`, `/strength`, `/transit`, `/simulation`, `/destiny_tier`

### 7-3. 분석 흐름

```
useBirthForm()   →  BirthData (위도/경도/시간대/생년월일시 포함)
useAstrologyAnalysis.runAnalysis()
  ├─ backendClient.getSajuAnalysis()   →  analysisState.saju
  ├─ backendClient.getVedicAnalysis()  →  analysisState.vedic
  ├─ backendClient.getTransitAnalysis()→  analysisState.transit
  └─ backendClient.getDestinyTier()    →  analysisState.tier
```

### 7-4. VulnerabilitySection (주의 시점)

- `SajuAnalysisResult.vulnerability_report.critical_vectors` 배열 전체를 표시.
- 현재 8건/페이지 페이지네이션 + 유형 필터 탭 구현됨.
- 점수가 낮을수록 더 위험 (0.0 = 치명적, 100 = 경미).

---

## 8. 자주 참조하는 파일

| 파일 | 용도 |
|------|------|
| `app/src/store/useAppStore.ts` | 전역 상태 정의 |
| `app/src/types/saju.ts` | 사주 분석 결과 타입 |
| `app/src/types/analysis.ts` | 공통 분석 타입 (AnalysisBundleState 등) |
| `app/src/hooks/useAstrologyAnalysis.ts` | 분석 실행 훅 |
| `app/src/lib/backend.ts` | Tauri/WASM 분기 클라이언트 |
| `crates/eon-service/src/dto.rs` | Rust DTO 정의 + 생성자 |
| `crates/eon-service/src/facade.rs` | 도메인 분석 진입점 |
| `app/src/utils/exportMarkdown.ts` | 마크다운 리포트 생성 로직 |
| `DOCS/ANALYSIS.md` | 110점 가중치 등 도메인 알고리즘 문서 |
