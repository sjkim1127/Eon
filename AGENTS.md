# AGENTS.md — Eon 리포지토리 AI 에이전트 가이드

이 파일은 AI 에이전트(GitHub Copilot, Antigravity, Cursor, Claude Code 등)가 Eon 리포지토리를 정확하게 이해하고 안전하게 기여할 수 있도록 작성된 지침서입니다.

---

## 1. 프로젝트 개요

Eon은 사주 명리학, 베딕 점성학, 서양 점성학, 자미두수, 휴먼 디자인, 기문둔갑, 피타고라스 수비학을 **시스템 공학/리버스 엔지니어링** 관점으로 재해석한 차세대 다차원 운명 분석 플랫폼입니다.

기존의 프론트엔드 UI(Dioxus/React)를 전면 제거하고, **LLM 및 AI 에이전트가 직접 호출하고 해석할 수 있는 초고속 네이티브 Rust CLI 및 MCP(Model Context Protocol) 서버(`crates/eon-cli`)** 체제로 전환되었습니다.

- **실행 바이너리 (CLI & MCP)**: `eon` (`crates/eon-cli`)
- **백엔드/도메인 로직**: `crates/eon-service` façade (모든 분석 엔진의 단일 진입점 SSOT)
- **8대 분석 엔진**:
  1. 사주 명리학 (`eon-saju`)
  2. 베딕 점성학 (`eon-vedic`)
  3. 서양 점성술 (`eon-western`)
  4. 자미두수 (`eon-zwds`)
  5. 휴먼 디자인 (`eon-human-design`)
  6. 기문둔갑 (`eon-qimen`)
  7. 피타고라스 수비학 (`eon-numerology`)
  8. 통합 운명 티어 시스템 7.0 (`OmniDestinyTierInput` 기반 14대 가중치 퀀텀 매트릭스)

---

## 2. 디렉토리 맵

```
Eon/
├── crates/
│   ├── eon-cli/                 # 고성능 Rust CLI, LLM 프롬프트 생성기 및 MCP 서버
│   │   ├── src/
│   │   │   ├── commands/        # analyze, prompt, mcp, saju, vedic, etc.
│   │   │   ├── format/          # agent(LLM 주입용 마크다운), json, markdown, text
│   │   │   └── main.rs          # Clap 기반 CLI 엔트리포인트
│   │   └── Cargo.toml
│   ├── eon-core/                # 공통 타입 (HeavenlyStem, EarthlyBranch 등)
│   ├── eon-data/                # 만세력 바이너리 캐시
│   ├── eon-astro/               # Swiss Ephemeris FFI 바인딩
│   ├── eon-saju/                # 사주 엔진 (Four Pillars, 신살, 격국, 용신, 대운)
│   ├── eon-vedic/               # 베딕 엔진 (Shadbala, Ashtakavarga, Dasha, Yoga)
│   ├── eon-western/             # 서양 점성술 (행성 위치, 하우스, 메이저 아스펙트)
│   ├── eon-zwds/                # 자미두수 (12궁, 14주성, 사화, 성계 격국)
│   ├── eon-human-design/        # 휴먼 디자인 (타입, 권위, 9센터, 채널, 게이트)
│   ├── eon-qimen/               # 기문둔갑 (9궁, 8문, 9성, 8신, 시가팔문)
│   ├── eon-numerology/          # 수비학 (Life Path, Expression, Soul Urge)
│   ├── eon-ai/                  # 규칙 기반 신뢰도 감사 및 휴리스틱 린터
│   └── eon-service/             # 통합 Façade + DTO (SSOT)
│       ├── src/dto.rs           # AnalysisInput / TierInput 등 new() 생성자
│       └── src/facade.rs        # analyze_saju, analyze_all_synthesized 등
│
├── .github/workflows/           # CI/CD (포맷, 린트, 테스트, 릴리즈 바이너리 빌드)
├── scripts/                     # 패키징 및 릴리즈 헬퍼 스크립트
└── docs/                        # 도메인 분석 문서
```

---

## 3. 핵심 아키텍처 원칙

### 3-1. DTO 조립 — 생성자 사용
`eon_service::dto`에 구현된 `new()` 생성자를 통해 입력을 조립하여 도메인 로직을 호출합니다. 구조체 리터럴을 통해 필드를 직접 매핑하지 마세요.

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

### 3-2. AI Agent 최적화 포맷
CLI는 AI 에이전트 및 LLM 컨텍스트 주입에 최적화된 포맷을 기본 지원합니다:
- `--format agent` (또는 `eon prompt`): 고밀도 마크다운 프롬프트 컨텍스트 (LLM의 System/User 프롬프트에 직접 삽입 가능)
- `--format json`: 구조화된 JSON 데이터 출력
- `--format markdown`: 사람이 읽기 쉬운 상세 보고서
- `--format text`: 간결한 텍스트 요약

### 3-3. MCP (Model Context Protocol) 지원
`eon mcp` 명령어를 통해 stdio 기반 JSON-RPC 2.0 MCP 서버로 구동됩니다. Claude Desktop, Cursor, Antigravity 등 MCP 지원 AI 도구에서 바로 도구(`eon_analyze`)로 호출할 수 있습니다.

---

## 4. 빌드 & 실행 가이드

### 4-1. CLI 로컬 설치 & 빌드
```bash
# 로컬 바이너리 설치
cargo install --path crates/eon-cli

# 개발 빌드 및 즉시 실행
cargo run --bin eon -- --help
cargo run --bin eon -- analyze --birth "1990-05-15 10:30" --gender m
cargo run --bin eon -- prompt --birth "1990-05-15 10:30" --system
```

### 4-2. 워크스페이스 검증
```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
python3 scripts/sync_release_version.py --check
```

---

## 5. 에이전트가 하면 절대 안 되는 일

1. **Dioxus / React / 웹 UI 구조 코드로 롤백 금지**: `crates/eon-ui`, `package.json`, `app/`, `dx.toml` 등을 재생성하거나 웹 UI 종속성을 추가하지 마세요.
2. **비표준 외부 종속성 임의 추가 금지**: 순수 Rust 및 Swiss Ephemeris FFI 외에 불필요한 C 라이브러리나 무거운 런타임을 추가하지 마세요.
3. **도메인 엔진 계산 로직의 결정론적 순수성 보장**: 모든 분석 엔진은 입력값이 같으면 항상 동일한 결과를 출력해야 합니다.
