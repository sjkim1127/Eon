import type { AnalysisMeta } from "./analysis";

/** 요가 품질 */
export type YogaQuality = "VeryHigh" | "High" | "Medium" | { Weak: string };

/** 요가 */
export interface Yoga {
  name: string;
  yogaType: string;
  description: string;
  planetsInvolved: string[];
  quality: YogaQuality;
}

/** 하우스 요약 */
export interface HouseSummary {
  house: number;
  rating: string;
  totalScore: number;
  summary: string;
  description: string;
  reasons: string[];
}

/** 다샤 기간 (DashaPeriod Rust struct) */
export interface DashaPeriod {
  type: "planet";
  lord: string;
  startTime: string; // ISO 8601 UTC
  endTime: string;   // ISO 8601 UTC
  subDashas: DashaPeriod[];
  name?: string;      // For Yogini Dasha names like "Mangala"
}

/** Jaimini Sign Dasha Period */
export interface SignDashaPeriod {
  type: "sign";
  rasi: number;
  startTime: string;
  endTime: string;
}

/** 사함 (Sensitive Points in Annual Chart) */
export interface Saham {
  name: string;
  longitude: number;
  rasi: number;
}

/** 분할 차트 해석 결과 (Varga Interpretation) */
export interface VargaInterpretation {
  planet: string;
  isVargottama: boolean;
  isPushkarNavamsa: boolean;
  d9Rasi: number;
  d10Rasi: number;
  d60Rasi: number;
  summary: string;
  description: string;
  reasons: string[];
}

/** 타지카 연간 분석 리포트 (TajikaReport Rust struct) */
export interface TajikaReport {
  yearLord: string | null;
  munthaRasi: number;
  sahams: Saham[];
  harshaBalaSummary: [string, number][];
  summary: string;
}

/** 베딕 분석 리포트 (VedicAnalysisReport Rust struct) */
export interface VedicAnalysisReport {
  primaryKarakas: {
    atmakaraka: string;
    amatyakaraka: string;
    bhratrukaraka: string | null;
    matrukaraka: string | null;
    pitrikaraka: string | null;
    putrakaraka: string | null;
    gnatikaraka: string | null;
    darakaraka: string;
  };
  houseSummary: HouseSummary[];
  dashaFocus: string;
  dashaTimeline: DashaPeriod[];
  yoginiTimeline: DashaPeriod[];
  charaDashaTimeline: SignDashaPeriod[];
  allKarakas: KarakaAssignment[];
  nakshatraInfo: string;
  overallStrengthScore: number;
  sadeSati: "None" | "Rising" | "Peak" | "Setting";
  yogas: Yoga[];

  // Advanced Metrics
  arudhaLagna: number;
  upapadaLagna: number;
  specialLagnasSummary: [string, number][];

  // Varga Integration
  vargaInterpretations?: VargaInterpretation[];
  d9MarriageAnalysis?: string;
  d10CareerAnalysis?: string;
}

// ── 하우스 점수 (SAV / Bhava) ─────────────────────

/** 하우스별 강도 지표 (BhavaStrength Rust struct) */
export interface BhavaStrength {
  house: number;
  totalScore: number;
  relativeStrength: number;
  rating: string;
  lordScore: number;
  occupantScore: number;
  aspectScore: number;
}

/** 슈타카바르가 점수 (VimshopakaScore Rust struct) */
export interface VimshopakaScore {
  shadvargaScore: number;
  shodashavargaScore: number;
}

/** 아슈타카바르가 항목 (BAV/SAV Entry) */
export interface BavEntry {
  planet: string;
  points: number[];           // 원시 점수
  trikonaPoints: number[];    // 트리코나 쇼다나 후
  shodhanaPoints: number[];   // 에카디파탸 쇼다나 후
  sodyaPinda: number;         // 최종 핀다 점수
}

/** 사르바아슈타카바르가 (SAV) */
export interface Sarvashtakavarga {
  points: number[]; // 12개 하우스 합산 점수
}

