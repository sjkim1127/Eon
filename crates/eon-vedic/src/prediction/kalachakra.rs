use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KalaChakraPeriod {
    pub rasi: u8,
    pub duration_years: u32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

pub struct KalaChakraDasha;

impl KalaChakraDasha {
    pub fn calculate(moon_longitude: f64, birth_date: DateTime<Utc>) -> Vec<KalaChakraPeriod> {
        let nak_len = 360.0 / 27.0; // 13.3333333
        let nak_idx = (moon_longitude / nak_len).floor() as usize; // 0..26
        let nakshatra = (nak_idx + 1) as u8;

        let progression = (moon_longitude % nak_len) / (nak_len / 4.0);
        let pada = (progression.floor() as u8) + 1; // 1..4

        // Savya/Apasa classification
        // Savya: Nakshatras 1..9, 19..27
        // Apasa: 10..18
        let is_savya = ![10, 11, 12, 13, 14, 15, 16, 17, 18].contains(&nakshatra);

        // Sequence of signs
        let base_seq_savya = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, // Mesh to Dhanu
            10, 11, 12, 8, 7, 6, 5, 4, 3, // Makara to Mithuna
        ];

        let base_seq_apasa = [12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 5, 6, 7, 8, 9, 10];

        let start_idx = (((nakshatra - 1) % 3) * 4 + (pada - 1)) as usize;

        let mut sequence = Vec::new();
        if is_savya {
            for i in 0..9 {
                let idx = (start_idx + i) % base_seq_savya.len();
                sequence.push(base_seq_savya[idx]);
            }
        } else {
            for i in 0..9 {
                let idx = (start_idx + i) % base_seq_apasa.len();
                sequence.push(base_seq_apasa[idx]);
            }
        }

        let mut periods = Vec::new();
        let mut current_start = birth_date;

        for rasi in sequence {
            let years = Self::get_years_for_rasi(rasi);
            let end_time = current_start
                + Duration::seconds((years as f64 * 365.2425 * 24.0 * 60.0 * 60.0) as i64);

            periods.push(KalaChakraPeriod {
                rasi,
                duration_years: years,
                start_time: current_start,
                end_time,
            });

            current_start = end_time;
        }

        periods
    }

    fn get_years_for_rasi(rasi: u8) -> u32 {
        match rasi {
            1 | 8 => 7,   // Aries, Scorpio
            2 | 7 => 16,  // Taurus, Libra
            3 | 6 => 9,   // Gemini, Virgo
            4 => 21,      // Cancer
            5 => 5,       // Leo
            9 | 12 => 10, // Sagittarius, Pisces
            10 | 11 => 4, // Capricorn, Aquarius
            _ => 10,
        }
    }
}
