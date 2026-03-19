import { SajuAnalysisResult, TransitResult } from "./saju";
import { VedicAnalysisResult } from "./vedic";

export type RequestStatus = "idle" | "loading" | "success" | "error";

export interface AnalysisTaskState<T> {
  status: RequestStatus;
  data: T | null;
  error: string | null;
}

export interface AiAuditReport {
  meta: AnalysisMeta;
  context_dump: string;
  peak_age: number;
  valley_age: number;
}

export interface AnalysisMeta {
  precision: "Exact" | "UnknownTimeNoonProxy";
  input_time: string;
  corrected_time: string;
  is_dst: boolean;
  dst_offset_hours: number | null;
  analysis_timezone: string;
}

export interface TierGrade {
  grade: string;
  label: string;
  desc: string;
}

export interface ScoreResult {
  score: number;
  highlights: string[];
}

export interface DomainTier {
  house: number;
  domain: string;
  tier: string;
}

export interface TierResult {
  natalScore: number;
  currentScore: number;
  destinyScore: number;
  destinyTier: TierGrade;
  potentialScore: number;
  potentialTier: TierGrade;
  domainTiers: DomainTier[];
  sajuResult: ScoreResult;
  vedicResult: ScoreResult;
  transitResult: ScoreResult;
  strengths: string[];
  weaknesses: string[];
  growthGap: number;
  riskLevel: string;
  profile: string;
  version: string;
}

export interface AnalysisBundleState {
  vedic: AnalysisTaskState<VedicAnalysisResult>;
  saju: AnalysisTaskState<SajuAnalysisResult>;
  transit: AnalysisTaskState<TransitResult>;
  aiAudit: AnalysisTaskState<AiAuditReport>;
  tier: AnalysisTaskState<TierResult>;
}

export type RunAnalysisResult = {
  ok: boolean;
  partial: boolean;
  completed: Array<keyof AnalysisBundleState>;
  failed: Array<keyof AnalysisBundleState>;
};

export interface CompatibilityAuditDto {
  sync_score: number;
  synergies: string[];
  conflicts: string[];
  deadlocks: string[];
  merged_esil_trace: string;
}

export interface VedicCompatibilityResultDto {
  total_score: number;
  varna: number;
  vashya: number;
  tara: number;
  yoni: number;
  maitri: number;
  gana: number;
  bhakoot: number;
  nadi: number;
  message: string;
}

export interface CompatibilityOutput {
  person1_meta: AnalysisMeta;
  person2_meta: AnalysisMeta;
  saju: CompatibilityAuditDto;
  vedic: VedicCompatibilityResultDto;
}
