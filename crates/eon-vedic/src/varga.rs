use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VargaType {
    D1,   // Rasi
    D2,   // Hora
    D3,   // Drekkana
    D4,   // Chaturthamsha
    D7,   // Saptamsa
    D9,   // Navamsa
    D10,  // Dasamsa
    D12,  // Dwadasamsa
    D16,  // Shodashamsa
    D20,  // Vimsamsa
    D24,  // Chaturvimshamsha
    D27,  // Saptavimsamsa
    D30,  // Trimsamsa
    D40,  // Khavedamsa
    D45,  // Akshavedamsa
    D60,  // Shashtyamsa
}

impl VargaType {
    pub fn calculate_rasi(&self, longitude: f64) -> u8 {
        match self {
            Self::D1 => ((longitude / 30.0).floor() as u8 % 12) + 1,
            Self::D2 => calculate_hora(longitude),
            Self::D3 => calculate_drekkana(longitude),
            Self::D4 => calculate_chaturthamsha(longitude),
            Self::D7 => calculate_saptamsa(longitude),
            Self::D9 => calculate_navamsa(longitude),
            Self::D10 => calculate_dasamsa(longitude),
            Self::D12 => calculate_dwadasamsa(longitude),
            _ => 1, // Default fallback
        }
    }
    
    // ... division_count implementation remains the same ...
    pub fn division_count(&self) -> u32 {
        match self {
            Self::D1 => 1,
            Self::D2 => 2,
            Self::D3 => 3,
            Self::D4 => 4,
            Self::D7 => 7,
            Self::D9 => 9,
            Self::D10 => 10,
            Self::D12 => 12,
            Self::D16 => 16,
            Self::D20 => 20,
            Self::D24 => 24,
            Self::D27 => 27,
            Self::D30 => 30,
            Self::D40 => 40,
            Self::D45 => 45,
            Self::D60 => 60,
        }
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
