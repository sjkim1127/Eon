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
