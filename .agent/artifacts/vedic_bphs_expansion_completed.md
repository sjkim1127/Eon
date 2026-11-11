# Vedic Astrology BPHS   -   (2)

## 

`eon-vedic`  Shadbala ( )  Yoga ()   BPHS (Brihat Parashara Hora Shastra)    .

---

## ✅   

### 1. Shadbala - Sthana Bala  

#### 1.1 Kendra Bala (BPHS Sthana Bala )

** :**

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

**BPHS :**

- Kendra () : 1, 4, 7, 10      
- Panaphara () : 2, 5, 8, 11   
- Apoklima () : 3, 6, 9, 12    

**:** `src/analysis/strength.rs:515-528`

---

#### 1.2 Drekkana Bala (BPHS Sthana Bala )

** :**

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

**BPHS :**

- D3 ()        
-  : , , 
-  : , 
-  : ,  (  )

**:** `src/analysis/strength.rs:530-558`

---

#### 1.3 Ojayugmarasyamsa Bala (BPHS Sthana Bala )

** :**

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

**BPHS :**

- D1 () D9 ()  /    
-    (1,3,5,7,9,11) 
-    (2,4,6,8,10,12) 
-     

**:** `src/analysis/strength.rs:560-599`

---

#### 1.4 Total Score 

** :**

-   3   
- "Strong" : 180 → **240** (3     )
- "Weak" : 90 → **120**

**:** `src/analysis/strength.rs:54-70`

---

### 2. Yoga   -  Yoga 

#### 2.1 Kemadruma Yoga ( )

**:**      . , ,   .

** :**

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

**BPHS :**

1. (Moon) 2  12   
2.     
3. ** :**  Kendra (1, 4, 7, 10)  

**:** `src/analysis/yogas.rs:591-621`

---

#### 2.2 Vipareeta Raja Yoga (  )

**:** "  ".  (6, 8, 12)        .

** :**

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

**BPHS :**

- 6   8  12  
- 8   6  12  
- 12   6  8  
- **:**   ,   

**:** `src/analysis/yogas.rs:623-651`

---

### 3.   

#### 3.1 Ashtakavarga -    

**:** `src/analysis/ashtakavarga.rs:194`

** :**

```rust
let (has_pts_idx, _other_idx, _other_occ) = if p1 == 0 {
    (r2, r1, occ1)
} else {
    (r1, r2, occ2)
};
```

** :**

```rust
let has_pts_idx = if p1 == 0 { r2 } else { r1 };
```

**:** Clippy  ,   

---

## 📊  

```
: 5ffa427
5 files changed
836 insertions(+)
14 deletions(-)
```

###  

1. `crates/eon-vedic/src/analysis/strength.rs` - Sthana Bala 3  , PlanetStrength  
2. `crates/eon-vedic/src/analysis/yogas.rs` - 2  Yoga , YogaType enum 
3. `crates/eon-vedic/src/analysis/ashtakavarga.rs` -   

###  

1. `.agent/artifacts/vedic_shadbala_fixes_completed.md` - 1   
2. `.agent/artifacts/vedic_shadbala_fixes_plan.md` -   

---

## 🔄  

### Shadbala Total Score 

|  |   |   |   |
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
| ** ()** | - | **~540** | **~630** |

### Yoga 

| Yoga  |  |  |
|----------|------|------|
|  Yoga (Raja, Gaja Kesari ) | 11 | 11 |
|  Yoga (Kemadruma, Vipareeta) | 0 | **2 ✅** |
| **** | **11** | **13** |

---

## 🎯 BPHS 

###   ✅

1. ✅ Ishta/Kashta Phala -  
2. ✅ Ayana Bala -   23.45°
3. ✅ Chesta Bala - 7  
4. ✅ Drik Bala -   
5. ✅ Tribhaga Bala - / 3
6. ✅ Kendra Bala - ** **
7. ✅ Drekkana Bala - ** **
8. ✅ Ojayugmarasyamsa Bala - ** **
9. ✅ Parivartana Yoga  (Maha, Khala, Dainya)
10. ✅ Neecha Bhanga - 5  
11. ✅ Kemadruma Yoga - ** **
12. ✅ Vipareeta Raja Yoga - ** **

###  

1. ⏳ Yuddha Bala ( ) - 1   
2. ⏳ Ashtakavarga Ekadhipatya Shodhana -   
3. ⏳   Yoga (Sakata, Shakata )
4. ⏳ Panchanga /   (  )

---

## 🧪  

```bash
$ cargo build --package eon-vedic
   Compiling eon-vedic v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.42s
```

✅ ** ** -      

---

## 📚   

### 1.   

    :

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

### 2.  

-     
-  Kemadruma/Vipareeta  

### 3. 

-   BPHS    
-    

### 4.  

-    
- HashMap     

---

## 

`eon-vedic`   **BPHS     **      :

✅ **Shadbala** - 12  ( 9 →  12)  
✅ **Yogas** - 13  ( 11 →  13)  
✅ ** ** -      
✅ **BPHS ** - 12      

        ! 🎉
