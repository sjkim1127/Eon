//! eon-astro: 초정밀 천문 연산 엔진
//!
//! Swiss Ephemeris C API를 직접 활용하여 초정밀 천문 연상을 수행합니다.

use chrono::{DateTime, Datelike, Timelike, Utc};
use std::sync::Mutex;

static ASTRO_LOCK: Mutex<()> = Mutex::new(());

pub struct AstroEngine;

impl AstroEngine {
    pub fn new() -> Self {
        let _lock = ASTRO_LOCK.lock().unwrap();
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

        let _lock = ASTRO_LOCK.lock().unwrap();
        unsafe {
            // Body::Sun = 0
            // Flag::SPEED = 256
            let ret = swiss_eph::swe_calc_ut(
                julian_day,
                0,   // SE_SUN
                256, // SEFLG_SPEED
                results.as_mut_ptr(),
                error.as_mut_ptr() as *mut i8,
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
        target_long: f64,
    ) -> Result<DateTime<Utc>, String> {
        use chrono::Duration;

        let mut current_time = start_time;

        // 최대 10회 반복 (충분히 수렴함)
        for _ in 0..10 {
            let current_long = self.get_sun_longitude(current_time)?;

            // 각도 차이 계산 (360도 보정)
            let mut diff = target_long - current_long;
            while diff > 180.0 {
                diff -= 360.0;
            }
            while diff < -180.0 {
                diff += 360.0;
            }

            if diff.abs() < 0.00001 {
                // 약 1초 미만 오차
                break;
            }

            // 태양은 하루에 약 0.9856도 이동
            // 초당 이동 거리로 환산하여 시간 보정
            let seconds_to_move = (diff / 0.9856) * 86400.0;
            current_time += Duration::seconds(seconds_to_move as i64);
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
        while diff > 180.0 {
            diff -= 360.0;
        }
        while diff < -180.0 {
            diff += 360.0;
        }

        // 대략적인 시각 산출 (1도 = 1일로 계산)
        use chrono::Duration;
        let approx_time = birth_time + Duration::seconds((diff * 86400.0 / 0.9856) as i64);

        self.find_time_for_longitude(approx_time, target_long)
    }

    /// 행성, 달, 노드의 상세 데이터 (황경, 속도) 계산
    pub fn get_planet_full(
        &self,
        datetime: DateTime<Utc>,
        planet_id: i32,
        flag: i32,
    ) -> Result<(f64, f64), String> {
        let julian_day = self.to_julian_day(datetime);
        let mut results = [0.0; 6];
        let mut error = [0; 256];

        let _lock = ASTRO_LOCK.lock().unwrap();
        unsafe {
            let ret = swiss_eph::swe_calc_ut(
                julian_day,
                planet_id,
                flag,
                results.as_mut_ptr(),
                error.as_mut_ptr() as *mut i8,
            );

            if ret < 0 {
                let err_msg = std::ffi::CStr::from_ptr(error.as_ptr() as *const i8)
                    .to_string_lossy()
                    .into_owned();
                Err(format!("Astro Error: {}", err_msg))
            } else {
                Ok((results[0], results[3])) // Longitude, Speed
            }
        }
    }

    /// 행성, 달, 노드의 위치 계산 (Vedic 점성술 지원용)
    pub fn get_planet_position(
        &self,
        datetime: DateTime<Utc>,
        planet_id: i32,
        flag: i32,
    ) -> Result<f64, String> {
        self.get_planet_full(datetime, planet_id, flag)
            .map(|(long, _)| long)
    }

    /// 적도 좌표계 (Equatorial) 데이터 (RA, Declination) 계산
    pub fn get_planet_equatorial(
        &self,
        datetime: DateTime<Utc>,
        planet_id: i32,
    ) -> Result<(f64, f64), String> {
        let julian_day = self.to_julian_day(datetime);
        let mut results = [0.0; 6];
        let mut error = [0; 256];
        let flag = 2048 | 2; // SEFLG_EQUATORIAL | SEFLG_SWIEPH

        let _lock = ASTRO_LOCK.lock().unwrap();
        unsafe {
            let ret = swiss_eph::swe_calc_ut(
                julian_day,
                planet_id,
                flag,
                results.as_mut_ptr(),
                error.as_mut_ptr() as *mut i8,
            );

            if ret < 0 {
                let err_msg = std::ffi::CStr::from_ptr(error.as_ptr() as *const i8)
                    .to_string_lossy()
                    .into_owned();
                Err(format!("Astro Error: {}", err_msg))
            } else {
                Ok((results[0], results[1])) // RA, Declination
            }
        }
    }

    /// Chrono DateTime을 Julian Day로 변환
    fn to_julian_day(&self, dt: DateTime<Utc>) -> f64 {
        let year = dt.year();
        let month = dt.month() as i32;
        let day = dt.day() as i32;
        let hour = dt.hour() as f64 + (dt.minute() as f64 / 60.0) + (dt.second() as f64 / 3600.0);

        let _lock = ASTRO_LOCK.lock().unwrap();
        unsafe {
            // Gregorian = 1
            swiss_eph::swe_julday(year, month, day, hour, 1)
        }
    }

    /// 특정 시점 이전 혹은 포함된 가장 가까운 합삭(New Moon) 시각을 찾습니다.
    pub fn find_new_moon_before(&self, datetime: DateTime<Utc>) -> Result<DateTime<Utc>, String> {
        self.find_relative_conjunction_backward(datetime, 0.0)
    }

    /// 특정 시점 이전의 가장 가까운 동지(Winter Solstice, 황경 270도)를 찾습니다.
    /// 음력 연도를 동지 기준으로 정렬할 때 필요합니다.
    pub fn find_winter_solstice_before(
        &self,
        datetime: DateTime<Utc>,
    ) -> Result<DateTime<Utc>, String> {
        self.find_longitude_time_backward(datetime, 270.0)
    }

    /// 태양과 달의 상대적 위치(상대적 황경 차이)가 target_diff가 되는 시점을 역방향으로 찾습니다.
    fn find_relative_conjunction_backward(
        &self,
        start_time: DateTime<Utc>,
        target_diff: f64,
    ) -> Result<DateTime<Utc>, String> {
        use chrono::Duration;

        let mut t = start_time;

        // 1. 현재 시점의 차이 확인 (Newton-Raphson 근사 시작점)
        let sun_l = self.get_sun_longitude(t)?;
        let moon_l = self.get_planet_position(t, 1, 256)?;
        let diff = (moon_l - sun_l + 360.0) % 360.0;

        // 달-태양 상대 속도 약 12.19도/일.
        let days_back = diff / 12.19;
        t -= Duration::seconds((days_back * 86400.0) as i64);

        // 2. 수렴
        for _ in 0..10 {
            let s = self.get_sun_longitude(t)?;
            let m = self.get_planet_position(t, 1, 256)?;
            let d = (m - s + 360.0) % 360.0;

            let mut err = target_diff - d;
            while err > 180.0 {
                err -= 360.0;
            }
            while err < -180.0 {
                err += 360.0;
            }

            if err.abs() < 0.000001 {
                break;
            }

            let days_adj = err / 12.19;
            t += Duration::seconds((days_adj * 86400.0) as i64);
        }

        // 3. 만약 결과가 start_time보다 미래라면 한 달 더 뒤로 가서 다시 찾기
        if t > start_time + Duration::seconds(1) {
            t -= Duration::days(29);
            return self.find_relative_conjunction_backward(t, target_diff);
        }

        Ok(t)
    }

    /// 태양 황경이 특정 target_long이 되는 시점을 역방향으로 찾습니다.
    fn find_longitude_time_backward(
        &self,
        start_time: DateTime<Utc>,
        target_long: f64,
    ) -> Result<DateTime<Utc>, String> {
        use chrono::Duration;
        let mut t = start_time;

        let current_long = self.get_sun_longitude(t)?;
        let mut diff = current_long - target_long;
        while diff < 0.0 {
            diff += 360.0;
        }

        // 태양 속도 약 0.9856도/일
        let days_back = diff / 0.9856;
        t -= Duration::seconds((days_back * 86400.0) as i64);

        for _ in 0..10 {
            let c = self.get_sun_longitude(t)?;
            let mut d = target_long - c;
            while d > 180.0 {
                d -= 360.0;
            }
            while d < -180.0 {
                d += 360.0;
            }

            if d.abs() < 0.000001 {
                break;
            }

            let days_adj = d / 0.9856;
            t += Duration::seconds((days_adj * 86400.0) as i64);
        }

        if t > start_time + Duration::seconds(1) {
            t -= Duration::days(360);
            return self.find_longitude_time_backward(t, target_long);
        }
        Ok(t)
    }
    /// 특정 시점, 위치에서의 하우스(1~12) 및 ASC/MC를 계산합니다.
    pub fn get_houses(
        &self,
        datetime: DateTime<Utc>,
        latitude: f64,
        longitude: f64,
        house_system: i32, // 'P' as char as i32, etc.
    ) -> Result<(Vec<f64>, [f64; 10]), String> {
        let julian_day = self.to_julian_day(datetime);
        let mut cusps = [0.0; 13]; // 1-based usually
        let mut ascmc = [0.0; 10]; // ASC, MC, ARMC, Vertex...

        let _lock = ASTRO_LOCK.lock().unwrap();
        unsafe {
            // 'P' as i32 for Placidus, 'W' for Whole Sign if supported,
            // but SE typically takes char byte.
            // swe_houses(tjd_ut, lat, lon, hsys, cusps, ascmc)
            let ret = swiss_eph::swe_houses(
                julian_day,
                latitude,
                longitude,
                house_system,
                cusps.as_mut_ptr(),
                ascmc.as_mut_ptr(),
            );

            if ret < 0 {
                Err("Failed to calculate houses".to_string())
            } else {
                // Convert cusps to Vec (1..13) ignoring index 0
                let cusps_vec = cusps[1..13].to_vec();
                Ok((cusps_vec, ascmc))
            }
        }
    }

    /// 특정 시점의 아야남사(Ayanamsa) 값을 계산합니다.
    /// flag: SE_SIDBIT_ELEM_PLAN (8) 등 옵션 설정 가능. 보통 0.
    pub fn get_ayanamsa_ut(&self, datetime: DateTime<Utc>) -> f64 {
        let julian_day = self.to_julian_day(datetime);
        let _lock = ASTRO_LOCK.lock().unwrap();
        unsafe { swiss_eph::swe_get_ayanamsa_ut(julian_day) }
    }

    /// 아야남사 모드 설정 (예: Lahiri, Raman 등)
    /// method_id: SE_SIDM_LAHIRI (1), SE_SIDM_RAMAN (3) 등
    pub fn set_sidereal_mode(&self, method_id: i32, t0: f64, ayan_t0: f64) {
        let _lock = ASTRO_LOCK.lock().unwrap();
        unsafe {
            swiss_eph::swe_set_sid_mode(method_id, t0, ayan_t0);
        }
    }
}

impl Default for AstroEngine {
    fn default() -> Self {
        Self::new()
    }
}
