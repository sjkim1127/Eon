/* tslint:disable */
/* eslint-disable */

export class Position {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Ecliptic longitude in degrees
   */
  longitude: number;
  /**
   * Ecliptic latitude in degrees  
   */
  latitude: number;
  /**
   * Distance (AU for planets, Earth radii for Moon)
   */
  distance: number;
  /**
   * Longitude speed (degrees/day)
   */
  longitude_speed: number;
  /**
   * Latitude speed (degrees/day)
   */
  latitude_speed: number;
  /**
   * Distance speed (AU/day)
   */
  distance_speed: number;
}

export class SwissEphError {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Error message from the library
   */
  message: string;
  /**
   * Return code
   */
  code: number;
}

/**
 * Calculate planetary position using UT (Universal Time)
 */
export function calc_ut(jd_ut: number, planet: number, flags: number): Position;

/**
 * 사주(四柱) 분석 — WASM에서 호출 가능
 *
 * 생년월일시 + 성별 + 좌표 + 타임존을 받아 사주 분석 결과를 반환합니다.
 * BirthInfo를 사용하여 DST(서머타임) + 경도 기반 진태양시 보정을 자동 적용합니다.
 */
export function get_saju_analysis(year: number, month: number, day: number, hour: number, minute: number, is_lunar: boolean, is_leap_month: boolean, is_male: boolean, lon: number, lat: number, timezone: string): any;

/**
 * 사주 궁합 분석 - WASM에서 호출 가능
 */
export function get_saju_compatibility(year1: number, month1: number, day1: number, hour1: number, minute1: number, is_lunar1: boolean, is_leap_month1: boolean, is_male1: boolean, lon1: number, lat1: number, year2: number, month2: number, day2: number, hour2: number, minute2: number, is_lunar2: boolean, is_leap_month2: boolean, is_male2: boolean, lon2: number, lat2: number, timezone1: string, timezone2: string): any;

/**
 * 현재 운세(세운/월운/일운) 분석 — WASM에서 호출 가능
 */
export function get_transit_analysis(year: number, month: number, day: number, hour: number, minute: number, is_lunar: boolean, is_leap_month: boolean, is_male: boolean, lon: number, lat: number, timezone: string, current_year: number, current_month: number, current_day: number): any;

export function get_vedic_analysis(year: number, month: number, day: number, hour: number, minute: number, is_lunar: boolean, is_leap_month: boolean, lat: number, lon: number, timezone: string): Promise<any>;

/**
 * 베딕 궁합 분석 (Ashta Kuta) - WASM에서 호출 가능
 */
export function get_vedic_compatibility(year1: number, month1: number, day1: number, hour1: number, minute1: number, is_lunar1: boolean, is_leap_month1: boolean, lat1: number, lon1: number, year2: number, month2: number, day2: number, hour2: number, minute2: number, is_lunar2: boolean, is_leap_month2: boolean, lat2: number, lon2: number, timezone1: string, timezone2: string): Promise<any>;

export function greet(name: string): string;

/**
 * WASM 패닉 메시지를 브라우저 콘솔에 표시
 */
export function init_panic_hook(): void;

/**
 * Set the ephemeris path
 */
export function set_ephe_path(path: string): void;

/**
 * Get Swiss Ephemeris version
 */
export function version(): string;
