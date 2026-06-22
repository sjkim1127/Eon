# Milestone M2 — Ashtakoota Guna Milan 상세 고도화 분석 리포트 (Analysis Report)

## 1. 개요 및 요약 (Overview & Summary)
본 리포트는 Eon 프로젝트의 마일스톤 M2(R1: 아쉬타쿠타 구나 밀란 상세 고도화)를 위한 요구사항 분석 및 UI/i18n 설계 제안서입니다. 
기존의 하드코딩된 한국어 테이블 형태의 궁합 출력을 각 8대 요인(Ashtakoota)의 의미와 매칭 점수에 따른 프로그레스 바를 적용한 카드형 그리드 디자인으로 고도화하고, 4개 국어(한국어, 영어, 중국어, 러시아어) 대응을 위한 i18n 번역 구조와 UI 컴포넌트의 다국어 적용 방식을 정의합니다.

---

## 2. 베딕 매칭 엔진 및 DTO 분석 (Vedic Matching Engine & DTOs)

### 2-1. 8대 쿠타(Ashtakoota) 요인 분석 (`crates/eon-vedic/src/analysis/matching.rs`)
베딕 궁합(Guna Milan)은 달(Moon)의 성좌(Rasi) 및 나크샤트라(Nakshatra)를 기준으로 총 36점 만점의 8대 요인을 평가합니다.

1. **Varna (정신적 성향 / 직업군) [최대 1.0점]**:
   - **산출 방법**: 달 성좌(Rasi)에 따른 카스트 등급(Brahmin=4, Kshatriya=3, Vaishya=2, Shudra=1)을 매겨, 남성의 등급이 여성의 등급보다 크거나 같으면 1.0점, 그렇지 않으면 0.0점 부여.
2. **Vashya (상호 영향력 / 매력) [최대 2.0점]**:
   - **산출 방법**: Rasi의 분류 유형(Chatushpada, Manushya, Jalachara, Vanachara, Keeta) 간의 호환성 평가. 동일 유형은 2.0점, 특정 우호 조합은 1.0점 또는 0.5점, 그 외에는 0.0점.
3. **Tara (운명과 건강 / 별자리 거리) [최대 3.0점]**:
   - **산출 방법**: 남녀 나크샤트라 간의 상대적 거리 (mod 9) + 1을 계산하여 흉수(3, 5, 7 - Vipat, Pratyak, Naidhana)에 해당하는지 평가. 둘 다 흉수가 아니면 3.0점, 한쪽만 흉수이면 1.5점, 둘 다 흉수이면 0.0점(Tara Dosha).
4. **Yoni (생물학적/성적 조화) [최대 4.0점]**:
   - **산출 방법**: 27 나크샤트라를 14가지 동물 상징에 매핑. 상극 동물(예: 말과 버팔로, 코끼리와 사자 등)인 경우 0.0점, 동일 동물인 경우 4.0점, 우호 관계는 3.0점, 중립 관계는 2.0점.
5. **Graha Maitri (정신적 교감 / 지배성 우정) [최대 5.0점]**:
   - **산출 방법**: 남녀 달 성좌 지배성(Lord) 간의 자연적 관계(우호, 중립, 적대)를 분석. 쌍방 우호는 5.0점, 적대는 0.0점, 그 외 중간 등급에 따라 4.0, 3.0, 2.0, 1.0점 부여.
6. **Gana (기질과 성품) [최대 6.0점]**:
   - **산출 방법**: 나크샤트라 기질인 Deva(신), Manushya(인간), Rakshasa(악마/격렬함) 분류 비교. 동일 기질은 6.0점, Deva-Manushya는 5.0점, Manushya-Rakshasa는 3.0점, Deva-Rakshasa는 1.0점. (Rakshasa-Deva/Manushya 격차 시 Gana Dosha 영향).
7. **Bhakoot (정서적 유대 / 월성좌 거리) [최대 7.0점]**:
   - **산출 방법**: 달 성좌 간의 상대적 거리가 길한 조합(1, 7, 3, 11, 4, 10)인 경우 7.0점, 흉한 조합(2/12, 5/9, 6/8)인 경우 0.0점(Bhakoot Dosha).
8. **Nadi (유전적 호환성 / 신경계 체질) [최대 8.0점]**:
   - **산출 방법**: 나크샤트라 체질인 Adi(바타), Madhya(피타), Antya(카파) 비교. 남녀 체질이 다르면 8.0점, 같으면 0.0점(Nadi Dosha - 과도한 동일 기운으로 체질적 불일치 및 자녀 출산 장애 위험).

### 2-2. 화성살(Mangal Dosha) 및 조화성 판정
- 남녀 각각의 차트에서 화성이 Lagna 또는 Moon 기준 1, 2, 4, 7, 8, 12 하우스에 있는지를 기준으로 화성살(Manglik) 감지.
- **Dosha Samya**: 남녀 모두 화성살이 있는 경우 상호 상쇄(`mangal_dosha_cancelled = true`) 처리되어 부정적 영향 소멸.
- **최종 판정**: 총점 18.0점 이상이며, 주요 살(Nadi Dosha = 0점, Bhakoot Dosha = 0점)이 없거나 화성살이 상쇄된 경우에만 `is_compatible = true`가 됩니다.

### 2-3. DTO 및 라우팅 분석
- 서비스 진입점: `crates/eon-service/src/services/vedic.rs` -> `analyze_compatibility` 함수.
- 입력 DTO: `VedicCompatibilityInput` (남녀 각각의 `AnalysisInput` 포함).
- 출력 DTO: `VedicCompatibilityOutput`
  ```rust
  pub struct VedicCompatibilityOutput {
      pub meta: AnalysisMeta,
      pub report: eon_vedic::analysis::matching::CompatibilityReport,
  }
  ```
  `CompatibilityReport` 구조체에는 `total_score` (f64), `is_compatible` (bool), `kootas` (Vec<KootaScore>), `male_mangal_dosha` (bool), `female_mangal_dosha` (bool), `mangal_dosha_cancelled` (bool), `explanation` (String)이 담겨 있습니다.

