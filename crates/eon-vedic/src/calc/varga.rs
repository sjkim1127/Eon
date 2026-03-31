use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VargaType {
    D1,   // Rasi
    D2,   // Hora
    D3,   // Drekkana
    D4,   // Chaturthamsha
    D5,   // Panchamsa
    D6,   // Shashtamsa
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
            Self::D6 => calculate_shashtamsa(longitude),
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
                // D9 of D9 (Nava-Navamsa)
                // Harmonic equivalent: (longitude * 81) mod 360
                let harmonic_long = (longitude * 81.0) % 360.0;
                ((harmonic_long / 30.0).floor() as u8 % 12) + 1
            },
            Self::D108 => {
                // D9 of D12 (Ashtottaramsa)
                // Harmonic equivalent: (longitude * 108) mod 360
                let harmonic_long = (longitude * 108.0) % 360.0;
                ((harmonic_long / 30.0).floor() as u8 % 12) + 1
            },
            Self::D144 => {
                // D12 of D12 (Dwadas-Dwadasamsa)
                // Harmonic equivalent: (longitude * 144) mod 360
                let harmonic_long = (longitude * 144.0) % 360.0;
                ((harmonic_long / 30.0).floor() as u8 % 12) + 1
            },
        }
    }
    
    pub fn division_count(&self) -> u32 {
        match self {
            Self::D1 => 1, Self::D2 => 2, Self::D3 => 3, Self::D4 => 4,
            Self::D5 => 5, Self::D6 => 6, Self::D7 => 7, Self::D8 => 8, Self::D9 => 9,
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

/// Helper: Is odd sign?
fn is_odd_sign(sign_idx: u8) -> bool {
    (sign_idx + 1) % 2 != 0
}

fn calculate_hora(longitude: f64) -> u8 {
    let sign_degree = longitude % 30.0;
    let sign_idx = get_sign_idx(longitude);
    let is_first_half = sign_degree < 15.0;
    
    if is_odd_sign(sign_idx) {
        if is_first_half { 5 } else { 4 } // Leo / Cancer
    } else if is_first_half { 4 } else { 5 }
}

fn calculate_drekkana(longitude: f64) -> u8 {
    let sign_degree = longitude % 30.0;
    let sign_idx = get_sign_idx(longitude);
    let offset = if sign_degree < 10.0 { 0 } else if sign_degree < 20.0 { 4 } else { 8 };
    (sign_idx + offset) % 12 + 1
}

fn calculate_chaturthamsha(longitude: f64) -> u8 {
    let sign_degree = longitude % 30.0;
    let sign_idx = get_sign_idx(longitude);
    let part = (sign_degree / 7.5).floor() as u8;
    let offset = match part {
        0 => 0, 1 => 3, 2 => 6, 3 => 9, _ => 0,
    };
    (sign_idx + offset) % 12 + 1
}

/// D6 (Shashtamsa) - Disease, Enemies
/// Odd signs: Count from sign itself
/// Even signs: Count from the 6th sign from itself
fn calculate_shashtamsa(longitude: f64) -> u8 {
    let sign_degree = longitude % 30.0;
    let sign_idx = get_sign_idx(longitude);
    let part = (sign_degree / 5.0).floor() as u8; // 0..5
    let start_sign = if is_odd_sign(sign_idx) { sign_idx } else { (sign_idx + 5) % 12 };
    (start_sign + part) % 12 + 1
}

fn calculate_saptamsa(longitude: f64) -> u8 {
    let sign_degree = longitude % 30.0;
    let sign_idx = get_sign_idx(longitude);
    let part = (sign_degree / (30.0 / 7.0)).floor() as u8;
    let start_sign_idx = if is_odd_sign(sign_idx) { sign_idx } else { (sign_idx + 6) % 12 };
    (start_sign_idx + part) % 12 + 1
}

fn calculate_navamsa(longitude: f64) -> u8 {
    let d1_rasi_idx = get_sign_idx(longitude);
    let sign_degree = longitude % 30.0;
    let pada_idx = (sign_degree / (30.0 / 9.0)).floor() as u8;
    let start_sign = match d1_rasi_idx % 4 {
        0 => 0, 1 => 9, 2 => 6, 3 => 3, _ => unreachable!(),
    };
    (start_sign + pada_idx) % 12 + 1
}

fn calculate_dasamsa(longitude: f64) -> u8 {
    let sign_degree = longitude % 30.0;
    let sign_idx = get_sign_idx(longitude);
    let part = (sign_degree / 3.0).floor() as u8;
    let start_sign_idx = if is_odd_sign(sign_idx) { sign_idx } else { (sign_idx + 8) % 12 };
    (start_sign_idx + part) % 12 + 1
}

fn calculate_dwadasamsa(longitude: f64) -> u8 {
    let sign_degree = longitude % 30.0;
    let sign_idx = get_sign_idx(longitude);
    let part = (sign_degree / 2.5).floor() as u8;
    (sign_idx + part) % 12 + 1
}

fn calculate_panchamsa(longitude: f64) -> u8 {
    let sign_degree = longitude % 30.0;
    let sign_idx = get_sign_idx(longitude);
    let part = (sign_degree / 6.0).floor() as u8;
    let start_sign = if is_odd_sign(sign_idx) { 0 } else { 6 };
    (start_sign + part) % 12 + 1
}

fn calculate_ashtamsa(longitude: f64) -> u8 {
    let sign_degree = longitude % 30.0;
    let sign_idx = get_sign_idx(longitude);
    let part = (sign_degree / 3.75).floor() as u8;
    let offset = match sign_idx % 3 {
        0 => 0, 1 => 8, 2 => 4, _ => unreachable!(),
    };
    (offset + part) % 12 + 1
}

fn calculate_rudramsa(longitude: f64) -> u8 {
    let sign_degree = longitude % 30.0;
    let sign_idx = get_sign_idx(longitude);
    let part = (sign_degree / (30.0 / 11.0)).floor() as i32;
    let mut target = sign_idx as i32 - part;
    while target < 0 { target += 12; }
    (target as u8) % 12 + 1
}

fn calculate_shodashamsa(longitude: f64) -> u8 {
    let sign_degree = longitude % 30.0;
    let sign_idx = get_sign_idx(longitude);
    let part = (sign_degree / (30.0 / 16.0)).floor() as u8;
    let start_sign = match sign_idx % 3 {
        0 => 0, 1 => 4, 2 => 8, _ => unreachable!(),
    };
    (start_sign + part) % 12 + 1
}

fn calculate_vimsamsa(longitude: f64) -> u8 {
     let sign_degree = longitude % 30.0;
     let sign_idx = get_sign_idx(longitude);
     let part = (sign_degree / 1.5).floor() as u8;
     let start_sign = match sign_idx % 3 {
        0 => 0, 1 => 8, 2 => 4, _ => unreachable!(),
    };
    (start_sign + part) % 12 + 1
}

fn calculate_chaturvimshamsa(longitude: f64) -> u8 {
    let sign_degree = longitude % 30.0;
    let sign_idx = get_sign_idx(longitude);
    let part = (sign_degree / 1.25).floor() as u8;
    let start_sign = if is_odd_sign(sign_idx) { 4 } else { 3 };
    (start_sign + part) % 12 + 1
}

fn calculate_saptavimsamsa(longitude: f64) -> u8 {
    let sign_degree = longitude % 30.0;
    let sign_idx = get_sign_idx(longitude);
    let part = (sign_degree / (30.0 / 27.0)).floor() as u8;
    let start_sign = match sign_idx % 4 {
        0 => 0, 1 => 3, 2 => 6, 3 => 9, _ => unreachable!(),
    };
    (start_sign + part) % 12 + 1
}

fn calculate_trimsamsa(longitude: f64) -> u8 {
    let sign_degree = longitude % 30.0;
    let sign_idx = get_sign_idx(longitude);
    if is_odd_sign(sign_idx) {
        if sign_degree < 5.0 { 1 } else if sign_degree < 10.0 { 11 } else if sign_degree < 18.0 { 9 } else if sign_degree < 25.0 { 3 } else { 7 }
    } else if sign_degree < 5.0 { 2 } else if sign_degree < 12.0 { 6 } else if sign_degree < 20.0 { 12 } else if sign_degree < 25.0 { 10 } else { 8 }
}

fn calculate_khavedamsa(longitude: f64) -> u8 {
    let sign_degree = longitude % 30.0;
    let sign_idx = get_sign_idx(longitude);
    let part = (sign_degree / 0.75).floor() as u8;
    let start_sign = if is_odd_sign(sign_idx) { 0 } else { 6 };
    (start_sign + part) % 12 + 1
}

fn calculate_akshavedamsa(longitude: f64) -> u8 {
    let sign_degree = longitude % 30.0;
    let sign_idx = get_sign_idx(longitude);
    let part = (sign_degree / (30.0 / 45.0)).floor() as u8;
    let start_sign = match sign_idx % 3 {
        0 => 0, 1 => 4, 2 => 8, _ => unreachable!(),
    };
    (start_sign + part) % 12 + 1
}

fn calculate_shashtyamsa(longitude: f64) -> u8 {
    let sign_degree = longitude % 30.0;
    let sign_idx = get_sign_idx(longitude);
    let part = (sign_degree * 2.0).floor() as u8;
    (sign_idx + part) % 12 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harmonic_varga_distinction() {
        // Two points in the same D9 division (Aries 0-3.333...)
        // Point A: 1.0 degree
        // Point B: 2.0 degree
        let long_a = 1.0;
        let long_b = 2.0;

        let d9_a = VargaType::D9.calculate_rasi(long_a);
        let d9_b = VargaType::D9.calculate_rasi(long_b);
        assert_eq!(d9_a, d9_b, "Should be in the same D9 division");

        let d81_a = VargaType::D81.calculate_rasi(long_a);
        let d81_b = VargaType::D81.calculate_rasi(long_b);

        // Point A: 1 * 81 = 81 mod 360 = 81 (Gemini)
        // Point B: 2 * 81 = 162 mod 360 = 162 (Virgo)
        assert_ne!(d81_a, d81_b, "D81 should distinguish points within the same D9 division");
        assert_eq!(d81_a, 3, "1.0 degree in D81 should be Gemini(3)");
        assert_eq!(d81_b, 6, "2.0 degree in D81 should be Virgo(6)");
    }

    #[test]
    fn test_d144_harmonic() {
        // Point in D12 division (Aries 0-2.5 deg)
        let long_a = 0.5; // 0.5 * 144 = 72 (Gemini-3)
        let long_b = 1.5; // 1.5 * 144 = 216 (Scorpio-8)

        let d12_a = VargaType::D12.calculate_rasi(long_a);
        let d12_b = VargaType::D12.calculate_rasi(long_b);
        assert_eq!(d12_a, d12_b, "Should be in the same D12 division");

        let d144_a = VargaType::D144.calculate_rasi(long_a);
        let d144_b = VargaType::D144.calculate_rasi(long_b);

        assert_ne!(d144_a, d144_b);
        assert_eq!(d144_a, 3);
        assert_eq!(d144_b, 8);
    }
}
