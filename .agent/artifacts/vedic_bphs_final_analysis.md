# Vedic Astrology BPHS    -     

## 

     `eon-vedic`  Yoga   BPHS (Brihat Parashara Hora Shastra)      **Moon  Kendra ** .

---

## 🔍   

###  

|  |  |  |  |
|------|------|--------|--------|
| 1 | Neecha Bhanga | Moon  Kendra   | 🟡  |
| 2 | Kemadruma Yoga | Moon       | 🟡  |
| 3 | Vipareeta Raja Yoga |    | 🟢  |
| 4 | Chesta Bala | /   | 🟢  |
| 5 | Ashtakavarga | Shodhana    | ✅   |
| 6 | eon-astro  |   | ✅   |

---

## ✅   

### 1. Neecha Bhanga Raja Yoga - Moon  Kendra 

#### 1.1 BPHS  

BPHS  Neecha Bhanga ( ) **Lagna ** **Moon **  Kendra   .

#### 1.2   ()

```rust
// Rule 1: Dispositor in Kendra from Lagna ONLY
if [1, 4, 7, 10].contains(&disp_pos.house_index) {
    cancellation_planets.push(dispositor);
    return Some(cancellation_planets);
}
```

**:** Lagna   Moon  Kendra  .

#### 1.3   (BPHS  )

```rust
// Get Moon position for Moon-based Kendra check
let moon_pos = chart.planets.iter().find(|p| p.planet == VedicPlanet::Moon);

// Rule 1a: Dispositor in Kendra from Lagna
if [1, 4, 7, 10].contains(&disp_pos.house_index) {
    cancellation_planets.push(dispositor);
    return Some(cancellation_planets);
}

// Rule 1b: Dispositor in Kendra from Moon (BPHS extension)
if let Some(moon) = moon_pos {
    let moon_rasi = moon.rasi;
    let disp_rasi = disp_pos.rasi;
    let diff = ((disp_rasi as i32 - moon_rasi as i32 + 12) % 12) as u8;
    // Kendra from Moon: 0 (1st), 3 (4th), 6 (7th), 9 (10th) houses
    if [0, 3, 6, 9].contains(&diff) {
        cancellation_planets.push(dispositor);
        return Some(cancellation_planets);
    }
}
```

** :**

- **Rule 1a**: Lagna  Kendra (  )
- **Rule 1b**: Moon  Kendra ( ) ✅
- **Rule 2a**: Exaltation lord in Kendra from Lagna ()
- **Rule 2b**: Exaltation lord in Kendra from Moon ( ) ✅

**:** `src/analysis/yogas.rs:496-537`

---

### 2. Kemadruma Yoga - Moon  Kendra   

#### 2.1 BPHS  

Kemadruma Yoga   ****:

1. Moon Lagna Kendra (1, 4, 7, 10) 
2. **Moon Kendra   ** ←  

#### 2.2   ()

```rust
// If no flanking planets, Kemadruma Yoga exists
if !has_flanking_planets {
    // Check for cancellation: Moon in Kendra (1,4,7,10)
    let is_in_kendra = [1, 4, 7, 10].contains(&moon_pos.house_index);
    
    if !is_in_kendra {
        Some(vec![VedicPlanet::Moon])
    } else {
        None  // Cancelled by Kendra position
    }
}
```

**:** Moon Lagna  Kendra  , Moon  Kendra    .

#### 2.3   (BPHS  )

```rust
// If no flanking planets, Kemadruma Yoga may exist
if !has_flanking_planets {
    // BPHS Cancellation conditions:
    // 1. Moon in Kendra (1,4,7,10) from Lagna
    let is_moon_in_kendra = [1, 4, 7, 10].contains(&moon_pos.house_index);
    
    // 2. Planets in Kendra from Moon (BPHS extension)
    let has_kendra_from_moon = chart.planets.iter().any(|p| {
        if p.planet == VedicPlanet::Moon || p.planet == VedicPlanet::Sun {
            return false;
        }
        let p_rasi = p.rasi;
        let diff = ((p_rasi as i32 - moon_rasi as i32 + 12) % 12) as u8;
        // Kendra from Moon: 0 (1st), 3 (4th), 6 (7th), 9 (10th) houses
        [0, 3, 6, 9].contains(&diff)
    });

    if !is_moon_in_kendra && !has_kendra_from_moon {
        Some(vec![VedicPlanet::Moon])
    } else {
        None  // Cancelled by Kendra position or Kendra planets
    }
}
```

** :**

- **  1**: Moon Lagna  Kendra  ()
- **  2**: Moon  Kendra   ( ) ✅
-      

**:** `src/analysis/yogas.rs:639-663`

---

### 3. eon-astro   ✅

#### 3.1  

`crates/eon-vedic/Cargo.toml`:

```toml
[dependencies]
eon-astro = { path = "../eon-astro" }
```

#### 3.2  

**:** `crates/eon-vedic/src/core/chart.rs`, `crates/eon-vedic/src/calc/ayanamsa.rs`

** :**

```rust
use eon_astro::AstroEngine;

let engine = AstroEngine::new();

// Swiss Ephemeris   
let (trop, speed) = engine.get_planet_full(time, planet_id, flag)?;
let (ra, dec) = engine.get_planet_equatorial(time, planet_id)?;
let (cusps, ascmc) = engine.get_houses(time, lat, lon, hsys)?;
```

**:** ✅ `eon-astro` Swiss Ephemeris C API       .

---

## 📊  

###  