---

## 3. UI 변경 설계 (UI Design Proposal)

### 3-1. 디자인 컴포넌트 구조
기존 테이블 방식을 탈피하여 궁합 매칭 상태를 시각화합니다.
1. **종합 결과 카드 (Overall Status Card)**:
   - 좌측에 획득한 총점과 최대 점수(`{total_score} / 36 Gunas`)를 표시하고, 우측에 종합 판정 뱃지(`✓ 상성 우수` 또는 `⚠️ 신중함 요구`)를 다국어 텍스트로 배치합니다.
   - 다국어화된 종합 설명 문구(Explanation)를 노출합니다.
2. **화성살 진단 영역 (Mangal Dosha Cards)**:
   - 남성과 여성의 화성살 여부를 명확한 카드 디자인으로 표시합니다.
   - 화성살 감지 시 붉은색 글씨(`🔥 화성살 감지`), 정상인 경우 초록색 글씨(`✓ 해당 없음`)로 상태를 시각화합니다.
   - 상호 상쇄(Dosha Samya)가 일어난 경우 정보 배너를 통해 사용자에게 친절히 설명합니다.
3. **8대 요인 상세 카드 그리드 (8-Koota Detailed Grid)**:
   - `grid grid-cols-1 md:grid-cols-2 gap-4` 레이아웃을 통해 8개 항목을 카드 형태로 배치합니다.
   - 각 카드에는 **[요인 이름]**, **[설명 / 의미]**, **[점수 바 (Progress Bar)]**, **[점수별 상태 해석]**을 포함합니다.
   - **점수 바(Progress Bar)** 색상 매핑:
     - 만점(Ratio == 1.0): Emerald 색상 (`bg-emerald-500`)
     - 부분 점수(Ratio > 0.0): Purple 색상 (`bg-purple-500`)
     - 0점(Ratio == 0.0): Red 색상 (`bg-red-500`)

---

## 4. 다국어 (i18n) 설계 (Localization Strategy)

현재 엔진에서 생성하는 `explanation` 및 Koota `description`은 하드코딩된 한국어 및 영어 문자열입니다. 코드 수정 없이 완벽한 4개 국어(Ko, En, Zh, Ru) 지원을 달성하기 위해 UI단에서 획득 점수 및 요인 인덱스를 바탕으로 다국어 해석을 동적 조립하는 헬퍼 함수를 적용합니다.

### 4-1. 추가되는 다국어 키 (Translation Keys in `TK` enum)
`crates/eon-ui/src/i18n/mod.rs`에 아래 다국어 키를 등록합니다.

```rust
    // Vedic Compatibility
    VedicCompatPartnerInputHeader,     // "상대방 출생 정보 입력"
    VedicCompatPartnerYearLabel,       // "년도"
    VedicCompatPartnerMonthLabel,      // "월"
    VedicCompatPartnerDayLabel,        // "일"
    VedicCompatPartnerHourLabel,       // "시간 (시)"
    VedicCompatPartnerMinLabel,        // "시간 (분)"
    VedicCompatPartnerLatLabel,        // "위도 (Latitude)"
    VedicCompatPartnerLonLabel,        // "경도 (Longitude)"
    VedicCompatRunButton,              // "💞 궁합 분석 실행"
    VedicCompatLoadingText,            // "궁합 연산 중..."
    VedicCompatOverallJudgementHeader, // "종합 매칭 판정"
    VedicCompatScoreLabelText,         // "호환성 점수"
    VedicCompatStatusExcellentText,    // "✓ 상성 우수"
    VedicCompatStatusCautionText,      // "⚠️ 신중함 요구"
    VedicCompatMaleMangalTitle,        // "남성 화성살 (Male Mangal Dosha)"
    VedicCompatFemaleMangalTitle,      // "여성 화성살 (Female Mangal Dosha)"
    VedicCompatMangalDetected,         // "🔥 화성살(Manglik) 감지"
    VedicCompatMangalNotDetected,      // "✓ 해당 없음 (양호)"
    VedicCompatMangalCancelledText,    // "ℹ️ 상호 화성살 상쇄(Dosha Samya)..."
    VedicCompatAshtakootaTableHeader,  // "아쉬타쿠타(Ashtakoota) 세부 매칭 평점표"
```

---

## 5. 상세 제안 코드 (Proposed Code Changes)

### 5-1. `crates/eon-ui/src/i18n/mod.rs` 변경 제안
`TK` 열거형에 신규 키를 추가합니다.

```rust
<<<< BEFORE
    VedicLagnaRasi,
    VedicHouseScore,

    // ── Tier ──────────────────────────────────────────────────────────
==== AFTER
    VedicLagnaRasi,
    VedicHouseScore,

    // ── Vedic Compatibility ──
    VedicCompatPartnerInputHeader,
    VedicCompatPartnerYearLabel,
    VedicCompatPartnerMonthLabel,
    VedicCompatPartnerDayLabel,
    VedicCompatPartnerHourLabel,
    VedicCompatPartnerMinLabel,
    VedicCompatPartnerLatLabel,
    VedicCompatPartnerLonLabel,
    VedicCompatRunButton,
    VedicCompatLoadingText,
    VedicCompatOverallJudgementHeader,
    VedicCompatScoreLabelText,
    VedicCompatStatusExcellentText,
    VedicCompatStatusCautionText,
    VedicCompatMaleMangalTitle,
    VedicCompatFemaleMangalTitle,
    VedicCompatMangalDetected,
    VedicCompatMangalNotDetected,
    VedicCompatMangalCancelledText,
    VedicCompatAshtakootaTableHeader,

    // ── Tier ──────────────────────────────────────────────────────────
>>>>
```

### 5-2. `ko.rs`, `en.rs`, `zh.rs`, `ru.rs` 다국어 매핑 추가 제안

