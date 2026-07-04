# Eon

동양 명리학(사주·자미두수)과 서양 점성술(베딕·서양·휴먼디자인)을 단일 Rust 워크스페이스로 통합한 운명 분석 엔진 라이브러리입니다.

**Live Demo:** [eon-sage.vercel.app](https://eon-sage.vercel.app)

---

## 개요

Eon은 다음 두 가지를 동시에 지향합니다.

1. **라이브러리**: `eon-service` 크레이트를 진입점으로 삼아 Rust 프로젝트에서 직접 의존성 추가 후 사용할 수 있는 분석 엔진 모음입니다. 각 도메인 엔진(사주·베딕·자미두수·서양·휴먼디자인)은 독립 크레이트로 분리되어 필요한 엔진만 선택적으로 가져올 수 있습니다.

2. **WASM 웹 앱**: `eon-ui` 크레이트가 Dioxus Web 프레임워크 위에서 전체 분석 UI를 제공하며, WebAssembly로 컴파일되어 Vercel에 정적 호스팅됩니다. 별도 백엔드 서버 없이 모든 연산이 브라우저 내에서 실행됩니다.

---

## 아키텍처

```
eon-core          # 공유 원시 타입 (HeavenlyStem, EarthlyBranch, BirthInfo)
eon-data          # 만세력 바이너리 캐시 (음·양력 변환 DB)
eon-astro         # Swiss Ephemeris C FFI 바인딩 (행성 황경 계산)
    │
    ├── eon-saju          # 사주 분석 엔진
    ├── eon-vedic         # 베딕 점성술 엔진 (BPHS 표준)
    ├── eon-zwds          # 자미두수 엔진
    ├── eon-western       # 서양 점성술 엔진
    └── eon-human-design  # 휴먼디자인 엔진
            │
        eon-service       # 통합 Façade + DTO (SSOT)
            │
         eon-ui           # Dioxus Web SPA → WASM → Vercel
```

모든 분석 함수는 `eon_service::facade`를 통해 단일 진입점으로 노출됩니다.

---

## 크레이트 상세

### `eon-service` — 통합 Façade

모든 엔진을 통합하는 단일 진입점입니다. 각 분석 함수는 타입 안전한 Input/Output DTO를 기반으로 동작합니다.

```rust
use eon_service::{dto::*, facade::*};

// 사주 분석
let input = SajuAnalysisInput::new(base, is_male, use_night_rat_hour, None);
let output: SajuAnalysisOutput = analyze_saju(input)?;

// 베딕 점성술 분석
let input = VedicAnalysisInput::new(base, None, None);
let output: VedicAnalysisOutput = analyze_vedic(input)?;

// 자미두수 분석
let input = ZwdsAnalysisInput { base, .. };
let output: ZwdsAnalysisOutput = analyze_zwds(input)?;

// 서양 점성술 분석
let output: WesternAnalysisOutput = analyze_western(input)?;

// 휴먼디자인 분석
let output: HumanDesignAnalysisOutput = analyze_human_design(input)?;

// 사주+베딕 통합 생애 등급 평가
let tier: TierResult = analyze_destiny_tier(saju_out, vedic_out, None)?;

// 테마별 보고서 생성 (재물·연애·건강)
let report: ThemedReportOutput = generate_themed_report(input)?;
```

**주요 입력 공통 구조 (`AnalysisInput`)**

```rust
pub struct AnalysisInput {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub is_lunar: bool,       // true면 음력 입력으로 처리
    pub is_leap_month: bool,  // 윤달 여부
    pub lat: f64,
    pub lon: f64,
    pub timezone: String,     // e.g. "Asia/Seoul"
}
```

---

### `eon-saju` — 사주 분석 엔진

사주팔자(四柱八字)를 규칙 기반으로 분석하는 핵심 엔진입니다. `SajuReport` 하나에 모든 분석 결과가 집약됩니다.

| 모듈 | 설명 |
|---|---|
| **FourPillars** | 출생 일시로부터 연·월·일·시 4주(柱) 8자(字) 도출. 음력 변환(만세력 DB 기반) 및 야자시 처리 지원 |
| **StrengthAnalysis** | 일간(日干) 기준 5원소 점수 산출 및 신강/신약 판단 |
| **YongshinAnalysis** | 구조적 균형에 따른 용신·희신·기신·구신·한신 결정 |
| **StructureAnalysis** | 격국(格局) 분류 — 내격 10격 + 외격 판별, 성격·파격 조건 및 근거 텍스트 포함 |
| **TenGodAnalysis** | 천간·지지 각 글자별 십성(十星) 산출 (비겁·식상·재성·관성·인성) |
| **SpiritMarkerAnalysis** | 신살(神殺) 탐지 — 도화살·역마살·공망·천을귀인 등 주요 신살 |
| **QiTopology** | 오행 에너지 순환망 분석 — 생(生)·극(克) 경로의 지배 흐름·단절 구간 탐지 |
| **RelationshipAnalysis** | 천간합·지지합·충·형·파·해 관계 목록화 |
| **VoidAnalysis** | 공망(空亡) 계산 |
| **SupplementaryPillars** | 태주(胎柱)·태원(胎元)·명궁(命宮)·대운(大運) 기준 보조 기둥 산출 |
| **MajorLuckAnalysis** | 10년 단위 대운(大運) 타임라인 — 입운 나이·천간·지지 포함 |
| **LifecycleDiagnostic** | 대운×세운 전체 조합 시뮬레이션 — 취약 구간 스코어링 |
| **GoldenTime** | 생애 점수 곡선 기반 최적 상승 구간 탐지 |
| **StructuralLinter** | 격국·신강신약·신살 기반 ERROR/WARN/INFO 레벨 진단 메시지 (`SajuLint { code, message, advice }`) |

---

### `eon-vedic` — 베딕 점성술 엔진

BPHS(Brihat Parashara Hora Shastra) 기준으로 구현된 고정밀 베딕 점성술 엔진입니다. Swiss Ephemeris를 통해 Lahiri Ayanamsa 기반 행성 황경을 산출합니다.

| 분야 | 세부 항목 |
|---|---|
| **차트 계산** | D1(Rasi)부터 D144까지 16개 Varga 분할 차트 계산 |
| **Shadbala** | 행성별 6대 강도(Sthana·Dig·Kala·Cheshta·Naisargika·Drik Bala) + Yuddha Bala |
| **Bhava Bala** | 12하우스 강도 점수 (Lord·Dig·Drishti 분해) |
| **Ashtakavarga** | SAV 12하우스 그리드 + BAV Trikona/Ekadhipatya 축약 행렬 |
| **Dasha** | Vimshottari Mahadasha 타임라인, Yogini Dasha 36년 주기 |
| **Yoga 탐지** | Raj Yoga·Dhana Yoga·Mahabhagya Yoga 등 주요 요가 자동 인식 |
| **Panchanga** | Vara·Tithi·Nakshatra·Yoga·Karana 5지(支) + Sade Sati 토성 전이 분석 |
| **궁합 (Guna Milan)** | Ashtakoota 8대 요소별 점수 산출 (총 36점) |
| **KP System** | 12하우스 Cusp별 Sign Lord / Star Lord / Sub Lord 분석 |
| **Tajika** | 연간 태양 복귀 차트(Solar Return) 행성 위치 산출 |
| **Arudha Padas** | A1~A12 아루다 파다 하우스 산출 |

---

### `eon-zwds` — 자미두수 엔진

중국 전통 성반 명리학 자미두수(紫微斗數)를 구현한 엔진입니다.
알고리즘 레퍼런스: [SylarLong/iztro](https://github.com/SylarLong/iztro) (TypeScript, MIT)

- 12궁(宮) 성반 배치 계산
- 14주성(主星) + 6보조성 + 중잡성 약 50성 배치
- 사화(四化: 化祿·化權·化科·化忌) 비행
- 대한(大限, 10년 운) / 유년(流年, 1년 운)
- 명주(命主) / 신주(身主) 산출

---

### `eon-western` — 서양 점성술 엔진

Swiss Ephemeris FFI(`eon-astro`)를 활용하여 서양 점성술 차트를 계산합니다.

- 10대 행성 + Chiron + True Node 황경 및 역행 여부
- Placidus / Koch / Whole Sign / Equal House 하우스 시스템 지원
- 행성 간 메이저·마이너 Aspect 계산 (Orb 허용치 설정 가능)
- 원소·성질·극성 기반 성향 지표 분석

---

### `eon-human-design` — 휴먼디자인 엔진

Swiss Ephemeris 기반으로 정확한 I Ching Gate 배치를 계산합니다. Jovian Archive 레퍼런스와 검증 완료.

- 의식(Personality)·무의식(Design) 행성 10개씩 — Gate·Line 산출
- 9센터 정의(Defined/Undefined) 판별
- 채널(Channel) 활성화 및 Circuit 분류
- Type / Authority / Profile / Definition 도출

---

### `eon-astro` — Swiss Ephemeris FFI

C 라이브러리 `libswe`를 Rust에서 안전하게 래핑한 천문 계산 기반 크레이트입니다.

- 임의 날짜/시각의 행성 황경(Longitude), 위도(Latitude), 속도(Speed) 산출
- Julian Day 변환, Ayanamsa 적용(Lahiri/True Chitrapaksha/Fagan-Bradley)
- 어센던트(Lagna) 및 하우스 Cusp 계산

---

## 빠른 시작

### 의존성 추가

```toml
[dependencies]
eon-service = { git = "https://github.com/sjkim1127/Eon" }
```

### 사주 분석 예시

```rust
use eon_service::{dto::AnalysisInput, facade::analyze_saju};

fn main() {
    let base = AnalysisInput {
        year: 1990, month: 5, day: 15,
        hour: 10, minute: 0,
        is_lunar: false, is_leap_month: false,
        lat: 37.5665, lon: 126.9780,
        timezone: "Asia/Seoul".to_string(),
    };
    let input = eon_service::dto::SajuAnalysisInput::new(base, true, false, None);
    let result = analyze_saju(input).unwrap();

    println!("일주: {:?}", result.report.pillars.day);
    println!("용신: {:?}", result.report.yongshin);
    println!("격국: {}", result.report.structure.name);
}
```

---

## 웹 앱 실행

```bash
# 개발 서버 (hot-reload)
cd crates/eon-ui
dx serve

# 프로덕션 WASM 빌드
dx build --release
# 결과물: target/dx/eon-ui/release/web/public
```

GitHub Actions(`.github/workflows/deploy.yml`)가 `main` 브랜치 푸시 시 자동으로 Vercel에 배포합니다.

---

## 지원 분석 탭 (웹 UI)

| 탭 | 주요 내용 |
|---|---|
| **사주** | 8자 차트, 십성, 용신, 격국, 신살, 오행 순환망 |
| **베딕** | SVG 출생 차트(남·북인도 스타일), 판창가, 아루다 파다, 행성 표, 하우스 강도, 다샤 타임라인, 타지카, 바르가 차트, 아슈타카바르가 |
| **자미두수** | 12궁 성반, 주성 배치, 사화, 대한·유년 운세 |
| **서양** | 행성 위치, 하우스, 메이저 Aspect 차트 |
| **휴먼디자인** | Type·Authority·Profile·센터 정의 |
| **등급(Tier)** | 사주+베딕 통합 생애 등급 (S+ ~ D) |
| **강도(Strength)** | 사주·베딕 원소 강도 비교 바 차트 |
| **Transit** | 대운·세운·Mahadasha 통합 타임라인 |
| **Simulation** | 0~100세 운세 곡선, 황금기 구간 시각화 |

---

## 라이선스

MIT — [LICENSE](LICENSE) 참조.
