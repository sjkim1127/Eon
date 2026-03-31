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
                let sign_degree = longitude % 30.0;
                let pada_idx = (sign_degree * 9.0 / 30.0).floor();
                let r1 = calculate_navamsa(longitude);
                let d_rem = sign_degree - (pada_idx * 30.0 / 9.0);
                let d_scaled = d_rem * 9.0;
                calculate_navamsa(((r1 as f64 - 1.0) * 30.0 + d_scaled) % 360.0)
            },
            Self::D108 => {
                // D9 of D12 (Ashtottaramsa)
                let sign_degree = longitude % 30.0;
                let div_idx = (sign_degree * 12.0 / 30.0).floor();
                let r1 = calculate_dwadasamsa(longitude);
                let d_rem = sign_degree - (div_idx * 30.0 / 12.0);
                let d_scaled = d_rem * 12.0;
                calculate_navamsa(((r1 as f64 - 1.0) * 30.0 + d_scaled) % 360.0)
            },
            Self::D144 => {
                // D12 of D12 (Dwadas-Dwadasamsa)
                let sign_degree = longitude % 30.0;
                let div_idx = (sign_degree * 12.0 / 30.0).floor();
                let r1 = calculate_dwadasamsa(longitude);
                let d_rem = sign_degree - (div_idx * 30.0 / 12.0);
                let d_scaled = d_rem * 12.0;
                calculate_dwadasamsa(((r1 as f64 - 1.0) * 30.0 + d_scaled) % 360.0)
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
        let sign_idx = get_sign_idx(deg);

        let (scaled_degree, _div_size) = if matches!(self, Self::D30) {
            let (start, width) = if is_odd_sign(sign_idx) {
                if sign_degree < 5.0 { (0.0, 5.0) }
                else if sign_degree < 10.0 { (5.0, 5.0) }
                else if sign_degree < 18.0 { (10.0, 8.0) }
                else if sign_degree < 25.0 { (18.0, 7.0) }
                else { (25.0, 5.0) }
            } else {
                if sign_degree < 5.0 { (0.0, 5.0) }
                else if sign_degree < 12.0 { (5.0, 7.0) }
                else if sign_degree < 20.0 { (12.0, 8.0) }
                else if sign_degree < 25.0 { (20.0, 5.0) }
                else { (25.0, 5.0) }
            };
            let degree_in_division = sign_degree - start;
            ((degree_in_division / width) * 30.0, width)
        } else {
            let division_size = 30.0 / self.division_count() as f64;
            let degree_in_division = sign_degree % division_size;
            (degree_in_division * self.division_count() as f64, division_size)
        };

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
    fn test_d81_composite() {
        // Aries 10.0 degrees
        let long = 10.0;
        // D9 of 10.0 is Cancer (4).
        // Degree within Navamsa: 10.0 - 3*(30/9) = 0.0.
        // D9 of 0.0 in Cancer is Cancer (4).
        let d81 = VargaType::D81.calculate_rasi(long);
        assert_eq!(d81, 4);
    }

    #[test]
    fn test_d144_composite() {
        // Aries 2.6 degrees
        let long = 2.6;
        // D12 of 2.6 is Taurus (2).
        // Degree within D12: 2.6 % 2.5 = 0.1.
        // Scaled: 0.1 * 12 = 1.2.
        // D12 of 1.2 in Taurus is Taurus (2).
        let d144 = VargaType::D144.calculate_rasi(long);
        assert_eq!(d144, 2);
    }

    #[test]
    fn test_d30_effective_longitude() {
        // D30 Trimsamsa: Odd sign, 0-5 degrees is Aries (1).
        // 2.0 degrees in Aries (1) is 40% into the 5-degree division.
        // 40% of 30 degrees is 12.0 degrees.
        let long = 2.0;
        let varga_rasi = VargaType::D30.calculate_rasi(long);
        assert_eq!(varga_rasi, 1);
        
        let eff_long = VargaType::D30.effective_longitude_for_nakshatra(long, varga_rasi);
        assert_eq!(eff_long, 12.0); 
    }
}