#### `crates/eon-ui/src/i18n/ko.rs`
```rust
        // Vedic-specific (Vedic Compatibility)
        TK::VedicCompatPartnerInputHeader => "상대방 출생 정보 입력",
        TK::VedicCompatPartnerYearLabel => "년도",
        TK::VedicCompatPartnerMonthLabel => "월",
        TK::VedicCompatPartnerDayLabel => "일",
        TK::VedicCompatPartnerHourLabel => "시간 (시)",
        TK::VedicCompatPartnerMinLabel => "시간 (분)",
        TK::VedicCompatPartnerLatLabel => "위도 (Latitude)",
        TK::VedicCompatPartnerLonLabel => "경도 (Longitude)",
        TK::VedicCompatRunButton => "💞 궁합 분석 실행",
        TK::VedicCompatLoadingText => "궁합 연산 중...",
        TK::VedicCompatOverallJudgementHeader => "종합 매칭 판정",
        TK::VedicCompatScoreLabelText => "호환성 점수",
        TK::VedicCompatStatusExcellentText => "✓ 상성 우수",
        TK::VedicCompatStatusCautionText => "⚠️ 신중함 요구",
        TK::VedicCompatMaleMangalTitle => "남성 화성살 (Male Mangal Dosha)",
        TK::VedicCompatFemaleMangalTitle => "여성 화성살 (Female Mangal Dosha)",
        TK::VedicCompatMangalDetected => "🔥 화성살(Manglik) 감지",
        TK::VedicCompatMangalNotDetected => "✓ 해당 없음 (양호)",
        TK::VedicCompatMangalCancelledText => "ℹ️ 상호 화성살 상쇄(Dosha Samya)가 성립되어 화성살의 부정적 영향이 소멸되었습니다.",
        TK::VedicCompatAshtakootaTableHeader => "아쉬타쿠타(Ashtakoota) 세부 매칭 평점표",
```

#### `crates/eon-ui/src/i18n/en.rs`
```rust
        // Vedic-specific (Vedic Compatibility)
        TK::VedicCompatPartnerInputHeader => "Partner's Birth Information",
        TK::VedicCompatPartnerYearLabel => "Year",
        TK::VedicCompatPartnerMonthLabel => "Month",
        TK::VedicCompatPartnerDayLabel => "Day",
        TK::VedicCompatPartnerHourLabel => "Hour",
        TK::VedicCompatPartnerMinLabel => "Minute",
        TK::VedicCompatPartnerLatLabel => "Latitude",
        TK::VedicCompatPartnerLonLabel => "Longitude",
        TK::VedicCompatRunButton => "💞 Run Compatibility Analysis",
        TK::VedicCompatLoadingText => "Analyzing compatibility...",
        TK::VedicCompatOverallJudgementHeader => "Overall Compatibility Judgement",
        TK::VedicCompatScoreLabelText => "Compatibility Score",
        TK::VedicCompatStatusExcellentText => "✓ Excellent Compatibility",
        TK::VedicCompatStatusCautionText => "⚠️ Caution Required",
        TK::VedicCompatMaleMangalTitle => "Male Mangal Dosha",
        TK::VedicCompatFemaleMangalTitle => "Female Mangal Dosha",
        TK::VedicCompatMangalDetected => "🔥 Manglik Detected",
        TK::VedicCompatMangalNotDetected => "✓ N/A (Clear)",
        TK::VedicCompatMangalCancelledText => "ℹ️ Dosha Samya established: Both partners have Manglik status, cancelling the negative effects.",
        TK::VedicCompatAshtakootaTableHeader => "Ashtakoota Guna Milan Detailed Scorecard",
```

#### `crates/eon-ui/src/i18n/zh.rs`
```rust
        // Vedic-specific (Vedic Compatibility)
        TK::VedicCompatPartnerInputHeader => "输入对方出生信息",
        TK::VedicCompatPartnerYearLabel => "年",
        TK::VedicCompatPartnerMonthLabel => "月",
        TK::VedicCompatPartnerDayLabel => "日",
        TK::VedicCompatPartnerHourLabel => "时",
        TK::VedicCompatPartnerMinLabel => "分",
        TK::VedicCompatPartnerLatLabel => "纬度 (Latitude)",
        TK::VedicCompatPartnerLonLabel => "经度 (Longitude)",
        TK::VedicCompatRunButton => "💞 运行合婚分析",
        TK::VedicCompatLoadingText => "正在进行合婚计算...",
        TK::VedicCompatOverallJudgementHeader => "综合配对判定",
        TK::VedicCompatScoreLabelText => "配对分值",
        TK::VedicCompatStatusExcellentText => "✓ 极佳配对",
        TK::VedicCompatStatusCautionText => "⚠️ 需谨慎处理",
        TK::VedicCompatMaleMangalTitle => "男方火星煞 (Male Mangal Dosha)",
        TK::VedicCompatFemaleMangalTitle => "女方火星煞 (Female Mangal Dosha)",
        TK::VedicCompatMangalDetected => "🔥 检测到火星煞 (Manglik)",
        TK::VedicCompatMangalNotDetected => "✓ 无 (良好)",
        TK::VedicCompatMangalCancelledText => "ℹ️ 双方火星煞相互抵消 (Dosha Samya) 成立，消除了负面影响。",
        TK::VedicCompatAshtakootaTableHeader => "阿什塔库塔 (Ashtakoota) 详细评分表",
```

