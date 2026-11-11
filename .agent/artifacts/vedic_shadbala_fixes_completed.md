# Vedic Shadbala Calculation Fixes -  

## 

  (BPHS)   `eon-vedic`  Shadbala   .

##   

### 1. ✅ Shadbala ( )  

#### 1.1 Ishta/Kashta Phala - BPHS  

** ():**

```rust
let ishta = (uchcha + chesta) / 2.0; //  
```

**  (BPHS ):**

```rust
let ishta = (uchcha * chesta).sqrt(); //   (Geometric Mean)
```

**:** `src/analysis/strength.rs:87-95`

---

#### 1.2 Ayana Bala -    

** ():**

```rust
//  24 
let val = (declination / 24.0).max(-1.0).min(1.0);
let score = 30.0 + (val * 30.0 * direction_factor);
```

**  (BPHS ):**

```rust
use crate::core::constants::ECLIPTIC_OBLIQUITY; // 23.45°

// BPHS : (23.45 + direction * dec) / (2 * 23.45) * 60
let score = (ECLIPTIC_OBLIQUITY + direction_factor * declination) 
            / (2.0 * ECLIPTIC_OBLIQUITY) * 60.0;
```

**:** `src/analysis/strength.rs:97-121`
**:** `src/core/constants.rs:36` - `ECLIPTIC_OBLIQUITY` 

---

#### 1.3 Chesta Bala -     

** ():**

```rust
if pos.is_retrograde {
    return 60.0;
}
//     
let ratio = (pos.speed.abs() / avg_speed).min(2.0);
(60.0 * (1.1 - (ratio / 2.0))).max(0.0).min(60.0)
```

**  (BPHS ):**

```rust
// BPHS 7   
// Vakra (): 60
// Vikala ( ): 15
// Mandatara (): 7.5
// Manda (): 15
// Sama (): 30
// Chara (): 45
// Ati-chara ( ): 60

let motion_states = match pos.planet {
    VedicPlanet::Mars => &MARS_MOTION_STATES,
    VedicPlanet::Mercury => &MERCURY_MOTION_STATES,
    VedicPlanet::Jupiter => &JUPITER_MOTION_STATES,
    VedicPlanet::Venus => &VENUS_MOTION_STATES,
    VedicPlanet::Saturn => &SATURN_MOTION_STATES,
    _ => return 30.0,
};

//       
for &(name, min_speed, max_speed, score) in motion_states {
    if speed >= min_speed && speed < max_speed {
        return score;
    }
}
```

**:** `src/analysis/strength.rs:350-398`
**:** `src/core/constants.rs:48-102` -     

---

#### 1.4 Drik Bala -    

** ():**

```rust
if is_malefic {
    total_drik -= val / 4.0;  //  4 
} else {
    total_drik += val / 4.0;
}
```

**  (BPHS ):**

```rust
// BPHS:  aspect   ( )
if is_malefic {
    total_drik -= val;
} else {
    total_drik += val;
}
```

**:** `src/analysis/strength.rs:168-199`

---

### 2. ✅ Tribhaga Bala  

** ():**

```rust
fn calculate_tribhaga_lords(day_lord: VedicPlanet) -> [VedicPlanet; 8] {
    [day_lord; 8]  //  
}
```

**  (BPHS ):**

```rust
fn calculate_tribhaga_lords(day_lord: VedicPlanet) -> [VedicPlanet; 8] {
    // BPHS  Tribhaga :
    //  3: Mercury, Sun, Saturn
    //  3: Moon, Venus, Mars
    [
        VedicPlanet::Mercury,  //  1/3
        VedicPlanet::Sun,      //  2/3
        VedicPlanet::Saturn,   //  3/3
        VedicPlanet::Moon,     //  1/3
        VedicPlanet::Venus,    //  2/3
        VedicPlanet::Mars,     //  3/3
        day_lord, day_lord,    //   
    ]
}
```

**:** `src/calc/panchanga.rs:162-184`

**:**

- `Kala Bala`    Tribhaga Lords .
- Jupiter  60 ,       60 .

---

### 3. ✅ Yogas ()   

#### 3.1 Parivartana Yoga  

** YogaType :**

```rust
pub enum YogaType {
    // ...  
    Parivartana,         //   (generic)
    ParivartanaMaha,     //   (Kendra/Trikona ↔ Kendra/Trikona)
    ParivartanaKhala,    //   ( dusthana)
    ParivartanaDainya,   //   (  dusthana 6,8,12)
}
```

