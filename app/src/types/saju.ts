// ──────────────────────────────────────────────
// 사주 관련 타입 정의 (Rust 직렬화 기준)
// ──────────────────────────────────────────────

/** Rust enum 문자열 → 한자/한글 정보 */
export interface HanjaHangul {
  hanja: string;
  hangul: string;
}

/** 천간 (Heavenly Stem) 키 */
export type StemKey =
  | "Jia" | "Yi" | "Bing" | "Ding" | "Wu"
  | "Ji" | "Geng" | "Xin" | "Ren" | "Gui";

/** 지지 (Earthly Branch) 키 */
export type BranchKey =
  | "Zi" | "Chou" | "Yin" | "Mao" | "Chen" | "Si"
  | "Wu" | "Wei" | "Shen" | "You" | "Xu" | "Hai";

/** 오행 키 */
export type ElementKey = "Wood" | "Fire" | "Earth" | "Metal" | "Water";

/** 신강/신약/중화 */
export type StrengthTypeKey = "Strong" | "Weak" | "Balanced";

/** 간지 객체 (GanZi Rust struct) */
export interface GanZi {
  stem: string;   // HeavenlyStem 변형명: "Jia" | "Yi" | ...
  branch: string; // EarthlyBranch 변형명: "Zi" | "Chou" | ...
}

/** 주주(柱) 위치 */
export type PillarPosition = "Year" | "Month" | "Day" | "Hour";

// ── 사주 원국 ────────────────────────────────

/** 사주 사주(四柱) */
export interface FourPillars {
  year: GanZi;
  month: GanZi;
  day: GanZi;
  hour: GanZi;
}

// ── 신강/신약 분석 ────────────────────────────

/** 득령/득지/득시 결과 */
export interface DeukBase {
  acquired: boolean;
}

/** 득세(得勢) 결과 — 세력 분포 포함 */
export interface DeukSe extends DeukBase {
  bijie_count: number;
  yinxing_count: number;
  shishang_count: number;
  caisheng_count: number;
  guanxing_count: number;
  support_ratio: number;
}

/** 신강/신약 분석 (StrengthAnalysis Rust struct) */
export interface StrengthAnalysis {
  day_master: string;         // HeavenlyStem 변형명
  strength_type: StrengthTypeKey;
  deuk_ryeong: DeukBase;
  deuk_ji: DeukBase;
  deuk_si: DeukBase;
  deuk_se: DeukSe;
  acquired_count: number;
  strength_score: number;
}

// ── 용신 분석 ────────────────────────────────

/** 용신 추천 항목 (RecommendedYongshin Rust struct) */
export interface YongshinRecommendation {
  yongshin_type: string; // "Eokbu" | "Johu" | "Tonggwan" | "Byeongyak"
  element: string;       // Element 변형명
  summary: string;
  description: string;
  reasons: string[];
}

/** 용신 분석 결과 (YongshinAnalysis Rust struct) */
export interface YongshinAnalysis {
  recommendations: YongshinRecommendation[];
  primary: string;    // Element 변형명: "Wood" | "Fire" | ...
  assistant: string;  // Element 변형명
}

// ── 신살 ─────────────────────────────────────

/** 발견된 신살 항목 (FoundMarker Rust struct) */
export interface FoundMarker {
  marker: string;   // SpiritMarker 변형명
  position: string; // PillarPosition 변형명
  is_stem: boolean;
}

/** 신살 상세 정보 (Explainable Spirit Marker) */
export interface SpiritMarkerDetail {
  marker: string;   // SpiritMarker 변형명
  position: string; // PillarPosition 변형명
  is_stem: boolean;
  level: InterpretationLevel;
  summary: string;
  description: string;
  reasons: string[];
}

/** 신살 분석 (SpiritMarkerAnalysis Rust struct) */
export interface SpiritMarkerAnalysis {
  /** 발견된 모든 신살 상세 정보 */
  mapped_markers: SpiritMarkerDetail[];
  /** 발견된 모든 신살 (레거시 호환용) */
  markers: FoundMarker[];
  /** 길신 목록 */
  auspicious: string[];
  /** 흉살 목록 */
  inauspicious: string[];
  /** 보조 기둥 관련 신살 [기둥명, 기준, 신살명] */
  aux_shinsals?: [string, string, string][];
}

// ── 격국 ─────────────────────────────────────

/** 격국 분석 (StructureAnalysis Rust struct) */
export interface StructureAnalysis {
  structure: string;
  projected_stem: string | null;
  projection_path: string | null;
  summary: string;
  description: string;
  reasons: string[];
}

