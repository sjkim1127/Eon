use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VargaType {
    D1,   // Rasi
    D2,   // Hora
    D3,   // Drekkana
    D4,   // Chaturthamsha
    D5,   // Panchamsa
    D7,   // Saptamsa
    D8,   // Ashtamsa
    D9,   // Navamsa
    D10,  // Dasamsa
    D11,  // Rudramsa
    D12,  // Dwadasamsa
    D16,  // Shodashamsa
    D20,  // Vimsamsa
    D24,  // Chaturvimshamsha
    D27,  // Saptavimsamsa
    D30,  // Trimsamsa
    D40,  // Khavedamsa
    D45,  // Akshavedamsa
    D60,  // Shashtyamsa
    D81,  // Nava-Navamsa (D9 of D9)
    D108, // Ashtottaramsa (D9 of D12)
    D144, // Dwadas-Dwadasamsa (D12 of D12)
}

impl VargaType {
    pub fn calculate_rasi(&self, longitude: f64) -> u8 {
        match self {
            Self::D1 => ((longitude / 30.0).floor() as u8 % 12) + 1,
            Self::D2 => calculate_hora(longitude),
            Self::D3 => calculate_drekkana(longitude),
            Self::D4 => calculate_chaturthamsha(longitude),
            Self::D5 => calculate_panchamsa(longitude),
            Self::D7 => calculate_saptamsa(longitude),
            Self::D8 => calculate_ashtamsa(longitude),
            Self::D9 => calculate_navamsa(longitude),
            Self::D10 => calculate_dasamsa(longitude),
            Self::D11 => calculate_rudramsa(longitude),
            Self::D12 => calculate_dwadasamsa(longitude),
            Self::D16 => calculate_shodashamsa(longitude),
            Self::D20 => calculate_vimsamsa(longitude),
            Self::D24 => calculate_chaturvimshamsa(longitude),
            Self::D27 => calculate_saptavimsamsa(longitude),
            Self::D30 => calculate_trimsamsa(longitude),
            Self::D40 => calculate_khavedamsa(longitude),
            Self::D45 => calculate_akshavedamsa(longitude),
            Self::D60 => calculate_shashtyamsa(longitude),
            Self::D81 => {
                let d9_rasi = Self::D9.calculate_rasi(longitude);
                let fake_long = (d9_rasi as f64 - 1.0) * 30.0 + 15.0; // Center of rasi
                Self::D9.calculate_rasi(fake_long)
            },
            Self::D108 => {
                let d12_rasi = Self::D12.calculate_rasi(longitude);
                let fake_long = (d12_rasi as f64 - 1.0) * 30.0 + 15.0;
                Self::D9.calculate_rasi(fake_long)
            },
            Self::D144 => {
                let d12_rasi = Self::D12.calculate_rasi(longitude);
                let fake_long = (d12_rasi as f64 - 1.0) * 30.0 + 15.0;
                Self::D12.calculate_rasi(fake_long)
            },
        }
    }
    
    // ... division_count implementation remains the same ...
    pub fn division_count(&self) -> u32 {
        match self {
            Self::D1 => 1, Self::D2 => 2, Self::D3 => 3, Self::D4 => 4,
            Self::D5 => 5, Self::D7 => 7, Self::D8 => 8, Self::D9 => 9,
            Self::D10 => 10, Self::D11 => 11, Self::D12 => 12, Self::D16 => 16,
            Self::D20 => 20, Self::D24 => 24, Self::D27 => 27, Self::D30 => 30,
            Self::D40 => 40, Self::D45 => 45, Self::D60 => 60,
            Self::D81 => 81, Self::D108 => 108, Self::D144 => 144,
        }
    }

    /// Effective longitude (0..360) for nakshatra calculation in a varga chart.
    /// D1: use sidereal directly. Other vargas: project degree within division onto full sign.
    pub fn effective_longitude_for_nakshatra(
        &self,
        sidereal_deg: f64,
        varga_rasi: u8,
    ) -> f64 {
        if self.division_count() <= 1 {
            return (sidereal_deg % 360.0 + 360.0) % 360.0;
        }
        let deg = (sidereal_deg % 360.0 + 360.0) % 360.0;
        let sign_degree = deg % 30.0;
        let division_size = 30.0 / self.division_count() as f64;
        let degree_in_division = sign_degree % division_size;
        let scaled_degree = degree_in_division * self.division_count() as f64;
        ((varga_rasi as f64 - 1.0) * 30.0 + scaled_degree) % 360.0
    }

}

