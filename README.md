# Eon

동양 명리학(사주·자미두수)과 서양 점성술(베딕·서양·휴먼디자인)을 단일 Rust 워크스페이스로 통합한 운명 분석 엔진 라이브러리입니다.

모든 연산은 순수 Rust로 구현되며, `eon-service` 크레이트 하나로 전체 엔진에 접근할 수 있습니다.
프론트엔드(`eon-ui`)는 Dioxus Web 기반 SPA로 WebAssembly로 컴파일되어 Vercel에 정적 호스팅됩니다.

**Live Demo:** [eon-sage.vercel.app](https://eon-sage.vercel.app)

---

## 목차

- [워크스페이스 구조](#워크스페이스-구조)
- [빠른 시작](#빠른-시작)
- [eon-core — 공통 타입](#eon-core--공통-타입)
- [eon-astro — Swiss Ephemeris FFI](#eon-astro--swiss-ephemeris-ffi)
- [eon-data — 만세력 DB](#eon-data--만세력-db)
- [eon-saju — 사주 분석 엔진](#eon-saju--사주-분석-엔진)
  - [FourPillars & LifecycleMachine](#fourpillars--lifecyclemachine)
  - [용신 분석](#용신-분석)
  - [격국 분석](#격국-분석)
  - [신강신약 판정](#신강신약-판정)
  - [십성 분석](#십성-분석)
  - [신살 탐지](#신살-탐지)
  - [합충형해 관계 분석](#합충형해-관계-분석)
  - [오행 순환망 (Qi Topology)](#오행-순환망-qi-topology)
  - [대운·세운 분석](#대운세운-분석)
  - [생애 점수 & 황금기](#생애-점수--황금기)
  - [구조 진단 (Structural Linter)](#구조-진단-structural-linter)
  - [테마 보고서 생성기](#테마-보고서-생성기)
  - [하하도(何河圖) / 주역(周易)](#하하도하-/-주역周易)
- [eon-vedic — 베딕 점성술 엔진](#eon-vedic--베딕-점성술-엔진)
  - [차트 계산 및 Varga](#차트-계산-및-varga)
  - [Shadbala & Bhava Bala](#shadbala--bhava-bala)
  - [Ashtakavarga](#ashtakavarga)
  - [Dasha 시스템](#dasha-시스템)
  - [Yoga 탐지](#yoga-탐지)
  - [Panchanga](#panchanga)
  - [Guna Milan 궁합](#guna-milan-궁합)
  - [KP System](#kp-system)
  - [Tajika 연간 차트](#tajika-연간-차트)
  - [Arudha Padas](#arudha-padas)
- [eon-zwds — 자미두수 엔진](#eon-zwds--자미두수-엔진)
- [eon-western — 서양 점성술 엔진](#eon-western--서양-점성술-엔진)
- [eon-human-design — 휴먼디자인 엔진](#eon-human-design--휴먼디자인-엔진)
- [eon-service — 통합 Façade](#eon-service--통합-façade)
- [eon-ui — Dioxus Web SPA](#eon-ui--dioxus-web-spa)
- [빌드 및 배포](#빌드-및-배포)
- [라이선스](#라이선스)

---

## 워크스페이스 구조

```
Eon/
├── crates/
│   ├── eon-core/               # 공통 원시 타입 (BirthInfo, Location, 타임존)
│   ├── eon-data/               # 만세력 바이너리 캐시 (음·양력 변환 DB)
│   ├── eon-astro/              # Swiss Ephemeris C FFI 바인딩
│   ├── eon-saju/               # 사주 분석 엔진
│   ├── eon-vedic/              # 베딕 점성술 엔진 (BPHS 표준)
│   ├── eon-zwds/               # 자미두수 엔진
│   ├── eon-western/            # 서양 점성술 엔진
│   ├── eon-human-design/       # 휴먼디자인 엔진
│   ├── eon-service/            # 통합 Façade + DTO (단일 진입점)
│   └── eon-ui/                 # Dioxus Web SPA (WASM)
├── .github/workflows/          # Vercel 자동 배포 CI/CD
└── DOCS/                       # 도메인 레퍼런스 문서
```

**의존성 레이어링**

```
eon-core ──────────────────────────────────────┐
eon-data (만세력 DB)                            │
eon-astro (Swiss Ephemeris FFI) ◄──────────────┤
        │                                       │
        ├── eon-saju                            │
        ├── eon-vedic                           │
        ├── eon-zwds                            │
        ├── eon-western                         │
        └── eon-human-design                    │
                    └──────────► eon-service ◄──┘
                                      │
                                   eon-ui
                                (Dioxus WASM)
```

---

## 빠른 시작

### Cargo.toml 의존성

```toml
[dependencies]
eon-service = { git = "https://github.com/sjkim1127/Eon" }
```

개별 엔진만 필요한 경우:

```toml
[dependencies]
eon-saju  = { git = "https://github.com/sjkim1127/Eon" }
eon-vedic = { git = "https://github.com/sjkim1127/Eon" }
```

### 사주 분석 예시

```rust
use eon_service::dto::{AnalysisInput, SajuAnalysisInput};
use eon_service::facade::analyze_saju;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base = AnalysisInput {
        year: 1990, month: 5, day: 15,
        hour: 10, minute: 0,
        is_lunar: false,
        is_leap_month: false,
        lat: 37.5665,
        lon: 126.9780,
        timezone: "Asia/Seoul".to_string(),
    };

    let input = SajuAnalysisInput::new(base, /* is_male */ true, /* night_rat */ false, None);
    let result = analyze_saju(input)?;

    println!("일주: {} {}", result.report.pillars.day.stem.hangul(), result.report.pillars.day.branch.hangul());
    println!("용신: {}", result.report.yongshin.primary.hangul());
    println!("격국: {}", result.report.structure.name);
    println!("신강신약: {}", result.report.strength.strength_type.hangul());

    Ok(())
}
```

### 베딕 점성술 분석 예시

```rust
use eon_service::dto::{AnalysisInput, VedicAnalysisInput};
use eon_service::facade::analyze_vedic;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base = AnalysisInput {
        year: 1990, month: 5, day: 15,
        hour: 10, minute: 0,
        is_lunar: false, is_leap_month: false,
        lat: 37.5665, lon: 126.9780,
        timezone: "Asia/Seoul".to_string(),
    };

    let input = VedicAnalysisInput::new(base, None, None);
    let result = analyze_vedic(input)?;

    let chart = &result.chart;
    println!("어센던트: {} ({:.2}°)", chart.ascendant.sign_name(), chart.ascendant.degree);

    for planet in &chart.planets {
        println!("{}: {} {:.2}°", planet.name, planet.sign_name(), planet.degree_in_sign);
    }

    Ok(())
}
```

---

## eon-core — 공통 타입

모든 크레이트에서 공통으로 사용되는 기반 타입을 제공합니다.

### 주요 타입

| 타입 | 설명 |
|---|---|
| `BirthInfo` | 출생 일시·위치·성별·달력 종류를 담는 통합 구조체 |
| `Location` | 위도·경도·표준 경도 기반 지역시 보정 |
| `Gender` | `Male` / `Female` |
| `CalendarType` | `Solar` (양력) / `Lunar` (음력) |
| `EonError` | 워크스페이스 공통 에러 타입 |

### 타임존 지원

`chrono-tz`를 통해 IANA 타임존 DB를 사용합니다. 역사적 DST(썸머타임)를 자동 처리합니다.

```rust
// 예: 1988년 서울 올림픽 기간 썸머타임(UTC+10) 자동 적용
let tz: chrono_tz::Tz = "Asia/Seoul".parse().unwrap();
```

---

## eon-astro — Swiss Ephemeris FFI

C 라이브러리 `libswe`(Swiss Ephemeris)를 Rust에서 안전하게 래핑하는 천문 계산 기반 크레이트입니다.
모든 행성 황경 계산은 이 크레이트를 통해 이루어집니다.

### 기능

- **행성 황경(Longitude)**: 태양·달·화·수·목·금·토·라후·케투·명왕성·키론·트루 노드
- **Julian Day 변환**: 날짜/시각 → JD 변환 및 역변환
- **Ayanamsa 지원**
  - Lahiri (Chitra Paksha) — 베딕 기본값
  - True Chitrapaksha
  - Fagan-Bradley
  - Raman
- **하우스 Cusp**: Placidus / Koch / Whole Sign / Equal House / Porphyry
- **어센던트 계산**: 위도·경도·시각 기반 Lagna 도수 산출

```rust
use eon_astro::AstroEngine;

let engine = AstroEngine::new();
let pos = engine.planet_position(eon_astro::Planet::Sun, jd)?;
println!("태양 황경: {:.4}°", pos.longitude);
```

---

## eon-data — 만세력 DB

사주 계산에서 음력→양력(또는 역방향) 변환에 사용되는 만세력(萬歲曆) 바이너리 캐시입니다.

- 1800~2100년 범위 커버
- `bincode` 직렬화 포맷으로 바이너리 임베딩 (`include_bytes!`)
- 24절기(節氣) 기준 월주(月柱) 경계 산출

---

## eon-saju — 사주 분석 엔진

사주팔자(四柱八字) 전체 분석 파이프라인을 제공합니다.
출생 일시 입력부터 격국·용신·신살·대운 분석까지 모두 `SajuReport` 한 구조체에 집약됩니다.

```rust
pub struct SajuReport {
    pub pillars: FourPillars,               // 4주 8자
    pub strength: StrengthAnalysis,         // 신강신약
    pub yongshin: YongshinAnalysis,         // 용신·희신
    pub spirit_markers: SpiritMarkerAnalysis, // 신살
    pub structure: StructureAnalysis,       // 격국
    pub ten_gods: TenGodAnalysis,           // 십성
    pub voids: VoidAnalysis,                // 공망
    pub relationships: RelationshipAnalysis, // 합충형해
    pub supplementary_pillars: SupplementaryPillars, // 보조 기둥
    pub major_luck: Option<MajorLuckAnalysis>,  // 대운
    pub golden_time: Option<GoldenTime>,        // 황금기
    pub timeline: Vec<YearlyScore>,             // 연간 점수 곡선
    pub simulation_frames: Vec<LifeFrame>,      // 생애 프레임
    pub power: IntegratedAnalysis,              // 통합 강도 분석
}
```

---

### FourPillars & LifecycleMachine

**FourPillars** — 연·월·일·시 4주(柱) 8자(字) 계산

```rust
use eon_saju::core::pillars::FourPillars;

let pillars = FourPillars::from_birth_info(&birth_info)?;
println!("연주: {} {}", pillars.year.stem.hangul(), pillars.year.branch.hangul());
println!("월주: {} {}", pillars.month.stem.hangul(), pillars.month.branch.hangul());
println!("일주: {} {}", pillars.day.stem.hangul(), pillars.day.branch.hangul());
println!("시주: {} {}", pillars.hour.stem.hangul(), pillars.hour.branch.hangul());
```

- **음력 지원**: `eon-data` 만세력 DB 기반 음·양력 자동 변환
- **야자시(夜子時)**: `use_night_rat_hour` 플래그로 자시(子時) 분기 처리
- **보완 기둥**: 태주(胎柱)·태원(胎元)·명궁(命宮) 별도 산출

**LifecycleMachine (내부 에뮬레이터)**

원국(原局)을 기저 상태로 고정하고, 대운·세운을 입력으로 받아 오행 레지스터(`QiRegisters { r0_wood, r1_fire, r2_earth, r3_metal, r4_water }`) 상태를 연산합니다. 1년 단위 `LifeFrame` 스냅샷을 생성하며, 100년 전체 시뮬레이션이 ~0.01초 내에 완료됩니다.

---

### 용신 분석

용신(用神)은 사주의 균형을 맞추는 핵심 오행입니다. 4가지 분석 관점을 모두 검토합니다.

| 유형 | 한자 | 설명 |
|---|---|---|
| `Eokbu` | 억부(抑扶) | 일간의 신강·신약에 따른 조절 오행 |
| `Johu` | 조후(調候) | 월지 기후(한난조습)를 교정하는 오행 |
| `Tonggwan` | 통관(通關) | 충·극 대립을 중재하는 소통 오행 |
| `Byeongyak` | 병약(病藥) | 명식의 병(病)이 되는 기운을 치유하는 오행 |

```rust
pub struct YongshinAnalysis {
    pub recommendations: Vec<RecommendedYongshin>, // 복수 용신 추천 목록
    pub primary: Element,   // 제1용신
    pub assistant: Element, // 희신 (제1용신 보조)
}

pub struct RecommendedYongshin {
    pub yongshin_type: YongshinType,
    pub element: Element,
    pub summary: String,      // 한 줄 요약
    pub description: String,  // 상세 설명
    pub reasons: Vec<String>, // 판정 근거 목록
}
```

---

### 격국 분석

월지(月支)의 지장간이 천간에 투출한 상태를 분석하여 사주의 틀(格局)을 결정합니다.

**내격 (普通格, 10格)**

| 격국 | 한자 | 십성 |
|---|---|---|
| `ShiShen` | 식신격(食神格) | 식신 |
| `ShangGuan` | 상관격(傷官格) | 상관 |
| `PianCai` | 편재격(偏財格) | 편재 |
| `ZhengCai` | 정재격(正財格) | 정재 |
| `PianGuan` | 편관격(偏官格) | 편관 |
| `ZhengGuan` | 정관격(正官格) | 정관 |
| `PianYin` | 편인격(偏印格) | 편인 |
| `ZhengYin` | 정인격(正印格) | 정인 |
| `JianLu` | 건록격(建祿格) | 일간 월지 건록 |
| `YangIn` | 양인격(陽刃格) | 일간 월지 제왕 |

**외격 (從格)**

| 격국 | 한자 | 조건 |
|---|---|---|
| `JongAh` | 종아격(從兒格) | 식상(食傷)으로 종(從) |
| `JongJae` | 종재격(從財格) | 재성(財星)으로 종 |
| `JongSal` | 종살격(從殺格) | 관성(官星)으로 종 |
| `JongGang` | 종강격(從强格) | 인성(印星)으로 종 |
| `JongWang` | 종왕격(從旺格) | 비겁(比劫)으로 종 |
| `SpecialTransformation` | 전왕격(專旺格) | 단일 오행 극도 집중 |

```rust
pub struct StructureAnalysis {
    pub structure: StructureType, // 격국 분류
    pub name: String,             // 한글 격국명
    pub is_formed: bool,          // 성격(成格) 여부
    pub reasons: Vec<String>,     // 성격·파격 근거
    pub description: String,      // 격국 해설
}
```

---

### 신강신약 판정

일간(日干)의 강약을 4가지 기준으로 종합 판정합니다.

| 기준 | 설명 |
|---|---|
| 득령(得令) | 월지가 일간을 생(生)하거나 비겁 관계인가 |
| 득지(得地) | 지지 4개에 일간의 통근(通根)이 있는가 |
| 득시(得時) | 시지(時支)가 일간을 생하거나 비겁인가 |
| 득세(得勢) | 비겁(比劫)·인성(印星)의 글자 수가 우세한가 |

```rust
pub enum StrengthType {
    Strong,   // 신강(身强) — 식상·재성·관성 필요
    Weak,     // 신약(身弱) — 인성·비겁 필요
    Balanced, // 중화(中和) — 균형 상태 (드묾)
}
```

---

### 십성 분석

일간(日干) 기준으로 나머지 7글자(천간 3 + 지지 4)의 관계 원리를 십성(十星)으로 분류합니다.

| 오행 관계 | 음양 동 | 음양 이 |
|---|---|---|
| 비화(比和) | 비견(比肩) | 겁재(劫財) |
| 일간이 생함 | 식신(食神) | 상관(傷官) |
| 일간이 극함 | 편재(偏財) | 정재(正財) |
| 일간을 극함 | 편관(偏官) | 정관(正官) |
| 일간을 생함 | 편인(偏印) | 정인(正印) |

```rust
pub struct TenGodAnalysis {
    pub year_stem: TenGod,
    pub month_stem: TenGod,
    pub hour_stem: TenGod,
    pub year_branch_hidden: Vec<(HeavenlyStem, TenGod)>, // 지장간
    pub month_branch_hidden: Vec<(HeavenlyStem, TenGod)>,
    pub day_branch_hidden: Vec<(HeavenlyStem, TenGod)>,
    pub hour_branch_hidden: Vec<(HeavenlyStem, TenGod)>,
}
```

---

### 신살 탐지

사주에서 특정 간지 조합으로 나타나는 길신(吉神)과 흉살(凶煞)을 자동 탐지합니다.

**길신 (吉神)**

| 신살명 | 한자 | 의미 |
|---|---|---|
| `Tianyi` | 천을귀인(天乙貴人) | 귀인의 도움 |
| `Wenchang` | 문창귀인(文昌貴人) | 학문·시험운 |
| `Taiji` | 태극귀인(太極貴人) | 영적 보호 |
| `Yuede` | 월덕귀인(月德貴人) | 덕의 보호 |
| `Tiande` | 천덕귀인(天德貴人) | 천간의 덕 |
| `Zhenglu` | 정록(正祿) | 재물·녹성 |
| `Jinyu` | 금여록(金輿祿) | 귀하고 화려한 운 |
| `Xuetang` | 학당귀인(學堂貴人) | 학업 성취 |

**흉살 (凶煞)**

| 신살명 | 한자 | 의미 |
|---|---|---|
| `Yima` | 역마살(驛馬煞) | 이동·변화 |
| `Taohua` | 도화살(桃花煞) | 이성 인기·색정 |
| `Kuigang` | 괴강살(魁罡煞) | 강렬한 카리스마, 극단적 흥망 |
| `Baekhosal` | 백호살(白虎殺) | 혈광지사, 급작스러운 사고 |
| `Huagai` | 화개살(華蓋煞) | 예술·종교·고독 |
| `Guchen` | 고신살(孤辰煞) | 고독 |
| `Jiesha` | 겁살(劫煞) | 강탈·재해 |
| `Gongmang` | 공망(空亡) | 비어 있음·실속 없음 |

**12신살 (十二神殺)**

지살·년살·월살·망신살·장성살·반안살·역마살·육해살·화개살·겁살·재살·천살

---

### 합충형해 관계 분석

천간과 지지 간의 특수 관계를 자동으로 목록화합니다.

**천간 관계**

| 관계 | 쌍 |
|---|---|
| 천간합(天干合) | 甲己·乙庚·丙辛·丁壬·戊癸 |
| 천간충(天干沖) | 甲庚·乙辛·丙壬·丁癸 |

**지지 관계**

| 관계 | 설명 |
|---|---|
| 삼합(三合) | 寅午戌·申子辰·巳酉丑·亥卯未 |
| 반합(半合) | 삼합 중 두 글자 |
| 방합(方合) | 寅卯辰·巳午未·申酉戌·亥子丑 |
| 육합(六合) | 子丑·寅亥·卯戌·辰酉·巳申·午未 |
| 충(沖) | 子午·丑未·寅申·卯酉·辰戌·巳亥 |
| 형(刑) | 삼형(寅巳申)·상형(丑戌未)·자형(辰午酉亥) |
| 해(害) | 子未·丑午·寅巳·卯辰·申亥·酉戌 |
| 파(破) | 子酉·丑辰·寅亥·卯午·巳申·午未 |

---

### 오행 순환망 (Qi Topology)

사주 8글자의 오행 에너지를 방향성 그래프로 모델링합니다.

- **생(生) 경로**: 木→火→土→金→水→木 방향의 에너지 공급 연결
- **극(克) 경로**: 木→土, 火→金, 土→水, 金→木, 水→火 억제 연결
- **지배 흐름 탐지**: 가장 강하게 흐르는 에너지 경로
- **병목 탐지**: 단절 또는 과부하 구간
- **순환 복잡도(Cyclomatic Complexity)**: 명식의 에너지 복잡성 정량화
- **안정성 등급**: S / A / B / C / D

---

### 대운·세운 분석

**대운(大運)** — 10년 단위 운세 주기

```rust
pub struct MajorLuckAnalysis {
    pub start_age: u32,                   // 첫 대운 시작 나이
    pub cycles: Vec<MajorLuckCycle>,      // 10년 단위 대운 목록
}

pub struct MajorLuckCycle {
    pub age_start: u32,
    pub age_end: u32,
    pub ganzi: GanZi,                     // 대운 간지
    pub stem_ten_god: TenGod,             // 천간 십성
    pub branch_ten_god: TenGod,           // 지지 십성
    pub score: f32,                       // 운세 점수
}
```

**세운(歲運)** — 연간 운세 분석. 원국과 대운·세운 천간지지의 합충형해를 종합 연산합니다.

---

### 생애 점수 & 황금기

연간 운세 점수(`YearlyScore`)를 0~100세 전 구간 계산하여 다음을 산출합니다.

- **황금기(GoldenTime)**: 점수 상승세가 가장 가파른 연속 구간
- **위험 구간**: 하락·충극이 집중되는 취약 연령대
- SVG 라인 차트로 시각화 (`eon-ui` 내 Simulation 탭)

---

### 구조 진단 (Structural Linter)

격국·신강신약·신살을 기반으로 명식의 구조적 문제를 정적 분석 방식으로 진단합니다.

```rust
pub struct SajuLint {
    pub code: String,      // 진단 코드 (예: "E001")
    pub level: LintLevel,  // ERROR / WARN / INFO
    pub message: String,   // 진단 메시지
    pub advice: String,    // 교정 제안
}
```

---

### 테마 보고서 생성기

AI API 호출 없이 규칙 기반으로 테마별 마크다운 보고서를 자동 조립합니다.

```rust
pub enum ReportTheme {
    WealthAndCareer,    // 재물·커리어
    LoveAndMarriage,    // 연애·결혼
    HealthAndVitality,  // 건강·활력
}

pub struct ThemedReportOutput {
    pub meta: AnalysisMeta,
    pub theme: ReportTheme,
    pub user_name: String,
    pub title: String,
    pub content: String, // 마크다운 형식 보고서 본문
}
```

**생성 로직 개요**

1. `analyze_saju`로 원국 분석 수행
2. 일간·용신·격국·오행 병목·대운 데이터를 조합
3. 사전 정의된 한국어 해설 템플릿 블록을 조건에 따라 선택·조립
4. 절(Section) 단위 마크다운 문서 반환

---

### 하하도(河圖) / 주역(周易)

하락수(河洛數) 및 주역(周易) 64괘 관련 해석 모듈입니다.

```rust
use eon_service::facade::analyze_iching;
let output = analyze_iching(saju_input)?;
```

---

## eon-vedic — 베딕 점성술 엔진

BPHS(Brihat Parashara Hora Shastra) 기준으로 구현된 고정밀 베딕 점성술 엔진입니다.
Swiss Ephemeris를 통해 Lahiri Ayanamsa 기반 황경을 산출합니다.

모듈 구조:

```
eon-vedic/src/
├── core/         # 차트·행성·상수·설정 기본 구조체
├── calc/         # 수학적 계산 (Ayanamsa, Panchanga, Varga)
├── analysis/     # 해석 로직 (Yoga, Gochara, KP, 궁합)
└── prediction/   # 예측 시스템 (Vimshottari Dasha, Kalachakra)
```

---

### 차트 계산 및 Varga

**D1 (Rasi) 차트** — Swiss Ephemeris 기반 행성 황경에 Lahiri Ayanamsa를 차감하여 황도 12궁 배치를 결정합니다.

**지원 Varga (분할 차트)**

| Varga | 분할 | 의미 |
|---|---|---|
| D1 | 1 | Rasi — 기본 차트 |
| D2 | 2 | Hora — 재물 |
| D3 | 3 | Drekkana — 형제 |
| D4 | 4 | Chaturthamsa — 부동산 |
| D7 | 7 | Saptamsa — 자녀 |
| D9 | 9 | Navamsa — 배우자·결혼 |
| D10 | 10 | Dasamsa — 직업·커리어 |
| D12 | 12 | Dwadasamsa — 부모 |
| D16 | 16 | Shodasamsa — 이동수단 |
| D20 | 20 | Vimsamsa — 영성 |
| D24 | 24 | Chaturvimsamsa — 교육 |
| D27 | 27 | Bhamsa — 체력·성격 |
| D30 | 30 | Trimsamsa — 불운·악행 |
| D40 | 40 | Khavedamsa — 길흉 조짐 |
| D45 | 45 | Akshavedamsa — 조상 |
| D60 | 60 | Shastiamsa — 업(業) |
| D108 | 108 | — |
| D144 | 144 | — |

---

### Shadbala & Bhava Bala

행성별 강도를 6대 요인으로 정밀 산출합니다.

| 요인 | 설명 |
|---|---|
| Sthana Bala | 점유 위치(하우스·사인)의 기본 강도 |
| Dig Bala | 방향 강도 — 각 행성의 최강 방향 하우스 기준 |
| Kala Bala | 시간 강도 — 출생 시각(낮/밤), 태음월, 계절 등 |
| Cheshta Bala | 운동 강도 — 순행/역행 속도 기준 |
| Naisargika Bala | 자연 강도 — 행성 고유 위계 (태양 최강 → 토성 최약) |
| Drik Bala | 시선(Drishti) 강도 — 받는 Aspect의 합산 |

Shadbala 총점과 요구 기준치(Rupa) 대비 충족 여부를 퍼센티지로 산출합니다.

**Bhava Bala (하우스 강도)** — 각 하우스를 점유하는 행성의 Shadbala, Lord 강도, Drishti 가중치를 합산합니다.

---

### Ashtakavarga

행성별 통과(Transit) 적합성을 하우스 단위 점수(0~8 Bindus)로 산출합니다.

- **BAV (Bhinna Ashtakavarga)**: 개별 행성의 하우스별 Bindu 분포
- **SAV (Sarvashtakavarga)**: 7개 행성(태양~토성) BAV 합산
- **Trikona Reduction**: 삼각 하우스(1·5·9번)의 동일 Bindu 균등 분배
- **Ekadhipatya Reduction**: 동일 Lord 두 하우스 간의 Bindu 재배분

---

### Dasha 시스템

**Vimshottari Dasha** — 120년 주기, 9개 행성 순환

| 행성 | 기간(년) |
|---|---|
| 케투(Ketu) | 7 |
| 금성(Venus) | 20 |
| 태양(Sun) | 6 |
| 달(Moon) | 10 |
| 화성(Mars) | 7 |
| 라후(Rahu) | 18 |
| 목성(Jupiter) | 16 |
| 토성(Saturn) | 19 |
| 수성(Mercury) | 17 |

3단계 계층 구조: Mahadasha(대운) → Antardasha(소운) → Pratyantardasha(세운)

**Yogini Dasha** — 36년 주기, 8신(神) 순환. 단기 예측에 특화됩니다.

---

### Yoga 탐지

차트에서 길흉 요가(행성 배치 패턴)를 자동 인식합니다.

| 카테고리 | 주요 Yoga |
|---|---|
| 왕권 (Raj Yoga) | 케ndra·Trikona Lord 합일 |
| 재물 (Dhana Yoga) | 2·11번 하우스 Lord 연결 |
| 위대한 운 (Mahabhagya) | 성별·출생 시각·Lagna 조건 부합 |
| 전도 (Viparita Raj) | 두쉬타나(6·8·12) Lord 교환 |
| 지식 (Saraswati Yoga) | 목성·금성·수성 특정 배치 |
| 파괴 (Daridra Yoga) | 11번 하우스 Lord 두쉬타나 진입 |

---

### Panchanga

출생일의 전통 5지(支)를 산출합니다.

| 지(支) | 설명 |
|---|---|
| Vara | 요일 — 7행성 기반 |
| Tithi | 음력 날짜 — 달의 위상(0°~360°를 30° 단위) |
| Nakshatra | 월성(月星) — 달의 황경 기준 27 별자리 |
| Yoga | 태양+달 황경 합산 기반 27 Yoga |
| Karana | Tithi의 절반 단위, 11종 |

**Sade Sati** — 토성이 달자리(Natal Moon) 앞뒤 1궁씩 총 3궁을 통과하는 7.5년 주기 분석.

---

### Guna Milan 궁합

아슈타코타(Ashtakoota) 8대 요소별 점수를 산출하여 두 사람의 궁합을 정량화합니다.

| 요소 | 만점 | 평가 기준 |
|---|---|---|
| Varna | 1 | 사회적 계층 궁합 |
| Vashya | 2 | 지배·피지배 관계 |
| Tara | 3 | 출생 Nakshatra 간 간격 |
| Yoni | 4 | 성적 궁합 (동물 상징) |
| Maitri | 5 | 달자리 Lord 친화도 |
| Gana | 6 | 기질 그룹 (Deva·Manava·Rakshasa) |
| Bhakoot | 7 | 달자리 하우스 관계 |
| Nadi | 8 | 체질 궁합 (Vata·Pitta·Kapha) |
| **합계** | **36** | 18점 이상 권장 |

---

### KP System

크리쉬나무르티 패다티(Krishnamurti Paddhati) 시스템으로 하우스 Cusp를 정밀 분석합니다.

- 12하우스 Cusp 황경에서 Sign Lord → Star Lord → Sub Lord 3단계 연쇄 결정
- Significator 목록: 각 행성이 영향을 미치는 하우스 번호 목록화
- 정밀 예측에 활용되는 Sub-Sub Lord까지 확장 가능

---

### Tajika 연간 차트

태양이 출생 황경으로 돌아오는 시점(Solar Return)을 기준으로 연간 예측 차트를 산출합니다.

- 연간 태양 복귀 JD(Julian Day) 정밀 계산
- 복귀 시점의 전 행성 황경 재산출
- Tajika Aspect(Itthasala, Ishrafa 등) 적용

---

### Arudha Padas

하우스 Lord의 위치를 반사하여 외적 현시(外的顯示)를 분석합니다.

- A1(Arudha Lagna)~A12 전 하우스 파다 산출
- 특히 AL(자아 이미지), A7(배우자 이미지), A10(경력 이미지)이 핵심

---

## eon-zwds — 자미두수 엔진

중국 전통 성반(星盤) 명리학 자미두수(紫微斗數) Rust 구현체입니다.

> 알고리즘 레퍼런스: [SylarLong/iztro](https://github.com/SylarLong/iztro) (TypeScript, MIT)

### 성반 구성

- **12궁(宮)**: 명궁(命宮)·형제궁·부처궁·자녀궁·재백궁·질액궁·천이궁·노복궁·관록궁·전택궁·복덕궁·부모궁
- **14주성(主星)**: 자미·천기·태양·무곡·천동·염정·천부·태음·탐랑·거문·천상·천량·칠살·파군
- **6보조성**: 문창·문곡·좌보·우필·천괴·천월
- **중잡성(中雜星)**: ~50종 보조 별자리

### 모듈 목록

| 모듈 | 설명 |
|---|---|
| `palace` | 12궁 배치 계산 |
| `stars` | 주성·보조성·중잡성 배치 |
| `transformations` | 사화(四化) — 化祿·化權·化科·化忌 비행 |
| `brightness` | 묘왕(廟旺)·함지(陷地) 등 별 밝기 등급 |
| `decadal` | 대한(大限, 10년 운) |
| `annual` | 유년(流年, 1년 운) |
| `destiny_patterns` | 명주(命主)·신주(身主) 결정 |

```rust
use eon_service::facade::analyze_zwds;

let output = analyze_zwds(input)?;
println!("명궁 주성: {:?}", output.chart.ming_palace.main_stars);
println!("사화: {:?}", output.chart.transformations);
```

---

## eon-western — 서양 점성술 엔진

Swiss Ephemeris(`eon-astro`)를 활용한 서양 점성술 차트 계산 엔진입니다.

### 지원 기능

**행성 및 감수점**

| 대상 | 수 |
|---|---|
| 행성 | 태양·달·수성·금성·화성·목성·토성·천왕성·해왕성·명왕성 |
| 소행성 | 키론(Chiron) |
| 교점 | True Node (라후) |

각 천체별: 황경·속도·역행 여부·하우스 번호·사인 내 도수

**하우스 시스템**

| 시스템 | 특징 |
|---|---|
| Placidus | 가장 보편적. 출생지 위도 의존 |
| Koch | Placidus 변형. 고위도 약점 동일 |
| Whole Sign | 상승궁 사인 전체를 1하우스로 취급 |
| Equal House | Ascendant부터 30° 등분 |

**Aspect 계산**

| Aspect | 각도 | 분류 |
|---|---|---|
| Conjunction | 0° | Major |
| Opposition | 180° | Major |
| Trine | 120° | Major |
| Square | 90° | Major |
| Sextile | 60° | Major |
| Quincunx | 150° | Minor |
| Semi-Sextile | 30° | Minor |
| Semi-Square | 45° | Minor |
| Sesquiquadrate | 135° | Minor |

Orb 허용치는 행성별·Aspect별 별도 설정 가능합니다.

**성향 분석**

- 원소 분포 (火·土·風·水 비율)
- 성질 분포 (활동궁·고정궁·변통궁)
- 극성 분포 (양·음)

---

## eon-human-design — 휴먼디자인 엔진

Ra Uru Hu의 휴먼디자인 시스템을 Swiss Ephemeris 기반으로 정밀 구현합니다.
Jovian Archive 레퍼런스와 교차 검증 완료.

### 계산 과정

1. **출생 시각** → Personality(의식) 행성 10개 황경 산출
2. **출생 88일 전 태양 황경** → Design(무의식) 행성 10개 황경 산출
3. 각 황경 → **I Ching Gate (1~64)** 및 **Line (1~6)** 결정
4. Gate 활성화 → **채널(Channel)** 연결 확인
5. 채널 → **센터(Center) 정의(Defined/Undefined)** 결정

### 출력 구조

```rust
pub struct HumanDesignResult {
    pub personality_planets: Vec<HdPlanetData>, // 의식 (빨간색)
    pub design_planets: Vec<HdPlanetData>,      // 무의식 (검은색)
    pub defined_centers: Vec<HdCenter>,         // 정의된 9센터
    pub undefined_centers: Vec<HdCenter>,       // 미정의 센터
    pub active_channels: Vec<HdChannel>,        // 활성화된 채널
    pub hd_type: HdType,                        // Type (Manifestor 등)
    pub authority: HdAuthority,                 // Authority (Emotional 등)
    pub profile: (u8, u8),                      // 예: (3, 5)
    pub definition: HdDefinition,               // Single/Split/Triple/Quad
}
```

**9센터**

| 센터 | 기능 영역 |
|---|---|
| Head | 영감·압박 |
| Ajna | 개념화·확실성 |
| Throat | 소통·행동화 |
| Self (G) | 정체성·방향 |
| Heart | 의지·자존심 |
| Sacral | 생명력·성(性) |
| Root | 스트레스·추진력 |
| Spleen | 직관·건강·면역 |
| Solar Plexus | 감정·감수성 |

---

## eon-service — 통합 Façade

모든 엔진을 단일 진입점으로 통합합니다. 외부에서 이 크레이트만 의존해도 전체 기능에 접근할 수 있습니다.

### 공개 함수 목록

```rust
// eon_service::facade

// 사주
pub fn analyze_saju(input: SajuAnalysisInput) -> Result<SajuAnalysisOutput, ServiceError>

// 베딕 점성술
pub fn analyze_vedic(input: VedicAnalysisInput) -> Result<VedicAnalysisOutput, ServiceError>
pub fn analyze_vedic_compatibility(input: VedicCompatibilityInput) -> Result<VedicCompatibilityOutput, ServiceError>

// 자미두수
pub fn analyze_zwds(input: ZwdsAnalysisInput) -> Result<ZwdsAnalysisOutput, ServiceError>

// 서양 점성술
pub fn analyze_western(input: WesternAnalysisInput) -> Result<WesternAnalysisOutput, ServiceError>

// 휴먼디자인
pub fn analyze_human_design(input: HumanDesignAnalysisInput) -> Result<HumanDesignAnalysisOutput, ServiceError>

// 대운·세운 통합 Transit
pub fn analyze_transit(input: TransitAnalysisInput) -> Result<TransitAnalysisOutput, ServiceError>

// 사주+베딕 통합 생애 등급
pub fn analyze_destiny_tier(
    saju: SajuAnalysisOutput,
    vedic: VedicAnalysisOutput,
    transit: Option<TransitAnalysisOutput>,
) -> Result<TierResult, ServiceError>

// 주역(河洛)
pub fn analyze_iching(input: SajuAnalysisInput) -> Result<IChingAnalysisOutput, ServiceError>

// 테마별 보고서
pub fn generate_themed_report(input: ThemedReportInput) -> Result<ThemedReportOutput, ServiceError>
```

### 공통 입력 구조체

```rust
pub struct AnalysisInput {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub is_lunar: bool,       // true → 음력 입력
    pub is_leap_month: bool,  // 윤달 여부
    pub lat: f64,             // 위도
    pub lon: f64,             // 경도
    pub timezone: String,     // IANA 타임존 (예: "Asia/Seoul")
}

pub enum BirthTimePrecision {
    Exact,                  // 정확한 출생 시각
    UnknownTimeNoonProxy,   // 시각 불명 → 정오(12:00) 대리
}
```

### 에러 처리

```rust
#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Saju engine error: {0}")]
    Saju(String),
    #[error("Vedic engine error: {0}")]
    Vedic(#[from] VedicError),
    #[error("Astronomical calculation error: {0}")]
    Astro(#[from] AstroError),
    #[error("Calendar conversion error: {0}")]
    Calendar(String),
    // ...
}
```

---

## eon-ui — Dioxus Web SPA

Dioxus Web 프레임워크 기반 SPA입니다. WebAssembly로 컴파일되며 별도 백엔드 없이 전 연산이 브라우저에서 실행됩니다.

### 상태 관리

Dioxus `Signal` 기반 전역 컨텍스트 `AnalysisState`를 단일 진실 공급원(SSOT)으로 사용합니다.

```rust
// 올바른 패턴
let mut state = use_context::<AnalysisState>();
state.saju.write().status = TaskStatus::Loading;

// 비동기 분석 호출 패턴
spawn(async move {
    let result = analyze_saju(input).await;
    state.saju.write().data = Some(result);
});
```

UI 스레드 블로킹을 막기 위해 모든 분석 호출은 `spawn(async move { ... })` 블록 내에서 실행합니다.

### 탭 라우팅

| 경로 | 컴포넌트 | 내용 |
|---|---|---|
| `/` | `SajuTab` | 사주 8자, 십성, 용신, 격국, 신살, 오행 순환망 |
| `/vedic_charts` | `VedicTab` | SVG 출생 차트, 판창가, 아루다 파다, 행성 표, 하우스 강도, 다샤, 타지카, 바르가 차트, 아슈타카바르가 |
| `/zwds` | `ZwdsTab` | 자미두수 12궁 성반, 사화, 대한·유년 |
| `/western` | `WesternTab` | 서양 점성술 차트, Aspect 표 |
| `/human_design` | `HumanDesignTab` | Type·Authority·Profile·센터 |
| `/iching` | `IChingTab` | 주역·하락수 분석 |
| `/strength` | `StrengthTab` | 사주·베딕 원소 강도 비교 |
| `/transit` | `TransitTab` | 대운·세운·Mahadasha 통합 타임라인 |
| `/simulation` | `SimulationTab` | 0~100세 운세 곡선, 황금기 시각화 |
| `/destiny_tier` | `TierTab` | 통합 생애 등급 (S+ ~ D) |

### 성능 최적화

- **Web Worker**: `gloo-worker` 기반 백그라운드 스레드에서 분석 연산 실행 — UI 스레드 무중단
- **IndexedDB**: 출생 프로필을 브라우저 로컬 DB에 영속 저장 — 새로고침 후에도 즉시 복원
- **SVG 차트**: 베딕 출생 차트(남인도·북인도 스타일)를 Rust 코드로 직접 SVG 생성 — 외부 차트 라이브러리 의존 없음

---

## 빌드 및 배포

### 전제 조건

- Rust (stable)
- Dioxus CLI (`dx`)

```bash
cargo install dioxus-cli --locked
# 또는 바이너리 직접 설치
cargo binstall dioxus-cli -y
```

### 로컬 개발 서버

```bash
cd crates/eon-ui
dx serve
# 브라우저에서 http://localhost:8080 자동 오픈, 핫 리로드 지원
```

### 엔진 유닛 테스트

```bash
# 전체 워크스페이스 컴파일 검증
cargo check --workspace

# 사주 엔진 테스트
cargo test --package eon-saju

# 베딕 엔진 테스트
cargo test --package eon-vedic

# 자미두수 엔진 테스트
cargo test --package eon-zwds

# 서비스 레이어 통합 테스트
cargo test --package eon-service
```

### 프로덕션 WASM 빌드

```bash
cd crates/eon-ui
dx build --release
# 결과물: target/dx/eon-ui/release/web/public/
# (index.html + wasm + js chunks)
```

### CI/CD 자동 배포

`.github/workflows/deploy.yml` — `main` 브랜치에 푸시 시 자동 실행:

1. `dx build --release` 수행
2. `target/dx/eon-ui/release/web/public` 디렉토리를 Vercel CLI로 배포

**릴리스 빌드 최적화 (Cargo.toml)**

```toml
[profile.release]
opt-level = 'z'      # 바이너리 크기 최소화
lto = true           # Link Time Optimization
codegen-units = 1    # 단일 코드 생성 단위
panic = 'abort'      # 언와인드 코드 제거
```

---

## 라이선스

MIT — [LICENSE](LICENSE) 참조.
