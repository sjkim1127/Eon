# Vedic Astrology BPHS 표준 확장 - 완료 보고서 (2차)

## 개요

`eon-vedic` 크레이트의 Shadbala (행성 강도) 계산과 Yoga (요가) 판별 시스템을 BPHS (Brihat Parashara Hora Shastra) 표준에 맞게 추가 확장했습니다.

---

## ✅ 완료된 수정 사항

### 1. Shadbala - Sthana Bala 구성요소 추가

#### 1.1 Kendra Bala (BPHS Sthana Bala 구성요소)

**구현 내용:**

```rust
/// Planets in Kendra houses (1, 4, 7, 10) get full strength (60)
/// Planets in Panaphara houses (2, 5, 8, 11) get half strength (30)
/// Planets in Apoklima houses (3, 6, 9, 12) get quarter strength (15)
fn calculate_kendra_bala(house: u8) -> f64 {
    match house {
        1 | 4 | 7 | 10 => 60.0,  // Kendra (Angular)
        2 | 5 | 8 | 11 => 30.0,  // Panaphara (Succedent)
        3 | 6 | 9 | 12 => 15.0,  // Apoklima (Cadent)
        _ => 0.0,
    }
}
```

**BPHS 참조:**

- Kendra (각형) 하우스: 1, 4, 7, 10번 하우스는 행동과 현현의 힘이 가장 강함
- Panaphara (후속) 하우스: 2, 5, 8, 11번 하우스는 중간 강도
- Apoklima (타락) 하우스: 3, 6, 9, 12번 하우스는 가장 약한 영향력

**위치:** `src/analysis/strength.rs:515-528`

---

#### 1.2 Drekkana Bala (BPHS Sthana Bala 구성요소)

**구현 내용:**

```rust
/// Based on masculine/feminine/neuter nature of planet and drekkana placement
/// Masculine planets: Sun, Mars, Jupiter
/// Feminine planets: Moon, Venus
/// Neuter planets: Mercury, Saturn
fn calculate_drekkana_bala(pos: &VedicPosition) -> f64 {
    let is_masculine = matches!(
        pos.planet,
        VedicPlanet::Sun | VedicPlanet::Mars | VedicPlanet::Jupiter
    );
    let is_feminine = matches!(pos.planet, VedicPlanet::Moon | VedicPlanet::Venus);
    
    let drekk_rasi = pos.drekkana_rasi;
    let drekk_is_masculine = drekk_rasi % 2 == 1;  // Odd = masculine
    
    // BPHS: Matching nature gets 15 points
    if is_masculine && drekk_is_masculine {
        15.0
    } else if is_feminine && !drekk_is_masculine {
        15.0
    } else if !is_masculine && !is_feminine {
        7.5  // Neuter planets (Mercury, Saturn) get 7.5 always
    } else {
        0.0
    }
}
```

**BPHS 참조:**

- D3 (드레카나) 차트에서 행성의 성별과 별자리의 성별이 일치하면 강도 증가
- 남성 행성: 태양, 화성, 목성
- 여성 행성: 달, 금성
- 중성 행성: 수성, 토성 (항상 절반 점수)

**위치:** `src/analysis/strength.rs:530-558`

---

#### 1.3 Ojayugmarasyamsa Bala (BPHS Sthana Bala 구성요소)

**구현 내용:**

```rust
/// Based on odd/even sign placement in D1 (Rasi) and D9 (Navamsa)
/// Masculine planets get strength in odd signs
/// Feminine planets get strength in even signs
/// Mercury gets strength in both (neuter)
fn calculate_ojayugmarasyamsa_bala(pos: &VedicPosition) -> f64 {
    let is_masculine = matches!(
        pos.planet,
        VedicPlanet::Sun | VedicPlanet::Mars | VedicPlanet::Jupiter
    );
    let is_feminine = matches!(pos.planet, VedicPlanet::Moon | VedicPlanet::Venus);
    let is_mercury = pos.planet == VedicPlanet::Mercury;
    
    let rasi_is_odd = pos.rasi % 2 == 1;
    let navamsa_is_odd = pos.navamsa_rasi % 2 == 1;
    
    let mut score = 0.0;
    
    // D1 (Rasi) strength: 7.5 points max
    if is_mercury {
        score += 7.5;  // Mercury always gets points
    } else if is_masculine && rasi_is_odd {
        score += 7.5;
    } else if is_feminine && !rasi_is_odd {
        score += 7.5;
    }
    
    // D9 (Navamsa) strength: 7.5 points max
    if is_mercury {
        score += 7.5;
    } else if is_masculine && navamsa_is_odd {
        score += 7.5;
    } else if is_feminine && !navamsa_is_odd {
        score += 7.5;
    }
    
    score  // Max: 15.0
}
```

**BPHS 참조:**

- D1 (라시)와 D9 (나밤사) 양쪽에서 홀수/짝수 별자리 배치에 따른 강도
- 남성 행성은 홀수 별자리(1,3,5,7,9,11)에서 강함
- 여성 행성은 짝수 별자리(2,4,6,8,10,12)에서 강함
- 수성은 모든 별자리에서 점수 획득