#### `crates/eon-ui/src/i18n/ru.rs`
```rust
        // Vedic-specific (Vedic Compatibility)
        TK::VedicCompatPartnerInputHeader => "Ввод данных партнёра",
        TK::VedicCompatPartnerYearLabel => "Год",
        TK::VedicCompatPartnerMonthLabel => "Месяц",
        TK::VedicCompatPartnerDayLabel => "День",
        TK::VedicCompatPartnerHourLabel => "Час",
        TK::VedicCompatPartnerMinLabel => "Минута",
        TK::VedicCompatPartnerLatLabel => "Широта",
        TK::VedicCompatPartnerLonLabel => "Долгота",
        TK::VedicCompatRunButton => "💞 Запустить анализ совместимости",
        TK::VedicCompatLoadingText => "Анализ совместимости...",
        TK::VedicCompatOverallJudgementHeader => "Общая оценка совместимости",
        TK::VedicCompatScoreLabelText => "Балл совместимости",
        TK::VedicCompatStatusExcellentText => "✓ Отличная совместимость",
        TK::VedicCompatStatusCautionText => "⚠️ Требуется осторожность",
        TK::VedicCompatMaleMangalTitle => "Марсианское влияние у мужчины (Mangal Dosha)",
        TK::VedicCompatFemaleMangalTitle => "Марсианское влияние у женщины (Mangal Dosha)",
        TK::VedicCompatMangalDetected => "🔥 Обнаружена Манглик Доша",
        TK::VedicCompatMangalNotDetected => "✓ Нет (В норме)",
        TK::VedicCompatMangalCancelledText => "ℹ️ Установлена Доша Самья: Оба партнёра имеют Манглик Доша, нейтрализуя её негативные эффекты.",
        TK::VedicCompatAshtakootaTableHeader => "Детальная карта оценки Аштакута",
```

---

### 5-3. `crates/eon-ui/src/components/tabs/vedic_tab.rs` 변경 제안