/// Helper: Get 0-based sign index (0=Aries, 11=Pisces)
fn get_sign_idx(longitude: f64) -> u8 {
    (longitude / 30.0).floor() as u8
}

/// Helper: Get element (0=Fire, 1=Earth, 2=Air, 3=Water)
fn get_element(sign_idx: u8) -> u8 {
    sign_idx % 4
}

/// Helper: Is odd sign? (Aries=0(Odd for Algo), Taurus=1(Even))
/// In calculation: 0(Arie)=Odd, 1(Tau)=Even.
/// Wait, standard: 1(Aries) is Odd.
fn is_odd_sign(sign_idx: u8) -> bool {
    // sign_idx 0 (Aries) -> 1st sign -> Odd
    // sign_idx 1 (Taurus) -> 2nd sign -> Even
    (sign_idx + 1) % 2 != 0
}

/// D2 (Hora) - Wealth
/// Parashara Hora:
/// Odd Signs: 1st half = Sun (Leo=5), 2nd half = Moon (Cancer=4)
/// Even Signs: 1st half = Moon (Cancer=4), 2nd half = Sun (Leo=5)
fn calculate_hora(longitude: f64) -> u8 {
    let sign_degree = longitude % 30.0;
    let sign_idx = get_sign_idx(longitude);
    let is_first_half = sign_degree < 15.0;
    
    if is_odd_sign(sign_idx) {
        if is_first_half { 5 } else { 4 } // Leo (5) / Cancer (4)
    } else {
        if is_first_half { 4 } else { 5 } // Cancer (4) / Leo (5)
    }
}

/// D3 (Drekkana) - Siblings
/// Tritaamsa
/// 1st part (0-10): Sign itself
/// 2nd part (10-20): 5th from Sign
/// 3rd part (20-30): 9th from Sign
fn calculate_drekkana(longitude: f64) -> u8 {
    let sign_degree = longitude % 30.0;
    let sign_idx = get_sign_idx(longitude);
    
    let offset = if sign_degree < 10.0 {
        0
    } else if sign_degree < 20.0 {
        4 // 5th sign = idx + 4
    } else {
        8 // 9th sign = idx + 8
    };
    
    (sign_idx + offset) % 12 + 1
}

/// D4 (Chaturthamsha) - Fortune
/// 1st part: Sign itself
/// 2nd part: 4th from Sign
/// 3rd part: 7th from Sign
/// 4th part: 10th from Sign
fn calculate_chaturthamsha(longitude: f64) -> u8 {
    let sign_degree = longitude % 30.0;
    let sign_idx = get_sign_idx(longitude);
    let part = (sign_degree / (30.0 / 4.0)).floor() as u8; // 0..3
    
    let offset = match part {
        0 => 0,
        1 => 3, // 4th
        2 => 6, // 7th
        3 => 9, // 10th
        _ => 0,
    };
    
    (sign_idx + offset) % 12 + 1
}

/// D7 (Saptamsa) - Children
/// Odd Sign: Count from Sign itself
/// Even Sign: Count from 7th from Sign
fn calculate_saptamsa(longitude: f64) -> u8 {
    let sign_degree = longitude % 30.0;
    let sign_idx = get_sign_idx(longitude);
    let part = (sign_degree / (30.0 / 7.0)).floor() as u8; // 0..6
    
    let start_sign_idx = if is_odd_sign(sign_idx) {
        sign_idx
    } else {
        (sign_idx + 6) % 12 // 7th from sign
    };
    
    (start_sign_idx + part) % 12 + 1
}

/// D9 (Navamsa) - Spouse, Dharma
fn calculate_navamsa(longitude: f64) -> u8 {
    let d1_rasi_idx = get_sign_idx(longitude);
    let sign_degree = longitude % 30.0;
    
    let pada_idx = (sign_degree / (30.0 / 9.0)).floor() as u8; // 0~8
    let element = get_element(d1_rasi_idx);
    
    let start_sign = match element {
        0 => 0, // Fire -> Aries
        1 => 9, // Earth -> Capricorn
        2 => 6, // Air -> Libra
        3 => 3, // Water -> Cancer
        _ => unreachable!(),
    };
    
    (start_sign + pada_idx) % 12 + 1
}

