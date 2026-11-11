# Vedic Shadbala Calculation Fixes - Implementation Plan

## Overview

  (BPHS)   Shadbala   .

## Problems Identified

### 1. Shadbala ( )  

#### 1.1 Chesta Bala (  -  )

** :**

-   60 
-      

**BPHS :**

- /:   (Ayana Bala )
- 5 (~):
  - Vakra (): 60
  - Anuvakra (  ): 30
  - Vikala ( ): 15
  - Mandatara (): 7.5
  - Manda (): 15
  - Sama (): 30
  - Chara (): 45
  - Ati-chara ( ): 60
  
** :**

```rust
//   
const SPEED_THRESHOLDS: [(f64, &str, f64); 8] = [
    (0.0, "Stationary", 0.0),
    (0.05, "Vakra", 60.0),      // 
    (0.1, "Vikala", 15.0),      //  
    (0.5, "Mandatara", 7.5),    // 
    (1.0, "Manda", 15.0),       // 
    (1.5, "Sama", 30.0),        // 
    (2.0, "Chara", 45.0),       // 
    (f64::MAX, "Ati-chara", 60.0), //  
];
```

#### 1.2 Ishta Phala (  -  )

** :**

- `(uchcha + chesta) / 2.0` ( )

**BPHS :**

- `sqrt(uchcha * chesta)` ( )
-  `(uchcha * chesta * 60) / (uchcha + chesta)` ( )

**:**

```rust
fn calculate_ishta_kashta(uchcha: f64, chesta: f64) -> (f64, f64) {
    // BPHS: Geometric Mean
    let ishta = (uchcha * chesta).sqrt();
    let kashta = (60.0 - ishta).max(0.0);
    (ishta, kashta)
}
```

#### 1.3 Ayana Bala (  -  )

** :**

-  24 
-   

**BPHS :**

-   23°27' (23.45) 
- : `(23.45° ± Declination) / 46.9° * 60`
-    (, , , ): +
-    (, ): -

**:**

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

#### 1.4 Drik Bala (  -  )

** :**

-   4  (`val / 4.0`)

**BPHS :**

- Drik Bala   aspect  
- Benefic aspects:  
- Malefic aspects:  
- : Rupas = (Drik Bala in degrees) / 60
-   Virupa (1/60 Rupa)  

**:**

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

### 2. Tribhaga Bala ( ) 

** :**

- `calculate_tribhaga_lords` day_lord 8   
-      

**BPHS :**

-  3: Mercury, Sun, Saturn ()
-  3: Moon, Venus, Mars ()
- Jupiter  60

**:**

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

### 3. Yogas ()  

#### 3.1 Parivartana Yoga 

** :**

-    

**BPHS :**

- Maha Parivartana: Trikona/Kendra houses 
- Khala Parivartana: Dusthana houses  
- Dainya Parivartana: 6, 8, 12 houses 

**:**

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

#### 3.2 Neecha Bhanga Raja Yoga 

** :**

-     

**BPHS  :**

1. Dispositor Kendra 
2. Exalted planet debilitated planet aspect
3. Debilitated planet lord debilitated planet  
4. Debilitated planet  sign  ( )
5. Moon paksha bala   ( )

## Implementation Order

1. ✅ Constants  (`src/constants.rs`)
2. ✅ Ayana Bala 
3. ✅ Ishta/Kashta Phala 
4. ✅ Chesta Bala 
5. ✅ Drik Bala 
6. ✅ Tribhaga Bala  (panchanga.rs)
7. ✅ Kala Bala  (strength.rs)
8. ✅ Parivartana Yoga  (yogas.rs)
9. ✅ Neecha Bhanga  (yogas.rs)

## Testing Strategy

  :

1.    
2. BPHS   
3. Edge cases 

## Files to Modify

- `src/constants.rs` -   
- `src/analysis/strength.rs` - Shadbala  
- `src/calc/panchanga.rs` - Tribhaga 
- `src/analysis/yogas.rs` - Yoga  