#### 1) Koota 다국어화 헬퍼 함수 정의 (컴포넌트 파일 내 또는 별도 유틸리티에 추가)
```rust
struct LocalizedKoota {
    name: &'static str,
    meaning: &'static str,
    interpretation: String,
}

fn localize_koota(locale: Locale, index: usize, earned: f64, max: f64) -> LocalizedKoota {
    match locale {
        Locale::Ko => match index {
            0 => LocalizedKoota {
                name: "바르나 (Varna) - 정신적 성향",
                meaning: "사회적 역할, 영적 성숙도 및 직업적 성향의 조화를 나타냅니다.",
                interpretation: if earned == 1.0 {
                    "상호 협력과 역할 분담이 순조로운 긍정적인 관계입니다.".to_string()
                } else {
                    "가치관과 일에 대한 태도 차이로 갈등이 생길 수 있습니다.".to_string()
                }
            },
            1 => LocalizedKoota {
                name: "바샤 (Vashya) - 상호 영향력",
                meaning: "부부 사이의 주도권, 상호 매력과 영향력의 균형을 분석합니다.",
                interpretation: format!("상호 지배 및 협력도 평점: {}/2.", earned),
            },
            2 => LocalizedKoota {
                name: "타라 (Tara) - 운명과 건강",
                meaning: "두 사람의 탄생 별자리 관계를 통해 수명과 전반적인 운세를 판단합니다.",
                interpretation: if earned == 3.0 {
                    "최상의 운명적 조화와 장수운을 나타냅니다.".to_string()
                } else if earned == 1.5 {
                    "무난한 수준의 건강 및 활동성 호환을 보입니다.".to_string()
                } else {
                    "건강과 장수에 주의를 요하는 배치입니다 (타라 도샤).".to_string()
                }
            },
            3 => LocalizedKoota {
                name: "요니 (Yoni) - 생물학적 조화",
                meaning: "신체적, 성적 조화 및 본능적 친밀도를 상징적 동물 유형으로 평가합니다.",
                interpretation: format!("생물학적/본능적 친밀도: {}/4.", earned),
            },
            4 => LocalizedKoota {
                name: "그라하 마이트리 (Graha Maitri) - 정신적 교감",
                meaning: "두 사람의 달(지배성) 관계를 통해 지적 소통과 우정을 측정합니다.",
                interpretation: if earned >= 4.0 {
                    "정신적 깊은 유대와 따뜻한 우정이 성립됩니다.".to_string()
                } else if earned >= 2.0 {
                    "평범한 지적 교감을 나누며 원활히 대화할 수 있습니다.".to_string()
                } else {
                    "소통 방식의 차이로 오해가 생기기 쉬운 조합입니다.".to_string()
                }
            },
            5 => LocalizedKoota {
                name: "가나 (Gana) - 기질과 성품",
                meaning: "성격과 기질(Deva-신, Manushya-인간, Rakshasa-격렬함)의 호환성입니다.",
                interpretation: if earned >= 5.0 {
                    "기질이 조화로워 감정적 충돌이 거의 없습니다.".to_string()
                } else if earned >= 3.0 {
                    "보통 수준의 조화로, 상호 양보가 필요합니다.".to_string()
                } else {
                    "성격 차이가 커 잦은 마찰이 일어날 수 있습니다 (가나 도샤).".to_string()
                }
            },
            6 => LocalizedKoota {
                name: "바쿠트 (Bhakoot) - 정서적 유대",
                meaning: "상대적인 월성좌 거리를 통해 정서적 건강, 번영과 부부애를 분석합니다.",
                interpretation: if earned == 7.0 {
                    "감정의 기복이 없고 안정적인 정서적 유대감이 지속됩니다.".to_string()
                } else {
                    "정서적 소외감이나 가계 재정 흐름의 기복이 우려됩니다 (바쿠트 도샤).".to_string()
                }
            },
            7 => LocalizedKoota {
                name: "나디 (Nadi) - 유전적 호환성",
                meaning: "가장 중요한 요소로, 신경 계통의 기운(체질)과 유전적 어울림을 봅니다.",
                interpretation: if earned == 8.0 {
                    "신체적 면역계가 서로 잘 결합하여 건강한 자녀운을 지닙니다.".to_string()
                } else {
                    "같은 계열의 과도한 에너지로 유전적 불일치 위험이 큽니다 (나디 도샤).".to_string()
                }
            },
            _ => LocalizedKoota { name: "", meaning: "", interpretation: "".to_string() }
        },
        Locale::En => match index {
            0 => LocalizedKoota {
                name: "Varna - Spiritual Inclination",
                meaning: "Represents ego development, spiritual growth, and natural work inclination.",
                interpretation: if earned == 1.0 {
                    "Smooth collaboration and healthy division of roles.".to_string()
                } else {
                    "Possible friction due to different life perspectives and career attitudes.".to_string()
                }
            },
            1 => LocalizedKoota {
                name: "Vashya - Mutual Influence",
                meaning: "Analyzes the degree of mutual attraction, influence, and dominance in relationship.",
                interpretation: format!("Mutual attraction and influence rating: {}/2.", earned),
            },
            2 => LocalizedKoota {
                name: "Tara - Destiny and Health",
                meaning: "Evaluates health, longevity, and overall destiny based on birth stars.",
                interpretation: if earned == 3.0 {
                    "Excellent alignment of longevity and fortune.".to_string()
                } else if earned == 1.5 {
                    "Moderate health and energy compatibility.".to_string()
                } else {
                    "Challenging health and energy match (Tara Dosha).".to_string()
                }
            },
            3 => LocalizedKoota {
                name: "Yoni - Biological Affinity",
                meaning: "Represents physical, biological compatibility and instinctual intimacy through animal symbols.",
                interpretation: format!("Biological and physical compatibility: {}/4.", earned),
            },
            4 => LocalizedKoota {
                name: "Graha Maitri - Mental Bond",
                meaning: "Measures intellectual communication, friendship, and emotional resonance based on ruling planets.",
                interpretation: if earned >= 4.0 {
                    "Deep mental harmony and mutual friendship.".to_string()
                } else if earned >= 2.0 {
                    "Average intellectual connection and understanding.".to_string()
                } else {
                    "Potential gaps in communication and mental alignment.".to_string()
                }
            },
            5 => LocalizedKoota {
                name: "Gana - Temperament",
                meaning: "Compares fundamental temperaments (Deva-Divine, Manushya-Human, Rakshasa-Demon).",
                interpretation: if earned >= 5.0 {
                    "Compatible temperaments, low emotional friction.".to_string()
                } else if earned >= 3.0 {
                    "Moderate gaps in temperament; patience is required.".to_string()
                } else {
                    "High emotional friction and temperamental clash (Gana Dosha).".to_string()
                }
            },
            6 => LocalizedKoota {
                name: "Bhakoot - Emotional Node",
                meaning: "Analyzes emotional health, prosperity, and marital longevity based on Moon sign distances.",
                interpretation: if earned == 7.0 {
                    "Stable emotional bond and harmonious family life.".to_string()
                } else {
                    "Challenging emotional cycles or potential financial stress (Bhakoot Dosha).".to_string()
                }
            },
            7 => LocalizedKoota {
                name: "Nadi - Genetic Health",
                meaning: "The most critical factor; checks genetic health, nervous energy, and lineage compatibility.",
                interpretation: if earned == 8.0 {
                    "Balanced nervous energies and excellent genetic compatibility.".to_string()
                } else {
                    "Excessive similar nervous energy, risk of genetic mismatch (Nadi Dosha).".to_string()
                }
            },
            _ => LocalizedKoota { name: "", meaning: "", interpretation: "".to_string() }
        },
        Locale::Zh => match index {
            0 => LocalizedKoota {
                name: "Varna (瓦尔纳) - 精神倾向",
                meaning: "代表精神成长、自尊以及天然的工作倾斜度与社会角色分工。",
                interpretation: if earned == 1.0 {
                    "相互合作与角色分工非常顺利，关系积极。".to_string()
                } else {
                    "因价值观和对待工作的态度不同，可能会产生分歧。".to_string()
                }
            },
            1 => LocalizedKoota {
                name: "Vashya (瓦夏) - 相互吸引力",
                meaning: "分析夫妻之间的主导权、相互吸引力与控制力平衡。",
                interpretation: format!("相互吸引与合作度评分: {}/2。", earned),
            },
            2 => LocalizedKoota {
                name: "Tara (塔拉) - 命运与健康",
                meaning: "通过双方出生星宿的相对距离，来判断健康、寿命及整体运势。",
                interpretation: if earned == 3.0 {
                    "极佳的命运契合度与长寿运。".to_string()
                } else if earned == 1.5 {
                    "中等水平的健康与活动兼容性。".to_string()
                } else {
                    "健康与寿命需要注意的配置 (Tara 煞/Tara Dosha)。".to_string()
                }
            },
            3 => LocalizedKoota {
                name: "Yoni (约尼) - 生物学亲和力",
                meaning: "以象征性的动物类型来评估身体、性契合度及本能的亲密感。",
                interpretation: format!("生物学和生理上的契合度: {}/4。", earned),
            },
            4 => LocalizedKoota {
                name: "Graha Maitri (行星友谊) - 精神交流",
                meaning: "通过双方月亮守护星的关系，衡量智力沟通、友谊和共鸣程度。",
                interpretation: if earned >= 4.0 {
                    "精神上有深厚的纽带和温暖的友谊。".to_string()
                } else if earned >= 2.0 {
                    "平稳的智力交流，沟通顺畅。".to_string()
                } else {
                    "因沟通方式差异容易产生误解的组合。".to_string()
                }
            },
            5 => LocalizedKoota {
                name: "Gana (迦纳) - 气质与性情",
                meaning: "比较双方的基本气质类型（Deva-天神，Manushya-人类，Rakshasa-罗刹）。",
                interpretation: if earned >= 5.0 {
                    "气质契合，极少有情绪冲突。".to_string()
                } else if earned >= 3.0 {
                    "中等水平的气质和谐，需要相互妥协。".to_string()
                } else {
                    "性格差异大，可能发生频繁摩擦 (Gana 煞/Gana Dosha)。".to_string()
                }
            },
            6 => LocalizedKoota {
                name: "Bhakoot (巴库特) - 情感纽带",
                meaning: "基于相对月亮星座的距离，分析情感健康、繁荣和婚姻的稳定性。",
                interpretation: if earned == 7.0 {
                    "情感没有大起大落，维持稳定的情感纽带。".to_string()
                } else {
                    "容易感到情感疏离或面临家庭财务波动的风险 (Bhakoot 煞/Bhakoot Dosha)。".to_string()
                }
            },
            7 => LocalizedKoota {
                name: "Nadi (纳迪) - 遗传兼容性",
                meaning: "最关键的因素，检测双方神经系统能量、体质以及遗传兼容性。",
                interpretation: if earned == 8.0 {
                    "身体免疫系统契合，有利于子嗣健康。".to_string()
                } else {
                    "相同类型的过剩能量，有遗传不契合的风险 (Nadi 煞/Nadi Dosha)。".to_string()
                }
            },
            _ => LocalizedKoota { name: "", meaning: "", interpretation: "".to_string() }
        },
        Locale::Ru => match index {
            0 => LocalizedKoota {
                name: "Варна (Varna) - Ментальный склад",
                meaning: "Отражает духовное развитие, эго и естественные склонности в работе и социальной роли.",
                interpretation: if earned == 1.0 {
                    "Гармоничное сотрудничество и легкое разделение обязанностей.".to_string()
                } else {
                    "Возможны разногласия из-за несовпадающих взглядов на карьеру и жизнь.".to_string()
                }
            },
            1 => LocalizedKoota {
                name: "Вашья (Vashya) - Взаимное влияние",
                meaning: "Анализирует степень взаимного притяжения, влияния и баланса авторитета в паре.",
                interpretation: format!("Оценка взаимного притяжения и влияния: {}/2.", earned),
            },
            2 => LocalizedKoota {
                name: "Тара (Tara) - Судьба и здоровье",
                meaning: "Оценивает здоровье, долголетие и общую удачу на основе взаимного положения накшатр.",
                interpretation: if earned == 3.0 {
                    "Отличное соответствие жизненных циклов и долголетия.".to_string()
                } else if earned == 1.5 {
                    "Умеренная совместимость по уровню здоровья и энергии.".to_string()
                } else {
                    "Сложная совместимость, требующая внимания к здоровью (Тара-доша).".to_string()
                }
            },
            3 => LocalizedKoota {
                name: "Йони (Yoni) - Биологическая близость",
                meaning: "Определяет телесную и инстинктивную близость через символы животных типов накшатр.",
                interpretation: format!("Биологическая и инстинктивная совместимость: {}/4.", earned),
            },
            4 => LocalizedKoota {
                name: "Граха Майтри (Graha Maitri) - Ментальная связь",
                meaning: "Показывает интеллектуальное взаимопонимание, дружбу на основе отношений планет-управителей Луны.",
                interpretation: if earned >= 4.0 {
                    "Глубокая ментальная гармония и искренняя дружба в паре.".to_string()
                } else if earned >= 2.0 {
                    "Средний уровень понимания, спокойный диалог.".to_string()
                } else {
                    "Возможны частые недопонимания из-за разного стиля мышления.".to_string()
                }
            },
            5 => LocalizedKoota {
                name: "Гана (Gana) - Темперамент",
                meaning: "Сравнивает базовые психотипы (Дева - божественный, Манушья - человеческий, Ракшас - импульсивный).",
                interpretation: if earned >= 5.0 {
                    "Гармоничное сочетание характеров, минимум эмоциональных трений.".to_string()
                } else if earned >= 3.0 {
                    "Среднее совпадение, требуется взаимная уступчивость.".to_string()
                } else {
                    "Высокий риск эмоциональных конфликтов из-за несовместимости характеров (Гана-доша).".to_string()
                }
            },
            6 => LocalizedKoota {
                name: "Бхакут (Bhakoot) - Эмоциональная связь",
                meaning: "Оценивает эмоциональную стабильность, семейное процветание по взаимному положению знаков Луны.",
                interpretation: if earned == 7.0 {
                    "Стабильная эмоциональная привязанность без резких перепадов.".to_string()
                } else {
                    "Вероятны эмоциональное отдаление или финансовые трудности в семье (Бхакут-доша).".to_string()
                }
            },
            7 => LocalizedKoota {
                name: "Нади (Nadi) - Генетическая совместимость",
                meaning: "Важнейший фактор. Проверяет баланс нервной энергии и генетическое здоровье потомства.",
                interpretation: if earned == 8.0 {
                    "Сбалансированное здоровье и отличная совместимость на уровне потомства.".to_string()
                } else {
                    "Избыток однородной энергии, генетическая несовместимость (Нади-доша).".to_string()
                }
            },
            _ => LocalizedKoota { name: "", meaning: "", interpretation: "".to_string() }
        }
    }
}
```

