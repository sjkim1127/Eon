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
 * AI 감사 리포트 — WASM에서 호출 가능
 *
 * eon-ai의 `DestinyAIAuditor`를 사용하여 LLM 프롬프트(context_dump)와
 * 생애 peak/valley 나이를 반환합니다. API 키 없이 Google AI Studio 등에 붙여넣기 가능.
 */
export function get_ai_audit(year: number, month: number, day: number, hour: number, minute: number, is_lunar: boolean, is_leap_month: boolean, is_male: boolean, use_night_rat_hour: boolean, lon: number, lat: number, timezone: string): any;

/**
 * 사주(四柱) 분석 — WASM에서 호출 가능
 *
 * 생년월일시 + 성별 + 좌표 + 타임존을 받아 사주 분석 결과를 반환합니다.
 * BirthInfo를 사용하여 DST(서머타임) + 경도 기반 진태양시 보정을 자동 적용합니다.
 */
export function get_saju_analysis(year: number, month: number, day: number, hour: number, minute: number, is_lunar: boolean, is_leap_month: boolean, is_male: boolean, use_night_rat_hour: boolean, lon: number, lat: number, timezone: string): any;

/**
 * 사주 궁합 분석 - WASM에서 호출 가능
 */
export function get_saju_compatibility(year1: number, month1: number, day1: number, hour1: number, minute1: number, is_lunar1: boolean, is_leap_month1: boolean, is_male1: boolean, lon1: number, lat1: number, use_night_rat_hour1: boolean, year2: number, month2: number, day2: number, hour2: number, minute2: number, is_lunar2: boolean, is_leap_month2: boolean, is_male2: boolean, lon2: number, lat2: number, use_night_rat_hour2: boolean, timezone1: string, timezone2: string): any;

/**
 * 현재 운세(세운/월운/일운) 분석 — WASM에서 호출 가능
 */
export function get_transit_analysis(year: number, month: number, day: number, hour: number, minute: number, is_lunar: boolean, is_leap_month: boolean, is_male: boolean, use_night_rat_hour: boolean, lon: number, lat: number, timezone: string, current_year: number, current_month: number, current_day: number): any;

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

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly get_ai_audit: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number) => [number, number, number];
  readonly get_saju_analysis: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number) => [number, number, number];
  readonly get_saju_compatibility: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number, s: number, t: number, u: number, v: number, w: number, x: number, y: number, z: number) => [number, number, number];
  readonly get_transit_analysis: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number) => [number, number, number];
  readonly get_vedic_analysis: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => any;
  readonly get_vedic_compatibility: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number, s: number, t: number, u: number, v: number) => any;
  readonly greet: (a: number, b: number) => [number, number];
  readonly init_panic_hook: () => void;
  readonly __wbg_get_position_distance: (a: number) => number;
  readonly __wbg_get_position_distance_speed: (a: number) => number;
  readonly __wbg_get_position_latitude: (a: number) => number;
  readonly __wbg_get_position_latitude_speed: (a: number) => number;
  readonly __wbg_get_position_longitude: (a: number) => number;
  readonly __wbg_get_position_longitude_speed: (a: number) => number;
  readonly __wbg_get_swissepherror_code: (a: number) => number;
  readonly __wbg_get_swissepherror_message: (a: number) => [number, number];
  readonly __wbg_position_free: (a: number, b: number) => void;
  readonly __wbg_set_position_distance: (a: number, b: number) => void;
  readonly __wbg_set_position_distance_speed: (a: number, b: number) => void;
  readonly __wbg_set_position_latitude: (a: number, b: number) => void;
  readonly __wbg_set_position_latitude_speed: (a: number, b: number) => void;
  readonly __wbg_set_position_longitude: (a: number, b: number) => void;
  readonly __wbg_set_position_longitude_speed: (a: number, b: number) => void;
  readonly __wbg_set_swissepherror_code: (a: number, b: number) => void;
  readonly __wbg_set_swissepherror_message: (a: number, b: number, c: number) => void;
  readonly __wbg_swissepherror_free: (a: number, b: number) => void;
  readonly calc_ut: (a: number, b: number, c: number) => [number, number, number];
  readonly set_ephe_path: (a: number, b: number) => void;
  readonly version: () => [number, number];
  readonly free: (a: number) => void;
  readonly malloc: (a: number) => number;
  readonly wasm_swe_calc: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly wasm_swe_calc_ut: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly wasm_swe_close: () => void;
  readonly wasm_swe_houses: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly wasm_swe_julday: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly wasm_swe_revjul: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly wasm_swe_set_ephe_path: (a: number) => void;
  readonly wasm_swe_set_topo: (a: number, b: number, c: number) => void;
  readonly wasm_swe_sidtime: (a: number) => number;
  readonly wasm_swe_version: (a: number) => number;
  readonly wasm_bindgen__convert__closures_____invoke__h4cd75a9079ae9638: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__closure__destroy__h5e9e2a75c5727948: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h22aa556dbecabb1f: (a: number, b: number, c: any, d: any) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
