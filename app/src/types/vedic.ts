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

/** 베딕 분석 리포트 */
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
  sade_sati: string;
  yogas: Yoga[];
}

/** 베딕 분석 결과 (report + raw chart) */
export interface VedicAnalysisResult {
  report: VedicAnalysisReport;
  chart: any; // Raw VedicChart data
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

/** Ashta Kuta 궁합 결과 */
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