/// D10 (Dasamsa) - Career
/// Odd Sign: Count from Sign itself
/// Even Sign: Count from 9th from Sign
fn calculate_dasamsa(longitude: f64) -> u8 {
    let sign_degree = longitude % 30.0;
    let sign_idx = get_sign_idx(longitude);
    let part = (sign_degree / (30.0 / 10.0)).floor() as u8; // 0..9
    
    let start_sign_idx = if is_odd_sign(sign_idx) {
        sign_idx
    } else {
        (sign_idx + 8) % 12 // 9th from sign
    };
    
    (start_sign_idx + part) % 12 + 1
}

/// D12 (Dwadasamsa) - Parents
/// Starts from Sign itself
fn calculate_dwadasamsa(longitude: f64) -> u8 {
    let sign_degree = longitude % 30.0;
    let sign_idx = get_sign_idx(longitude);
    let part = (sign_degree / (30.0 / 12.0)).floor() as u8; // 0..11
    
    (sign_idx + part) % 12 + 1
}

/// D5 (Panchamsa) - Fame, Spiritual Power
/// Odd signs: Aries, Taurus, ... (Start from sign 1)
/// Even signs: Libra, Scorpio, ... (Start from sign 7)
fn calculate_panchamsa(longitude: f64) -> u8 {
    let sign_degree = longitude % 30.0;
    let sign_idx = get_sign_idx(longitude);
    let part = (sign_degree / 6.0).floor() as u8; // 0..4
    
    let start_sign = if is_odd_sign(sign_idx) { 0 } else { 6 };
    (start_sign + part) % 12 + 1
}

/// D8 (Ashtamsa) - Longevity, Sudden events
/// Movable signs (Ar/Cn/Li/Cp): Aries (0 offset)
/// Fixed signs (Ta/Le/Sc/Aq): Sagittarius (8 offset)
/// Dual signs (Ge/Vi/Sg/Pi): Leo (4 offset)
fn calculate_ashtamsa(longitude: f64) -> u8 {
    let sign_degree = longitude % 30.0;
    let sign_idx = get_sign_idx(longitude);
    let part = (sign_degree / 3.75).floor() as u8; // 0..7
    
    let offset = match sign_idx % 3 {
        0 => 0, // Movable: Aries
        1 => 8, // Fixed: Sagittarius
        2 => 4, // Dual: Leo
        _ => unreachable!(),
    };
    
    (offset + part) % 12 + 1
}

/// D11 (Rudramsa) - Success, Victory/Defeat
/// Count from sign in reverse order
fn calculate_rudramsa(longitude: f64) -> u8 {
    let sign_degree = longitude % 30.0;
    let sign_idx = get_sign_idx(longitude);
    let part = (sign_degree / (30.0 / 11.0)).floor() as i32; // 0..10
    
    // Logic: Start from sign_idx, go backwards part times
    let mut target = sign_idx as i32 - part;
    while target < 0 { target += 12; }
    
    (target as u8) % 12 + 1
}

/// D16 (Shodashamsa) - Conveyances, General Happiness
/// Movable: Aries
/// Fixed: Leo
/// Dual: Sagittarius
fn calculate_shodashamsa(longitude: f64) -> u8 {
    let sign_degree = longitude % 30.0;
    let sign_idx = get_sign_idx(longitude);
    let part = (sign_degree / (30.0 / 16.0)).floor() as u8; // 0..15
    
    let start_sign = match sign_idx % 3 {
        0 => 0, // Movable: Aries
        1 => 4, // Fixed: Leo
        2 => 8, // Dual: Sagittarius
        _ => unreachable!(),
    };
    
    (start_sign + part) % 12 + 1
}

/// D20 (Vimsamsa) - Spiritual activities
/// Movable: Aries
/// Fixed: Sagittarius
/// Dual: Leo
fn calculate_vimsamsa(longitude: f64) -> u8 {
     let sign_degree = longitude % 30.0;
     let sign_idx = get_sign_idx(longitude);
     let part = (sign_degree / 1.5).floor() as u8; // 0..19
     
     let start_sign = match sign_idx % 3 {
        0 => 0, // Movable: Aries
        1 => 8, // Fixed: Sagittarius
        2 => 4, // Dual: Leo
        _ => unreachable!(),
    };
    
    (start_sign + part) % 12 + 1
}