// ── 자이미니 (Jaimini) ─────────────────────────

/** 자이미니 카라카 역할 */
export type JaiminiKarakaRole =
  | "Atmakaraka"    // 영혼
  | "Amatyakaraka"   // 직업/조언자
  | "Bhratrukaraka"  // 형제/동료
  | "Matrukaraka"    // 어머니
  | "Pitrikaraka"    // 아버지
  | "Putrakaraka"    // 자녀
  | "Gnatikaraka"    // 경쟁자
  | "Darakaraka";    // 배우자

/** 카라카 배정 결과 */
export interface KarakaAssignment {
  planet: string;
  role: JaiminiKarakaRole;
  degreeInRasi: number;
}

/** 아루다 파다 (Arudha Pada) */
export interface ArudhaPada {
  house: number;
  rasi: number;
  name: string;
}

/** 특별 라그나 (Special Lagna) */
export interface SpecialLagna {
  name: string;
  longitude: number;
  rasi: number;
}

/** 행성 시선(Drishti/Aspect) 관계 */
export interface AspectRelation {
  aspectingPlanet: string;
  aspectedHouses: number[]; // 1~12
}

// ── VedicChart 원시 데이터 ────────────────────

/** 판창가 (Panchanga Rust struct) */
export interface VedicPanchanga {
  vara: string;
  tithi: number;
  tithiName: string;
  nakshatra: number;
  yoga: number;
  karanaName: string;
  dayLord: string;
  hourLord: string;
  isDayBirth: boolean;
}

/** 행성 위치 (VedicPosition Rust struct) — 바르가 필드 포함 */
export interface VedicPosition {
  planet: string;
  tropicalDeg: number;
  siderealDeg: number;
  nakshatra: number;
  pada: number;
  rasi: number;
  houseIndex: number;
  speed: number;
  isRetrograde: boolean;
  isCombust: boolean;
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
  houseCusps: number[];
  karakas: KarakaAssignment[];
  arudhaPadas: ArudhaPada[];
  specialLagnas: SpecialLagna[];
  bhavaStrengths: BhavaStrength[];
  vimshopakaScores: [string, VimshopakaScore][];
  avasthas: { planet: string; baladi: string; jagradadi: string; deeptaadi: string }[];
  panchanga: VedicPanchanga;
  analysisReport: VedicAnalysisReport | null;
}

/** 단일 바르가 낙샤트라 리포트 행 */
export interface VargaNakshatraReportRow {
  planet: string;
  positionStr: string;
  sign: number;
  house: number;
  nakshatra: number;
  nakshatraName: string;
  pada: number;
  padaRange: string;
  nakshatraLord: string;
  padaLord: string;
  deity: string;
  purpose: string;
  isRetrograde: boolean;
  isCombust: boolean;
}

/** 단일 바르가(D1/D9/D10/D108) 낙샤트라 리포트 */
export interface VargaNakshatraReport {
  vargaId: string;
  vargaLabel: string;
  lagnaRasi: number;
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
  tajikaReport?: TajikaReport | null;
  chart: VedicChartData;
  annualChart: VedicChartData | null;
  gochara: GocharaSummary;
  vargaNakshatraReports: VargaNakshatraReports;
}

/** 낙샤트라 원시 데이터 */
export interface NakshatraEntry {
  name: string;
  lord: string;
  deity: string;
  startDeg: number;
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

// ── 고차라 트랜싯 (Gochara) ──────────────────────

/** 행성 트랜싯 위치 (TransitPosition Rust struct) */
export interface TransitPosition {
  planet: string;
  currentRasi: number;
  houseFromMoon: number;
  isBeneficTransit: boolean;
  isBlocked: boolean;
  murti: "Gold" | "Silver" | "Copper" | "Iron" | "Unknown";
  summary: string;
  description: string;
  reasons: string[];
}

/** 고차라 요약 (GocharaSummary Rust struct) */
export interface GocharaSummary {
  transits: TransitPosition[];
  sadeSati: "None" | "Rising" | "Peak" | "Setting";
}
