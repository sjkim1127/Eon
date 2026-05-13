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

export function get_destiny_tier_analysis(request: any): any;

export function get_saju_analysis(request: any): any;

export function get_transit_analysis(request: any): any;

export function get_vedic_analysis(request: any): any;

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
  readonly get_saju_analysis: (a: any) => [number, number, number];
  readonly get_transit_analysis: (a: any) => [number, number, number];
  readonly get_vedic_analysis: (a: any) => [number, number, number];
  readonly get_destiny_tier_analysis: (a: any) => [number, number, number];
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
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