/// D24 (Chaturvimshamsa) - Education, Knowledge
/// Odd: Leo (4)
/// Even: Cancer (3)
fn calculate_chaturvimshamsa(longitude: f64) -> u8 {
    let sign_degree = longitude % 30.0;
    let sign_idx = get_sign_idx(longitude);
    let part = (sign_degree / 1.25).floor() as u8; // 0..23
    
    let start_sign = if is_odd_sign(sign_idx) { 4 } else { 3 };
    (start_sign + part) % 12 + 1
}

/// D27 (Saptavimsamsa) - General Strength
/// Fire (0/4/8): Aries (0)
/// Earth (1/5/9): Cancer (3)
/// Air (2/6/10): Libra (6)
/// Water (3/7/11): Capricorn (9)
fn calculate_saptavimsamsa(longitude: f64) -> u8 {
    let sign_degree = longitude % 30.0;
    let sign_idx = get_sign_idx(longitude);
    let part = (sign_degree / (30.0 / 27.0)).floor() as u8; // 0..26
    
    let start_sign = match get_element(sign_idx) {
        0 => 0,
        1 => 3,
        2 => 6,
        3 => 9,
        _ => unreachable!(),
    };
    
    (start_sign + part) % 12 + 1
}

/// D30 (Trimsamsa) - Evils, Danger
/// Unequal divisions. 
/// Odd signs: 5:Mars, 5:Saturn, 8:Jupiter, 7:Mercury, 5:Venus
/// Even signs: 5:Venus, 7:Mercury, 8:Jupiter, 5:Saturn, 5:Mars
fn calculate_trimsamsa(longitude: f64) -> u8 {
    let sign_degree = longitude % 30.0;
    let sign_idx = get_sign_idx(longitude);
    
    if is_odd_sign(sign_idx) {
        if sign_degree < 5.0 { 1 } // Mars (Aries)
        else if sign_degree < 10.0 { 11 } // Saturn (Aquarius)
        else if sign_degree < 18.0 { 9 } // Jupiter (Sagittarius)
        else if sign_degree < 25.0 { 3 } // Mercury (Gemini)
        else { 7 } // Venus (Libra)
    } else {
        if sign_degree < 5.0 { 2 } // Venus (Taurus)
        else if sign_degree < 12.0 { 6 } // Mercury (Virgo)
        else if sign_degree < 20.0 { 12 } // Jupiter (Pisces)
        else if sign_degree < 25.0 { 10 } // Saturn (Capricorn)
        else { 8 } // Mars (Scorpio)
    }
}

/// D40 (Khavedamsa) - Auspicious/Inauspicious
/// Odd: Aries (0)
/// Even: Libra (6)
fn calculate_khavedamsa(longitude: f64) -> u8 {
    let sign_degree = longitude % 30.0;
    let sign_idx = get_sign_idx(longitude);
    let part = (sign_degree / 0.75).floor() as u8; // 0..39
    
    let start_sign = if is_odd_sign(sign_idx) { 0 } else { 6 };
    (start_sign + part) % 12 + 1
}

/// D45 (Akshavedamsa) - Character, General Welfare
/// Movable: Aries
/// Fixed: Leo
/// Dual: Sagittarius
fn calculate_akshavedamsa(longitude: f64) -> u8 {
    let sign_degree = longitude % 30.0;
    let sign_idx = get_sign_idx(longitude);
    let part = (sign_degree / (30.0 / 45.0)).floor() as u8; // 0..44
    
    let start_sign = match sign_idx % 3 {
        0 => 0,
        1 => 4,
        2 => 8,
        _ => unreachable!(),
    };
    
    (start_sign + part) % 12 + 1
}

/// D60 (Shashtyamsa) - General details, Past life
/// 0.5 degrees per division. Start from sign occupied.
fn calculate_shashtyamsa(longitude: f64) -> u8 {
    let sign_degree = longitude % 30.0;
    let sign_idx = get_sign_idx(longitude);
    let part = (sign_degree * 2.0).floor() as u8; // 0..59
    
    (sign_idx + part) % 12 + 1
}
