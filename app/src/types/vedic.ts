import type { AnalysisMeta } from "./analysis";
import type { CompatibilityAudit } from "./saju";

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

/** 다샤 기간 (DashaPeriod Rust struct) */
export interface DashaPeriod {
  lord: string;
  start_time: string; // ISO 8601 UTC
  end_time: string;   // ISO 8601 UTC
  sub_dashas: DashaPeriod[];
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
  dasha_timeline: DashaPeriod[];
  nakshatra_info: string;
  overall_strength_score: number;
  sade_sati: "None" | "Rising" | "Peak" | "Setting";
  yogas: Yoga[];
}

// ── 하우스 점수 (SAV / Bhava) ─────────────────────

/** 사르바아슈타카바르가 (전체 하우스 점수) */
export interface Sarvashtakavarga {
  points: number[]; // 12개 하우스별 합산 빈두 포인트
}

/** 행성별 빈나슈타카바르가 (BAV) 점수 */
export interface BavEntry {
  planet: string;        // 행성 이름
  points: number[];      // raw 빈두 포인트 [12]
  trikona_points: number[];   // Trikona Shodhana 후
  shodhana_points: number[];  // Ekadhipatya Shodhana 후
  sodya_pinda: number;        // 최종 Pinda 점수
}

/** 하우스(바바) 강점 상세 */
export interface BhavaStrength {
  house: number;        // 1~12
  lord_score: number;   // 하우스 주인 행성의 힘
  dig_score: number;    // 방위 힘 (Dig Bala)
  drishti_score: number; // 행성 시선의 영향
  total_score: number;
}

// ── 행성 힘 (Vimshopaka / Karakas / Aspects) ──────

/** 빔쇼파카 발라 (행성 종합 힘 20점 만점) */
export interface VimshopakaScore {
  shadvarga_score: number;       // 6분할 기준 점수 (0~20)
  shodashavarga_score: number;   // 16분할 기준 점수 (0~20)
  details: [string, number][];   // [분할차트명, 점수] 쌍
}

/** 제미니 카라카 역할 */
export type JaiminiKarakaRole =
  | "Atmakaraka"     // 영혼
  | "Amatyakaraka"   // 직업/대신
  | "Bhratrukaraka"  // 형제
  | "Matrukaraka"    // 어머니
  | "Pitrikaraka"    // 아버지
  | "Putrakaraka"    // 자식
  | "Gnatikaraka"    // 경쟁자/친척
  | "Darakaraka";    // 배우자

/** 카라카 배정 결과 */
export interface KarakaAssignment {
  planet: string;
  role: JaiminiKarakaRole;
  degree_in_rasi: number;
}

/** 행성 시선(Drishti/Aspect) 관계 */
export interface AspectRelation {
  aspecting_planet: string;
  aspected_houses: number[]; // 1~12
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
  aspects: AspectRelation[];
  sav: Sarvashtakavarga;
  bav: BavEntry[];    // 행성별 BAV [Sun~Saturn]
  house_cusps: number[];
  karakas: KarakaAssignment[];
  bhava_strengths: BhavaStrength[];
  vimshopaka_scores: [string, VimshopakaScore][];
  avasthas: { planet: string; baladi: string; jagradadi: string }[];
  panchanga: VedicPanchanga;
  analysis_report: VedicAnalysisReport | null;
}

/** 단일 바르가 낙샤트라 리포트 행 */
export interface VargaNakshatraReportRow {
  planet: string;
  position_str: string;
  sign: number;
  house: number;
  nakshatra: number;
  nakshatra_name: string;
  pada: number;
  pada_range: string;
  nakshatra_lord: string;
  pada_lord: string;
  deity: string;
  purpose: string;
  is_retrograde: boolean;
  is_combust: boolean;
}

/** 단일 바르가(D1/D9/D10/D108) 낙샤트라 리포트 */
export interface VargaNakshatraReport {
  varga_id: string;
  varga_label: string;
  lagna_rasi: number;
  rows: VargaNakshatraReportRow[];
}

/** D1~D144 전체 바르가 낙샤트라 리포트 맵 (varga_id -> report) */
export interface VargaNakshatraReports {
  reports: Record<string, VargaNakshatraReport>;
}

/** 베딕 분석 결과 (VedicAnalysisOutput Rust struct) */
export interface VedicAnalysisResult {
  meta: AnalysisMeta;
  report: VedicAnalysisReport;
  chart: VedicChartData;
  gochara: GocharaSummary;
  varga_nakshatra_reports: VargaNakshatraReports;
}

/** 낙샤트라 원시 데이터 */
export interface NakshatraEntry {
  name: string;
  lord: string;
  deity: string;
  start_deg: number;
  purpose: string;
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
  /** 분할 수 (D3=3, D9=9 등) — 바르가별 낙샤트라 계산용 */
  divisionCount: number;
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

/** 궁합 분석 결과 (사주 + 베딕) */
export interface CompReport {
  saju: CompatibilityAudit;
  vedic: AshtaKutaResult;
}

// ── 고차라 트랜싯 (Gochara) ──────────────────────

/** 행성 트랜싯 위치 (TransitPosition Rust struct) */
export interface TransitPosition {
  planet: string;
  current_rasi: number;
  house_from_moon: number;
  is_benefic_transit: boolean;
  is_blocked: boolean;
  murti: "Gold" | "Silver" | "Copper" | "Iron" | "Unknown";
}

/** 고차라 요약 (GocharaSummary Rust struct) */
export interface GocharaSummary {
  transits: TransitPosition[];
}