#### 2) `vedic_tab.rs` 내 호환성 탭 렌더링 코드 변경 제안
(기존 라인 1996~2185 영역의 Dioxus 컴포넌트 마크업을 아래 코드로 대체)

```rust
                                3 => rsx! {
                                    // Compatibility Tab
                                    div { class: "space-y-6",
                                        // 1. Partner Input Form
                                        div { class: "p-5 bg-slate-900 border border-slate-800 rounded-2xl space-y-4",
                                            h3 { class: "text-lg font-semibold text-slate-200", "{t(locale, TK::VedicCompatPartnerInputHeader)}" }
                                            div { class: "grid grid-cols-2 md:grid-cols-5 gap-3",
                                                div { class: "flex flex-col gap-1",
                                                    label { class: "text-xs text-slate-500", "{t(locale, TK::VedicCompatPartnerYearLabel)}" }
                                                    input {
                                                        class: "bg-slate-800 border border-slate-700 rounded-xl px-3 py-2 text-slate-200",
                                                        r#type: "number",
                                                        value: "{partner_year}",
                                                        oninput: move |e| *partner_year.write() = e.value().parse().unwrap_or(1992)
                                                    }
                                                }
                                                div { class: "flex flex-col gap-1",
                                                    label { class: "text-xs text-slate-500", "{t(locale, TK::VedicCompatPartnerMonthLabel)}" }
                                                    input {
                                                        class: "bg-slate-800 border border-slate-700 rounded-xl px-3 py-2 text-slate-200",
                                                        r#type: "number",
                                                        value: "{partner_month}",
                                                        oninput: move |e| *partner_month.write() = e.value().parse().unwrap_or(8)
                                                    }
                                                }
                                                div { class: "flex flex-col gap-1",
                                                    label { class: "text-xs text-slate-500", "{t(locale, TK::VedicCompatPartnerDayLabel)}" }
                                                    input {
                                                        class: "bg-slate-800 border border-slate-700 rounded-xl px-3 py-2 text-slate-200",
                                                        r#type: "number",
                                                        value: "{partner_day}",
                                                        oninput: move |e| *partner_day.write() = e.value().parse().unwrap_or(24)
                                                    }
                                                }
                                                div { class: "flex flex-col gap-1",
                                                    label { class: "text-xs text-slate-500", "{t(locale, TK::VedicCompatPartnerHourLabel)}" }
                                                    input {
                                                        class: "bg-slate-800 border border-slate-700 rounded-xl px-3 py-2 text-slate-200",
                                                        r#type: "number",
                                                        value: "{partner_hour}",
                                                        oninput: move |e| *partner_hour.write() = e.value().parse().unwrap_or(14)
                                                    }
                                                }
                                                div { class: "flex flex-col gap-1",
                                                    label { class: "text-xs text-slate-500", "{t(locale, TK::VedicCompatPartnerMinLabel)}" }
                                                    input {
                                                        class: "bg-slate-800 border border-slate-700 rounded-xl px-3 py-2 text-slate-200",
                                                        r#type: "number",
                                                        value: "{partner_minute}",
                                                        oninput: move |e| *partner_minute.write() = e.value().parse().unwrap_or(30)
                                                    }
                                                }
                                            }
                                            div { class: "grid grid-cols-2 gap-3",
                                                div { class: "flex flex-col gap-1",
                                                    label { class: "text-xs text-slate-500", "{t(locale, TK::VedicCompatPartnerLatLabel)}" }
                                                    input {
                                                        class: "bg-slate-800 border border-slate-700 rounded-xl px-3 py-2 text-slate-200",
                                                        r#type: "number",
                                                        step: "any",
                                                        value: "{partner_lat}",
                                                        oninput: move |e| *partner_lat.write() = e.value().parse().unwrap_or(37.5665)
                                                    }
                                                }
                                                div { class: "flex flex-col gap-1",
                                                    label { class: "text-xs text-slate-500", "{t(locale, TK::VedicCompatPartnerLonLabel)}" }
                                                    input {
                                                        class: "bg-slate-800 border border-slate-700 rounded-xl px-3 py-2 text-slate-200",
                                                        r#type: "number",
                                                        step: "any",
                                                        value: "{partner_lon}",
                                                        oninput: move |e| *partner_lon.write() = e.value().parse().unwrap_or(126.9780)
                                                    }
                                                }
                                            }
                                            button {
                                                class: "w-full py-3 bg-gradient-to-r from-purple-700 to-indigo-700 hover:from-purple-600 hover:to-indigo-600 rounded-xl font-bold text-white shadow-lg",
                                                onclick: run_compatibility,
                                                "{t(locale, TK::VedicCompatRunButton)}"
                                            }
                                        }

                                        // 2. Compatibility Results
                                        match &*compat_status.read() {
                                            TaskStatus::Loading => rsx! {
                                                div { class: "flex flex-col items-center py-10 gap-2",
                                                    div { class: "w-10 h-10 rounded-full border-4 border-purple-500/30 border-t-purple-400 animate-spin" }
                                                    p { class: "text-purple-400 font-medium text-sm animate-pulse", "{t(locale, TK::VedicCompatLoadingText)}" }
                                                }
                                            },
                                            TaskStatus::Error(e) => rsx! {
                                                div { class: "p-4 rounded-xl bg-red-900/20 border border-red-800/50 text-red-400 text-sm", "{t(locale, TK::StatusError)}: {e}" }
                                            },
                                            TaskStatus::Success => {
                                                if let Some(compat) = &*compat_data.read() {
                                                    // Dynamically localize the overall matching explanation
                                                    let explanation_text = match locale {
                                                        Locale::Ko => format!(
                                                            "총 {}점 획득 (36점 만점). {}",
                                                            compat.report.total_score,
                                                            if compat.report.is_compatible {
                                                                "전반적으로 조화로운 매칭입니다. 추천합니다."
                                                            } else if compat.report.total_score >= 18.0 {
                                                                "점수는 높으나 주요 살(Nadi/Bhakoot Dosha)의 영향으로 신중한 주의가 필요합니다."
                                                            } else {
                                                                "성향적 차이가 커 상호 조율과 깊은 이해가 요구되는 상성입니다."
                                                            }
                                                        ),
                                                        Locale::En => format!(
                                                            "Earned {} points in total (out of 36). {}",
                                                            compat.report.total_score,
                                                            if compat.report.is_compatible {
                                                                "Overall a very harmonious match. Highly recommended."
                                                            } else if compat.report.total_score >= 18.0 {
                                                                "High score, but requires careful attention due to critical Nadi/Bhakoot Dosha."
                                                            } else {
                                                                "Significant temperamental differences; requires mutual adjustment and deep understanding."
                                                            }
                                                        ),
                                                        Locale::Zh => format!(
                                                            "共获得 {} 分（满分 36 分）。{}",
                                                            compat.report.total_score,
                                                            if compat.report.is_compatible {
                                                                "总体而言是非常和谐的配对，值得推荐。"
                                                            } else if compat.report.total_score >= 18.0 {
                                                                "虽然得分较高，但由于存在关键的纳迪/巴库特煞（Nadi/Bhakoot Dosha），需要谨慎对待。"
                                                            } else {
                                                                "性情差异较大，需要相互包容与深度理解。"
                                                            }
                                                        ),
                                                        Locale::Ru => format!(
                                                            "Всего набрано {} баллов (из 36). {}",
                                                            compat.report.total_score,
                                                            if compat.report.is_compatible {
                                                                "В целом гармоничное сочетание. Рекомендуется."
                                                            } else if compat.report.total_score >= 18.0 {
                                                                "Высокий балл, но требуется осторожность из-за критических Нади/Бхакут дош."
                                                            } else {
                                                                "Значительные различия в характерах; требуются взаимные уступки и глубокое понимание."
                                                            }
                                                        ),
                                                    };

                                                    rsx! {
                                                        div { class: "space-y-6 animate-in fade-in duration-500",
                                                            // Overall Judgement Card
                                                            div { class: "p-5 bg-slate-900 border border-slate-800 rounded-2xl flex flex-col md:flex-row md:items-center justify-between gap-4",
                                                                div {
                                                                    h3 { class: "text-xs text-slate-500 uppercase tracking-widest font-bold", "{t(locale, TK::VedicCompatOverallJudgementHeader)}" }
                                                                    p { class: "text-2xl font-bold text-slate-200 mt-1",
                                                                        "{t(locale, TK::VedicCompatScoreLabelText)}: "
                                                                        span { class: "text-purple-400", "{compat.report.total_score} / 36 Gunas" }
                                                                    }
                                                                    p { class: "text-sm text-slate-400 mt-1.5", "{explanation_text}" }
                                                                }
                                                                div { class: "flex gap-2",
                                                                    if compat.report.is_compatible {
                                                                        span { class: "px-4 py-2 rounded-xl bg-emerald-950/60 border border-emerald-800/60 text-emerald-400 text-sm font-bold", "{t(locale, TK::VedicCompatStatusExcellentText)}" }
                                                                    } else {
                                                                        span { class: "px-4 py-2 rounded-xl bg-amber-950/60 border border-amber-800/60 text-amber-400 text-sm font-bold", "{t(locale, TK::VedicCompatStatusCautionText)}" }
                                                                    }
                                                                }
                                                            }

                                                            // Mangal Dosha Cards
                                                            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                                                div { class: "p-4 rounded-2xl bg-slate-900 border border-slate-800 space-y-2",
                                                                    h4 { class: "text-xs text-slate-500 font-semibold uppercase tracking-wider", "{t(locale, TK::VedicCompatMaleMangalTitle)}" }
                                                                    p {
                                                                        class: if compat.report.male_mangal_dosha {
                                                                            "text-lg font-bold text-red-400"
                                                                        } else {
                                                                            "text-lg font-bold text-emerald-400"
                                                                        },
                                                                        if compat.report.male_mangal_dosha { "{t(locale, TK::VedicCompatMangalDetected)}" } else { "{t(locale, TK::VedicCompatMangalNotDetected)}" }
                                                                    }
                                                                }
                                                                div { class: "p-4 rounded-2xl bg-slate-900 border border-slate-800 space-y-2",
                                                                    h4 { class: "text-xs text-slate-500 font-semibold uppercase tracking-wider", "{t(locale, TK::VedicCompatFemaleMangalTitle)}" }
                                                                    p {
                                                                        class: if compat.report.female_mangal_dosha {
                                                                            "text-lg font-bold text-red-400"
                                                                        } else {
                                                                            "text-lg font-bold text-emerald-400"
                                                                        },
                                                                        if compat.report.female_mangal_dosha { "{t(locale, TK::VedicCompatMangalDetected)}" } else { "{t(locale, TK::VedicCompatMangalNotDetected)}" }
                                                                    }
                                                                }
                                                            }
                                                            if compat.report.mangal_dosha_cancelled {
                                                                div { class: "p-4 rounded-xl bg-blue-950/40 border border-blue-800/40 text-blue-300 text-xs font-semibold",
                                                                    "{t(locale, TK::VedicCompatMangalCancelledText)}"
                                                                }
                                                            }

                                                            // Ashtakoota Detailed Cards Grid with Progress Bars
                                                            div { class: "space-y-4",
                                                                h3 { class: "font-semibold text-slate-200 text-base", "{t(locale, TK::VedicCompatAshtakootaTableHeader)}" }
                                                                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                                                    {compat.report.kootas.iter().enumerate().map(|(idx, k)| {
                                                                        let local = localize_koota(locale, idx, k.earned_points, k.max_points);
                                                                        let pct = (k.earned_points / k.max_points * 100.0).min(100.0).max(0.0);
                                                                        
                                                                        // Determine colors based on performance
                                                                        let (bar_color, text_color, bg_light) = if k.earned_points == k.max_points {
                                                                            ("bg-emerald-500", "text-emerald-400", "bg-emerald-500/10")
                                                                        } else if k.earned_points > 0.0 {
                                                                            ("bg-purple-500", "text-purple-400", "bg-purple-500/10")
                                                                        } else {
                                                                            ("bg-red-500", "text-red-400", "bg-red-500/10")
                                                                        };

                                                                        rsx! {
                                                                            div { class: "p-5 bg-slate-900 border border-slate-800 rounded-2xl flex flex-col justify-between space-y-3.5 hover:border-slate-700 transition-all duration-200",
                                                                                div { class: "space-y-1.5",
                                                                                    div { class: "flex items-center justify-between",
                                                                                        span { class: "text-sm font-bold text-slate-200", "{idx + 1}. {local.name}" }
                                                                                        span { class: "px-2 py-0.5 rounded font-mono text-xs font-bold {text_color} {bg_light}", "{k.earned_points:.1} / {k.max_points:.1} {t(locale, TK::LabelScore)}" }
                                                                                    }
                                                                                    p { class: "text-xs text-slate-500 leading-relaxed", "{local.meaning}" }
                                                                                }
                                                                                
                                                                                // Progress Bar
                                                                                div { class: "space-y-2",
                                                                                    div { class: "w-full bg-slate-800 rounded-full h-2 overflow-hidden",
                                                                                        div { 
                                                                                            class: "h-full rounded-full {bar_color} transition-all duration-500", 
                                                                                            style: "width: {pct}%" 
                                                                                        }
                                                                                    }
                                                                                    p { class: "text-[11px] {text_color} font-medium leading-relaxed", "{local.interpretation}" }
                                                                                }
                                                                            }
                                                                        }
                                                                    })}
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    rsx! { div {} }
                                                }
                                            },
                                            _ => rsx! { div {} }
                                        }
                                    }
                                },
```
