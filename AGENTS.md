# AGENTS.md — Eon 리포지토리 AI 에이전트 가이드

이 파일은 AI 에이전트(GitHub Copilot, Antigravity, Cursor 등)가 Eon 리포지토리를 정확하게 이해하고 안전하게 기여할 수 있도록 작성된 지침서입니다.

---

## 1. 프로젝트 개요

Eon은 사주 명리학과 베딕 점성학을 **시스템 공학/리버스 엔지니어링** 관점으로 재해석한 운명 분석 플랫폼입니다.
과거 React/Tauri 체제에서 전환되어, 현재는 프론트엔드부터 백엔드 엔진까지 **100% Rust 단일 언어(Dioxus Web)**로 구축되어 있습니다.

- **프론트엔드 (웹 UI)**: Dioxus 프레임워크 (`crates/eon-ui`)
- **백엔드/도메인 로직**: `crates/eon-service` façade를 Dioxus 컴포넌트가 브라우저 Wasm 상에서 직접 비동기 호출 (`eon-wasm` 직렬화 브릿지 없음)
- **배포 (Vercel)**: GitHub Actions에서 `dx build --release` 실행 후 산출물(`target/dx/eon-ui/release/web/public`)을 Vercel CLI로 자동 배포

---

## 2. 디렉토리 맵

```
Eon/
├── crates/
│   ├── eon-ui/                  # Dioxus 기반 프론트엔드 (기존 React app/ 대체)
│   │   ├── src/
│   │   │   ├── components/      # UI 컴포넌트 (layout, shared, tabs)
│   │   │   ├── store/           # AnalysisState (Signal 기반 전역 상태)
│   │   │   └── router.rs        # Dioxus Router 설정
│   │   ├── public/              # 정적 에셋 및 vercel.json
│   │   └── Dioxus.toml
│   ├── eon-core/                # 공통 타입 (HeavenlyStem, EarthlyBranch 등)
│   ├── eon-data/                # 만세력 바이너리 캐시
│   ├── eon-astro/               # Swiss Ephemeris FFI 바인딩
│   ├── eon-saju/                # 사주 엔진 (VM, Fuzzer, Topology, Linter, …)
│   ├── eon-vedic/               # 베딕 엔진 (Shadbala, Ashtakavarga, Dasha, Yoga)
│   └── eon-service/             # 통합 Façade + DTO (SSOT)
│       ├── src/dto.rs           # AnalysisInput / SajuAnalysisInput 등 new() 생성자
│       └── src/facade.rs        # analyze_saju / analyze_vedic 등 핵심 진입점
│
├── .github/workflows/           # Vercel 배포용 CI/CD 액션
└── DOCS/                        # 도메인 분석 문서
```

---

## 3. 핵심 아키텍처 원칙

### 3-1. 상태 관리 — 단일 소스 (SSOT)
> **절대 규칙:** 모든 분석 결과는 `crates/eon-ui/src/store/mod.rs`에 정의된 `AnalysisState` (Dioxus `Signal` 컨텍스트)를 통해서만 읽고 씁니다. React 시절의 `useAppStore(Zustand)`는 폐기되었습니다.

```rust
// 올바른 패턴: Dioxus 전역 컨텍스트 활용
let mut state = use_context::<AnalysisState>();
let form = state.form.read().clone();
state.saju.write().status = TaskStatus::Loading;
```

### 3-2. DTO 조립 — 생성자 사용
`eon_service::dto`에 구현된 `new()` 생성자를 통해 입력을 조립하여 도메인 로직을 호출합니다.
구조체 리터럴을 통해 필드를 직접 매핑하지 마세요.

### 3-3. 프론트엔드 비동기 호출 패턴
Dioxus UI 탭에서 백엔드 로직(`eon_service::facade`)을 호출할 때는, 메인 스레드(UI)를 블로킹하지 않도록 반드시 `spawn(async move { ... })` 내에서 백그라운드 태스크로 처리합니다.

---

## 4. 빌드 시스템 & 검증

### 4-1. 프론트엔드 빌드 (Dioxus)
```bash
cd crates/eon-ui
cargo check        # 문법/타입 검증
dx build           # 개발용 빌드
dx build --release # 프로덕션 Wasm 빌드 (결과물: target/dx/eon-ui/release/web/public)
```
**절대 규칙**: `npm install`, `npm run dev`, `npm run build` 명령어는 더 이상 존재하지 않으므로 사용해서는 안 됩니다.

### 4-2. 엔진 통합 테스트
```bash
cargo check --workspace
cargo test --package eon-saju
cargo test --package eon-vedic
```

---

## 5. 에이전트가 하면 절대 안 되는 일

1. **React / Tauri 구조 코드로 롤백하려는 시도 금지**: `package.json`, `app/` 디렉토리, TypeScript 파일을 찾거나 npm 스크립트를 실행하려고 하지 마세요.
2. **eon-wasm 브릿지 재생성 금지**: Dioxus Wasm이 `eon-service`를 직접 호출하므로 예전 방식의 직렬화/역직렬화(JSON) 기반 Wasm 브릿지 API를 만들지 마세요.
3. **동기 블로킹 호출 금지**: UI 컴포넌트 블록(`rsx!`) 내에서 곧바로 무거운 분석 함수를 동기로 돌리면 브라우저 Wasm 스레드가 프리징됩니다. 로직은 반드시 비동기 `spawn` 블록 내에 위치해야 합니다.