**위치:** `src/analysis/strength.rs:560-599`

---

#### 1.4 Total Score 조정

**변경 내용:**

- 총점 계산에 3가지 새로운 구성요소 추가
- "Strong" 임계값: 180 → **240** (3개 구성요소 추가로 최대 점수 증가)
- "Weak" 임계값: 90 → **120**

**위치:** `src/analysis/strength.rs:54-70`

---

### 2. Yoga 시스템 확장 - 부정적 Yoga 추가

#### 2.1 Kemadruma Yoga (케마드루마 요가)

**정의:** 달 주변에 행성이 없어 고립된 상태. 빈곤, 고난, 정서적 고립을 의미.

**구현 내용:**

```rust
YogaCondition::KemadrumaCheck => {
    let moon_pos = chart.planets.iter().find(|p| p.planet == VedicPlanet::Moon)?;
    let moon_rasi = moon_pos.rasi;
    
    // Calculate 2nd and 12th houses from Moon
    let second_from_moon = ((moon_rasi % 12) + 1).max(1);
    let twelfth_from_moon = if moon_rasi == 1 { 12 } else { moon_rasi - 1 };
    
    // Check if any planet (except Sun/Moon) is in 2nd or 12th from Moon
    let has_flanking_planets = chart.planets.iter().any(|p| {
        if p.planet == VedicPlanet::Moon || p.planet == VedicPlanet::Sun {
            return false;
        }
        p.rasi == second_from_moon || p.rasi == twelfth_from_moon
    });
    
    if !has_flanking_planets {
        // Check for cancellation: Moon in Kendra (1,4,7,10)
        let is_in_kendra = [1, 4, 7, 10].contains(&moon_pos.house_index);
        
        if !is_in_kendra {
            Some(vec![VedicPlanet::Moon])  // Kemadruma exists
        } else {
            None  // Cancelled by Kendra position
        }
    } else {
        None
    }
}
```

**BPHS 조건:**

1. 달(Moon)의 2번째 하우스와 12번째 하우스에 행성이 없음
2. 태양과 달 자신은 카운트하지 않음
3. **취소 조건:** 달이 Kendra (1, 4, 7, 10)에 위치하면 취소됨

**위치:** `src/analysis/yogas.rs:591-621`

---

#### 2.2 Vipareeta Raja Yoga (비파리타 라자 요가)

**정의:** "역전의 왕 요가". 흉한 하우스(6, 8, 12)의 주인이 다른 흉한 하우스에 위치하면 흉이 길로 변함.

**구현 내용:**

```rust
YogaCondition::VipareetaRajaYogaCheck => {
    let lagna_rasi = chart.ascendant.rasi;
    let mut vipareeta_planets = Vec::new();
    
    for dusthana_house in [6, 8, 12] {
        let lord = Self::get_lord_of_house(lagna_rasi, dusthana_house);
        
        if let Some(lord_pos) = chart.planets.iter().find(|p| p.planet == lord) {
            let lord_house = lord_pos.house_index;
            
            // Check if lord is in another dusthana house (6, 8, or 12)
            if [6, 8, 12].contains(&lord_house) && lord_house != dusthana_house {
                // Vipareeta Raja Yoga found!
                vipareeta_planets.push(lord);
            }
        }
    }
    
    if !vipareeta_planets.is_empty() {
        vipareeta_planets.sort();
        vipareeta_planets.dedup();
        Some(vipareeta_planets)
    } else {
        None
    }
}
```

**BPHS 조건:**

- 6번 하우스 주인이 8번 또는 12번 하우스에 위치
- 8번 하우스 주인이 6번 또는 12번 하우스에 위치
- 12번 하우스 주인이 6번 또는 8번 하우스에 위치
- **효과:** 장애물이 기회로 변함, 어려움 속에서 성공

**위치:** `src/analysis/yogas.rs:623-651`

---

### 3. 코드 품질 개선

#### 3.1 Ashtakavarga - 사용하지 않는 변수 제거

**파일:** `src/analysis/ashtakavarga.rs:194`

**변경 전:**

```rust
let (has_pts_idx, _other_idx, _other_occ) = if p1 == 0 {
    (r2, r1, occ1)
} else {
    (r1, r2, occ2)
};
```

**변경 후:**

```rust
let has_pts_idx = if p1 == 0 { r2 } else { r1 };
```

**효과:** Clippy 경고 제거, 코드 간결성 향상

---

## 📊 변경 통계

```
커밋: 5ffa427
5 files changed
836 insertions(+)
14 deletions(-)
```

### 수정된 파일

1. `crates/eon-vedic/src/analysis/strength.rs` - Sthana Bala 3개 함수 추가, PlanetStrength 구조체 확장
2. `crates/eon-vedic/src/analysis/yogas.rs` - 2개 부정적 Yoga 추가, YogaType enum 확장
3. `crates/eon-vedic/src/analysis/ashtakavarga.rs` - 불필요한 변수 제거

