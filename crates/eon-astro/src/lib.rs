//! eon-astro: 초정밀 천문 연산 엔진
//! 
//! Swiss Ephemeris C API를 직접 활용하여 초정밀 천문 연상을 수행합니다.

use chrono::{DateTime, Utc, Datelike, Timelike};

pub struct AstroEngine;

impl AstroEngine {
    pub fn new() -> Self {
        // 경로 설정 (필요시)
        unsafe {
            swiss_eph::swe_set_ephe_path(std::ptr::null());
        }
        Self
    }

    /// 특정 시점의 태양 황경(Tropical Longitude)을 계산합니다.
    pub fn get_sun_longitude(&self, datetime: DateTime<Utc>) -> Result<f64, String> {
        let julian_day = self.to_julian_day(datetime);
        
        // SE_SUN = 0, SEFLG_SPEED = 256 (보통)
        // 실제 상수는 swiss_eph에서 제공하는지 확인 필요
        let mut results = [0.0; 6];
        let mut error = [0; 256];
        
        unsafe {
            // Body::Sun = 0
            // Flag::SPEED = 256
            let ret = swiss_eph::swe_calc_ut(
                julian_day,
                0, // SE_SUN
                256, // SEFLG_SPEED
                results.as_mut_ptr(),
                error.as_mut_ptr() as *mut i8
            );
            
            if ret < 0 {
                let err_msg = std::ffi::CStr::from_ptr(error.as_ptr() as *const i8)
                    .to_string_lossy()
                    .into_owned();
                Err(format!("Astro Error: {}", err_msg))
            } else {
                Ok(results[0]) // Longitude
            }
        }
    }

    /// 특정 시점의 24절기 인덱스(0~23)를 산출합니다.
    /// 0: 입춘, 1: 우수, 2: 경칩 ... 23: 대한
    pub fn get_solar_term_index(&self, datetime: DateTime<Utc>) -> u8 {
        let sun_long = self.get_sun_longitude(datetime).unwrap_or(0.0);
        
        // 입춘(315도)을 0으로 맞춤
        let adjusted = (sun_long - 315.0 + 360.0) % 360.0;
        
        // 15도마다 절기가 바뀜 (24절기)
        let index = (adjusted / 15.0).floor() as u8;
        index % 24
    }

    /// 특정 황경(target_long)에 도달하는 정확한 시각을 추적합니다. (Root Finding)
    pub fn find_time_for_longitude(
        &self, 
        start_time: DateTime<Utc>, 
        target_long: f64
    ) -> Result<DateTime<Utc>, String> {
        use chrono::Duration;
        
        let mut current_time = start_time;
        
        // 최대 10회 반복 (충분히 수렴함)
        for _ in 0..10 {
            let current_long = self.get_sun_longitude(current_time)?;
            
            // 각도 차이 계산 (360도 보정)
            let mut diff = target_long - current_long;
            while diff > 180.0 { diff -= 360.0; }
            while diff < -180.0 { diff += 360.0; }
            
            if diff.abs() < 0.00001 { // 약 1초 미만 오차
                break;
            }
            
            // 태양은 하루에 약 0.9856도 이동
            // 초당 이동 거리로 환산하여 시간 보정
            let seconds_to_move = (diff / 0.9856) * 86400.0;
            current_time = current_time + Duration::seconds(seconds_to_move as i64);
        }
        
        Ok(current_time)
    }

    /// 특정 절기(SolarTerm)의 정확한 시작 시각을 찾습니다.
    pub fn find_solar_term_time(
        &self,
        birth_time: DateTime<Utc>,
        term_idx: u8, // 0: 입춘, 1: 우수 ... 23: 대한
    ) -> Result<DateTime<Utc>, String> {
        let target_long = (315.0 + (term_idx as f64) * 15.0) % 360.0;
        let current_long = self.get_sun_longitude(birth_time)?;
        
        let mut diff = target_long - current_long;
        while diff > 180.0 { diff -= 360.0; }
        while diff < -180.0 { diff += 360.0; }
        
        // 대략적인 시각 산출 (1도 = 1일로 계산)
        use chrono::Duration;
        let approx_time = birth_time + Duration::seconds((diff * 86400.0 / 0.9856) as i64);
        
        self.find_time_for_longitude(approx_time, target_long)
    }

    /// Chrono DateTime을 Julian Day로 변환
    fn to_julian_day(&self, dt: DateTime<Utc>) -> f64 {
        let year = dt.year();
        let month = dt.month() as i32;
        let day = dt.day() as i32;
        let hour = dt.hour() as f64 + (dt.minute() as f64 / 60.0) + (dt.second() as f64 / 3600.0);
        
        unsafe {
            // Gregorian = 1
            swiss_eph::swe_julday(year, month, day, hour, 1)
        }
    }
}

impl Drop for AstroEngine {
    fn drop(&mut self) {
        unsafe {
            swiss_eph::swe_close();
        }
    }
}

impl Default for AstroEngine {
    fn default() -> Self {
        Self::new()
    }
}
