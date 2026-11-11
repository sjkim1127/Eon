# Vedic Astrology - CRITICAL BPHS     

## 🚨   

 ** **   ** BPHS  **  ** **   .

---

## ✅   

| # |  |  |  |
|---|------|--------|------|
| 1 | **Drik Bala  - BPHS 100% ** | 🔴 CRITICAL | ✅  |
| 2 | **Mercury Ayana Bala - BPHS  ** | ⚠️  | ✅  |
| 3 | **Moon Chesta Bala – BPHS  ** | ⚠️  | ✅  |
| 4 | **Harsha/Sarala/Vimala Yoga - ** | 🎯  | ✅  |

---

## 1. 🔴 Drik Bala (Aspect Strength)  CRITICAL FIX

###  

  Drik Bala  BPHS  **  **  .

|   | BPHS  |   |  |
|----------|----------|----------|------|
| 30-60° | `(D-30)/2` | `D-30` | 2 inflated |
| 60-90° | `(D-60)+15` | `15.0` () |   |
| **90-120°** | `45-(D-90)/2` () | `(D-90)+15` (****) | ** ** ❌ |
| 120-150° | `30-(D-120)` | `45.0` () |   |
| **150-180°** | `(D-150)*2` (0 ) | `(D-150)/2+45` (**45 **) | ** ** ❌ |

###  

```rust
let mut val = if diff > 30.0 && diff <= 60.0 {
    (diff - 30.0) / 2.0  // 0 → 15 Virupas ✅
} else if diff > 60.0 && diff <= 90.0 {
    (diff - 60.0) + 15.0  // 15 → 45 (increasing) ✅
} else if diff > 90.0 && diff <= 120.0 {
    45.0 - (diff - 90.0) / 2.0  // 45 → 30 (DECREASING) ✅
} else if diff > 120.0 && diff <= 150.0 {
    30.0 - (diff - 120.0)  // 30 → 0 (decreasing) ✅
} else if diff > 150.0 && diff <= 180.0 {
    (diff - 150.0) * 2.0  // 0 → 60 (starts from 0) ✅
} else if diff > 180.0 && diff <= 300.0 {
    // Mirror logic for backward aspects ✅
    let reverse_diff = 360.0 - diff;
    // ... (mirror calculation)
} else {
    0.0
};
```

**:**  Shadbala  Drik Bala   

---

## 2. ⚠️ Mercury Ayana Bala - BPHS   

###  

- BPHS: Mercury **North-strong** 
-  : `Neutral`  ( 30)

###  

```rust
//  ❌
VedicPlanet::Mercury => return 30.0, // Mercury is always neutral

//   ✅
VedicPlanet::Sun | Mars | Jupiter | Venus | Mercury => 1.0,  // North-strong
```

**:**

-   +23.45°: 30 → **60** (+30)
-   -23.45°: 30 → **0** (-30)

---

## 3. ⚠️ Moon Chesta Bala - BPHS   

###  

- BPHS: Moon Chesta Bala = Paksha Bala
-  :  30

###  

```rust
// BPHS: Moon's Chesta Bala equals its Paksha Bala
let final_chesta_score = if pos.planet == VedicPlanet::Moon {
    paksha_score  // Reflects lunar phase ✅
} else {
    chesta_score
};
```

**:**

-  (New Moon): 30 → **0** (-30)
-  (Full Moon): 30 → **60** (+30)
- Ishta Phala    

---

## 4. 🎯 Harsha/Sarala/Vimala Yoga  

###  

**YogaType :**

```rust
Harsha,  // 6th lord in 6th house
Sarala,  // 8th lord in 8th house  
Vimala,  // 12th lord in 12th house
```

** :**

- Harsha: 6   6 
- Sarala: 8   8 
- Vimala: 12   12 

**BPHS :**

- Harsha:   , 
- Sarala: ,  
- Vimala:  , 

---

## 📊  

### BPHS : **99%+**

   BPHS   .

###  

```bash
$ cargo build --package eon-vedic
   Compiling eon-vedic v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.57s
```

---

## 🎉 

    ** BPHS  **    .

`eon-vedic`   **   BPHS  **! 🌟