// ── 대운 ─────────────────────────────────────

/** 단일 대운 항목 (MajorLuck Rust struct) */
export interface MajorLuck {
  ganzi: GanZi;
  start_age: number;
  end_age: number;
  stem_god: string;   // TenGod 변형명
  branch_god: string; // TenGod 변형명
  start_date: string; // ISO 8601 UTC 문자열
}

/** 대운 분석 (MajorLuckAnalysis Rust struct) */
export interface MajorLuckAnalysis {
  direction: string; // "Forward" | "Reverse"
  start_age: number;
  start_months: number;
  start_days: number;
  start_date: string;
  cycles: MajorLuck[];
  day_master: string;
}

import type { AnalysisMeta } from "./analysis";

// ── 골든타임 / 시뮬레이션 ─────────────────────

/** 골든타임 (GoldenTime Rust struct) */
export interface GoldenTime {
  start_age: number;
  end_age: number;
  average_score: number;
  description: string;
}

/** 연도별 다차원 시계열 점수 (YearlyScore Rust struct) */
export interface YearlyScore {
  year: number;
  age: number;
  total_score: number;
  wealth_score: number;
  career_score: number;
  academic_score: number;
  health_score: number;
  volatility_index: number;
  is_transition_period: boolean;
  trend_ma_5yr: number | null;
}

/** 오행 에너지 레지스터 */
export interface QiRegisters {
  r0_wood: number;
  r1_fire: number;
  r2_earth: number;
  r3_metal: number;
  r4_water: number;
}

/** 연도별 시뮬레이션 프레임 (LifeFrame Rust struct) */
export interface LifeFrame {
  age: number;
  ganzi: GanZi;
  major_ganzi: GanZi;
  score: number;
  tags: string[];       // TraceTag 변형명 배열
  esil_trace: string;
  register_state: QiRegisters;
}

// ── 심화 분석 (고급 통계/AI/물리) ─────────────────

/** 운명 복잡도 등급 */
export type ObfuscationLevel = "단순형" | "보통형" | "복합형" | "복잡형";

/** 운명 복잡도 지수 (EntropyAnalysis Rust struct) */
export interface EntropyAnalysis {
  score: number;
  level: ObfuscationLevel;
  description: string;
  unpacker_element: string | null; // Element 변형명
}

/** 오행 위상 노드 (QiNode Rust struct) */
export interface QiNode {
  element: string; // Element 변형명 (또는 { english: string, hangul: string } 등)
  capacity: number;
  output: number;
}

/** 운명 에너지 흐름 (TopologyAnalysis Rust struct) */
export interface TopologyAnalysis {
  nodes: QiNode[];
  throughput: number;
  bottleneck: string | null; // Element 변형명
}

/** 린트 심각도 */
export type LintSeverity = "Error" | "Warning" | "Info";

/** 사주 체크업 진단 (SajuLint Rust struct) */
export interface SajuLint {
  code: string;
  severity: LintSeverity;
  message: string;
  advice: string;
}

/** 십성 분석 (TenGodAnalysis Rust struct) */
export interface TenGodAnalysis {
  day_master: string;
  year_stem: string;
  month_stem: string;
  day_stem: string;
  hour_stem: string;
  year_branch: string;
  month_branch: string;
  day_branch: string;
  hour_branch: string;
}

export interface AnalysisOptions {
  apply_transform: boolean;
  apply_correction: boolean;
}

/** 정밀 분석 엔진 결과 (IntegratedAnalysis Rust struct) */
export interface IntegratedAnalysis {
  options: AnalysisOptions;
  element_scores: [string, number, number][]; // [Element, Percentage, Score]
  ten_god_scores: [string, number, number][]; // [TenGod, Percentage, Score]
  dominant_element: string;
  dominant_ten_god: string;
}

// ── 보조 기둥 (태원/명궁/신궁) ─────────────────

/** 보조 기둥 메타 정보 */
export interface SupplementaryPillarsMeta {
  formula_version: string;
  formula_name: string;
  hour_sensitive: boolean;
}

/** 보조 기둥 해석 레벨 */
export type InterpretationLevel = "Auspicious" | "Caution" | "Neutral";

/** 보조 기둥 해석 상세 */
export interface SupplementaryInterpretation {
  pillar_name: string;
  level: InterpretationLevel;
  summary: string;
  description: string;
  reasons: string[];
}

/** 보조 기둥 분석 결과 (SupplementaryPillars Rust struct) */
export interface SupplementaryPillars {
  taewon: GanZi;
  myeonggung: GanZi;
  shingung: GanZi;
  meta: SupplementaryPillarsMeta;
  interpretations: SupplementaryInterpretation[];
}