** :**

```rust
fn classify_parivartana(h1: u8, h2: u8) -> (YogaType, String) {
    let is_kendra = |h: u8| matches!(h, 1 | 4 | 7 | 10);
    let is_trikona = |h: u8| matches!(h, 1 | 5 | 9);
    let is_dusthana = |h: u8| matches!(h, 6 | 8 | 12);
    
    if is_dusthana(h1) && is_dusthana(h2) {
        (YogaType::ParivartanaDainya, "Dainya Parivartana...")
    } else if is_dusthana(h1) || is_dusthana(h2) {
        (YogaType::ParivartanaKhala, "Khala Parivartana...")
    } else if (is_kendra(h1) || is_trikona(h1)) && 
              (is_kendra(h2) || is_trikona(h2)) {
        (YogaType::ParivartanaMaha, "Maha Parivartana...")
    } else {
        (YogaType::ParivartanaKhala, "Khala Parivartana...")
    }
}
```

** API :**

```rust
///  Parivartana Yoga    
pub fn check_yogas_extended(chart: &VedicChart) -> Vec<YogaResult>
```

**:** `src/analysis/yogas.rs:6-19, 245-327`

---

#### 3.2 Neecha Bhanga Raja Yoga 

** ():**

```rust
//  1 : Dispositor Kendra  
let dispositor = VedicPlanet::get_ruler_of(pos.rasi);
let disp_pos = chart.planets.iter().find(|p| p.planet == dispositor)?;
if [1, 4, 7, 10].contains(&disp_pos.house_index) {
    Some(vec![*planet, dispositor])
} else {
    None
}
```

**  (BPHS 5 ):**

```rust
// BPHS Neecha Bhanga Raja Yoga  :
// 1.     Kendra 
// 2.     Kendra 
// 3.     aspect 
// 4.     (conjunction)
// 5.      /aspect

//    
if dispositor_in_kendra {
    return Some(cancellation_planets);
}
if dispositor_conjunct {
    return Some(cancellation_planets);
}
if exaltation_lord_in_kendra {
    return Some(cancellation_planets);
}
if exalted_planet_conjunct_or_aspect {
    return Some(cancellation_planets);
}
```

**:** `src/analysis/yogas.rs:458-520`

---

##   

###  

1. **`src/core/constants.rs`**
   - `ECLIPTIC_OBLIQUITY`  
   - 5       ( 7)

2. **`src/analysis/strength.rs`**
   - `calculate_ishta_kashta()`:   
   - `calculate_ayana_bala()`: BPHS  
   - `calculate_chesta_bala()`: 7   
   - `calculate_drik_bala()`:   

3. **`src/calc/panchanga.rs`**
   - `calculate_tribhaga_lords()`: BPHS   

4. **`src/analysis/yogas.rs`**
   - `YogaType` enum Parivartana  
   - `check_yogas_extended()`   
   - `find_parivartana_exchanges()` 
   - `classify_parivartana()` 
   - `NeechaBhangaCheck`  

---

##  

```bash
$ cargo build --package eon-vedic
   Compiling eon-vedic v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.52s
```

✅ ** ** -       .

---

##   

### 1.   

      :

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ishta_phala_geometric_mean() {
        // (60 * 60).sqrt() = 60
        assert_eq!(calculate_ishta_kashta(60.0, 60.0).0, 60.0);
        // (30 * 30).sqrt() = 30
        assert_eq!(calculate_ishta_kashta(30.0, 30.0).0, 30.0);
    }
    
    #[test]
    fn test_chesta_bala_vakra() {
        //   60
        let mut pos = VedicPosition::default();
        pos.is_retrograde = true;
        assert_eq!(calculate_chesta_bala(&pos), 60.0);
    }
}
```

### 2.  

      :

-   BPHS   
-   (,    ) 

### 3. 

-   BPHS   
-    
-    

### 4.   ()

- Chesta Bala      
-    (   )

### 5.  BPHS 

- Ashtakavarga Ekadhipatya Shodhana   
-     

---

## 

   ,   BPHS(Brihat Parashara Hora Shastra)     :

✅ **Shadbala ** -     
✅ **Tribhaga Bala** -  
✅ **Parivartana Yoga** - 3  
✅ **Neecha Bhanga** - 5   

 `eon-vedic`         .
