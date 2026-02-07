# Vedic Shadbala Calculation Fixes - Implementation Plan

## Overview

정통 베딕 점성학(BPHS) 표준에 맞게 Shadbala 계산 로직을 구현합니다.

## Problems Identified

### 1. Shadbala (행성 강도) 계산 오류

#### 1.1 Chesta Bala (체스타 발라 - 운동 강도)

**현재 문제:**

- 역행이면 무조건 60점 부여
- 평균 속도 대비 비율로 단순 계산

**BPHS 표준:**

- 태양/달: 별도 공식 (Ayana Bala와 연계)
- 5행성 (화성~토성):
  - Vakra (역행): 60
  - Anuvakra (역행 후 정지): 30
  - Vikala (매우 느림): 15
  - Mandatara (느림): 7.5
  - Manda (중간): 15
  - Sama (평균): 30
  - Chara (빠름): 45
  - Ati-chara (매우 빠름): 60
  
**구현 방법:**

```rust
// 속도 기준값 정의
const SPEED_THRESHOLDS: [(f64, &str, f64); 8] = [
    (0.0, "Stationary", 0.0),
    (0.05, "Vakra", 60.0),      // 역행
    (0.1, "Vikala", 15.0),      // 매우 느림
    (0.5, "Mandatara", 7.5),    // 느림
    (1.0, "Manda", 15.0),       // 중간
    (1.5, "Sama", 30.0),        // 평균
    (2.0, "Chara", 45.0),       // 빠름
    (f64::MAX, "Ati-chara", 60.0), // 매우 빠름
];
```

#### 1.2 Ishta Phala (이슈타 팔라 - 길성 점수)

**현재 문제:**

- `(uchcha + chesta) / 2.0` (산술 평균)

**BPHS 표준:**

- `sqrt(uchcha * chesta)` (기하 평균)
- 또는 `(uchcha * chesta * 60) / (uchcha + chesta)` (조화 평균)

**구현:**

```rust
fn calculate_ishta_kashta(uchcha: f64, chesta: f64) -> (f64, f64) {
    // BPHS: Geometric Mean
    let ishta = (uchcha * chesta).sqrt();
    let kashta = (60.0 - ishta).max(0.0);
    (ishta, kashta)
}
```

#### 1.3 Ayana Bala (아야나 발라 - 적위 강도)

**현재 문제:**

- 적위를 24도로 가정
- 선형 비례식 사용

**BPHS 표준:**

- 황도 경사각 23°27' (23.45도) 사용
- 공식: `(23.45° ± Declination) / 46.9° * 60`
- 북쪽 강한 행성 (태양, 화성, 목성, 금성): +
- 남쪽 강한 행성 (달, 토성): -

**구현:**

```rust
const ECLIPTIC_OBLIQUITY: f64 = 23.45; // 23°27'

fn calculate_ayana_bala(planet: VedicPlanet, declination: f64) -> f64 {
    let direction_factor = match planet {
        VedicPlanet::Sun | VedicPlanet::Mars | VedicPlanet::Jupiter | VedicPlanet::Venus => 1.0,
        VedicPlanet::Moon | VedicPlanet::Saturn => -1.0,
        VedicPlanet::Mercury => 0.0, // Neutral
        _ => 0.0,
    };
    
    let score = if direction_factor != 0.0 {
        (ECLIPTIC_OBLIQUITY + direction_factor * declination) / (2.0 * ECLIPTIC_OBLIQUITY) * 60.0
    } else {
        30.0 // Neutral
    };
    
    score.max(0.0).min(60.0)
}
```

#### 1.4 Drik Bala (드릭 발라 - 시각 강도)

**현재 문제:**

- 계산된 값을 4로 나눔 (`val / 4.0`)

**BPHS 표준:**

- Drik Bala는 모든 행성의 aspect 영향을 합산
- Benefic aspects: 양수 가중치
- Malefic aspects: 음수 가중치
- 변환: Rupas = (Drik Bala in degrees) / 60
- 최종 점수는 Virupa (1/60 Rupa) 단위로 표현

**구현:**

```rust
fn calculate_drik_bala(pos: &VedicPosition, chart: &VedicChart) -> f64 {
    let mut total_drik = 0.0;
    
    for aspector in &chart.planets {
        if aspector.planet == pos.planet {
            continue;
        }
        
        let diff = (pos.sidereal_deg - aspector.sidereal_deg + 360.0) % 360.0;
        let aspect_value = Self::get_aspect_value(aspector.planet, diff);
        
        // BPHS: Benefic aspects add, Malefic aspects subtract
        let is_malefic = matches!(
            aspector.planet,
            VedicPlanet::Sun | VedicPlanet::Mars | VedicPlanet::Saturn 
                | VedicPlanet::Rahu | VedicPlanet::Ketu
        );
        
        if is_malefic {
            total_drik -= aspect_value;
        } else {
            total_drik += aspect_value;
        }
    }
    
    // Convert to standard units (Virupas to Shadbala points)
    // No arbitrary division by 4
    total_drik
}
```