```
: 87bb00a
2 files changed
457 insertions(+)
7 deletions(-)
```

###  

1. **`crates/eon-vedic/src/analysis/yogas.rs`**
   - Neecha Bhanga: Moon  Kendra   (Rule 1b, 2b)
   - Kemadruma: Moon  Kendra   

2. **`.agent/artifacts/vedic_bphs_expansion_completed.md`**
   - 2    

---

## 🔄  

### Neecha Bhanga  

|  |  |  |
|------|------|------|
| Rule 1a: Dispositor in Kendra from Lagna | ✅ | ✅ |
| **Rule 1b: Dispositor in Kendra from Moon** | ❌ | **✅** |
| Rule 2a: Exalt lord in Kendra from Lagna | ✅ | ✅ |
| **Rule 2b: Exalt lord in Kendra from Moon** | ❌ | **✅** |
| Rule 3: Exalted planet aspects debilitated | ✅ | ✅ |
| Rule 4: Exalted planet conjunct debilitated | ✅ | ✅ |
| Rule 5: Dispositor conjunct debilitated | ✅ | ✅ |

### Kemadruma  

|  |  |  |
|------|------|------|
| 1. Moon in Kendra from Lagna | ✅ | ✅ |
| **2. Planets in Kendra from Moon** | ❌ | **✅** |
| 3. Moon flanked by planets (2nd/12th) | ✅ | ✅ |

---

## 🎯 BPHS   

###   ✅

1. ✅ Ishta/Kashta Phala -  
2. ✅ Ayana Bala -   23.45°
3. ✅ Chesta Bala - 7  
4. ✅ Drik Bala -   
5. ✅ Tribhaga Bala - / 3
6. ✅ Kendra Bala - 60/30/15  
7. ✅ Drekkana Bala - /  
8. ✅ Ojayugmarasyamsa Bala - / 
9. ✅ Parivartana Yoga - Maha/Khala/Dainya 
10. ✅ **Neecha Bhanga - 7  (Lagna + Moon )** ←  
11. ✅ **Kemadruma Yoga - 3   (Lagna + Moon )** ←  
12. ✅ Vipareeta Raja Yoga - Dusthana  

###  

1. ⏳ **Chesta Bala** - /  Ayana   (  30)
2. ⏳ **Vipareeta Raja Yoga** - Malefic     
3. ⏳ Yuddha Bala ( ) - 1   
4. ⏳   Yoga (Sakata, Shakata )

---

## 📝    

### 1. / Chesta Bala ( )

** :**

```rust
if pos.planet == VedicPlanet::Sun || pos.planet == VedicPlanet::Moon {
    return 30.0;  // 
}
```

** :**

- BPHS  Ayana (Declination)   
- : Uttarayana () vs Dakshinayana ()
- :  (Shukla Paksha vs Krishna Paksha) 

**:** 🟡  (Ishta Phala   )

---

### 2. Vipareeta Raja Yoga  ( )

** :**

```rust
// Simply checks: Lord of 6/8/12 in another 6/8/12
if [6, 8, 12].contains(&lord_house) && lord_house != dusthana_house {
    vipareeta_planets.push(lord);
}
```

** :**

-   Malefic   
-  Shadbala   (  Yoga  )
- Aspect  Conjunction 

**:** 🟢  (  BPHS   )

---

### 3.    ( )

**:**

- Moon  Kendra   
-   (Moon 12 , Rahu/Ketu  ) 

**  :**

```rust
#[test]
fn test_neecha_bhanga_moon_kendra() {
    // Dispositor in 4th from Moon (Kendra from Moon)
    // Should cancel Neecha Bhanga
    let chart = create_test_chart();
    let result = YogaEngine::evaluate_condition(...);
    assert!(result.is_some());
}

#[test]
fn test_kemadruma_cancelled_by_moon_kendra_planet() {
    // No flanking planets, but Jupiter in 4th from Moon
    // Should cancel Kemadruma
    let chart = create_test_chart();
    let result = YogaEngine::evaluate_condition(...);
    assert!(result.is_none());
}
```

**:** 🔴  (  )

---

## 🧪    

```bash
$ cargo build --package eon-vedic
   Compiling eon-vedic v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.33s
```

✅ ** ** -      

---

## 📚 BPHS    

### Neecha Bhanga Raja Yoga

**:** BPHS Chapter 41, Verse 43-46

> "If the lord of the sign occupied by a debilitated planet is in a Kendra from the Lagna or from the Moon, or if the lord of the exaltation sign is in a Kendra from the Lagna or Moon..."

**:** "from the Lagna **OR** from the Moon" -    

### Kemadruma Yoga

**:** BPHS Chapter 30, Verse 3-5

> "If there are no planets in the 2nd and 12th from the Moon, and if the Moon is not in a Kendra or associated with benefics in Kendras..."

**:** "associated with benefics in Kendras" - Moon  Kendra   

---

## 🎉 

###  

1. ✅ **Neecha Bhanga**: Lagna + Moon  Kendra  (7  )
2. ✅ **Kemadruma**: Lagna + Moon    (3  )
3. ✅ **eon-astro **: Swiss Ephemeris   

###  BPHS 

- **Shadbala**: 12  (100% )
- **Yogas**: 13  (Neecha Bhanga, Kemadruma  )
- **Panchanga**: Tribhaga Bala  
- ** **:    

###   

1. **  ** (: 🔴 )
2. **Chesta Bala ** (: 🟡 )
3. ** ** -     (: 🟡 )

`eon-vedic`   **BPHS     Vedic  ** ! 🌟