### 생성된 파일

1. `.agent/artifacts/vedic_shadbala_fixes_completed.md` - 1차 수정 완료 보고서
2. `.agent/artifacts/vedic_shadbala_fixes_plan.md` - 수정 계획 문서

---

## 🔄 전후 비교

### Shadbala Total Score 범위

| 구성요소 | 최대 점수 | 이전 총계 | 현재 총계 |
|---------|---------|---------|----------|
| Uchcha Bala | 60 | ✓ | ✓ |
| Dig Bala | 60 | ✓ | ✓ |
| Chesta Bala | 60 | ✓ | ✓ |
| Naisargika Bala | 60 | ✓ | ✓ |
| Kala Bala | 60 | ✓ | ✓ |
| Drik Bala | ±60 | ✓ | ✓ |
| Paksha Bala | 60 | ✓ | ✓ |
| Ayana Bala | 60 | ✓ | ✓ |
| Saptavargaja Bala | 60 | ✓ | ✓ |
| **Kendra Bala** | **60** | ❌ | **✅** |
| **Drekkana Bala** | **15** | ❌ | **✅** |
| **Ojayugmarasyamsa Bala** | **15** | ❌ | **✅** |
| **총계 (최대)** | - | **~540** | **~630** |

### Yoga 커버리지

| Yoga 유형 | 이전 | 현재 |
|----------|------|------|
| 긍정적 Yoga (Raja, Gaja Kesari 등) | 11개 | 11개 |
| 부정적 Yoga (Kemadruma, Vipareeta) | 0개 | **2개 ✅** |
| **총계** | **11개** | **13개** |

---

## 🎯 BPHS 준수도

### 완료된 항목 ✅

1. ✅ Ishta/Kashta Phala - 기하 평균
2. ✅ Ayana Bala - 황도 경사각 23.45°
3. ✅ Chesta Bala - 7단계 운동 상태
4. ✅ Drik Bala - 임의 스케일링 제거
5. ✅ Tribhaga Bala - 낮/밤 3분할
6. ✅ Kendra Bala - **새 추가**
7. ✅ Drekkana Bala - **새 추가**
8. ✅ Ojayugmarasyamsa Bala - **새 추가**
9. ✅ Parivartana Yoga 분류 (Maha, Khala, Dainya)
10. ✅ Neecha Bhanga - 5가지 취소 조건
11. ✅ Kemadruma Yoga - **새 추가**
12. ✅ Vipareeta Raja Yoga - **새 추가**

### 향후 고려사항

1. ⏳ Yuddha Bala (행성 전쟁) - 1도 이내 근접 시
2. ⏳ Ashtakavarga Ekadhipatya Shodhana - 엣지 케이스 테스트
3. ⏳ 추가 흉 Yoga (Sakata, Shakata 등)
4. ⏳ Panchanga 일출/일몰 정밀도 향상 (대기 굴절 보정)

---

## 🧪 테스트 결과

```bash
$ cargo build --package eon-vedic
   Compiling eon-vedic v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.42s
```

✅ **빌드 성공** - 모든 변경사항이 컴파일 오류 없이 통과

---

## 📚 권장 다음 단계

### 1. 단위 테스트 작성

각 새로운 함수에 대한 테스트:

```rust
#[test]
fn test_kendra_bala() {
    assert_eq!(StrengthEngine::calculate_kendra_bala(1), 60.0);   // Kendra
    assert_eq!(StrengthEngine::calculate_kendra_bala(2), 30.0);   // Panaphara
    assert_eq!(StrengthEngine::calculate_kendra_bala(3), 15.0);   // Apoklima
}

#[test]
fn test_kemadruma_detection() {
    // Moon isolated without flanking planets
    let chart = create_test_chart_with_isolated_moon();
    let yogas = YogaEngine::check_yogas(&chart);
    assert!(yogas.iter().any(|y| y.yoga_type == YogaType::Kemadruma));
}
```

### 2. 통합 테스트

- 실제 출생 차트 데이터로 검증
- 알려진 Kemadruma/Vipareeta 차트와 비교

### 3. 문서화

- 각 함수에 BPHS 참조 페이지 주석 추가
- 예제 계산 과정 문서화

### 4. 성능 최적화

- 반복되는 행성 검색 최적화
- HashMap을 이용한 행성 위치 캐싱 고려

---

## 결론

`eon-vedic` 크레이트는 이제 **BPHS 표준에 훨씬 더 가깝게 부합**하는 종합적인 베딕 점성학 계산 엔진으로 발전했습니다:

✅ **Shadbala** - 12개 구성요소 (이전 9개 → 현재 12개)  
✅ **Yogas** - 13가지 탐지 (이전 11개 → 현재 13개)  
✅ **코드 품질** - 모든 컴파일 경고 해결  
✅ **BPHS 준수** - 12개 주요 계산 표준화 완료  

이제 정통 베딕 점성학 분석을 위한 견고한 기반이 완성되었습니다! 🎉