// ── 핵심 사주 리포트 ─────────────────────────

/** 사주 분석 내부 리포트 (SajuReport Rust struct) */
export interface SajuReport {
  pillars: FourPillars;
  strength: StrengthAnalysis;
  yongshin: YongshinAnalysis;
  spirit_markers: SpiritMarkerAnalysis;
  structure: StructureAnalysis;
  major_luck: MajorLuckAnalysis | null;
  golden_time: GoldenTime | null;
  vm_summary: string | null;
  timeline: YearlyScore[];
  simulation_frames: LifeFrame[];
  ten_gods: TenGodAnalysis;
  power: IntegratedAnalysis;
  voids?: any;
  relationships?: any;
  supplementary_pillars: SupplementaryPillars;
}

/** 주의 시점 재현 벡터 (LuckVector Rust struct) */
export interface LuckVector {
  major: GanZi;
  yearly: GanZi;
  monthly: GanZi | null;
  daily: GanZi | null;
  hourly: GanZi | null;
}

/** 개별 주의 시점 (Vulnerability Rust struct) */
export interface Vulnerability {
  crash_score: number;
  vector: LuckVector;
  vulnerability_type: string;
  tags: string[];
  timestamp: string | null;
}

/** 주의 시점 리포트 (VulnerabilityReport Rust struct) */
export interface VulnerabilityReport {
  total_crashes: number;
  critical_vectors: Vulnerability[];
}

/** 운명 복잡도 분석 (ComplexityAnalysis Rust struct) */
export interface ComplexityAnalysis {
  cyclomatic_complexity: number;
  stability_grade: string;
  entropy: number;
  decision_nodes: number[];
}

/** 사주 분석 결과 최상위 래퍼 (SajuAnalysisOutput Rust struct) */
export interface SajuAnalysisResult {
  meta: AnalysisMeta;
  report: SajuReport;
  lints: SajuLint[];
  entropy: EntropyAnalysis | null;
  qi_topology: TopologyAnalysis | null;
  load_diagnostics: LoadBalanceDiagnostic[];
  crash_count: number;
  vulnerability_report: VulnerabilityReport | null;
  complexity?: ComplexityAnalysis | null;
  /** 합충형해 분석 (RelationshipAnalysis) */
  relationships?: unknown;
  /** 공망 분석 (VoidAnalysis) */
  void_analysis?: unknown;
}

/** 부하 진단 항목 (LoadBalanceDiagnostic Rust struct) */
export interface LoadBalanceDiagnostic {
  age: number;
  status: string;  // "SystemDown" | "Overloaded" | "Normal"
  reason: string;
  strategy: string;
}

// ── 운세(세운/월운) ───────────────────────────

/** 나타나는 운세 흐름 */
export interface LuckInfluence {
  relations_with_natal: string[];
}

/** 세운 (YearlyLuck Rust struct) */
export interface YearlyLuck {
  year: number;
  ganzi: GanZi;
  stem_god: string;
  branch_god: string;
  influence: LuckInfluence | null;
  special_events: string[];
  twelve_stage: string | null;
}

/** 월운 (MonthlyLuck Rust struct) */
export interface MonthlyLuck {
  year: number;
  month: number;
  ganzi: GanZi;
  stem_god: string;
  branch_god: string;
  influence: LuckInfluence | null;
  special_events: string[];
  twelve_stage: string | null;
}

/** 일운 (DailyLuckDto Rust struct) */
export interface DailyLuck {
  year: number;
  month: number;
  day: number;
  ganzi: GanZi;
  stem_god: string;
  branch_god: string;
  influence: LuckInfluence | null;
  special_events: string[];
  twelve_stage: string | null;
}

/** 시운 (HourlyLuck Rust struct) */
export interface HourlyLuck {
  year: number;
  month: number;
  day: number;
  hour: number;
  ganzi: GanZi;
  stem_god: string;
  branch_god: string;
  influence: LuckInfluence | null;
  special_events: string[];
  twelve_stage: string | null;
}

/** 운세 분석 결과 (TransitAnalysisOutput Rust struct) */
export interface TransitResult {
  meta: AnalysisMeta;
  yearly_luck: YearlyLuck;
  monthly_luck: MonthlyLuck;
  monthly_lucks: MonthlyLuck[];
  daily_luck: DailyLuck;
  hourly_luck: HourlyLuck;
  current_age: number;
  current_frame: LifeFrame | null;
  nearby_diagnostics: LoadBalanceDiagnostic[];
}

