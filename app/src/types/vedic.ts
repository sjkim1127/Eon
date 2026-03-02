// ──────────────────────────────────────────────
// 베딕 점성학 관련 타입 정의
// ──────────────────────────────────────────────

/** 요가 품질 */
export type YogaQuality = "VeryHigh" | "High" | "Medium" | { Weak: string };

/** 요가 */
export interface Yoga {
  name: string;
  yoga_type: string;
  description: string;
  planets_involved: string[];
  quality: YogaQuality;
}

/** 하우스 요약 */
export interface HouseSummary {
  house: number;
  rating: string;
  total_score: number;
}

/** 베딕 분석 리포트 (VedicAnalysisReport Rust struct) */
export interface VedicAnalysisReport {
  primary_karakas: {
    atmakaraka: string;
    amatyakaraka: string;
    darakaraka: string;
  };
  house_summary: HouseSummary[];
  dasha_focus: string;
  nakshatra_info: string;
  overall_strength_score: number;
  sade_sati: "None" | "Rising" | "Peak" | "Setting";
  yogas: Yoga[];
}

// ── VedicChart 원시 데이터 ────────────────────

/** 판창가 (Panchanga Rust struct) */
export interface VedicPanchanga {
  vara: string;
  tithi: number;
  tithi_name: string;
  nakshatra: number;
  yoga: number;
  karana_name: string;
  day_lord: string;
  hour_lord: string;
  is_day_birth: boolean;
}

/** 행성 위치 (VedicPosition Rust struct) — 바르가 필드 포함 */
export interface VedicPosition {
  planet: string;
  tropical_deg: number;
  sidereal_deg: number;
  nakshatra: number;
  pada: number;
  rasi: number;
  house_index: number;
  speed: number;
  is_retrograde: boolean;
  is_combust: boolean;
  declination: number;
  // D-차트 라시 포지션 (D2~D144)
  [key: string]: number | string | boolean;
}

/** 베딕 차트 원시 데이터 (VedicChart Rust struct) */
export interface VedicChartData {
  ascendant: VedicPosition;
  planets: VedicPosition[];
  aspects: unknown[];
  sav: unknown;
  house_cusps: number[];
  karakas: unknown[];
  bhava_strengths: unknown[];
  vimshopaka_scores: unknown[];
  panchanga: VedicPanchanga;
  analysis_report: VedicAnalysisReport | null;
}

/** 베딕 분석 결과 (report + raw chart) */
export interface VedicAnalysisResult {
  report: VedicAnalysisReport;
  chart: VedicChartData;
}

/** 낙샤트라 원시 데이터 */
export interface NakshatraEntry {
  name: string;
  lord: string;
  deity: string;
  start_deg: number;
}

/** 낙샤트라 계산 결과 */
export interface NakshatraInfo {
  name: string;
  pada: number;
  lord: string;
  padaLord: string;
  deity: string;
  purpose: string;
  range: string;
}

/** Varga 차트 정의 */
export interface VargaDef {
  id: string;
  label: string;
  name: string;
  key: string;
}

/** Ashta Kuta 궁합 결과 (CompatibilityResult Rust struct) */
export interface AshtaKutaResult {
  total_score: number;
  message?: string;
  varna?: number;
  vashya?: number;
  tara?: number;
  yoni?: number;
  maitri?: number;
  gana?: number;
  bhakoot?: number;
  nadi?: number;
}