### 2. Tribhaga Bala (트리바가 발라) 미구현

**현재 문제:**

- `calculate_tribhaga_lords`가 day_lord를 8번 반복한 배열 반환
- 실제 시간대별 지배 행성 계산 없음

**BPHS 표준:**

- 낮 3부분: Mercury, Sun, Saturn (차례대로)
- 밤 3부분: Moon, Venus, Mars (차례대로)
- Jupiter는 항상 60점

**구현:**

```rust
fn calculate_tribhaga_lords(
    is_day: bool,
    sunrise: DateTime<Utc>,
    sunset: DateTime<Utc>,
    current_time: DateTime<Utc>,
) -> [VedicPlanet; 3] {
    if is_day {
        [VedicPlanet::Mercury, VedicPlanet::Sun, VedicPlanet::Saturn]
    } else {
        [VedicPlanet::Moon, VedicPlanet::Venus, VedicPlanet::Mars]
    }
}

fn get_tribhaga_lord(
    lords: [VedicPlanet; 3],
    start_time: DateTime<Utc>,
    duration_secs: u64,
    current_time: DateTime<Utc>,
) -> VedicPlanet {
    let elapsed = current_time.signed_duration_since(start_time).num_seconds();
    let part_duration = duration_secs / 3;
    let index = (elapsed / part_duration as i64).clamp(0, 2) as usize;
    lords[index]
}
```

### 3. Yogas (요가) 판별 로직

#### 3.1 Parivartana Yoga 분류

**현재 문제:**

- 단순히 교환 여부만 확인

**BPHS 표준:**

- Maha Parivartana: Trikona/Kendra houses 교환
- Khala Parivartana: Dusthana houses 한쪽 포함
- Dainya Parivartana: 6, 8, 12 houses 교환

**구현:**

```rust
enum ParivartanaType {
    Maha,    // Great - Trikona/Kendra exchange
    Khala,   // Mixed - One dusthana
    Dainya,  // Difficult - Both dusthana (6,8,12)
}

fn classify_parivartana(house1: u8, house2: u8) -> ParivartanaType {
    let is_kendra = |h| matches!(h, 1 | 4 | 7 | 10);
    let is_trikona = |h| matches!(h, 1 | 5 | 9);
    let is_dusthana = |h| matches!(h, 6 | 8 | 12);
    
    if is_dusthana(house1) && is_dusthana(house2) {
        ParivartanaType::Dainya
    } else if is_dusthana(house1) || is_dusthana(house2) {
        ParivartanaType::Khala
    } else if (is_kendra(house1) || is_trikona(house1)) 
           && (is_kendra(house2) || is_trikona(house2)) {
        ParivartanaType::Maha
    } else {
        ParivartanaType::Khala
    }
}
```

#### 3.2 Neecha Bhanga Raja Yoga 확장

**현재 문제:**

- 몇 가지 단순 조건만 체크

**BPHS 표준 조건:**

1. Dispositor가 Kendra에 있음
2. Exalted planet이 debilitated planet을 aspect
3. Debilitated planet의 lord가 debilitated planet과 함께 있음
4. Debilitated planet이 자신의 sign에 있음 (특수 케이스)
5. Moon이 paksha bala가 강할 때 (보조 조건)

## Implementation Order

1. ✅ Constants 추가 (`src/constants.rs`)
2. ✅ Ayana Bala 수정
3. ✅ Ishta/Kashta Phala 수정
4. ✅ Chesta Bala 수정
5. ✅ Drik Bala 수정
6. ✅ Tribhaga Bala 구현 (panchanga.rs)
7. ✅ Kala Bala 업데이트 (strength.rs)
8. ✅ Parivartana Yoga 분류 (yogas.rs)
9. ✅ Neecha Bhanga 확장 (yogas.rs)

## Testing Strategy

각 수정 후:

1. 샘플 차트로 값 검증
2. BPHS 참고 값과 비교
3. Edge cases 테스트

## Files to Modify

- `src/constants.rs` - 새로운 상수 추가
- `src/analysis/strength.rs` - Shadbala 계산 수정
- `src/calc/panchanga.rs` - Tribhaga 구현
- `src/analysis/yogas.rs` - Yoga 분류 개선
