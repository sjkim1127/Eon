use crate::dto::CurrentContext;
use crate::error::ServiceError;
use chrono::{Datelike, Timelike};
use chrono_tz::Tz;
use std::str::FromStr;

pub fn resolve_analysis_local_date(
    current: &CurrentContext,
) -> Result<(i32, u32, u32), ServiceError> {
    let tz = Tz::from_str(&current.analysis_timezone)
        .map_err(|e| ServiceError::InvalidInput(format!("Invalid timezone: {}", e)))?;
    
    let local_dt = current.now_utc.with_timezone(&tz);
    
    Ok((local_dt.year(), local_dt.month(), local_dt.day()))
}

pub fn calculate_current_age(
    birth_year: i32,
    birth_month: u32,
    birth_day: u32,
    current: &CurrentContext,
) -> Result<u32, ServiceError> {
    let (cy, cm, cd) = resolve_analysis_local_date(current)?;
    
    let mut age = (cy - birth_year).max(0) as u32;
    
    // 생일이 지났는지 확인 (만 나이 기준)
    if cm < birth_month || (cm == birth_month && cd < birth_day) {
        if age > 0 {
            age -= 1;
        }
    }
    
    Ok(age)
}
